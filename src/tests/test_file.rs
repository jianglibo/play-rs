#[cfg(test)]
mod tests {
    use crate::tests::tutil::{init_log, get_out_file, get_fixture_file};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::OpenOptions;

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
        let fr = get_fixture_file(["gbkuni30.txt"], true);

        assert!(fr.is_ok());

        let reader = BufReader::new(OpenOptions::new().open(fr.unwrap())?);

        reader.lines().for_each(|line| {
            assert_eq!(line.unwrap().len(), 4);
        });

        Ok(())
    }
}