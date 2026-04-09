use crate::base::cmd::Cmd;
use clap::{ArgMatches, Command};

pub struct SubCmdList;

impl Cmd for SubCmdList {
    const NAME: &'static str = "list";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("list process")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        let all_processes = crate::models::processes::AllProcesses::load()?;
        for process in all_processes.process {
            println!("{}", process.pid);
        }
        Ok(())
    }
}
