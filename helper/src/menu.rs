use crate::{archiver, listener, task_creator};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

const OPTIONS: [&'static str; 4] = ["Run listener", "Create new task", "Archive tasks", "Exit"];

pub fn run_menu() {
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option:")
            .default(0)
            .items(&OPTIONS)
            .interact_on(&Term::stdout())
            .unwrap();
        match selection {
            0 => {
                listener::listen();
            }
            1 => {
                task_creator::create_task_wizard();
            }
            2 => {
                archiver::archive();
            }
            3 => {
                return;
            }
            _ => unreachable!(),
        }
    }
}
