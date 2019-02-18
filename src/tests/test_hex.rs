use crate::tests::test_iter::aton_1;

const THREE_1_POST_U32: u32 = 0b0000_0111;
const FOUR_1_POST_U32: u32 = 0b0000_1111;
const FIVE_1_POST_U32: u32 = 0b0001_1111;
const SIX_1_POST_U32: u32 = 0b0011_1111;

const ONE_1_PRE_U8: u8 = 0b10_000000;
const TWO_1_PRE_U8: u8 = 0b110_00000;
const THREE_1_PRE_U8: u8 = 0b1110_0000;
const FOUR_1_PRE_U8: u8 = 0b1111_0000;

const SIX_1_POST_U8: u8 = 0b0011_1111;
const FIVE_1_POST_U8: u8 = 0b0001_1111;
const FOUR_1_POST_U8: u8 = 0b0000_1111;
const THREE_1_POST_U8: u8 = 0b0000_0111;

pub enum BitNum {
    SIX,
    FIVE,
    FOUR,
    THREE,
}

pub fn hex_str_to_u32<T: AsRef<[u8]>>(hex_as: T) -> Option<u32> {
    let hex = hex_as.as_ref();
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

pub fn utf8_to_codepoint<T: AsRef<[u8]>>(utf8_as: T) -> Option<u32> {
    let utf8 = utf8_as.as_ref();
    match utf8.len() {
        l if l > 1 && l < 5 => {
            let n0 = utf8[0];
            if (n0 >> 3) == 0b1111_0 && l == 4 {
                // 11110xxx 10xx'xxxx 10xxxx'xx 10xxxxxx
                let n4 = utf8[3] & SIX_1_POST_U8;
                let n3 = utf8[2] & SIX_1_POST_U8;
                let n2 = utf8[1] & SIX_1_POST_U8;
                let n1 = utf8[0] & THREE_1_POST_U8;

                let n4v = (n4 | (n3 << 6)) as u32;
                let n3v = (((n3 >> 2) | (n2 << 4)) as u32) << 8;
                let n2v = (((n2 >> 4) | (n1 << 2)) as u32) << 16;
                Some(n4v + n3v + n2v)
            } else if (n0 >> 4) == 0b1110 && l == 3 {
                // 1110xxxx 10xxxx'xx 10xxxxxx
                let n3 = utf8[2] & SIX_1_POST_U8;
                let n2 = utf8[1] & SIX_1_POST_U8;
                let n1 = utf8[0] & FOUR_1_POST_U8;

                let n3v = (n3 | (n2 << 6)) as u32;
                let n2v = (((n2 >> 2) | (n1 << 4)) as u32) << 8;
                Some(n3v + n2v)
            } else if (n0 >> 5) == 0b110 && l == 2 {
                // 110xxx'xx 10xxxxxx
                let n2 = utf8[1] & SIX_1_POST_U8;
                let n1 = utf8[0] & FIVE_1_POST_U8;

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
        BitNum::THREE => (i & THREE_1_POST_U32) as u8 | FOUR_1_PRE_U8,
        BitNum::FOUR => (i & FOUR_1_POST_U32) as u8 | THREE_1_PRE_U8,
        BitNum::FIVE => (i & FIVE_1_POST_U32) as u8 | TWO_1_PRE_U8,
        BitNum::SIX => (i & SIX_1_POST_U32) as u8 | ONE_1_PRE_U8,
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

        let i = hex_str_to_u32("30DD".as_bytes()).unwrap();
        assert_eq!(i, 12509);

        let i = hex_str_to_u32("30DD".as_bytes()).unwrap();
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
            codepoint_to_utf8(hex_str_to_u32("4E3E".as_bytes()).unwrap()).unwrap(),
            [0xE4, 0xB8, 0xBE]
        );

        let r = utf8_to_codepoint(&[0xE4, 0xB8, 0xBE]);
        assert_eq!(r, Some(0x4E3E));

        let tai = [0xE5, 0x8F, 0xB0];
        let r = utf8_to_codepoint(tai).unwrap();

        assert_eq!(std::char::from_u32(r).unwrap(), '台');
    }

    #[test]
    fn test_decode_gb18030() {

    }
}
