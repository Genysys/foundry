use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use tracing::trace;

/// Gas reports
pub mod gas_report;

/// Coverage reports
pub mod coverage;

/// The Forge test runner
mod runner;
use ethers::types::U256;
pub use runner::ContractRunner;

/// Forge test runners for multiple contracts
mod multi_runner;
pub use multi_runner::{MultiContractRunner, MultiContractRunnerBuilder};

/// reexport
pub use foundry_common::traits::TestFilter;

pub mod result;

/// The Forge EVM backend
pub use foundry_evm::*;

/// Metadata on how to run fuzz/invariant tests
#[derive(Debug, Clone, Copy, Default)]
pub struct TestOptions {
    /// The number of test cases that must execute for each fuzz test
    pub fuzz_runs: u32,
    /// The maximum number of global test case rejections allowed
    /// by proptest, to be encountered during usage of `vm.assume`
    /// cheatcode.
    pub fuzz_max_local_rejects: u32,
    /// The maximum number of local test case rejections allowed
    /// by proptest, to be encountered during usage of `vm.assume`
    /// cheatcode.
    pub fuzz_max_global_rejects: u32,
    /// Optional seed for the fuzzing RNG algorithm
    pub fuzz_seed: Option<U256>,
    /// The number of runs that must execute for each invariant test group.
    pub invariant_runs: u32,
    /// The number of calls executed to attempt to break invariants in one run.
    pub invariant_depth: u32,
    /// Fails the invariant fuzzing if a revert occurs
    pub invariant_fail_on_revert: bool,
    /// Allows overriding an unsafe external call when running invariant tests. eg. reetrancy
    /// checks
    pub invariant_call_override: bool,
}

impl TestOptions {
    pub fn fuzzer(&self) -> TestRunner {
        // TODO: Add Options to modify the persistence
        let cfg = proptest::test_runner::Config {
            failure_persistence: None,
            cases: self.fuzz_runs,
            max_local_rejects: self.fuzz_max_local_rejects,
            max_global_rejects: self.fuzz_max_global_rejects,
            ..Default::default()
        };

        if let Some(ref fuzz_seed) = self.fuzz_seed {
            trace!(target: "forge::test", "building deterministic fuzzer with seed {}", fuzz_seed);
            let mut bytes: [u8; 32] = [0; 32];
            fuzz_seed.to_big_endian(&mut bytes);
            let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &bytes);
            proptest::test_runner::TestRunner::new_with_rng(cfg, rng)
        } else {
            trace!(target: "forge::test", "building stochastic fuzzer");
            proptest::test_runner::TestRunner::new(cfg)
        }
    }
}
