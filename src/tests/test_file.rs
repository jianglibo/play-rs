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

// find succession parts.
fn find_successions<T: AsRef<Path>>(fp: T) -> Result<Vec<u32>, FindSeccessionError> {
    let reader = BufReader::new(File::open(fp.as_ref())?);
    let mut count: u32 = 0;
    let mut prev_item: Option<u32> = None;
    let mut commited = false;

    let mut min: u32 = 0xFFFFFF;
    let mut max: u32 = 0;

    let mut v: Vec<u32> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(content) => {
                let mut pair = content.split(':');
                if let (Some(u), Some(g)) = (pair.next(), pair.next()) {
                    if let (Ok(_), Ok(i2)) =
                        (u32::from_str_radix(u, 16), u32::from_str_radix(g, 16))
                    {
                        if i2 > max {
                            max = i2;
                        }
                        if i2 < min {
                            min = i2;
                        }
                        Some(i2)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .map(|itm| {
            let v = match prev_item {
                Some(i) if itm == (i + 1) => {
                    count += 1;
                    commited = false;
                    None
                }
                Some(_) => {
                    let c = count; // copied
                    count = 1;
                    commited = true;
                    Some(c)
                }
                _ => {
                    commited = false;
                    None
                }
            };
            prev_item = Some(itm);
            v
        })
        .filter(Option::is_some)
        .flat_map(|x| x)
        .collect();

        if !commited {
            v.push(count);
        }

        println!("max: {}", max);
        println!("min: {}", min);
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tutil::{get_fixture_file, get_out_file, init_log};
    use std::fs::File;

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

        let velve = 5;
        let v: Vec<&u32> = r.iter().filter(|&&l|l > velve).collect();

        println!("length of great than {}: {:?}",velve, v);
        println!("length of great than {}' groups: {:?}",velve, v.len());
        println!("length of great than {}: {:?}",velve, v.iter().map(|&&x|x).sum::<u32>());
        Ok(())
    }
}
