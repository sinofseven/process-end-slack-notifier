use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdAdd;

const KEY_PID: &'static str = "PID";
const KEY_DESTINATION: &'static str = "DESTINATION";
const KEY_MEMO: &'static str = "MEMO";

impl Cmd for SubCmdAdd {
    const NAME: &'static str = "add";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("add monitored process")
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
            .arg(
                Arg::new(KEY_DESTINATION)
                    .help("destination name")
                    .required(true)
                    .long("destination")
                    .short('d'),
            )
            .next_help_heading("Options")
            .arg(
                Arg::new(KEY_MEMO)
                    .help("memo")
                    .required(false)
                    .long("memo")
                    .short('m'),
            )
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
        let destination: &String = args.get_one(KEY_DESTINATION).unwrap();
        let memo: Option<&String> = args.get_one(KEY_MEMO);

        let mut all_processes = crate::models::processes::AllProcesses::load()?;
        if let Some(_) = all_processes.get_process_info(pid) {
            return Err(format!(
                "There is a monitored process with the same pid. (pid: {})",
                pid
            ));
        }

        let configure = crate::models::configure::Configure::load()?;
        if let None = configure.resolve_destination(destination) {
            return Err(format!("Destination not found (name: {destination})"));
        }

        let info = crate::process::resolve_process(*pid)?;

        let process = crate::models::processes::Process {
            pid: *pid,
            cwd: format!("{}", info.cwd.display()),
            command: info.command,
            destination: destination.to_string(),
            memo: memo.map(|m| m.to_string()),
        };

        all_processes.process.push(process);
        all_processes.save()
    }
}
