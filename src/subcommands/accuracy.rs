use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg, ArgGroup, ArgMatches};
use conllu::graph::{Node, Sentence};
use conllu::io::Reader;
use failure::{bail, ensure, Error};
use stdinout::OrExit;

use crate::layer::{layer_callback, LayerCallback};
use crate::traits::ConlluApp;

const GOLD_TREEBANK: &str = "GOLD_TREEBANK";
const FEATURE: &str = "FEATURE";
const LAYER: &str = "LAYER";
const PREDICTED_TREEBANK: &str = "PREDICTED_TREEBANK";

pub struct AccuracyApp {
    callbacks: Vec<LayerCallback>,
    gold_treebank: String,
    predicted_treebank: String,
}

impl ConlluApp for AccuracyApp {
    fn app() -> App<'static, 'static> {
        App::new("accuracy")
            .about("Compute the accuracy of a layer")
            .arg(
                Arg::with_name(GOLD_TREEBANK)
                    .help("Input treebanks")
                    .required(true),
            )
            .arg(
                Arg::with_name(PREDICTED_TREEBANK)
                    .help("Input treebanks")
                    .required(true),
            )
            .arg(
                Arg::with_name(LAYER)
                    .short("l")
                    .long("layer")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(FEATURE)
                    .short("f")
                    .long("feature")
                    .takes_value(true),
            )
            .group(
                ArgGroup::with_name("source")
                    .args(&[FEATURE, LAYER])
                    .required(true),
            )
    }

    fn parse(matches: &ArgMatches) -> Self {
        let gold_treebank = matches.value_of(GOLD_TREEBANK).unwrap().to_owned();
        let predicted_treebank = matches.value_of(PREDICTED_TREEBANK).unwrap().to_owned();

        let callbacks = match (matches.value_of(LAYER), matches.value_of(FEATURE)) {
            (Some(layer), None) => {
                process_layer_callbacks(layer).or_exit("Cannot parse layer(s)", 1)
            }
            (None, Some(feature)) => vec![feature_callback(feature)],
            _ => unreachable!(),
        };

        AccuracyApp {
            callbacks,
            gold_treebank,
            predicted_treebank,
        }
    }

    fn run(&self) {
        let gold_file = File::open(&self.gold_treebank).or_exit(
            format!("Cannot open gold standard treebank: {}", self.gold_treebank),
            1,
        );
        let gold_reader = Reader::new(BufReader::new(gold_file));

        let predicted_file = File::open(&self.predicted_treebank).or_exit(
            format!(
                "Cannot open predicted treebank: {}",
                self.predicted_treebank
            ),
            1,
        );
        let predicted_reader = Reader::new(BufReader::new(predicted_file));

        let (total, correct) = compare_sentences(gold_reader, predicted_reader, &self.callbacks)
            .or_exit("Could not compare sentences", 1);

        println!(
            "Accuracy: {:.2} ({}/{})",
            (100. * correct as f64) / total as f64,
            correct,
            total
        );
    }
}

fn compare_sentences(
    reader1: impl IntoIterator<Item = Result<Sentence, Error>>,
    reader2: impl IntoIterator<Item = Result<Sentence, Error>>,
    diff_callbacks: &[LayerCallback],
) -> Result<(usize, usize), Error> {
    let mut total = 0;
    let mut correct = 0;

    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (sent1?, sent2?);

        ensure!(
            sent1.len() == sent2.len(),
            "Different number of tokens: {} {}",
            sent1.len(),
            sent2.len()
        );

        for (token1, token2) in sent1
            .iter()
            .filter_map(Node::token)
            .zip(sent2.iter().filter_map(Node::token))
        {
            for layer_callback in diff_callbacks {
                total += 1;

                if layer_callback(token1) == layer_callback(token2) {
                    correct += 1
                }
            }
        }
    }

    Result::Ok((total, correct))
}

fn feature_callback(feature: impl Into<String>) -> LayerCallback {
    let feature = feature.into();

    Box::new(move |token| {
        token
            .features()
            .get(&feature)
            .map(|s| Cow::Borrowed(s.as_str()))
    })
}

fn process_layer_callbacks(layers: &str) -> Result<Vec<LayerCallback>, Error> {
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
