// _   ___   _______ _   _  ________   __  _______   __  ___  ___  _________ _      _____
// | \ | \ \ / /_   _| | | | | ___ \ \ / / |  ___\ \ / / / _ \ |  \/  || ___ \ |    |  ___|
// |  \| |\ V /  | | | |_| | | |_/ /\ V /  | |__  \ V / / /_\ \| .  . || |_/ / |    | |__
// | . ` | \ /   | | |  _  | | ___ \ \ /   |  __| /   \ |  _  || |\/| ||  __/| |    |  __|
// | |\  | | |   | | | | | | | |_/ / | |   | |___/ /^\ \| | | || |  | || |   | |____| |___
// \_| \_/ \_/   \_/ \_| |_/ \____/  \_/   \____/\/   \/\_| |_/\_|  |_/\_|   \_____/\____/
//
//  _____                    ______     _            _
// |_   _|                   |  _  \   | |          | |
//   | | ___ ___ _   _  ___  | | | |___| |_ ___  ___| |_ ___  _ __
//   | |/ __/ __| | | |/ _ \ | | | / _ \ __/ _ \/ __| __/ _ \| '__|
//  _| |\__ \__ \ |_| |  __/ | |/ /  __/ ||  __/ (__| || (_) | |
//  \___/___/___/\__,_|\___| |___/ \___|\__\___|\___|\__\___/|_|
//
// This IssueDetector is responsible for finding state variables that are never used within a contract.
// The `detect` function does this following:
//
//   1. Accepts a WorkspaceContext as `context`.
//   2. Retrieves all ContractDefinition nodes from the `context`.
//   3. For each ContractDefinition, retrieves all state variables, by using the ReusableDetector: `StateVariablesInContractDefinitionDetector`.
//   4. For each state variable, checks if it is referenced within the contract, by searching for Identifier nodes that reference the state variable ID.
//   5. If no references are found, captures the state variable as an issue.
//   6. Returns true if any issues are found, otherwise false.
//

use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::{ASTNode, WorkspaceContext};
use aderyn_driver::core_ast::NodeID;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity, ReusableDetector};

use crate::state_variables_in_contract_definition::detector::StateVariablesInContractDefinitionDetector;

#[derive(Default)]
pub struct StateVariableIsNeverUsedDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableIsNeverUsedDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
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
        // Choose an appropriate title for the report
        String::from("State Variable is never used")
    }

    fn description(&self) -> String {
        // Choose an appropriate description for the report
        String::from("State variable is never used in the contract. This may be a mistake.")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "state-variable-is-never-set".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod state_variable_is_never_set_tests {

    use crate::config_tests::tests_configuration;

    use super::StateVariableIsNeverUsedDetector;

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
        let detector = StateVariableIsNeverUsedDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = StateVariableIsNeverUsedDetector::default();
            let context = load_contract(&contract_file);
            test_state_variable_is_never_set_for(contract_file, context, detector);
        }
    }
}
