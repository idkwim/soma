use clap::{Arg, ArgMatches, SubCommand};
use hyper::client::connect::Connect;

use soma::ops::clean;
use soma::prelude::*;
use soma::{Environment, Printer};

use crate::commands::{default_runtime, App, SomaCommand};

pub struct CleanCommand;

impl CleanCommand {
    pub fn new() -> CleanCommand {
        CleanCommand {}
    }
}

impl SomaCommand for CleanCommand {
    const NAME: &'static str = "clean";

    fn app(&self) -> App {
        SubCommand::with_name(Self::NAME)
            .about("Cleans up a problem image")
            .arg(
                Arg::with_name("problem")
                    .required(true)
                    .help("problem name with optional repository name prefix"),
            )
    }

    fn handle_match(
        &self,
        env: Environment<impl Connect, impl Printer>,
        matches: &ArgMatches,
    ) -> SomaResult<()> {
        clean(
            &env,
            matches.value_of("problem").unwrap(),
            &mut default_runtime(),
        )
    }
}
