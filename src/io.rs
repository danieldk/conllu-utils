use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

use flate2::write::GzEncoder;

pub fn create_writer<P>(filename: P, gzip: bool) -> io::Result<conllu::io::Writer<Box<dyn Write>>>
where
    P: AsRef<Path>,
{
    let file = File::create(filename)?;
    let boxed_writer: Box<dyn Write> = if gzip {
        Box::new(BufWriter::new(GzEncoder::new(file, Default::default())))
    } else {
        Box::new(BufWriter::new(file))
    };

    Ok(conllu::io::Writer::new(boxed_writer))
}

pub fn open_writer<P>(path: &P) -> io::Result<conllu::io::Writer<Box<dyn Write>>>
where
    P: AsRef<Path>,
{
    let compress = path.as_ref().extension() == Some(OsStr::new("gz"));
    create_writer(path, compress)
}
