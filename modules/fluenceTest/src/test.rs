use crate::main::{greeting};

use marine_rs_sdk_test::marine_test;


#[marine_test(
    config_path = "../../../.fluence/tmp/Config.toml",
    modules_dir = "../../../target/wasm32-wasi/release"
)]

fn test_dapp() {
    let greeting = greeting();
    assert_eq!(greeting, "Hello, test deployment".to_string());
}