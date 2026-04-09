mod add;
mod list;
mod remove;
mod show;

use crate::base::cmd::Cmd;
use clap::{ArgMatches, Command};

use add::SubCmdAdd;
use list::SubCmdList;
use remove::SubCmdRemove;
use show::SubCmdShow;

pub struct CmdDestination;

impl Cmd for CmdDestination {
    const NAME: &'static str = "destination";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("destination command (slack incoming webhook)")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SubCmdList::subcommand())
            .subcommand(SubCmdAdd::subcommand())
            .subcommand(SubCmdRemove::subcommand())
            .subcommand(SubCmdShow::subcommand())
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            Some((SubCmdList::NAME, sub_args)) => SubCmdList::run(sub_args),
            Some((SubCmdAdd::NAME, sub_args)) => SubCmdAdd::run(sub_args),
            Some((SubCmdRemove::NAME, sub_args)) => SubCmdRemove::run(sub_args),
            Some((SubCmdShow::NAME, sub_args)) => SubCmdShow::run(sub_args),
            _ => unreachable!("This is Bug!"),
        }
    }
}
