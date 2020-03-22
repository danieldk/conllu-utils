use std::borrow::Cow;
use std::io::{BufWriter, Write};

use clap::{App, Arg, ArgMatches};
use itertools::Itertools;
use stdinout::{Input, OrExit, Output};

use crate::layer::{layer_callback, LayerCallback};
use crate::traits::{ConlluApp, ConlluPipelineApp};

static LAYER: &str = "LAYER";

pub struct ToTextApp {
    input: Input,
    output: Output,
    layer_callback: LayerCallback,
}

impl ConlluPipelineApp for ToTextApp {}

impl ConlluApp for ToTextApp {
    fn app() -> App<'static, 'static> {
        Self::pipeline_app("to-text")
            .about("Convert a treebank to plain text")
            .arg(
                Arg::with_name(LAYER)
                    .short("l")
                    .possible_values(&["form", "lemma", "upos", "xpos"])
                    .default_value("form")
                    .help("Layer to output as text"),
            )
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));

        let layer = matches.value_of(LAYER).unwrap();
        let layer_callback = layer_callback(layer).or_exit(format!("Unknown layer: {}", layer), 1);

        ToTextApp {
            input,
            output,
            layer_callback,
        }
    }

    fn run(&self) {
        let reader = conllu::io::Reader::new(self.input.buf_read().or_exit("Cannot open input", 1));
        let mut writer = BufWriter::new(self.output.write().or_exit("Cannot open output", 1));

        for sentence in reader {
            let sentence = sentence.or_exit("Cannot read sentence", 1);

            writeln!(
                writer,
                "{}",
                sentence
                    .iter()
                    .filter_map(|n| n.token().map(|t| (*self.layer_callback)(t)
                        .map(Cow::into_owned)
                        .unwrap_or_else(|| "_".to_owned())))
                    .join(" ")
            )
            .or_exit("Cannot write sentence", 1);
        }
    }
}
