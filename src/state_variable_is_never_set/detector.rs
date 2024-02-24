use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::{ASTNode, WorkspaceContext};
use aderyn_driver::core_ast::NodeID;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity, ReusableDetector};

use crate::state_variables_in_contract_definition::detector::StateVariablesInContractDefinitionDetector;

#[derive(Default)]
pub struct StateVariableIsNeverSetDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for StateVariableIsNeverSetDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // capture!(self, context, ast_node);

        context
            .contract_definitions()
            .into_iter()
            .for_each(|contract_definition| {
                let mut get_state_variables = StateVariablesInContractDefinitionDetector::default();
                let state_variables = get_state_variables
                    .detect(context, &[], &[contract_definition.into()])
                    .unwrap();
                for state_variable in state_variables {
                    if let ASTNode::VariableDeclaration(sv) = state_variable {
                        let references = context
                            .identifiers()
                            .into_iter()
                            .filter(|identifier| identifier.referenced_declaration == sv.id);
                        if references.count() == 0 {
                            capture!(self, context, sv.clone());
                        }
                    }
                }
            });
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Title for StateVariableIsNeverSetDetector")
    }

    fn description(&self) -> String {
        String::from("Description for StateVariableIsNeverSetDetector")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "state-variable-is-never-set".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod state_variable_is_never_set_tests {

    use crate::config_tests::tests_configuration;

    use super::StateVariableIsNeverSetDetector;

    use aderyn_driver::context::workspace_context::WorkspaceContext;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::IssueDetector;

    fn test_state_variable_is_never_set_for(
        _contract_file: String,
        context: WorkspaceContext,
        mut detector: impl IssueDetector,
    ) {
        // assert that the detector finds instances
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert!(detector.instances().len() == 1);
    }

    #[test]
    fn test_state_variable_is_never_set() {
        let detector = StateVariableIsNeverSetDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = StateVariableIsNeverSetDetector::default();
            let context = load_contract(&contract_file);
            test_state_variable_is_never_set_for(contract_file, context, detector);
        }
    }
}
