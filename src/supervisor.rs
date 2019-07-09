use crate::report::{Report, Reporter};
use crate::Result;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

#[derive(Debug, Deserialize)]
pub struct ToolX {
    pub tool_x: String,
    pub number: u8,
}

#[derive(Debug, Deserialize)]
pub struct ToolY {
    pub tool_y: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiWrapper<T> {
    pub supervisor: T,
}

#[derive(Debug)]
pub struct Supervisor<T>
where
    T: DeserializeOwned,
{
    report: Report,
    report_tool: PhantomData<T>,
}

impl<T> Reporter<T> for Supervisor<T>
where
    T: DeserializeOwned,
{
    fn new(report: Report) -> Supervisor<T> {
        Self {
            report,
            report_tool: PhantomData,
        }
    }

    // NOTE 1: This cannot be implemented inside the trait because it does not have any associated field
    fn report(&self) -> &Report {
        &self.report
    }

    fn tool(&self) -> Result<T> {
        let text = &self.report.text;
        let reporter = self.deserialize::<ApiWrapper<T>>(&text)?;
        let reporter = reporter.supervisor;

        match &reporter {
            ToolX => println!("INFO: a report of X has been received"),
        };

        Ok(reporter)
    }
}

impl<T> Supervisor<T>
where
    T: DeserializeOwned,
{
    // NOTE 2: This method cannot be accessed
    pub fn review() {
        println!("REVIEWED");
    }
}

pub struct Api {}

impl Api {
    fn process<T>(&self, message: String) -> Result<Supervisor<T>>
    where
        T: DeserializeOwned,
    {
        let report = Report { text: message };
        println!("Incoming report");

        Ok(Supervisor::new(report))
    }

    // This methods is generated, so it cannot use a specific implementation of Reporter
    pub fn tool_x(&self, message: String) -> Result<impl Reporter<ToolX>> {
        self.process::<ToolX>(message)
    }

    // This methods is generated, so it cannot use a specific implementation of Reporter
    pub fn tool_y(&self, message: String) -> Result<impl Reporter<ToolY>> {
        println!("WARNING: Y tool is deprecated!");
        self.process::<ToolY>(message)
    }
}
