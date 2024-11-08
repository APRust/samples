use std::fmt::{Display, Formatter};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported!", input_string),
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            TaskStatus::DONE => {
                format!("DONE")
            }
            TaskStatus::PENDING => {
                format!("PENDING")
            }
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::DONE => {
                write!(f, "DONE")
            }
            TaskStatus::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}
