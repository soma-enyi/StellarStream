#[allow(clippy::too_many_arguments)]
mod v1_interface_inner {
    soroban_sdk::contractimport!(
        file = "../Contract-V1/target/wasm32-unknown-unknown/release/stellarstream_contracts.wasm"
    );
}

pub use v1_interface_inner::Client;
