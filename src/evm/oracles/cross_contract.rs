use bytes::Bytes;
use revm_primitives::Bytecode;

use crate::evm::{
    input::{ConciseEVMInput, EVMInput},
    types::{EVMAddress, EVMFuzzState, EVMOracleCtx, EVMQueueExecutor, EVMU256},
    vm::EVMState,
};
use crate::oracle::{Oracle, OracleCtx};

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
        _ctx: &mut OracleCtx<
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
        vec![]
    }
}