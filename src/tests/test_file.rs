use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

    let v: Vec<u32> = reader.lines().filter_map(|line| {
        match line {
            Ok(content) => {
                let mut pair = content.split(':');
                if let (Some(u), Some(g)) = (pair.next(), pair.next()) {
                    if let (Ok(_), Ok(i2)) = (u32::from_str_radix(u, 16), u32::from_str_radix(u, 16)) {
                        Some(i2)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(_) => {
                None
            }
        }
    }).scan((None, 0u32), |state: &mut (Option<u32>, u32), itm| {
        match state {
            (Some(i), _) => {
                if itm == *i + 1 {
                    state.1 += 1;
                    Some(Err(SuccessionMatchError::Continue))
                } else {
                    println!("with length: {}", state.1);
                    let c = state.1;
                    state.1 = 1;
                    Some(Ok(c))
                }
            },
            _ => {
                state.0 = Some(itm);
                state.1 = 1;
                Some(Err(SuccessionMatchError::Continue))
            }
        }
    }).filter(Result::is_ok).flat_map(|x|x).collect();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use crate::tests::tutil::{init_log, get_out_file, get_fixture_file};
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

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
        info!("here 1.");
        let fr = get_fixture_file(["gbkuni30.txt"], true);

        info!("here 2.");
        assert!(fr.is_ok());

        let fr = fr.unwrap();
        debug!("{:?}", fr);
        // let f = OpenOptions::new().read(true).open(fr)?;

        let r = find_successions(fr);
        println!("{:?}", r);
        Ok(())
    }
}