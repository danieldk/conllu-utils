use std::io::BufWriter;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use conllu::io::{Reader, WriteSentence, Writer};
use stdinout::{Input, Output};
use udgraph::graph::{Node, Sentence};

use crate::traits::{ConlluApp, ConlluPipelineApp};
use crate::unicode::{simplify_unicode, Normalization};

fn normalization_from<S>(value: S) -> Option<Normalization>
where
    S: AsRef<str>,
{
    match value.as_ref() {
        "none" => Some(Normalization::None),
        "nfd" => Some(Normalization::NFD),
        "nfkd" => Some(Normalization::NFKD),
        "nfc" => Some(Normalization::NFC),
        "nfkc" => Some(Normalization::NFKC),
        _ => None,
    }
}

const NORMALIZATION: &str = "NORMALIZATION";

pub struct CleanupApp {
    input: Input,
    output: Output,
    normalization: Normalization,
}

impl ConlluPipelineApp for CleanupApp {}

impl ConlluApp for CleanupApp {
    fn app() -> App<'static, 'static> {
        Self::pipeline_app("cleanup")
            .about("Cleanup the sentences of a corpus")
            .arg(
                Arg::with_name(NORMALIZATION)
                    .short("n")
                    .long("normalization")
                    .value_name("NORMALIZATION")
                    .possible_values(&["nfd", "nfkd", "nfc", "nfkc", "none"])
                    .help("Unicode normalization form"),
            )
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));
        let normalization = matches
            .value_of(NORMALIZATION)
            .map(|n| normalization_from(n).context(format!("Unknown normalization: {}", n)))
            .transpose()?
            .unwrap_or(Normalization::None);

        Ok(CleanupApp {
            input,
            output,
            normalization,
        })
    }

    fn run(&self) -> Result<()> {
        let reader = Reader::new(
            self.input
                .buf_read()
                .context("Cannot open input treebank")?,
        );
        let mut writer = Writer::new(BufWriter::new(
            self.output.write().context("Cannot open output treebank")?,
        ));

        for sentence in reader {
            let mut sentence = sentence.context("Cannot read sentence")?;
            cleanup(&mut sentence, self.normalization);
            writer
                .write_sentence(&sentence)
                .context("Cannot write sentence")?;
        }

        Ok(())
    }
}

fn cleanup(sentence: &mut Sentence, norm: Normalization) {
    for token in sentence.iter_mut().filter_map(Node::token_mut) {
        let clean_form = simplify_unicode(token.form(), norm);
        token.set_form(clean_form);
    }
}
