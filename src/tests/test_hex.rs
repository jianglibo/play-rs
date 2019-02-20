#[cfg(test)]
mod tests {
    use crate::code_util::{hex_str_to_u32, codepoint_to_utf8, utf8_to_codepoint};
    use crate::tests::tutil::init_log;
    // note: Non-UTF-8 output: LINK : fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0sqlite3.lib\xa1\xb1\r\n

    #[test]
    fn test_builtin() {
        let s = "C2CC";
        let s1 = s.trim_start_matches("0x");
        assert_eq!(s, s1);
        let i = u32::from_str_radix("C2CC", 16).unwrap();
        let i1 = hex_str_to_u32("C2CC").unwrap();
        assert_eq!(i, i1);

        let v = vec!(1,2,3);
        let _vs = v.as_slice();
        // let (first, ...rest) = (1,2,3);
    }
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
