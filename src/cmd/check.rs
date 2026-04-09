use crate::base::cmd::Cmd;
use clap::{ArgMatches, Command};

pub struct CmdCheck;

impl Cmd for CmdCheck {
    const NAME: &'static str = "check";

    fn subcommand() -> Command {
        Command::new(Self::NAME).about("check all monitored processes")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        let mut all_processes = crate::models::processes::AllProcesses::load()?;
        let configure = crate::models::configure::Configure::load()?;

        let all_monitored_pids: Vec<u32> = all_processes.process.iter().map(|p| p.pid).collect();
        let all_terminated_pids = crate::process::find_terminated_pids(&all_monitored_pids);

        notify_all(&all_terminated_pids, &configure, &mut all_processes)
    }
}

fn notify_all(
    all_terminated_pids: &[u32],
    configure: &crate::models::configure::Configure,
    all_processes: &mut crate::models::processes::AllProcesses,
) -> Result<(), String> {
    let mut all_errors: Vec<String> = Vec::new();

    for pid in all_terminated_pids {
        if let Err(error) = notify(*pid, configure, all_processes) {
            all_errors.push(error);
        }
    }

    if !all_errors.is_empty() {
        Err(all_errors.join("\n"))
    } else {
        Ok(())
    }
}

fn notify(
    pid: u32,
    configure: &crate::models::configure::Configure,
    all_processes: &mut crate::models::processes::AllProcesses,
) -> Result<(), String> {
    let process = all_processes.get_process_info(&pid).unwrap();

    let destination = if let Some(destination) = configure.resolve_destination(&process.destination)
    {
        destination
    } else {
        return Err(format!(
            "Destination not found (name: {})",
            process.destination
        ));
    };

    crate::slack::send_slack(&destination.url, process)?;

    all_processes.process = all_processes
        .process
        .iter()
        .filter(|p| p.pid != pid)
        .cloned()
        .collect();

    all_processes.save()
}
