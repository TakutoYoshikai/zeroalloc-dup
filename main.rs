
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;


fn main() -> io::Result<()> {
    let mut f = File::open("main")?;
    let out = File::create("dup")?;
    let mut writer = BufWriter::new(out);
    let mut buffer = [0; 2660344];
    f.read(&mut buffer)?;
    writer.write(&buffer)?;
    Ok(())
}
