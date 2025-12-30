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
    oracles::ADVANCED_ARITHMETIC_BUG_IDX,
    srcmap::SOURCE_MAP_PROVIDER,
    types::{EVMAddress, EVMFuzzState, EVMOracleCtx, EVMQueueExecutor, EVMU256},
    vm::EVMState,
};
use crate::oracle::{Oracle, OracleCtx};
use crate::oracle_should_skip;
use crate::state::HasExecutionResult;

/// Advanced arithmetic oracle (placeholder implementation).
///
/// This is currently a no-op oracle that conforms to ItyFuzz's `Oracle` trait.
/// The original enhancement stub used a different oracle interface (pre/post hooks).
pub struct AdvancedArithmeticOracle;

impl AdvancedArithmeticOracle {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AdvancedArithmeticOracle {
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
    > for AdvancedArithmeticOracle
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
        if ctx.post_state.integer_overflow.is_empty() {
            return vec![];
        }

        ctx.post_state
            .integer_overflow
            .iter()
            .map(|(contract, pc, kind)| {
                let mut hasher = DefaultHasher::new();
                contract.hash(&mut hasher);
                pc.hash(&mut hasher);
                kind.hash(&mut hasher);
                let real_bug_idx = (hasher.finish() << 8) + ADVANCED_ARITHMETIC_BUG_IDX;

                if oracle_should_skip!(ctx, real_bug_idx) {
                    return None;
                }

                EVMBugResult::new(
                    "Advanced Arithmetic".to_string(),
                    real_bug_idx,
                    format!("Possible arithmetic issue ({}) at {:?}:{:?}", kind, contract, pc),
                    ConciseEVMInput::from_input(ctx.input, ctx.fuzz_state.get_execution_result()),
                    SOURCE_MAP_PROVIDER
                        .lock()
                        .unwrap()
                        .get_raw_source_map_info(contract, *pc),
                    Some(format!("{:?}", contract)),
                )
                .push_to_output();

                Some(real_bug_idx)
            })
            .flatten()
            .unique()
            .collect_vec()
    }
}