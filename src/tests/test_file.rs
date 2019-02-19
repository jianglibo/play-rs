use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
enum FindSeccessionError {
    IO_ERROR(std::io::Error), //std::io::Error
}

impl std::convert::From<std::io::Error> for FindSeccessionError {
    fn from(err: std::io::Error) -> Self {
        FindSeccessionError::IO_ERROR(err)
    }
}

#[derive(Debug)]
pub enum SuccessionMatchError {
    Continue,
    Impossible,
}

type SuccessionMatchResult = Result<u32, SuccessionMatchError>;

type SuccessionGroup = Vec<(u32, (u32, u32), (u32, u32))>;

// find succession parts.
fn find_successions<T: AsRef<Path>>(fp: T) -> Result<SuccessionGroup, FindSeccessionError> {
    let reader = BufReader::new(File::open(fp.as_ref())?);
    let count: &mut u32 = &mut 0;
    let prev_item: &mut Option<(u32, u32)> = &mut None; // a mutable reference to Option.
    let committed: &mut bool = &mut false;
    let start: &mut (u32, u32) = &mut (0, 0);
    let end: &mut (u32, u32) = &mut (0, 0);

    let mut min: u32 = 0xFFFFFF;
    let mut max: u32 = 0;

    let mut v: SuccessionGroup = reader
        .lines()
        .filter_map(|line| match line {
            Ok(content) => {
                let mut pair = content.split(':');
                if let (Some(u), Some(g)) = (pair.next(), pair.next()) {
                    if let (Ok(i1), Ok(i2)) =
                        (u32::from_str_radix(u, 16), u32::from_str_radix(g, 16))
                    {
                        if i2 > max {
                            max = i2;
                        }
                        if i2 < min {
                            min = i2;
                        }
                        Some((i1, i2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(_) => None,
        })
        .map(|itm| {
            let v = match prev_item {
                Some(tp) if itm.1 == ((*tp).1 + 1) => {
                    *count += 1;
                    *committed = false;
                    None
                }
                Some(tp) => {
                    let prev_start = *start;
                    *start = itm;
                    let cur_end = *tp;
                    let c = *count; // copied
                    *count = 1;
                    *committed = true;
                    Some((c, prev_start, cur_end))
                }
                _ => {
                    *committed = false;
                    *start = itm;
                    None
                }
            };
            *prev_item = Some(itm);
            *end = itm;
            v
        })
        .filter(Option::is_some)
        .flat_map(|x| x)
        .collect();

        if !*committed {
            v.push((*count, *start, *end));
        }

        println!("max: {}", max);
        println!("min: {}", min);
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tutil::{get_fixture_file, get_out_file,get_src_file, init_log};
    use std::fs::{File, OpenOptions};
    use std::io::LineWriter;

    #[test]
    fn test_cfile() -> std::io::Result<()> {
        init_log();
        let t = get_out_file(&["foo.txt"]).unwrap();
        info!("{:?}", t);
        let mut file = File::create(t)?;
        file.write_all(b"Hello, world!")?;
        Ok(())
    }

    #[test]
    fn test_read_line() -> std::io::Result<()> {
        init_log();
        let fr = get_fixture_file(["gbkuni.txt"], true);
        assert!(fr.is_ok());
        let fr = fr.unwrap();
        debug!("{:?}", fr);
        // let f = OpenOptions::new().read(true).open(fr)?;

        let r = find_successions(fr).unwrap();

        let valve = 50;
        let v: Vec<&(u32, (u32, u32), (u32, u32))> = r.iter().filter(|&l|l.0 > valve).collect();

        for itm in &v {
            println!("({},({:X},{:X}),({:X},{:X}))", itm.0, (itm.1).0, (itm.1).1, (itm.2).0, (itm.2).1);
        }

        println!("length of great than {}: {:?}",valve, v);
        println!("length of great than {}' groups: {:?}",valve, v.len());
        println!("length of great than {}: {:?}",valve, v.iter().map(|&&x|x.0).sum::<u32>());
        Ok(())
    }

    #[test]
    fn test_mem() {
        let v = [Some(2u8), None];
        assert_eq!(8, std::mem::size_of::<Option<u32>>());
        let range = 0xFEFE - 0x8140;
        let mut all = [0u32;0xFEFE];
        all[1] = 66;
        assert_eq!(all[0], 0);
        assert_eq!(all[1], 66);
        let mut all = [1, 2, 3, 4, 5, 6];
    }

    #[test]
    fn test_write() {
        let prefix = r###"lazy_static! {
    static ref GBK_UNI: [u32;0xFEFE] = {
        let mut all = [0u32;0xFEFE];

        "###;

        let postfix = r###"
                all
    };
}
        "###;

        let fr = get_fixture_file(["gbkuni.txt"], true);
        let reader = BufReader::new(File::open(fr.unwrap()).unwrap());

        let wf = get_src_file(["table.rs"]).unwrap();

        let wf = OpenOptions::new().write(true).create(true).open(wf).unwrap();

        let mut writer = LineWriter::new(wf);

        writer.write_all(prefix.as_bytes()).unwrap();

        reader
        .lines()
        .filter_map(|line| match line {
            Ok(content) => {
                let mut pair = content.split(':');
                if let (Some(u), Some(g)) = (pair.next(), pair.next()) {
                    if let (Ok(i1), Ok(i2)) =
                        (u32::from_str_radix(u, 16), u32::from_str_radix(g, 16))
                    {
                        Some((i1, i2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(_) => None,
        }).for_each(|pp| {
            write!(writer, "all[{}]={};\n", pp.0, pp.1).unwrap();
        });

        writer.write_all(postfix.as_bytes()).unwrap();
    }

    // lazy_static! {
    //     static ref GBK_UNI: [u32;0xFEFE] = {
    //         let mut all = [0u32;0xFEFE];
    //         all[0x11]=0xaa;
    //         all
    //     };
    // }
}
