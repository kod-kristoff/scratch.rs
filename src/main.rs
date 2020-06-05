use std::{
    fs::File,
    io::{self, prelude::*}
};
fn main() -> io::Result<()> {
    // read_file_line_by_line()
    read_line_with_iterator()
}

fn read_whole_file_and_print_lines() -> io::Result<()> {
    let file = File::open("test.jsonl")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn read_file_line_by_line() -> io::Result<()> {
    let mut reader = my_reader::BufReader::open("test.jsonl")?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        println!("{}", line?.trim());
    }

    Ok(())
}

fn read_line_with_iterator() -> io::Result<()> {
    for line in my_iterator::BufReader::open("test.jsonl")? {
        println!("{}", line?.trim());
    }

    Ok(())
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

mod my_iterator {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024))
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}
