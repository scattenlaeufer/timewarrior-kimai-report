use futures::future::try_join_all;
use kimai::{load_config, log_timesheet_record, Config};
use std::fmt;
use std::process::Command;
use timewarrior_report::{Session, TimewarriorData};

#[derive(Debug)]
pub enum ReportError {
    Kimai(String),
    Timewarrior(String),
    ParseInt(String),
    IO(String),
    Other(String),
}

impl std::error::Error for ReportError {}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Kimai(e) => write!(f, "Kimai Error: {}", e),
            Self::Timewarrior(e) => write!(f, "Timewarrior Error: {}", e),
            Self::ParseInt(e) => write!(f, "Parse Int Error: {}", e),
            Self::IO(e) => write!(f, "IO Error: {}", e),
            Self::Other(e) => write!(f, "Other Error: {}", e),
        }
    }
}

impl From<timewarrior_report::ReportError> for ReportError {
    fn from(error: timewarrior_report::ReportError) -> Self {
        Self::Timewarrior(error.to_string())
    }
}

impl From<kimai::KimaiError> for ReportError {
    fn from(error: kimai::KimaiError) -> Self {
        Self::Kimai(error.to_string())
    }
}

impl From<std::num::ParseIntError> for ReportError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseInt(error.to_string())
    }
}

impl From<std::io::Error> for ReportError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.to_string())
    }
}

fn parse_kimai_id(input: &str, identifier: &str) -> Result<Option<usize>, ReportError> {
    if input.starts_with(identifier) {
        Ok(Some(input.split(':').collect::<Vec<_>>()[1].parse()?))
    } else {
        Ok(None)
    }
}

async fn log_session(config: &Config, session: Session) -> Result<(), ReportError> {
    let mut kimai_project: Option<usize> = None;
    let mut kimai_activity: Option<usize> = None;
    let mut kimai_id: Option<usize> = None;
    let mut tags = Vec::new();

    for tag in &session.tags {
        if let Some(i) = parse_kimai_id(tag, "kimai_project")? {
            kimai_project = Some(i);
        } else if let Some(i) = parse_kimai_id(tag, "kimai_activity")? {
            kimai_activity = Some(i);
        } else if let Some(i) = parse_kimai_id(tag, "kimai_id")? {
            kimai_id = Some(i);
        } else {
            tags.push(tag.clone());
        }
    }

    if let Some(id) = kimai_id {
        println!("@{}: already got logged with ID {}", session.id, id);
    } else if let (Some(project_id), Some(activity_id)) = (kimai_project, kimai_activity) {
        let record = log_timesheet_record(
            &config,
            0,
            project_id,
            activity_id,
            session.start,
            session.end,
            session.annotation,
            Some(tags),
        )
        .await?;
        let _cmd_result = Command::new("timew")
            .arg("tag")
            .arg(format!("@{}", session.id))
            .arg(format!("kimai_id:{}", record.id))
            .output()?;
        println!("@{}: logged to Kimai", session.id);
    } else {
        println!("@{}: required IDs not found!", session.id);
    }
    Ok(())
}

#[tokio::main]
pub async fn run(config_path: Option<String>) -> Result<(), ReportError> {
    let config = load_config(config_path)?;
    let timewarrior_data = TimewarriorData::from_stdin()?;

    let mut future_vec = Vec::new();
    for session in timewarrior_data.sessions {
        future_vec.push(log_session(&config, session))
    }
    let results = try_join_all(future_vec).await;
    results?;

    Ok(())
}
