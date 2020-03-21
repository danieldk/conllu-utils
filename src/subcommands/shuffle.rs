use std::io::BufWriter;

use clap::{App, Arg, ArgMatches};
use conllu::io::{Reader, WriteSentence, Writer};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use stdinout::{Input, OrExit, Output};

use crate::traits::{ConlluApp, ConlluPipelineApp};

const SEED: &str = "SEED";

pub struct ShuffleApp {
    input: Input,
    output: Output,
    seed: [u8; 16],
}

impl ConlluPipelineApp for ShuffleApp {}

impl ConlluApp for ShuffleApp {
    fn app() -> App<'static, 'static> {
        Self::pipeline_app("shuffle")
            .about("Shuffle the sentences of a corpus")
            .arg(
                Arg::with_name(SEED)
                    .short("s")
                    .long("seed")
                    .value_name("SEED")
                    .help("Random number generator seed"),
            )
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input = Input::from(matches.value_of(Self::INPUT));
        let output = Output::from(matches.value_of(Self::OUTPUT));

        let seed = if let Some(seed_str) = matches.value_of(SEED) {
            let mut seed = [0; 16];
            let seed_val: u32 = seed_str
                .parse()
                .or_exit(format!("Cannot not parse '{}' as an integer", seed_str), 1);
            (&mut seed[..4]).copy_from_slice(&seed_val.to_be_bytes());
            seed
        } else {
            rand::thread_rng().gen()
        };

        ShuffleApp {
            input,
            output,
            seed,
        }
    }

    fn run(&self) {
        let mut rng = XorShiftRng::from_seed(self.seed);

        let reader = Reader::new(self.input.buf_read().or_exit("Cannot open input corpus", 1));
        let mut writer = Writer::new(BufWriter::new(
            self.output.write().or_exit("Cannot open output corpus", 1),
        ));

        let mut sents: Vec<_> = reader
            .into_iter()
            .map(|r| r.or_exit("Cannot read sentence", 1))
            .collect();

        sents.shuffle(&mut rng);

        for sent in sents {
            writer
                .write_sentence(&sent)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
