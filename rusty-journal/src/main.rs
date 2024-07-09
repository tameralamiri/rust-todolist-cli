use anyhow::{anyhow, Ok};
use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
fn main() -> anyhow::Result<()> {
    // Get the comand-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file path.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find a journal file"))?;

    // Perform the action.
    match action {
        Add { task } => {
            let task = Task::new(task);
            tasks::add_task(journal_file, task)
        }
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }?;
    Ok(())
}
