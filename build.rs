use std::io;

pub fn main() -> io::Result<()> {
    manman::setup("fever", "man/fever.1")?;
    Ok(())
}
