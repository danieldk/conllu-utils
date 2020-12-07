use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, ensure, Context, Result};
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use conllu::io::Reader;
use udgraph::graph::Sentence;
use udgraph::token::Tokens;

use crate::layer::{layer_callback, LayerCallback};
use crate::traits::ConlluApp;

const FORCE_COLOR: &str = "FORCE_COLOR";
const LAYER: &str = "LAYER";
const SHOW: &str = "SHOW";
const TREEBANK_1: &str = "TREEBANK_1";
const TREEBANK_2: &str = "TREEBANK_2";

pub struct CompareApp {
    force_color: bool,
    layer_callbacks: Vec<LayerCallback>,
    show_callbacks: Vec<LayerCallback>,
    treebank_1: String,
    treebank_2: String,
}

impl ConlluApp for CompareApp {
    fn app() -> App<'static, 'static> {
        App::new("compare")
            .about("Compare two treebanks on specific layers")
            .arg(
                Arg::with_name(TREEBANK_1)
                    .help("First treebank")
                    .required(true),
            )
            .arg(
                Arg::with_name(TREEBANK_2)
                    .help("Second treebank")
                    .required(true),
            )
            .arg(
                Arg::with_name(FORCE_COLOR)
                    .short("c")
                    .long("force-color")
                    .help("Force colored output"),
            )
            .arg(
                Arg::with_name(LAYER)
                    .short("l")
                    .long("layer")
                    .takes_value(true)
                    .default_value("upos")
                    .help("Compare a layer"),
            )
            .arg(
                Arg::with_name(SHOW)
                    .short("s")
                    .long("show")
                    .takes_value(true)
                    .default_value("form")
                    .help("Compare a layer"),
            )
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        let treebank_1 = matches.value_of(TREEBANK_1).unwrap().to_owned();
        let treebank_2 = matches.value_of(TREEBANK_2).unwrap().to_owned();

        let force_color = matches.is_present(FORCE_COLOR);

        let layer_callbacks = process_layer_callbacks(matches.value_of(LAYER).unwrap())
            .context("Cannot parse layer(s) to compare")?;
        let show_callbacks = process_layer_callbacks(matches.value_of(SHOW).unwrap())
            .context("Cannot parse layer(s) to show")?;

        Ok(CompareApp {
            force_color,
            layer_callbacks,
            show_callbacks,
            treebank_1,
            treebank_2,
        })
    }

    fn run(&self) -> Result<()> {
        if self.force_color {
            colored::control::set_override(true);
        }

        let reader1 = Reader::new(BufReader::new(
            File::open(&self.treebank_1)
                .context(format!("Cannot open first treebank: {}", self.treebank_1))?,
        ));

        let reader2 = Reader::new(BufReader::new(
            File::open(&self.treebank_2)
                .context(format!("Cannot open first treebank: {}", self.treebank_2))?,
        ));

        compare_sentences(
            reader1,
            reader2,
            &self.layer_callbacks,
            &self.show_callbacks,
        )
        .context("Cannot compare sentences")
    }
}

fn process_layer_callbacks(layers: &str) -> Result<Vec<LayerCallback>> {
    let mut callbacks = Vec::new();
    for layer_str in layers.split(',') {
        match layer_callback(layer_str) {
            Some(c) => callbacks.push(c),
            None => {
                bail!("Unknown layer: {}", layer_str);
            }
        }
    }

    Ok(callbacks)
}

fn compare_sentences(
    reader1: Reader<impl BufRead>,
    reader2: Reader<impl BufRead>,
    diff_callbacks: &[LayerCallback],
    show_callbacks: &[LayerCallback],
) -> Result<()> {
    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (
            sent1.context("Cannot read sentence from first treebank")?,
            sent2.context("Cannot read sentence from second treebank")?,
        );

        let diff = diff_indices(&sent1, &sent2, diff_callbacks)?;

        if !diff.is_empty() {
            print_diff(&sent1, &sent2, diff_callbacks, show_callbacks);
            println!();
        }
    }

    Result::Ok(())
}

fn print_diff(
    sentence1: &Sentence,
    sentence2: &Sentence,
    diff_callbacks: &[LayerCallback],
    show_callbacks: &[LayerCallback],
) {
    for (idx, (token1, token2)) in sentence1.tokens().zip(sentence2.tokens()).enumerate() {
        let mut columns = Vec::new();

        for callback in show_callbacks {
            columns.push(callback(token1).unwrap_or(Cow::Borrowed("_")).into_owned());
        }

        for callback in diff_callbacks {
            let col1 = callback(token1).unwrap_or(Cow::Borrowed("_"));
            let col2 = callback(token2).unwrap_or(Cow::Borrowed("_"));

            if col1 != col2 {
                columns.push(format!("{}", col1.red()));
                columns.push(format!("{}", col2.red()));
            } else {
                columns.push(col1.into_owned());
                columns.push(col2.into_owned());
            }
        }

        println!("{}\t{}", idx + 1, columns.join("\t"));
    }
}

fn diff_indices(
    sentence1: &Sentence,
    sentence2: &Sentence,
    diff_callbacks: &[LayerCallback],
) -> Result<BTreeSet<usize>> {
    ensure!(
        sentence1.len() == sentence2.len(),
        "Different number of tokens: {} {}",
        sentence1.len(),
        sentence2.len()
    );

    let mut indices = BTreeSet::new();

    'tokenloop: for (idx, (token1, token2)) in
        sentence1.tokens().zip(sentence2.tokens()).enumerate()
    {
        for layer_callback in diff_callbacks {
            if layer_callback(token1) != layer_callback(token2) {
                indices.insert(idx);
                continue 'tokenloop;
            }
        }
    }

    Result::Ok(indices)
}
