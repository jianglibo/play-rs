#[cfg(test)]
mod tests {
    use crate::tests::tutil::{init_log, get_out_file};
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_cfile() -> std::io::Result<()> {
        init_log();
        let t = get_out_file(&["foo.txt"]).unwrap();
        info!("{:?}", t);
        let mut file = File::create(t)?;
        file.write_all(b"Hello, world!")?;
        Ok(())
    }
}