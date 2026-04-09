use crate::base::cmd::Cmd;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct SubCmdAdd;

const KEY_NAME: &str = "NAME";
const KEY_URL: &str = "URL";
const KEY_MEMO: &str = "MEMO";

impl Cmd for SubCmdAdd {
    const NAME: &'static str = "add";

    fn subcommand() -> Command {
        Command::new(Self::NAME)
            .about("add destination")
            .disable_help_flag(true)
            .next_help_heading("Required")
            .arg(
                Arg::new(KEY_NAME)
                    .help("destination name")
                    .required(true)
                    .long("name")
                    .short('n'),
            )
            .arg(
                Arg::new(KEY_URL)
                    .help("slack incoming webhook url")
                    .required(true)
                    .long("url")
                    .short('u'),
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
        let name: &String = args.get_one(KEY_NAME).unwrap();
        let url: &String = args.get_one(KEY_URL).unwrap();
        let memo: Option<&String> = args.get_one(KEY_MEMO);

        let mut configure = crate::models::configure::Configure::load()?;

        if configure.resolve_destination(name).is_some() {
            return Err(format!(
                "There is a destination with the same name. (name: {name})"
            ));
        }

        let destination = crate::models::configure::Destination {
            name: name.to_string(),
            url: url.to_string(),
            memo: memo.map(|m| m.to_string()),
        };

        configure.destination.push(destination);
        configure.save()
    }
}
