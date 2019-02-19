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

pub type TripleOption = (Option<u8>, Option<u8>, Option<u8>);

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
