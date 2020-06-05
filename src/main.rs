use std::{
    fs::File,
    io::{self, prelude::*, BufReader}
};
fn main() -> io::Result<()> {
    read_and_print_lines()
}

fn read_and_print_lines() -> io::Result<()> {
    let file = File::open("test.jsonl")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
