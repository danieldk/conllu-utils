use std::io::{BufRead, BufWriter};

use clap::{App, ArgMatches};
use conllu::graph::Sentence;
use conllu::io::WriteSentence;
use conllu::token::TokenBuilder;
use stdinout::{Input, OrExit, Output};

use crate::traits::{ConlluApp, ConlluPipelineApp};

pub struct FromTextApp {
    input: Input,
    output: Output,
}

impl ConlluPipelineApp for FromTextApp {}

impl ConlluApp for FromTextApp {
    fn app() -> App<'static, 'static> {
        Self::pipeline_app("from-text").about("Convert tokenized text to CoNLL-U")
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));

        FromTextApp { input, output }
    }

    fn run(&self) {
        let reader = self.input.buf_read().or_exit("Cannot open input", 1);

        let mut writer = conllu::io::Writer::new(BufWriter::new(
            self.output.write().or_exit("Cannot open output", 1),
        ));

        for line in reader.lines() {
            let line = line.or_exit("Cannot read sentence", 1);
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            let sentence: Sentence = trimmed
                .split(' ')
                .map(|t| TokenBuilder::new(t).into())
                .collect();

            if sentence.len() != 1 {
                writer
                    .write_sentence(&sentence)
                    .or_exit("Cannot write sentence", 1);
            }
        }
    }
}
