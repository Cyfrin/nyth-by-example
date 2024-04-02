use aderyn_driver::detector::IssueDetector;
use aderyn_driver::driver::{drive_with, Args};

use crate::public_state_variables::detector::PublicStateVariablesDetector;
use crate::state_variable_is_never_used::detector::StateVariableIsNeverUsedDetector;

// `cargo run` will run this function
pub fn run() {
    // Subscribe the detectors to the driver.
    // This will run the detectors and generate a report on `cargo run`.
    // Only detectors that are subscribed here will be run.
    let subscriptions: Vec<Box<dyn IssueDetector>> = vec![
        Box::<StateVariableIsNeverUsedDetector>::default(),
        Box::<PublicStateVariablesDetector>::default(),
    ];

    drive_with(
        Args {
            root: "./foundry_workspace".to_string(),
            output: "report.md".to_string(),
            exclude: None,
            scope: None,
            no_snippets: false,
            stdout: false,
        },
        subscriptions,
    )
}
