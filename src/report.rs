use crate::Result;
use serde::de::DeserializeOwned;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Report {
    /// The entire report, as text
    pub text: String,
}

pub trait Reporter<T>
where
    T: DeserializeOwned,
{
    fn new(report: Report) -> Self;

    /// The report associated to the reporter
    fn report(&self) -> &Report;

    fn deserialize<U>(&self, text: &String) -> Result<U>
    where
        U: DeserializeOwned,
    {
        serde_json::from_str::<U>(text)
            .map_err(|_err| Error::new(ErrorKind::Other, "Failed when deserializing"))
    }

    /// The tool is inferred from the report
    fn tool(&self) -> Result<T>;
}
