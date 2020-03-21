use std::process;

use clap::{App, Arg, ArgMatches};
use conllu::io::{PartitioningWriter, Reader, WriteSentence};
use stdinout::{Input, OrExit};

use crate::io::open_writer;
use crate::traits::ConlluApp;

const INPUT: &str = "INPUT";
const N_PARTS: &str = "N_PARTS";
const PREFIX: &str = "PREFIX";
const SUFFIX: &str = "SUFFIX";

pub struct PartitionApp {
    input: Input,
    n_parts: usize,
    prefix: String,
    suffix: String,
}

impl ConlluApp for PartitionApp {
    fn app() -> App<'static, 'static> {
        App::new("partition")
            .about("Partition a treebank")
            .arg(
                Arg::with_name(N_PARTS)
                    .value_name("N")
                    .required(true)
                    .help("Number of parts to split into"),
            )
            .arg(
                Arg::with_name(PREFIX)
                    .required(true)
                    .help("Prefix of the partition files"),
            )
            .arg(
                Arg::with_name(SUFFIX)
                    .required(true)
                    .help("Suffix of the partition files"),
            )
            .arg(Arg::with_name(INPUT).help("Treebank to partition"))
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input = Input::from(matches.value_of(INPUT));
        let n_parts = matches.value_of(N_PARTS).unwrap().parse().or_exit(
            format!(
                "Number of parts could not be parsed as an integer: {}",
                matches.value_of(N_PARTS).unwrap()
            ),
            1,
        );

        if n_parts == 0 {
            eprintln!("Cannot split the corpus into zero partitions");
            process::exit(1);
        }

        let prefix = matches.value_of(PREFIX).unwrap().to_owned();
        let suffix = matches.value_of(SUFFIX).unwrap().to_owned();

        PartitionApp {
            input,
            n_parts,
            prefix,
            suffix,
        }
    }

    fn run(&self) {
        let reader = Reader::new(self.input.buf_read().or_exit("Cannot open input corpus", 1));

        let writers: Vec<_> = (0..self.n_parts)
            .map(|part| {
                open_writer(&format!("{}{}{}", self.prefix, part, self.suffix))
                    .or_exit(format!("Cannot open writer for partition {}", part), 1)
            })
            .collect();

        let mut writer = PartitioningWriter::new(writers);

        for sentence in reader {
            let sentence = sentence.or_exit("Cannot parse sentence", 1);
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
