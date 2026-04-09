use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdShow;

const KEY_PID: &'static str = "PID";

impl Cmd for SubCmdShow {
    const NAME: &'static str = "show";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("show monitored process")
            .arg_required_else_help(true)
            .disable_help_flag(true)
            .next_help_heading("Required")
            .arg(
                Arg::new(KEY_PID)
                    .help("process id")
                    .required(true)
                    .long("pid")
                    .short('p')
                    .value_parser(clap::value_parser!(u32)),
            )
            .next_help_heading("Options")
            .arg(
                Arg::new("help")
                    .short('h')
                    .long("help")
                    .help("Print help")
                    .action(ArgAction::Help),
            )
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let pid: &u32 = args.get_one(KEY_PID).unwrap();

        let all_processes = crate::models::processes::AllProcesses::load()?;

        if let Some(process) = all_processes.get_process_info(pid) {
            println!("pid        : {}", process.pid);
            println!("command    : {}", process.command);
            println!("cwd        : {}", process.cwd);
            println!("destination: {}", process.destination);
            if let Some(memo) = &process.memo {
                println!("memo       : {}", memo);
            }
            Ok(())
        } else {
            Err(format!("Monitored process not found (name: {})", pid))
        }
    }
}
