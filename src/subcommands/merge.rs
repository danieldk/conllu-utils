use std::io::BufWriter;

use clap::{App, Arg, ArgMatches};
use conllu::io::{WriteSentence, Writer};
use stdinout::{OrExit, Output};

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

    fn parse(matches: &ArgMatches) -> Self {
        let inputs = matches
            .values_of(INPUTS)
            .unwrap()
            .map(ToOwned::to_owned)
            .collect();
        let output = Output::from(matches.value_of(OUTPUT));

        MergeApp { inputs, output }
    }

    fn run(&self) {
        let mut writer = Writer::new(BufWriter::new(
            self.output
                .write()
                .or_exit("Cannot open output for writing", 1),
        ));

        copy_sents(&mut writer, &self.inputs)
    }
}

fn copy_sents(writer: &mut impl WriteSentence, filenames: &[String]) {
    for filename in filenames {
        let reader =
            open_reader(&filename).or_exit(format!("Cannot open '{}' for reading", filename), 1);

        for sentence in reader {
            let sentence = sentence.or_exit("Cannot read sentence", 1);
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
