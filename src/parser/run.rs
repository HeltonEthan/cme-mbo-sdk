use color_eyre::eyre::Result;
use std::path::PathBuf;
use crate::parser::dbn;

pub fn run(path: &PathBuf) -> Result<()> {
    dbn::dbn_stream(path)?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::Config;

    #[test]
    fn run_test() -> Result<()> {
        let path = PathBuf::from(
            r"C:/Users/helto/GLBX-20250915-NGKNUL4VBG/glbx-mdp3-20250512-20250517.mbo.dbn.zst"
        );
        let mut config: Config = Default::default();
        config.path(&path);

        run(&config.path)?;

        Ok(())
    }
}
