mod base;
mod models;
mod process;

mod cmd;
pub mod slack;

use base::cmd::Cmd;
use clap::command;
use cmd::{CmdCheck, CmdDestination, CmdProcess};

fn main() -> Result<(), String> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(CmdDestination::subcommand())
        .subcommand(CmdProcess::subcommand())
        .subcommand(CmdCheck::subcommand())
        .get_matches();

    match matches.subcommand() {
        Some((CmdDestination::NAME, args)) => CmdDestination::run(args),
        Some((CmdCheck::NAME, args)) => CmdCheck::run(args),
        Some((CmdProcess::NAME, args)) => CmdProcess::run(args),
        _ => unreachable!(""),
    }
}
