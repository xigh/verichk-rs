use std::{error::Error, fmt, fs::File, path::PathBuf};

#[allow(unused)]
#[derive(Debug)]
pub enum VerilogError {
    NotImplemented,
}

impl fmt::Display for VerilogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "not implemented"),
        }
    }
}

impl Error for VerilogError {}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn parse_verilog(path: PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let content = std::io::read_to_string(file)?;
    for (l, line) in content.lines().enumerate() {
        println!("{:6>} {:?}", l + 1, line);
    }
    Ok(())
}
