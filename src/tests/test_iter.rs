
#[cfg(test)]
mod tests {
    use crate::code_util::{find_pair, TripleOption};
    use crate::tests::tutil::init_log;

    pub fn get_pair(a_slice: &[u8]) -> Vec<u8> {
        info!("-------------------**---------------------------");
        a_slice
            .iter()
            .scan((None, None, None), |state: &mut TripleOption, it| {
                Some(find_pair(state, *it))
            })
            .inspect(|x| match x {
                Ok(_) => (),
                Err(er) => info!("{:?}", er),
            })
            .filter(Result::is_ok)
            .map(|x| {
                // we can unwrap here, because all errors had filter outed.
                x.unwrap()
            })
            .collect()
    }

    // scan stop on None.
    #[test]
    fn test_scan() {
        init_log();

        let _s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        let s = r"\xce\xde";
        let v_slice = s.as_bytes();
        let u8_pair = get_pair(v_slice);

        assert_eq!(u8_pair.len(), 2);

        assert_eq!(u8_pair[0], 206);

        assert_eq!(u8_pair[1], 222);
        // assert_eq!(u8_pair[0], Ok((92, 101)));

        let s = r"ce\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

        let s = r"ce\xce\xde1234";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

        // this will happen.
        let s = r"cxe\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);
    }
}
