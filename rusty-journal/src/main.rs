use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;
fn main() {
    // Get the comand-line arguments.
    let CommandLineArgs { action, journal_file } = CommandLineArgs::from_args();
    
    // Unpack the journal file path.
    let journal_file = journal_file.expect("Failed to find journal file");

    // Perform the action.
    match action {
        Add { task } => {
            let task = Task::new(task);
            tasks::add_task(journal_file, task).expect("Failed to add task");
        }
        Done { position } => {
            tasks::complete_task(journal_file, position).expect("Failed to complete task");
        }
        List => {
            tasks::list_tasks(journal_file).expect("Failed to list tasks");
        }
        
    }
}
