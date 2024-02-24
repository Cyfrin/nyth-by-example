#[allow(unused_imports)]
use crate::bot_utils::{TestsConfig, TestsTarget};
use crate::{
    state_variable_is_never_set::detector::StateVariableIsNeverSetDetector,
    state_variables_in_contract_definition::detector::StateVariablesInContractDefinitionDetector,
};

// Look at the example below

pub fn tests_configuration() -> TestsConfig {
    vec![
        // Define your targets here. Example:
        TestsTarget::new("./foundry_workspace/out/Counter.sol/Counter.json")
            .with_reusable_detector(Box::<StateVariablesInContractDefinitionDetector>::default())
            .with_issue_detector(Box::<StateVariableIsNeverSetDetector>::default()),
    ]
    .into()
}
