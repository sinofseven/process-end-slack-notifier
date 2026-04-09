use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdRemove;
const KEY_NAME: &'static str = "NAME";

impl Cmd for SubCmdRemove {
    const NAME: &'static str = "remove";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("remove destination")
            .arg_required_else_help(true)
            .disable_help_flag(true)
            .next_help_heading("Required")
            .arg(
                Arg::new(KEY_NAME)
                    .help("destination name")
                    .required(true)
                    .long("name")
                    .short('n'),
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
        let name: &String = args.get_one(KEY_NAME).unwrap();

        let mut configure = crate::models::configure::Configure::load()?;
        let all_processes = crate::models::processes::AllProcesses::load()?;

        if let Some(_) = all_processes
            .process
            .iter()
            .find(|p| p.destination.eq(name))
        {
            return Err(format!(
                "The destination is currently in use. (name: {name})"
            ));
        }

        let count_prev = configure.destination.len();

        let destinations: Vec<crate::models::configure::Destination> = configure
            .destination
            .iter()
            .filter(|&d| d.name.ne(name))
            .map(|d| d.clone())
            .collect();

        let count_filtered = destinations.len();

        if count_filtered == count_prev {
            Err(format!("Destination not found (name: {})", name))
        } else {
            configure.destination = destinations;
            configure.save()
        }
    }
}
