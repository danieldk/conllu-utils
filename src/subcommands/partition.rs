use anyhow::{ensure, Context, Result};
use clap::{App, Arg, ArgMatches};
use conllu::io::{PartitioningWriter, Reader, WriteSentence};
use stdinout::Input;

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

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let input = Input::from(matches.value_of(INPUT));
        let n_parts = matches.value_of(N_PARTS).unwrap().parse().context(format!(
            "Number of parts could not be parsed as an integer: {}",
            matches.value_of(N_PARTS).unwrap()
        ))?;

        ensure!(n_parts != 0, "Cannot split the corpus into zero partitions");

        let prefix = matches.value_of(PREFIX).unwrap().to_owned();
        let suffix = matches.value_of(SUFFIX).unwrap().to_owned();

        Ok(PartitionApp {
            input,
            n_parts,
            prefix,
            suffix,
        })
    }

    fn run(&self) -> Result<()> {
        let reader = Reader::new(
            self.input
                .buf_read()
                .context("Cannot open input treebank")?,
        );

        let writers = (0..self.n_parts)
            .map(|part| {
                open_writer(&format!("{}{}{}", self.prefix, part, self.suffix))
                    .context(format!("Cannot open writer for partition {}", part))
            })
            .collect::<Result<Vec<_>>>()?;

        let mut writer = PartitioningWriter::new(writers);

        for sentence in reader {
            let sentence = sentence.context("Cannot parse sentence")?;
            writer
                .write_sentence(&sentence)
                .context("Cannot write sentence")?;
        }

        Ok(())
    }
}
