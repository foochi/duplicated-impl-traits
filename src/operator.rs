use crate::report::{Report, Reporter};
use crate::Result;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

#[derive(Debug, Deserialize)]
pub struct ToolA {
    pub tool_a: String,
}

#[derive(Debug, Deserialize)]
pub struct ToolB {
    pub tool_b: String,
    pub scale: u16,
}

#[derive(Debug, Deserialize)]
pub struct ApiWrapper<T> {
    pub operator: T,
}

#[derive(Debug)]
pub struct Operator<T>
where
    T: DeserializeOwned,
{
    report: Report,
    report_tool: PhantomData<T>,
}

impl<T> Reporter<T> for Operator<T>
where
    T: DeserializeOwned,
{
    fn new(report: Report) -> Operator<T> {
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
        Ok(reporter.operator)
    }
}

pub struct Api {}

impl Api {
    fn process<T>(&self, message: String) -> Result<Operator<T>>
    where
        T: DeserializeOwned,
    {
        let report = Report { text: message };
        Ok(Operator::new(report))
    }

    // This methods is generated, so it cannot use a specific implementation of Reporter
    pub fn tool_a(&self, message: String) -> Result<impl Reporter<ToolA>> {
        println!("WARNING: A tool is unstable!");
        self.process::<ToolA>(message)
    }

    // This methods is generated, so it cannot use a specific implementation of Reporter
    pub fn tool_b(&self, message: String) -> Result<impl Reporter<ToolB>> {
        self.process::<ToolB>(message)
    }
}
