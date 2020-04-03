use std::io::stdout;

use clap::{crate_version, App, AppSettings, Arg, Shell, SubCommand};

pub mod io;
pub mod layer;
pub mod subcommands;
pub mod traits;
pub mod unicode;

use traits::ConlluApp;

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
    AppSettings::SubcommandRequiredElseHelp,
];

fn main() {
    // Known subapplications.
    let apps = vec![
        subcommands::AccuracyApp::app(),
        subcommands::CleanupApp::app(),
        subcommands::FromTextApp::app(),
        subcommands::MergeApp::app(),
        subcommands::PartitionApp::app(),
        subcommands::ShuffleApp::app(),
        subcommands::ToTextApp::app(),
    ];

    let cli = App::new("conllu")
        .settings(DEFAULT_CLAP_SETTINGS)
        .about("CoNLL-U utilities")
        .version(crate_version!())
        .subcommands(apps)
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate completion scripts for your shell")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("shell").possible_values(&Shell::variants())),
        );
    let matches = cli.clone().get_matches();

    match matches.subcommand_name().unwrap() {
        "accuracy" => {
            subcommands::AccuracyApp::parse(matches.subcommand_matches("accuracy").unwrap()).run()
        }
        "completions" => {
            let shell = matches
                .subcommand_matches("completions")
                .unwrap()
                .value_of("shell")
                .unwrap();
            write_completion_script(cli, shell.parse::<Shell>().unwrap());
        }
        "cleanup" => {
            subcommands::CleanupApp::parse(matches.subcommand_matches("cleanup").unwrap()).run()
        }
        "from-text" => {
            subcommands::FromTextApp::parse(matches.subcommand_matches("from-text").unwrap()).run()
        }
        "merge" => subcommands::MergeApp::parse(matches.subcommand_matches("merge").unwrap()).run(),
        "partition" => {
            subcommands::PartitionApp::parse(matches.subcommand_matches("partition").unwrap()).run()
        }
        "shuffle" => {
            subcommands::ShuffleApp::parse(matches.subcommand_matches("shuffle").unwrap()).run()
        }
        "to-text" => {
            subcommands::ToTextApp::parse(matches.subcommand_matches("to-text").unwrap()).run()
        }
        _unknown => unreachable!(),
    }
}

fn write_completion_script(mut cli: App, shell: Shell) {
    cli.gen_completions_to("conllu", shell, &mut stdout());
}
