use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bytes::Bytes;
use itertools::Itertools;
use revm_primitives::Bytecode;

use crate::evm::{
    input::{ConciseEVMInput, EVMInput},
    oracle::EVMBugResult,
    oracles::CROSS_CONTRACT_BUG_IDX,
    types::{EVMAddress, EVMFuzzState, EVMOracleCtx, EVMQueueExecutor, EVMU256},
    vm::EVMState,
};
use crate::oracle::{Oracle, OracleCtx};
use crate::oracle_should_skip;
use crate::state::HasExecutionResult;

/// Cross-contract oracle (placeholder implementation).
///
/// The enhancement stub was written against a different oracle interface.
pub struct CrossContractOracle;

impl CrossContractOracle {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CrossContractOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl
    Oracle<
        EVMState,
        EVMAddress,
        Bytecode,
        Bytes,
        EVMAddress,
        EVMU256,
        Vec<u8>,
        EVMInput,
        EVMFuzzState,
        ConciseEVMInput,
        EVMQueueExecutor,
    > for CrossContractOracle
{
    fn transition(&self, _ctx: &mut EVMOracleCtx<'_>, _stage: u64) -> u64 {
        0
    }

    fn oracle(
        &self,
        ctx: &mut OracleCtx<
            EVMState,
            EVMAddress,
            Bytecode,
            Bytes,
            EVMAddress,
            EVMU256,
            Vec<u8>,
            EVMInput,
            EVMFuzzState,
            ConciseEVMInput,
            EVMQueueExecutor,
        >,
        _stage: u64,
    ) -> Vec<u64> {
        // Heuristic: report when a single execution mutates storage for many contracts.
        // This tends to indicate complex cross-contract logic and is a good starting signal.
        let mut changed_contracts: Vec<EVMAddress> = vec![];

        // Collect candidates from both pre and post.
        let candidates = ctx
            .pre_state
            .state
            .keys()
            .chain(ctx.post_state.state.keys())
            .copied()
            .unique()
            .collect_vec();

        for addr in candidates {
            let pre_slots = ctx.pre_state.state.get(&addr);
            let post_slots = ctx.post_state.state.get(&addr);

            if pre_slots != post_slots {
                changed_contracts.push(addr);
            }
        }

        // Avoid reporting on single-contract writes (very common).
        if changed_contracts.len() < 3 {
            return vec![];
        }

        // Stable bug id based on which contracts were mutated.
        let mut hasher = DefaultHasher::new();
        for a in changed_contracts.iter().map(|a| format!("{:?}", a)).sorted() {
            a.hash(&mut hasher);
        }
        let real_bug_idx = (hasher.finish() << 8) + CROSS_CONTRACT_BUG_IDX;
        if oracle_should_skip!(ctx, real_bug_idx) {
            return vec![];
        }

        let changed_preview = changed_contracts
            .iter()
            .take(5)
            .map(|a| format!("{:?}", a))
            .collect_vec()
            .join(", ");

        EVMBugResult::new_simple(
            "Cross-Contract".to_string(),
            real_bug_idx,
            format!(
                "State mutated across {} contracts (e.g. {})",
                changed_contracts.len(),
                changed_preview
            ),
            ConciseEVMInput::from_input(ctx.input, ctx.fuzz_state.get_execution_result()),
        )
        .push_to_output();

        vec![real_bug_idx]
    }
}