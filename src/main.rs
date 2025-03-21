mod hex;
mod input;
mod render;

use hex::HexEditor;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }

    let filename = args[1].clone();
    let data = fs::read(&filename)?;
    let mut editor = HexEditor::new(filename, data)?;
    editor.run()
}
