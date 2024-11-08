use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use processes::process_input;
use std::env;

mod processes;
mod state;
mod to_do;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state = read_file("./state.json");
    let status: String;

    match state.get(title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_owned();
        }
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);
}
