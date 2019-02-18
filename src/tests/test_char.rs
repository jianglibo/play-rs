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
pub fn code_point_to_gb18030(cp: u32) -> Vec<u8> {
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
pub fn gb18030_to_code_point<T: AsRef<[u8]>>(gb4: T) -> Option<u32> {
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

#[cfg(test)]
mod tests {
    use crate::tests::tutil::init_log;
    use std::thread;
    use std::str::FromStr;
    use super::*;
    // use std::convert::TryFrom;

    // scan stop on None.
    #[test]
    fn test_char_len() {
        init_log();
        let v = vec!['h', 'e', 'l', 'l', 'o'];

        // five elements times four bytes for each element
        assert_eq!(20, v.len() * std::mem::size_of::<char>());

        let s = String::from("hello");

        // five elements times one byte per element
        assert_eq!(5, s.len() * std::mem::size_of::<u8>());

        let mut chars = "é".chars();
        // U+00e9: 'latin small letter e with acute'
        assert_eq!(Some('\u{00e9}'), chars.next());
        assert_eq!(None, chars.next());

        let mut chars = "é".chars();
        // U+0065: 'latin small letter e'
        assert_eq!(Some('\u{0065}'), chars.next());
        // U+0301: 'combining acute accent'
        assert_eq!(Some('\u{0301}'), chars.next());
        assert_eq!(None, chars.next());
        // 台湾, utf8: E5 8F B0 E6 B9 BE,
        println!("{}", '\u{50a9}');

        let c = char::from(90);
        assert_eq!(c, 'Z');

        let c = char::from_str("Z");
        assert_eq!(c.unwrap(), 'Z');

        let c = char::from_str("Z1");
        assert!(c.is_err());

        let v = vec![0xE5, 0x8F, 0xB0, 0xE6, 0xB9, 0xBE];
        let s = String::from_utf8(v).unwrap();
        assert_eq!(s, "台湾");

        let u = "0x00dD".parse::<usize>();
        assert_eq!(u.unwrap(), 55);
        // let c = char::try_from(90u32);
    }

    #[test]
    fn test_encode_utf8() {
        let result = thread::spawn(|| {
            let mut b = [0; 1];

            // this panics
            'ß'.encode_utf8(&mut b);
        })
        .join();

        assert!(result.is_err());
    }

    // http://site.icu-project.org/
    // http://icu-project.org/repos/icu/data/trunk/charset/source/gb18030/
    // http://www.voidcn.com/article/p-gmchyris-za.html
    // https://en.wikipedia.org/wiki/Code_point
    // https://stackoverflow.com/questions/6240055/manually-converting-unicode-codepoints-into-utf-8-and-utf-16
    // https://www.ibm.com/developerworks/cn/linux/i18n/gb18030/
    // http://www.fmddlmyy.cn/text30.html

    #[test]
    fn test_from_utf8() {
        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];

        // We know these bytes are valid, so we'll use `unwrap()`.
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

        assert_eq!("💖", sparkle_heart);

        let a_u8 = vec![90];
        let a = String::from_utf8(a_u8).unwrap();
        assert_eq!("Z", a);
    }


    #[test]
    fn test_gb18030_to_cp() {
        let uc = 0x20000u32;
        assert_eq!(code_point_to_gb18030(uc), [0x95, 0x32, 0x82, 0x36]);

        assert_eq!(gb18030_to_code_point([0x95u8, 0x32u8, 0x82u8, 0x36u8]), Some(0x20000));
    }
}
