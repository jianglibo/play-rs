use crate::tests::test_iter::aton_1;

const three_1_post_u32: u32 = 0b0000_0111;
const four_1_post_u32: u32 = 0b0000_1111;
const five_1_post_u32: u32 = 0b0001_1111;
const six_1_post_u32: u32 = 0b0011_1111;

const one_1_pre_u8: u8 = 0b10_000000;
const two_1_pre_u8: u8 = 0b110_00000;
const three_1_pre_u8: u8 = 0b1110_0000;
const four_1_pre_u8: u8 = 0b1111_0000;

const six_1_post_u8: u8 = 0b0011_1111;
const five_1_post_u8: u8 = 0b0001_1111;
const four_1_post_u8: u8 = 0b0000_1111;
const three_1_post_u8: u8 = 0b0000_0111;

pub enum BitNum {
    SIX,
    FIVE,
    FOUR,
    THREE,
}

pub fn parse_code_point(hex: &[u8]) -> Option<u32> {
    match hex.len() {
        4 => {
            if let [Some(a), Some(b), Some(c), Some(d)] = [
                aton_1(hex[0]),
                aton_1(hex[1]),
                aton_1(hex[2]),
                aton_1(hex[3]),
            ] {
                let v: u32 =
                    ((a as u32) << 12) + ((b as u32) << 8) + ((c as u32) << 4) + (d as u32);
                Some(v)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn utf8_to_codepoint(utf8: &[u8]) -> Option<u32> {
    match utf8.len() {
        l if l > 1 && l < 5 => {
            let n0 = utf8[0];
            if (n0 >> 3) == 0b1111_0 && l == 4 {
                // 11110xxx 10xx'xxxx 10xxxx'xx 10xxxxxx
                let n4 = utf8[3] & six_1_post_u8;
                let n3 = utf8[2] & six_1_post_u8;
                let n2 = utf8[1] & six_1_post_u8;
                let n1 = utf8[0] & three_1_post_u8;

                let n4v = (n4 | (n3 << 6)) as u32;
                let n3v = (((n3 >> 2) | (n2 << 4)) as u32) << 8;
                let n2v = (((n2 >> 4) | (n1 << 2)) as u32) << 16;
                Some(n4v + n3v + n2v)
            } else if (n0 >> 4) == 0b1110 && l == 3 {
                // 1110xxxx 10xxxx'xx 10xxxxxx
                let n3 = utf8[2] & six_1_post_u8;
                let n2 = utf8[1] & six_1_post_u8;
                let n1 = utf8[0] & four_1_post_u8;

                let n3v = (n3 | (n2 << 6)) as u32;
                let n2v = (((n2 >> 2) | (n1 << 4)) as u32) << 8;
                Some(n3v + n2v)
            } else if (n0 >> 5) == 0b110 && l == 2 {
                // 110xxx'xx 10xxxxxx
                let n2 = utf8[1] & six_1_post_u8;
                let n1 = utf8[0] & five_1_post_u8;

                let n2v = (n2 | n1 << 6) as u32;
                let n1v = ((n1 >> 2) as u32) << 8;
                Some(n2v + n1v)
            } else if (n0 >> 7) == 0b0 && l == 1 {
                Some(n0 as u32)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn codepoint_to_utf8(cp: u32) -> Option<Vec<u8>> {
    match cp {
        0x00000000...0x0000007F => Some(vec![cp as u8]),
        // 110xxxxx 10xxxxxx
        0x00000080...0x000007FF => {
            let u2 = get_bits(cp, BitNum::SIX);
            let u1 = get_bits(cp >> 6, BitNum::FIVE);
            Some(vec![u1, u2])
        }
        // 1110xxxx 10xxxxxx 10xxxxxx
        0x00000800...0x0000FFFF => {
            let u3 = get_bits(cp, BitNum::SIX);
            let u2 = get_bits(cp >> 6, BitNum::SIX);
            let u1 = get_bits(cp >> 12, BitNum::FOUR);
            Some(vec![u1, u2, u3])
        }
        // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
        0x00010000...0x001FFFFF => {
            let u4 = get_bits(cp, BitNum::SIX);
            let u3 = get_bits(cp >> 6, BitNum::SIX);
            let u2 = get_bits(cp >> 12, BitNum::SIX);
            let u1 = get_bits(cp >> 18, BitNum::THREE);
            Some(vec![u1, u2, u3, u4])
        }
        _ => None,
    }
}

fn get_bits(i: u32, bn: BitNum) -> u8 {
    match bn {
        BitNum::THREE => (i & three_1_post_u32) as u8 | four_1_pre_u8,
        BitNum::FOUR => (i & four_1_post_u32) as u8 | three_1_pre_u8,
        BitNum::FIVE => (i & five_1_post_u32) as u8 | two_1_pre_u8,
        BitNum::SIX => (i & six_1_post_u32) as u8 | one_1_pre_u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tutil::init_log;
    // note: Non-UTF-8 output: LINK : fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0sqlite3.lib\xa1\xb1\r\n
    #[test]
    fn test_parse_hex_string() {
        // 30DD -> 12509
        assert_eq!(0x30DD, 12509);

        let c = 'c';
        let u = u32::from(c);
        let uu: u32 = c.into();
        assert_eq!(uu, u);
        assert!(4 == std::mem::size_of_val(&u));

        let i = parse_code_point("30DD".as_bytes()).unwrap();
        assert_eq!(i, 12509);

        let i = parse_code_point("30DD".as_bytes()).unwrap();
        assert_eq!(i, 12509);

        assert_eq!(format!("{:X}", i), "30DD");
    }

    #[test]
    fn test_bits() {
        init_log();
        let i = 1u32;
        let i1 = i << 4;
        assert_eq!(i, 1);
        assert_eq!(i1, 16);
        info!("{:#b}", i1);

        // let i = 16u32;
        // assert_eq!(i >> 2, 4);
        // 10000000‬ -> 0x80 //0x7FF
        let i = 0b0111_1111_1111;

        // take last 6 bits.
        let i_last6 = i & 0b111111;
        assert_eq!(i_last6, 0b111111);

        // preappend 10
        let i1 = 0b1000_0000 | i_last6;
        assert_eq!(i1, 0b1011_1111);

        // shift right 6 bits
        let i_remain = i >> 6;
        assert_eq!(i_remain, 0b11111);
        //take last 5 bites
        let i1 = i_remain & 0b11111;
        assert_eq!(i1, 0b11111);

        let i1 = 0b1100_0000 | i1;
        assert_eq!(i1, 0b1101_1111);

        assert_eq!(
            codepoint_to_utf8(parse_code_point("4E3E".as_bytes()).unwrap()).unwrap(),
            [0xE4, 0xB8, 0xBE]
        );

        let r = utf8_to_codepoint(&[0xE4, 0xB8, 0xBE]);
        assert_eq!(r, Some(0x4E3E));

        let tai = &[0xE5, 0x8F, 0xB0];
        let r = "台".as_bytes();
        assert_eq!(r, tai);
    }

}
