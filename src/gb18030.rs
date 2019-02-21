use std::char;
use crate::table::GBK_UNI;
use crate::code_util::get_hex_pairs;
// 从Unicode编码到GB18030编码的映射方法如下：
// U=Unicode编码-0x10000
// m1=U/12600
// n1=U%12600
// m2=n1/1260
// n2=n1%1260
// m3=n2/10
// n3=n2%10
// 第一字节b1=m1+0x90
// 第二字节b2=m2+0x30
// 第三字节b3=m3+0x81
// 第四字节b4=n3+0x30

// pub fn code_point_to_gb18030<T: AsRef<u32>>(cp: T) -> Vec<u8> {
pub fn code_point_to_gb18030_4b(cp: u32) -> Vec<u8> {
    let tmp = cp - 0x1_0000;
    let m1 = (tmp / 12600) as u8;
    let n1 = tmp % 12600;
    let m2 = (n1 / 1260) as u8;
    let n2 = n1 % 1260;
    let m3 = (n2 / 10) as u8;
    let n3 = (n2 % 10) as u8;
    vec![m1 + 0x90, m2 + 0x30, m3 + 0x81, n3 + 0x30]
}



// 从GB18030编码到Unicode编码的映射方法如下：
// 设GB18030编码的四个字节依次为：b1、b2、b3、b4，则
// Unicode编码=0x10000+(b1-0x90)*12600+(b2-0x30)*1260+(b3-0x81)*10+b4-0x30
pub fn gb18030_4b_to_code_point<T: AsRef<[u8]>>(gb4: T) -> Option<u32> {
    let gb4_1 = gb4.as_ref();
    match gb4_1.len() {
        4 => {
            let (b1, b2, b3, b4) = (gb4_1[0],gb4_1[1],gb4_1[2],gb4_1[3]);
            Some(0x10000
                + ((b1-0x90) as u32) * 12600
                + ((b2-0x30) as u32) * 1260
                + ((b3-0x81) as u32) * 10
                + ((b4-0x30) as u32)
            )},
        _ => None
    }
}

// pub fn gb18030_2b_to_code_point<T: AsRef<[u8]>>(gb2: T) -> Option<u32> {
//     let gb2_1 = gb2.as_ref();
//     match gb2_1.len() {
//         2 => {
//             let (b1, b2, b3, b4) = (gb4_1[0],gb4_1[1],gb4_1[2],gb4_1[3]);
//             Some(0x10000
//                 + ((b1-0x90) as u32) * 12600
//                 + ((b2-0x30) as u32) * 1260
//                 + ((b3-0x81) as u32) * 10
//                 + ((b4-0x30) as u32)
//             )},
//         _ => None
//     }
// }

//双字节 (0x81 - 0xfe), (0x40-0x7e, 0x80-0xfe)， 后面仅仅少了一个7f， 所以总数是 （0xfe-0x81 + 1) * (0xfe - 0x40 + 1 - 1) = 7E * BE = 5D84 (23940)
//填写查询表的时候，必须写入第二字节为7F的空行，这样就可以通过 公式 index = (byte 1 - 0x81) × 0xBF + (byte 2 - 0x40) 计算.

type TripleOptionU8 = (Option<u8>, Option<u8>, Option<u8>);

#[derive(Debug)]
pub enum MayBeMatchError {
    Continue,
    Impossible,
    Discard(Vec<u8>),
}

type CharResult = Result<char, MayBeMatchError>;

// https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
// for scan use.
pub fn try_get_char(state: &mut TripleOptionU8, current_byte: u8) -> CharResult {
    match *state { // we didn't move or copy it.
        (None, None, None) => match current_byte {
            0x00...0x7F => Ok(char::from_u32(current_byte as u32).unwrap()), // state keep empty.
            0x80 => Ok(char::from_u32(0x20AC).unwrap()), // state keep empty.
            _ => {
                state.0 = Some(current_byte);
                Err(MayBeMatchError::Continue)
            }
        },
        (Some(b1), None, None) => match b1 { // exam the first byte.
            0x81...0xFE => match current_byte {
                0x40...0x7E | 0x80...0xFE => { // a valid gb18030
                    let low_boundary = if b1 > 0x7E {0x41} else {0x40};
                    let index = u32::from(b1 - 0x81) * 190 + u32::from(current_byte - low_boundary);
                    let cp = GBK_UNI[index as usize];
                    debug!("got index: {}, codepoint: 0x{:X}", index, cp);
                    state.0 = None;
                    Ok(char::from_u32(cp).unwrap())
                },
                0x30...0x39 => { // maybe a four byte gb18030
                    state.1 = Some(current_byte);
                    Err(MayBeMatchError::Continue)
                },
                _ => { // the current byte is not valid. discard it.
                    state.0 = None;
                    Err(MayBeMatchError::Discard(vec![b1]))
                }
            },
            _ => { // the first byte is invalid. discard it.
                state.0 = None;
                Err(MayBeMatchError::Discard(vec![b1]))
            }
        },
        (Some(b1), Some(b2), None) => match current_byte { // b2 is always valid.
            0x81...0xFE => {
                state.2 = Some(current_byte);
                Err(MayBeMatchError::Continue)
            },
            _ => {
                *state = (None, None, None);
                Err(MayBeMatchError::Discard(vec![b1, b2, current_byte]))
            }
        },
        (Some(b1), Some(b2), Some(b3)) => match current_byte {
            0x30...0x39 => {
                *state = (None, None, None);
                let cp = gb18030_4b_to_code_point([b1, b2, b3, current_byte]).unwrap();
                Ok(char::from_u32(cp).unwrap())
            },
            _ => {
                *state = (None, None, None);
                Err(MayBeMatchError::Discard(vec![b1, b2, b3, current_byte]))
            }
        },
        _ => Err(MayBeMatchError::Continue)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tutil::{init_log, get_out_file, get_fixture_file};
    use std::fs::{OpenOptions, File};
    use std::io::{Write, Read};
    use std::time::Instant;

    #[test]
    fn test_decode_gb() {
        init_log();
        // CED2:6211 , index 14776.
        // let index: u32 = ((b1 - 0x81) as u32) * 191 + ((current_byte - low_boundary) as u32);
        // 总共126个区。每个区的长度 0xFE-0x40 190,不是191，中间没有7F。
        let index = ((0xCE - 0x81) as u32) * 190 + ((0xD2 - 0x41) as u32);
        assert_eq!(index, 14775);

        // 816A:4E6F, index: 43
        let index = ((0x81 - 0x81) as u32) * 190 + ((0x6A - 0x40) as u32);
        assert_eq!(index, 42);


        // CE D2 0D 0A B5 C4 B1 E0 C2 EB CA C7 47 42 32 33 31 32 A1 A3
        debug!("我：{}", '我'.escape_unicode());
        let path = get_fixture_file(["gb18030.txt"], true).unwrap();
        let f = File::open(path).unwrap();
        // let bs: &[u8] = &[0xCE, 0xD2, 0x0D, 0x0A, 0xB5, 0xC4, 0xB1, 0xE0, 0xC2, 0xEB, 0xCA, 0xC7, 0x47, 0x42, 0x32, 0x33, 0x31, 0x32, 0xA1, 0xA3];
        let mut trio = (None, None, None);
        let cs: Vec<char> = f.bytes()
            .map(|ob|ob.unwrap())
            .map(|b|try_get_char(&mut trio, b))
            .filter(Result::is_ok)
            .flat_map(|c|c)
            .collect();

        info!("{:?}", cs);

        let mut trio = (None, None, None);
        // let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0";
        let s = r"note: Non-UTF-8 output: LINK : fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0sqlite3.lib\xa1\xb1\r\n";
        let v8 = get_hex_pairs(s.as_bytes());

        let cs: Vec<char> = v8.iter()
            .map(|b|try_get_char(&mut trio, *b))
            .filter(Result::is_ok)
            .flat_map(|c|c)
            .collect();

        let s: String = cs.into_iter().collect();

        info!("{}", s);
    }

    #[test]
    fn t_str() {
        init_log();
        let s = "光与影"; // E5 85 89 E4 B8 8E E5 BD B1 
        assert_eq!(s.len(), 9);
        assert_eq!(s.chars().count(), 3);
        let mut f = OpenOptions::new().write(true).create(true).open(get_out_file(["t_zh.txt"]).unwrap()).unwrap();
        write!(f, "{}", s).unwrap();

        let s = "光と闇"; // E5 85 89 E3 81 A8 E9 97 87 
        let mut f = OpenOptions::new().write(true).create(true).open(get_out_file(["t_jp.txt"]).unwrap()).unwrap();
        write!(f, "{}", s).unwrap();

        let bs = s.as_bytes();
        assert_eq!((bs[0], bs[1], bs[2]), (0xE5, 0x85, 0x89));

        let now = Instant::now();
        for _ in 0..100000 {
            s.as_bytes();
        }
        let elapsed = now.elapsed();
        info!("{:?}", elapsed);

        let c = 'a';
        assert_eq!(4, std::mem::size_of_val(&c));

    }
}