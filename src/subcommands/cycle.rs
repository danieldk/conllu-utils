use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use conllu::display::ConlluSentence;
use conllu::io::Reader;
use itertools::Itertools;
use petgraph::algo::kosaraju_scc;
use stdinout::Input;
use udgraph::graph::Sentence;

use crate::traits::ConlluApp;

const INPUT: &str = "INPUT";

pub struct CycleApp {
    input: Input,
}

impl ConlluApp for CycleApp {
    fn app() -> App<'static, 'static> {
        App::new("cycle")
            .about("Find cycles in a treebank")
            .arg(Arg::with_name(INPUT).help("Treebank to process"))
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let input = Input::from(matches.value_of(INPUT));

        Ok(CycleApp { input })
    }

    fn run(&self) -> Result<()> {
        let reader = Reader::new(
            self.input
                .buf_read()
                .context("Cannot open input treebank")?,
        );

        for sentence in reader {
            let sentence = sentence.context("Cannot parse sentence")?;
            check_cycles(&sentence);
        }

        Ok(())
    }
}

fn check_cycles(sentence: &Sentence) {
    let mut sentence_printed = false;

    for component in kosaraju_scc(sentence.get_ref()) {
        if component.len() == 1 {
            continue;
        }

        if !sentence_printed {
            println!("{}\n", ConlluSentence::borrowed(sentence));
            sentence_printed = true
        }

        println!(
            "Cycle: {}",
            component.iter().map(|i| i.index().to_string()).join(", ")
        );
    }
}
