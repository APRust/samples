use super::super::enums::TaskStatus;
use super::base::Base;
use crate::to_do::traits::create::Create;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        Self {
            super_struct: Base {
                title: input_title.to_string(),
                status: TaskStatus::PENDING,
            },
        }
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Edit for Pending {}
