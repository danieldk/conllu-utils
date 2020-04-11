use std::io::BufWriter;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use conllu::io::{WriteSentence, Writer};
use stdinout::Output;

use crate::io::open_reader;
use crate::traits::ConlluApp;

static INPUTS: &str = "INPUTS";
static OUTPUT: &str = "OUTPUT";

pub struct MergeApp {
    inputs: Vec<String>,
    output: Output,
}

impl ConlluApp for MergeApp {
    fn app() -> App<'static, 'static> {
        App::new("merge")
            .about("Merge treebanks")
            .arg(
                Arg::with_name(INPUTS)
                    .help("Input treebanks")
                    .required(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name(OUTPUT)
                    .short("w")
                    .takes_value(true)
                    .help("Write merged treebank to a file"),
            )
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let inputs = matches
            .values_of(INPUTS)
            .unwrap()
            .map(ToOwned::to_owned)
            .collect();
        let output = Output::from(matches.value_of(OUTPUT));

        Ok(MergeApp { inputs, output })
    }

    fn run(&self) -> Result<()> {
        let mut writer = Writer::new(BufWriter::new(
            self.output
                .write()
                .context("Cannot open output for writing")?,
        ));

        copy_sents(&mut writer, &self.inputs)
    }
}

fn copy_sents(writer: &mut impl WriteSentence, filenames: &[String]) -> Result<()> {
    for filename in filenames {
        let reader =
            open_reader(&filename).context(format!("Cannot open '{}' for reading", filename))?;

        for sentence in reader {
            let sentence = sentence.context(format!("Cannot read sentence from: {}", filename))?;
            writer
                .write_sentence(&sentence)
                .context("Cannot write sentence")?;
        }
    }

    Ok(())
}
