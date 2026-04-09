use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdShow;
const KEY_NAME: &'static str = "NAME";

impl Cmd for SubCmdShow {
    const NAME: &'static str = "show";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("show destination")
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

        let configure = crate::models::configure::Configure::load()?;

        if let Some(destination) = configure.resolve_destination(name) {
            println!("name: {}", destination.name);
            println!("url : {}", destination.url);
            if let Some(memo) = &destination.memo {
                println!("memo: {}", memo);
            }
            Ok(())
        } else {
            Err(format!("Destination not found (name: {})", name))
        }
    }
}
