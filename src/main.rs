use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}


fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    // Perform the action.
    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
        Edit  => tasks::edit_task(journal_file),
        Delete { position } => tasks::complete_task(journal_file, position),
        Change{ position, task } => tasks::list_change(journal_file, position, Task::new(task))


    }?;

    Ok(())

}