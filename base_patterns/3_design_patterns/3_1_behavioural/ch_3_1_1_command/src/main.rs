// DESCRIPTION
// The basic idea of the Command pattern is to separate out actions into its own
// objects and pass them as parameters.
//
// MOTIVATION
// Suppose we have a sequence of actions or transactions encapsulated as objects.
// We want these actions or commands to be executed or invoked in some order later
// at different time. These commands may also be triggered as a result of some event.
// For example, when a user pushes a button, or on arrival of a data packet.
// In addition, these commands might be undoable.
// This may come in useful for operations of an editor.
// We might want to store logs of executed commands so that we could reapply
// the changes later if the system crashes.
//
// EXAMPLE
// Define two database operations create table and add field.
// Each of these operations is a command which knows how to undo the command, e.g.,
// drop table and remove field. When a user invokes a database migration operation
// then each command is executed in the defined order, and when the user invokes
// the rollback operation then the whole set of commands is invoked in reverse order.

use crate::command_pattern_fn_trait_objects as fn_trait_obj;
use crate::command_pattern_function_pointers as func_pointers;
use crate::command_pattern_trait_objects as trait_obj;

mod command_pattern_function_pointers;

mod command_pattern_fn_trait_objects;
mod command_pattern_trait_objects;

fn main() {
    // APPROACH: USING TRAIT OBJECTS
    // We define a common trait which encapsulates our command with two operations
    // `execute` and `rollback`. All command structs must implement this trait.
    let mut schema = trait_obj::Schema::new();

    let cmd = Box::new(trait_obj::CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(trait_obj::AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());

    // APPROACH: USING FUNCTION POINTERS
    let mut schema = func_pointers::Schema::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(func_pointers::add_field, func_pointers::remove_field);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());

    // APPROACH: USING FN TRAIT OBJECTS
    // Finally, instead of defining a common command trait we could store
    // each command implementing the Fn trait separately in vectors.

    let mut schema = fn_trait_obj::Schema::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(fn_trait_obj::add_field, fn_trait_obj::remove_field);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

// If our commands are small and may be defined as functions or passed as a closure
// then using function pointers might be preferable since it does not exploit
// dynamic dispatch. But if our command is a whole struct with a bunch of functions
// and variables defined as separated module then using trait objects would be more
// suitable. A case of application can be found in actix, which uses trait objects
// when it registers a handler function for routes. In case of using Fn trait objects
// we can create and use commands in the same way as we used in case of function pointers.
//
// As performance, there is always a trade-off between performance and code simplicity
// and organisation. Static dispatch gives faster performance, while dynamic
// dispatch provides flexibility when we structure our application.
