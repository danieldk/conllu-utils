use std::io::{BufRead, BufWriter};

use anyhow::{Context, Result};
use clap::{App, ArgMatches};
use conllu::graph::Sentence;
use conllu::io::WriteSentence;
use conllu::token::TokenBuilder;
use stdinout::{Input, Output};

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

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));

        Ok(FromTextApp { input, output })
    }

    fn run(&self) -> Result<()> {
        let reader = self.input.buf_read().context("Cannot open input corpus")?;

        let mut writer = conllu::io::Writer::new(BufWriter::new(
            self.output.write().context("Cannot open output treebank")?,
        ));

        for line in reader.lines() {
            let line = line.context("Cannot read sentence")?;
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
                    .context("Cannot write sentence")?;
            }
        }

        Ok(())
    }
}
