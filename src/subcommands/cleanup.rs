use std::io::BufWriter;

use clap::{App, Arg, ArgMatches};
use conllu::graph::{Node, Sentence};
use conllu::io::{Reader, WriteSentence, Writer};
use stdinout::{Input, OrExit, Output};

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

    fn parse(matches: &ArgMatches) -> Self {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));
        let normalization = matches
            .value_of(NORMALIZATION)
            .map(|n| normalization_from(n).or_exit(format!("Unknown normalization: {}", n), 1))
            .unwrap_or(Normalization::None);

        CleanupApp {
            input,
            output,
            normalization,
        }
    }

    fn run(&self) {
        let reader = Reader::new(self.input.buf_read().or_exit("Cannot open input corpus", 1));
        let mut writer = Writer::new(BufWriter::new(
            self.output.write().or_exit("Cannot open output corpus", 1),
        ));

        for sentence in reader {
            let mut sentence = sentence.or_exit("Cannot read sentence", 1);
            cleanup(&mut sentence, self.normalization);
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}

fn cleanup(sentence: &mut Sentence, norm: Normalization) {
    for token in sentence.iter_mut().filter_map(Node::token_mut) {
        let clean_form = simplify_unicode(token.form(), norm);
        token.set_form(clean_form);
    }
}
