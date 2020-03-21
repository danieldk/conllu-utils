use clap::{crate_version, App, AppSettings, Arg, ArgMatches};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

pub trait ConlluApp {
    fn app() -> App<'static, 'static>;

    fn parse(matches: &ArgMatches) -> Self;

    fn run(&self);
}

pub trait ConlluPipelineApp: ConlluApp {
    const INPUT: &'static str = "INPUT";
    const OUTPUT: &'static str = "OUTPUT";

    fn pipeline_app<'a, 'b>(name: &str) -> App<'a, 'b> {
        App::new(name)
            .settings(DEFAULT_CLAP_SETTINGS)
            .version(crate_version!())
            .arg(Arg::with_name(Self::INPUT).help("Input"))
            .arg(Arg::with_name(Self::OUTPUT).help("Output"))
    }
}
