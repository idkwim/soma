use clap::{Arg, ArgMatches, SubCommand};
use hyper::client::connect::Connect;

use soma::prelude::*;
use soma::{Environment, Printer};

use crate::commands::{default_runtime, App, SomaCommand};
use soma::ops::build;

pub struct BuildCommand;

impl BuildCommand {
    pub fn new() -> BuildCommand {
        BuildCommand {}
    }
}

impl SomaCommand for BuildCommand {
    const NAME: &'static str = "build";

    fn app(&self) -> App {
        SubCommand::with_name(Self::NAME)
            .about("Builds a problem image")
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
        build(
            &env,
            matches.value_of("problem").unwrap(),
            &mut default_runtime(),
        )
    }
}
