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
        0x00010000...0x010FFFF => {
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

type TripleOptionU8 = (Option<u8>, Option<u8>, Option<u8>);

#[derive(Debug)]
pub enum MayBeMatchError {
    Continue,
    Impossible,
    Discard(Vec<u8>),
}

type ScanResult = Result<Vec<u8>, MayBeMatchError>;

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

// a function used in scan body. at the middle of iterator chains.

pub fn find_2_hex_pair(state: &mut TripleOptionU8, it: u8) -> ScanResult {
    match state {
        (None, None, None) => {
            match it {
                BSL => {
                    state.0 = Some(BSL);
                    Err(MayBeMatchError::Continue)
                },
                // _ => Err(MayBeMatchError::Discard(vec![it])), // will only assing to tuple if meet \.
                _ => Ok(vec!(it)), // it's an ascii code.
            }
        }
        (Some(_), None, None) => {
            match it {
                XC => {
                    state.1 = Some(XC);
                    Err(MayBeMatchError::Continue)
                },
                _ => {
                    *state = (None, None, None); // only if second is 'x'.
                    Ok(vec!(BSL, it))
                },
            }
        }
        (Some(_), Some(_), None) => {
            match it {
                b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                    state.2 = Some(it);
                    Err(MayBeMatchError::Continue)
                },
                _ => {
                    *state = (None, None, None); // only if third character is valid.
                    Ok(vec!(BSL, XC, it))
                },
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
                Ok(vec!(tp.unwrap()))
            },
            _ => {
                *state = (None, None, None);
                Ok(vec![BSL, XC, state.2.unwrap(), it])
            },
        },
        // we got 2 items, but first is leaked, for example it is 'ga', replace state with new pair.
        _ => {
            *state = (Some(it), None, None);
            Err(MayBeMatchError::Impossible)
        }
    }
}

    pub fn get_hex_pairs(a_slice: &[u8]) -> Vec<u8> {
        info!("-------------------**---------------------------");
        a_slice
            .iter()
            .scan((None, None, None), |state: &mut TripleOptionU8, it| {
                Some(find_2_hex_pair(state, *it))
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
            .flat_map(|x|x)
            .collect()
    }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tutil::init_log;
    use std::time::Instant;

    // scan stop on None.
    #[test]
    fn test_scan() {
        init_log();

        let _s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        let s = r"\xce\xde";
        let v_slice = s.as_bytes();
        let u8_pair = get_hex_pairs(v_slice);

        assert_eq!(u8_pair.len(), 2);

        assert_eq!(u8_pair[0], 206);

        assert_eq!(u8_pair[1], 222);
        // assert_eq!(u8_pair[0], Ok((92, 101)));

        let s = r"ce\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_hex_pairs(v_slice).len(), 4);

        let s = r"ce\xce\xde1234";
        let v_slice = s.as_bytes();
        assert_eq!(get_hex_pairs(v_slice).len(), 8);

        // this will happen.
        let s = r"cxe\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_hex_pairs(v_slice).len(), 5);
    }

    fn run_parser(builtin: bool) {
        let now = Instant::now();
        match builtin {
            true =>  for _i in 0..100000 {
                  u32::from_str_radix("A1E8", 16).unwrap();
                },
            false =>  for _i in 0..100000 {
                    hex_str_to_u32("A1E8").unwrap();
            },
        }
        let m = if builtin {"builtin"} else {"custom"};
        info!("{} {:?}",m, now.elapsed());
    }

    #[test]
    fn test_hex_str_to_u32() {
        init_log();
        let a = hex_str_to_u32("A1E8");
        let a1 = u32::from_str_radix("A1E8", 16);
        assert_eq!(a.unwrap(), a1.unwrap());

        run_parser(true);
        run_parser(false);
        run_parser(true);
        run_parser(false);

    }
}
