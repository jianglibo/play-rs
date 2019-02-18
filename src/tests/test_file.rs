#[cfg(test)]
mod tests {
    use crate::tests::tutil::{init_log, get_out_file, get_fixture_file};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

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
        let f = File::open(fr)?; // open read only 
        debug!("{:?}", f);
        let reader = BufReader::new(f);

        info!("here 3.");
        reader.lines().for_each(|line| {
            match line {
                Ok(content) => {
                    // info!("{},{}", content, content.len());
                    let mut pair = content.split(':');
                    if let (Some(u), Some(g)) = (pair.next(), pair.next()) {
                        info!("{} = {}", u, g);
                    } else {
                        assert!(false);
                    }
                },
                Err(_) => {
                    assert!(false)
                }
            }
        });

        Ok(())
    }
}