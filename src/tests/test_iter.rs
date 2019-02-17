type TripleOption = (Option<u8>, Option<u8>, Option<u8>);

#[derive(Debug)]
pub enum MatchError {
    Continue,
    Impossible,
    Discard(Vec<u8>),
}

type ScanResult = Result<u8, MatchError>;

const BSL: u8 = b'\\';
const XC: u8 = b'x';

pub fn aton_1(b: u8) -> Option<u8> {
    match b {
        b'0'...b'9' => Some(b - b'0'),
        b'a'...b'f' => Some(b - b'a' + 10),
        b'A'...b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn aton_2(a: u8, b: u8) -> Option<u8> {
    if let (Some(high), Some(low)) = (aton_1(a), aton_1(b)) {
        Some(high << 4 | low)
    } else {
        None
    }
}

pub fn find_pair(state: &mut TripleOption, it: u8) -> ScanResult {
    match state {
        (None, None, None) => {
            match it {
                BSL => {
                    state.0 = Some(BSL);
                    Err(MatchError::Continue)
                }
                _ => Err(MatchError::Discard(vec![it])), // will only assing to tuple if meet \.
            }
        }
        (Some(_), None, None) => {
            match it {
                XC => {
                    state.1 = Some(XC);
                    Err(MatchError::Continue)
                }
                _ => {
                    *state = (None, None, None); // only if second is 'x'.
                    Err(MatchError::Discard(vec![BSL, it]))
                }
            }
        }
        (Some(_), Some(_), None) => {
            match it {
                b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                    state.2 = Some(it);
                    Err(MatchError::Continue)
                }
                _ => {
                    *state = (None, None, None); // only if third character is valid.
                    Err(MatchError::Discard(vec![BSL, XC, it]))
                }
            }
        }
        // last character may be invalid.
        (Some(BSL), Some(XC), Some(b'0'...b'9'))
        | (Some(BSL), Some(XC), Some(b'a'...b'f'))
        | (Some(BSL), Some(XC), Some(b'A'...b'F')) => match it {
            b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                let tp = aton_2(state.2.unwrap(), it);
                *state = (None, None, None);
                info!("{:?}", tp);
                Ok(tp.unwrap())
            }
            _ => {
                // it value is leaked. cx -> it = g? discard all.
                let er = Err(MatchError::Discard(vec![BSL, XC, state.2.unwrap(), it]));
                *state = (None, None, None);
                er
            }
        },
        // we got 2 items, but first is leaked, for example it is 'ga', replace state with new pair.
        _ => {
            *state = (Some(it), None, None);
            Err(MatchError::Impossible)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
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
