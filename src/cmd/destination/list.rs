use crate::base::cmd::Cmd;
use clap::{ArgMatches, Command};

pub struct SubCmdList;

impl Cmd for SubCmdList {
    const NAME: &'static str = "list";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("list destination")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        let configure = crate::models::configure::Configure::load()?;
        for destination in configure.destination {
            println!("{}", destination.name);
        }
        Ok(())
    }
}
