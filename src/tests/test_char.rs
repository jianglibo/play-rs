#[cfg(test)]
mod tests {
    use crate::tests::tutil::init_log;
    use std::thread;
    use std::str::FromStr;
    use crate::gb18030::{code_point_to_gb18030_4b, gb18030_4b_to_code_point};
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
        assert!(u.is_err());
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
        init_log();
        let uc = 0x20000u32;
        assert_eq!(code_point_to_gb18030_4b(uc), [0x95, 0x32, 0x82, 0x36]);
        assert_eq!(gb18030_4b_to_code_point([0x95u8, 0x32u8, 0x82u8, 0x36u8]), Some(0x20000));

        let c = std::char::from_u32(3056).unwrap();
        let mut buf = [0;4];
        debug!("{}", '台'.encode_utf8(&mut buf));
        debug!("{:?}", buf);
        debug!("{}", c);
        // gb18030 我 CE D2
        // 6211:CED2
        assert_eq!(std::char::from_u32(0x6211), Some('我'));
        let c: u32 = gb18030_4b_to_code_point([0xCE, 0xD2]).unwrap();
        assert_eq!(std::char::from_u32(c), Some('我'));
    }
}
