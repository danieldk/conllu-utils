use std::borrow::Cow;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use itertools::Itertools;
use stdinout::{Input, Output};

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

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));

        let layer = matches.value_of(LAYER).unwrap();
        let layer_callback = layer_callback(layer).context(format!("Unknown layer: {}", layer))?;

        Ok(ToTextApp {
            input,
            output,
            layer_callback,
        })
    }

    fn run(&self) -> Result<()> {
        let reader =
            conllu::io::Reader::new(self.input.buf_read().context("Cannot open treebank")?);
        let mut writer = BufWriter::new(
            self.output
                .write()
                .context("Cannot open output for writing")?,
        );

        for sentence in reader {
            let sentence = sentence.context("Cannot read sentence")?;

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
            .context("Cannot write sentence")?;
        }

        Ok(())
    }
}
