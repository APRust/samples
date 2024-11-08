use super::super::enums::TaskStatus;
use super::base::Base;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        Self {
            super_struct: Base {
                title: input_title.to_string(),
                status: TaskStatus::DONE,
            },
        }
    }
}

impl Delete for Done {}
impl Get for Done {}
impl Edit for Done {}
