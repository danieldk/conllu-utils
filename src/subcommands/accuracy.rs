use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg, ArgGroup, ArgMatches};
use conllu::graph::{Node, Sentence};
use conllu::io::Reader;
use failure::{bail, ensure, format_err, Error, ResultExt};
use stdinout::OrExit;
use unicode_categories::UnicodeCategories;

use crate::layer::{layer_callback, LayerCallback};
use crate::traits::ConlluApp;

const GOLD_TREEBANK: &str = "GOLD_TREEBANK";
const FEATURE: &str = "FEATURE";
const ATTACHMENT_SCORES: &str = "ATTACHMENT_SCORES";
const LAYER: &str = "LAYER";
const PREDICTED_TREEBANK: &str = "PREDICTED_TREEBANK";

pub enum Evaluation {
    Callbacks(Vec<LayerCallback>),
    AttachmentScore,
}

pub struct AccuracyApp {
    evaluation: Evaluation,
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
                Arg::with_name(ATTACHMENT_SCORES)
                    .short("a")
                    .long("attachment"),
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
                    .args(&[ATTACHMENT_SCORES, FEATURE, LAYER])
                    .required(true),
            )
    }

    fn parse(matches: &ArgMatches) -> Self {
        let gold_treebank = matches.value_of(GOLD_TREEBANK).unwrap().to_owned();
        let predicted_treebank = matches.value_of(PREDICTED_TREEBANK).unwrap().to_owned();

        let evaluation = match (
            matches.is_present(ATTACHMENT_SCORES),
            matches.value_of(LAYER),
            matches.value_of(FEATURE),
        ) {
            (false, Some(layer), None) => Evaluation::Callbacks(
                process_layer_callbacks(layer).or_exit("Cannot parse layer(s)", 1),
            ),
            (false, None, Some(feature)) => Evaluation::Callbacks(vec![feature_callback(feature)]),
            (true, None, None) => Evaluation::AttachmentScore,
            _ => unreachable!(),
        };

        AccuracyApp {
            evaluation,
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

        match &self.evaluation {
            Evaluation::Callbacks(callbacks) => {
                callback_eval(gold_reader, predicted_reader, callbacks)
            }
            Evaluation::AttachmentScore => dependency_eval(gold_reader, predicted_reader),
        }
        .or_exit("Cannot compute accuracy", 1)
    }
}

fn callback_eval(
    reader1: impl IntoIterator<Item = Result<Sentence, Error>>,
    reader2: impl IntoIterator<Item = Result<Sentence, Error>>,
    diff_callbacks: &[LayerCallback],
) -> Result<(), Error> {
    let mut total = 0;
    let mut correct = 0;

    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (
            sent1.context("Cannot read sentence from gold treebank")?,
            sent2.context("Cannot read sentence from predicted treebank")?,
        );

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

    println!(
        "Accuracy: {:.2} ({}/{})",
        (100. * correct as f64) / total as f64,
        correct,
        total
    );

    Ok(())
}

fn dependency_eval(
    reader1: impl IntoIterator<Item = Result<Sentence, Error>>,
    reader2: impl IntoIterator<Item = Result<Sentence, Error>>,
) -> Result<(), Error> {
    let mut labeled_correct = 0;
    let mut unlabeled_correct = 0;
    let mut total = 0;

    let mut nopunct_labeled_correct = 0;
    let mut nopunct_unlabeled_correct = 0;
    let mut nopunct_total = 0;

    for (sent1, sent2) in reader1.into_iter().zip(reader2.into_iter()) {
        let (sent1, sent2) = (
            sent1.or_exit("Cannot read sentence from gold treebank", 1),
            sent2.or_exit("Cannot read sentence from predicted treebank", 1),
        );

        ensure!(
            sent1.len() == sent2.len(),
            "Different number of tokens: {} {}",
            sent1.len(),
            sent2.len()
        );

        for idx in 1..sent1.len() {
            let form = sent1[idx].token().unwrap().form();
            let is_punct = form.chars().all(|c| c.is_punctuation());

            total += 1;
            if !is_punct {
                nopunct_total += 1;
            }

            let gold_triple = sent1
                .dep_graph()
                .head(idx)
                .ok_or_else(|| format_err!("Token without head: {} in:\n{}", idx, sent1))?;
            let predicted_triple = sent2
                .dep_graph()
                .head(idx)
                .ok_or_else(|| format_err!("Token without head: {} in:\n{}", idx, sent1))?;

            if predicted_triple == gold_triple {
                labeled_correct += 1;

                if !is_punct {
                    nopunct_labeled_correct += 1;
                }
            }

            if predicted_triple.head() == gold_triple.head() {
                unlabeled_correct += 1;

                if !is_punct {
                    nopunct_unlabeled_correct += 1;
                }
            }
        }
    }

    print_dep_result("LAS", labeled_correct, total);
    print_dep_result("LASnp", nopunct_labeled_correct, nopunct_total);
    print_dep_result("UAS", unlabeled_correct, total);
    print_dep_result("UASnp", nopunct_unlabeled_correct, nopunct_total);

    Ok(())
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

fn print_dep_result(desc: &str, correct: usize, total: usize) {
    println!(
        "{}\t{:.2}\t{}\t{}",
        desc,
        (100. * correct as f64) / total as f64,
        correct,
        total
    );
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
