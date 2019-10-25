use std::fs::File;
use std::{fs, env, io, mem};
use std::io::{Read, Write, BufWriter};

use count_write::CountWrite;
use human_size::{Size, Byte};
use main_error::MainError;

fn create_file(number: usize) -> io::Result<CountWrite<BufWriter<File>>> {
    let filename = format!("chunk.{}.json", number);
    let file = fs::File::create(&filename)?;
    println!("created {}", filename);
    let writer = BufWriter::new(file);
    Ok(CountWrite::from(writer))
}

fn main() -> Result<(), MainError> {
    let chunk_size = match env::args().nth(1) {
        Some(size) => size.parse::<Size>()?,
        None => return Err(MainError::from("missing chunk size argument (i.e. 10 MB)"))
    };

    let chunk_size = chunk_size.into::<Byte>().value() as u64;

    let mut content = String::new();
    io::stdin().lock().read_to_string(&mut content)?;
    let json = json::parse(&content)?;

    let mut i = 0;
    let mut writer = create_file(i)?;
    write!(writer, "[")?;

    let mut iter = json.members().peekable();

    while let Some(object) = iter.next() {
        let is_last = iter.peek().is_none();

        let string = json::stringify(object.clone());
        writer.write_all(string.as_bytes())?;

        if writer.count() >= chunk_size || is_last {
            write!(writer, "]")?;

            i += 1;

            if is_last { break }

            let new = create_file(i)?;
            mem::replace(&mut writer, new);
            write!(writer, "[")?;
        } else {
            write!(writer, ",")?;
        }
    }

    Ok(())
}
