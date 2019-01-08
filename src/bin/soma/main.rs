use std::string::ToString;

use clap::{App, AppSettings};
use hyper::client::connect::Connect;
use whoami::username;

use soma::data_dir::DataDirectory;
use soma::docker::connect_default;
use soma::error::Result as SomaResult;
use soma::{Environment, VERSION};

use crate::commands::*;
use crate::terminal_printer::TerminalPrinter;

mod commands;
mod terminal_printer;

fn cli_env(data_dir: DataDirectory) -> SomaResult<Environment<impl Connect, TerminalPrinter>> {
    Ok(Environment::new(
        username(),
        data_dir,
        connect_default()?,
        TerminalPrinter::new(),
    ))
}

fn main_result() -> SomaResult<()> {
    let add_command = AddCommand::new();
    let build_command = BuildCommand::new();
    let fetch_command = FetchCommand::new();
    let list_command = ListCommand::new();
    let run_command = RunCommand::new();

    let matches = App::new("soma")
        .version(VERSION)
        .about("Your one-stop CTF problem management tool")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(add_command.app())
        .subcommand(build_command.app())
        .subcommand(fetch_command.app())
        .subcommand(list_command.app())
        .subcommand(run_command.app())
        .get_matches();

    let data_dir = DataDirectory::new()?;
    let env = cli_env(data_dir)?;

    match matches.subcommand() {
        (AddCommand::NAME, Some(matches)) => add_command.handle_match(env, matches),
        (BuildCommand::NAME, Some(matches)) => build_command.handle_match(env, matches),
        (FetchCommand::NAME, Some(matches)) => fetch_command.handle_match(env, matches),
        (ListCommand::NAME, Some(matches)) => list_command.handle_match(env, matches),
        (RunCommand::NAME, Some(matches)) => run_command.handle_match(env, matches),
        _ => unreachable!(),
    }
}

fn main() {
    if let Err(err) = main_result() {
        eprintln!("{}", err.to_string());
        std::process::exit(1);
    }
}
