use std::error::Error;

use aderyn_driver::context::browser::ExtractVariableDeclarations;
use aderyn_driver::context::workspace_context::ASTNode;
use aderyn_driver::context::workspace_context::WorkspaceContext;
use aderyn_driver::detector::ReusableDetector;

#[derive(Default)]
pub struct StateVariablesInContractDefinitionDetector {
    found_instances: Vec<ASTNode>,
}

impl ReusableDetector for StateVariablesInContractDefinitionDetector {
    fn detect(
        &mut self,
        _: &WorkspaceContext,
        _: &[ASTNode],
        within: &[ASTNode],
    ) -> Result<&[ASTNode], Box<dyn std::error::Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // self.found_instances.push(my_ast_node.into());
        for node in within {
            if let ASTNode::ContractDefinition(contract_definition) = node.clone() {
                let variables = ExtractVariableDeclarations::from(&contract_definition).extracted;
                for variable in variables {
                    if variable.state_variable {
                        self.found_instances.push(variable.into());
                    }
                }
            }
        }

        Ok(&self.found_instances)
    }

    fn name(&self) -> String {
        "state-variables-in-contract-definition".to_string()
    }
}

#[cfg(test)]
mod state_variables_in_contract_definition_tests {

    use crate::config_tests::tests_configuration;

    use super::StateVariablesInContractDefinitionDetector;

    use aderyn_driver::context::workspace_context::WorkspaceContext;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::ReusableDetector;

    fn test_state_variables_in_contract_definition_for(
        _contract_file: String,
        context: WorkspaceContext,
        mut detector: impl ReusableDetector,
    ) {
        // assert that the detector finds instances
        let contract_definition_ast = context
            .nodes
            .get(&context.contract_definitions()[0].id)
            .unwrap()
            .clone();
        let contracts = vec![contract_definition_ast];
        let results = detector.detect(&context, &Vec::new(), &contracts).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_state_variables_in_contract_definition() {
        let detector = StateVariablesInContractDefinitionDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = StateVariablesInContractDefinitionDetector::default();
            let context = load_contract(&contract_file);
            test_state_variables_in_contract_definition_for(contract_file, context, detector);
        }
    }
}
