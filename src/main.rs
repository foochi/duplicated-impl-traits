#[macro_use]
extern crate serde_derive;

mod operator;
mod report;
mod supervisor;

use report::Reporter;

pub type Result<T> = ::std::result::Result<T, ::std::io::Error>;

fn main() {
    // Operator

    let _message_a = r#"
    { "operator": { "tool_a": "a111a" } }
    "#
    .to_string();

    let message_b = r#"
    { "operator": { "tool_b": "2b2bb", "scale": 400 } }
    "#
    .to_string();

    let api_operator = operator::Api {};
    let reporter_b = api_operator
        .tool_b(message_b)
        .expect("Error while processing report b");

    let report_b = reporter_b.report();
    let tool_b = reporter_b.tool().expect("Error while processing tool b");
    println!("Operator report: {} => {}", report_b.text, tool_b.tool_b);

    // Supervisor

    let message_x = r#"
    { "supervisor": { "tool_x": "xXxxxX", "number": 5 } }
    "#
    .to_string();

    let _message_y = r#"
    { "supervisor": { "tool_y": "yYYYy" } }
    "#
    .to_string();

    let api_supervisor = supervisor::Api {};
    let reporter_x = api_supervisor
        .tool_x(message_x)
        .expect("Error while processing report x");

    let report_x = reporter_x.report();
    let tool_x = reporter_x.tool().expect("Error while processing tool x");
    println!("Supervisor report: {} => {}", report_x.text, tool_x.tool_x);
}
