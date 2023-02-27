use std::{
    error::Error,
    process::{ExitCode, Termination},
};

use crate::profile::AboutMe;

pub struct NewJob {}

pub type MyNewJob = Result<NewJob, Box<dyn Error>>;

impl Termination for NewJob {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}

pub async fn find_new_job(_profile: AboutMe<'_>) -> Result<NewJob, Box<dyn Error>> {
    Ok(NewJob {})
}
