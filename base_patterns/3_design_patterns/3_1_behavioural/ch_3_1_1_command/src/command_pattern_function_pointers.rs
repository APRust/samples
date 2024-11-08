// APPROACH: USING FUNCTION POINTERS
// We could follow another approach by creating each individual command as a
// different function and store function pointers to invoke these functions later
// at a different time. Since function pointers implement
// all three traits Fn, FnMut, and FnOnce we could as well pass and store closures
// instead of function pointers.

type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

pub struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback })
    }

    pub fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    pub fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

pub(crate) fn add_field() -> String {
    "add field".to_string()
}

pub(crate) fn remove_field() -> String {
    "remove field".to_string()
}
