use bytes::Bytes;
use revm_primitives::Bytecode;

use crate::evm::{
    input::{ConciseEVMInput, EVMInput},
    types::{EVMAddress, EVMFuzzState, EVMOracleCtx, EVMQueueExecutor, EVMU256},
    vm::EVMState,
};
use crate::oracle::{Oracle, OracleCtx};

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