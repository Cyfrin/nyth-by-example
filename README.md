```
 _   ___   _______ _   _  ________   __  _______   __  ___  ___  _________ _      _____ 
| \ | \ \ / /_   _| | | | | ___ \ \ / / |  ___\ \ / / / _ \ |  \/  || ___ \ |    |  ___|
|  \| |\ V /  | | | |_| | | |_/ /\ V /  | |__  \ V / / /_\ \| .  . || |_/ / |    | |__  
| . ` | \ /   | | |  _  | | ___ \ \ /   |  __| /   \ |  _  || |\/| ||  __/| |    |  __| 
| |\  | | |   | | | | | | | |_/ / | |   | |___/ /^\ \| | | || |  | || |   | |____| |___ 
\_| \_/ \_/   \_/ \_| |_/ \____/  \_/   \____/\/   \/\_| |_/\_|  |_/\_|   \_____/\____/
```

# Where to Start?

For those learning how to use Nyth, take a look into...

1. [Foundry Workspace](./foundry_workspace/) - A playground to create Solidity contracts to test your detectors against.
2. Your first Reusable Detector [`StateVariablesInContractDefinition`](./src/state_variables_in_contract_definition/detector.rs) - Returns all state variables within a given ContractDefinition.
3. Your first Issue Detector [`StateVariableIsNeverUsed`](./src/state_variable_is_never_used/detector.rs).
4. [`config_tests.rs`](./src/config_tests.rs) - Define which contracts from the Foundry Workspace to pass into your detector tests.
5. `runner.rs` - Define which detectors should run when calling `cargo run`.

# Documentation

// TODO Documentation link

# Running the project

## Create a new Issue Detector

```
nyth new issue my_new_issue
```

## Create a new Reusable Detector

```
nyth new reusable my_new_reusable_thing
```

## Basic commands

```
cargo build
cargo test
cargo run
```