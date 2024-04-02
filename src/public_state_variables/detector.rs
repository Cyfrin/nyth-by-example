use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::WorkspaceContext;
use aderyn_driver::core_ast::{NodeID, Visibility};
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity};

#[derive(Default)]
pub struct PublicStateVariablesDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for PublicStateVariablesDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        context
            .variable_declarations()
            .into_iter()
            .for_each(|variable_declaration| {
                if variable_declaration.state_variable
                    && variable_declaration.visibility == Visibility::Public
                {
                    capture!(self, context, variable_declaration);
                }
            });

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Public State Variable")
    }

    fn description(&self) -> String {
        String::from("This detector simply finds public state variables, and nothing more!")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "public-state-variables".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod public_state_variables_tests {

    use crate::config_tests::tests_configuration;

    use super::PublicStateVariablesDetector;

    use aderyn_driver::context::workspace_context::WorkspaceContext;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::IssueDetector;

    fn test_public_state_variables_for(
        _contract_file: String,
        context: WorkspaceContext,
        mut detector: impl IssueDetector,
    ) {
        // assert that the detector finds instances
        let found = detector.detect(&context).unwrap();
        assert!(found);
    }

    #[test]
    fn test_public_state_variables() {
        let detector = PublicStateVariablesDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = PublicStateVariablesDetector::default();
            let context = load_contract(&contract_file);
            test_public_state_variables_for(contract_file, context, detector);
        }
    }
}
