use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use conllu::io::{Reader, Writer};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

pub fn create_writer<P>(filename: P, gzip: bool) -> io::Result<Writer<Box<dyn Write>>>
where
    P: AsRef<Path>,
{
    let file = File::create(filename)?;
    let boxed_writer: Box<dyn Write> = if gzip {
        Box::new(BufWriter::new(GzEncoder::new(file, Default::default())))
    } else {
        Box::new(BufWriter::new(file))
    };

    Ok(Writer::new(boxed_writer))
}

pub fn open_reader<P>(path: &P) -> io::Result<Reader<Box<dyn BufRead>>>
where
    P: AsRef<Path>,
{
    let f = File::open(path)?;

    let boxed_reader: Box<dyn BufRead> = if path.as_ref().extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::new(GzDecoder::new(f)))
    } else {
        Box::new(BufReader::new(f))
    };

    Ok(Reader::new(boxed_reader))
}

pub fn open_writer<P>(path: &P) -> io::Result<Writer<Box<dyn Write>>>
where
    P: AsRef<Path>,
{
    let compress = path.as_ref().extension() == Some(OsStr::new("gz"));
    create_writer(path, compress)
}
