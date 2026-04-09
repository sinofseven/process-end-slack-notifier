use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdRemove;

const KEY_PID: &str = "PID";

impl Cmd for SubCmdRemove {
    const NAME: &'static str = "remove";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("remove process")
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

        let mut all_processes = crate::models::processes::AllProcesses::load()?;

        let count_prev = all_processes.process.len();

        let processes: Vec<crate::models::processes::Process> = all_processes
            .process
            .iter()
            .filter(|p| &p.pid != pid)
            .cloned()
            .collect();

        let count_filtered = processes.len();

        if count_filtered == count_prev {
            Err(format!("Monitored process not found (pid: {})", pid))
        } else {
            all_processes.process = processes;
            all_processes.save()
        }
    }
}
