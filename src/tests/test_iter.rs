#[cfg(test)]
mod tests {

    // scan stop on None.
    #[test]
    fn test_scan() {
        ::std::env::set_var("RUST_LOG", "play_rs=debug");
        env_logger::init();
        type TripleOption = (Option<u8>, Option<u8>, Option<u8>);

        #[derive(Debug)]
        enum MatchError {
            Continue,
            Impossible,
            Discard(Vec<u8>)
        }
        type ScanResult = Result<(u8, u8), MatchError>;

        const BSL: u8 = b'\\';
        const XC: u8 = b'x';

        fn get_pair(a_slice: &[u8]) -> Vec<ScanResult> {
            info!("-------------------**---------------------------");
            a_slice.iter().scan((None, None, None), |state: &mut TripleOption, &it| {
                match state {
                    (None, None, None) => {
                        match it {
                            BSL => {
                                state.0 = Some(BSL);
                                Some(Err(MatchError::Continue))
                            },
                            _ => Some(Err(MatchError::Discard(vec![it]))), // will only assing to tuple if meet \.
                        }
                    },
                    (Some(_), None, None) => {
                        match it {
                            XC => {
                                state.1 = Some(XC);
                                Some(Err(MatchError::Continue))
                            },
                            _ => {
                                *state = (None, None, None); // only if second is 'x'.
                                Some(Err(MatchError::Discard(vec![BSL, it])))
                            }
                        }
                    },
                    // (Some(one), Some(two), None) => {
                    //     match it {
                    //         b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                    //             state.2 = Some(it);
                    //             Some(Err(MatchError::Continue))
                    //         },
                    //         _ => {
                    //             *state = (None, None, None); // if use variable one and two, this line will not work. state already be borrowed.
                    //             Some(Err(MatchError::Discard(vec![*one, *two, it])))
                    //         }
                    //     }
                    // },
                    (Some(_), Some(_), None) => {
                        match it {
                            b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                                state.2 = Some(it);
                                Some(Err(MatchError::Continue))
                            },
                            _ => {
                                *state = (None, None, None); // only if third character is valid.
                                Some(Err(MatchError::Discard(vec![BSL, XC, it])))
                            }
                        }
                    },
                    // last character may be invalid.
                    (Some(BSL), Some(XC), Some(b'0'...b'9')) |
                    (Some(BSL), Some(XC), Some(b'a'...b'f')) |
                    (Some(BSL), Some(XC), Some(b'A'...b'F')) => match it {
                        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                            let tp = (state.0.unwrap(), it);
                            *state = (None, None, None);
                            info!("{:?}", tp);
                            Some(Ok(tp))
                        },
                        _ => {
                            // it value is leaked. cx -> it = g? discard all.
                            let er = Err(MatchError::Discard(vec![BSL, XC, state.2.unwrap(), it]));
                            *state = (None, None, None);
                            Some(er)
                        },
                    },
                    // we got 2 items, but first is leaked, for example it is 'ga', replace state with new pair.
                    _ => {
                        *state = (Some(it), None, None);
                        Some(Err(MatchError::Impossible))
                    },
               }
            }).inspect(|x| {
                match x {
                    Ok(_) => (),
                    Err(er) => info!("{:?}", er),
                }
            }).filter(Result::is_ok).collect()
        }

        let s = r"\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

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

    // You cannot use one iterator two times.
    #[test]
    fn test_first() {

        let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        // let s = r"\xce\xde";
        let v_slice: &[u8] = s.as_bytes();
        let mut first_item = None;
        if let Some((first, _)) = v_slice.split_first() {
            first_item = Some(first);
        }
        // print!("{}", s);
        assert_eq!(first_item, Some(&b'\\'));
        let _bytes_enum = v_slice.iter().enumerate();
        // let mut bytes_it = Rc::new(v_slice.iter().peekable());
        let mut bytes_it = v_slice.iter().peekable();
        let bytes_it_clone = bytes_it.clone();
        // let std::iter::Map<std::iter::Peekable<std::slice::Iter<'_, u8>> = bytes_it.map(|x| x -1);
        let pair_it = bytes_it_clone.map(|&asc| match asc {
            b'\\' => {
                if let Some(&&c) = bytes_it.peek() {
                    if c == b'x' {
                        bytes_it.next();
                    }
                    None
                } else {
                    None
                }
            },
            b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                if let Some(&&c) = bytes_it.peek() {
                    match c {
                        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                            bytes_it.next();
                            println!("..........{}..{}............", &asc, &c);
                            Some((asc, c))
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            _ => None
        }).filter_map(|it| 
            it
        ).collect::<Vec<(u8, u8)>>();
        println!("{:?}", pair_it);
        assert_eq!(pair_it.len(), 0);
    }
}