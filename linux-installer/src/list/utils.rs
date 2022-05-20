use std::fmt::{Display, Formatter};
use tabled::{Tabled};

#[derive(Tabled)]
pub struct InstallTable<'a> {
    pub version: &'a String,
    pub location: &'a str,
    pub installed_on: String,
    pub id: String,
    pub up_to_date: UpToDate,
}


pub enum UpToDate {
    Yes,
    No(String),
}

impl Display for UpToDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpToDate::Yes => { write!(f, "Yes") }
            UpToDate::No(value) => { write!(f, "No(Latest: {})", value) }
        }
    }
}