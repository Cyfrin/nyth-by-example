#[allow(unused_imports)]
use crate::bot_utils::{TestsConfig, TestsTarget};
use crate::{
    public_state_variables::detector::PublicStateVariablesDetector,
    state_variable_is_never_used::detector::StateVariableIsNeverUsedDetector,
    state_variables_in_contract_definition::detector::StateVariablesInContractDefinitionDetector,
};

pub fn tests_configuration() -> TestsConfig {
    vec![
        // This test configuration passes the Counter.sol contract to the
        // StateVariablesInContractDefinitionDetector and the StateVariableIsNeverUsedDetector tests.
        TestsTarget::new("./foundry_workspace/out/Counter.sol/Counter.json")
            .with_reusable_detector(Box::<StateVariablesInContractDefinitionDetector>::default())
            .with_issue_detector(Box::<StateVariableIsNeverUsedDetector>::default())
            .with_issue_detector(Box::<PublicStateVariablesDetector>::default()),
    ]
    .into()
}
