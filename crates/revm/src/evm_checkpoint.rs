use crate::primitives::{Env, Spec};
use crate::{Database, EVMData, EVMImpl, Inspector, JournaledState};
use revm_interpreter::{InstructionResult, Interpreter};
use revm_precompile::Precompiles;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct EvmCheckpoint<GSPEC: Spec, DB: Database, const INSPECT: bool> {
    pub journaled_state: JournaledState,
    pub precompiles: Precompiles,

    interpreters: Vec<Box<Interpreter>>,
    last_result: Option<InstructionResult>,
    _phantom: PhantomData<(GSPEC, DB)>,
}

impl<GSPEC: Spec, DB: Database, const INSPECT: bool> EvmCheckpoint<GSPEC, DB, INSPECT> {
    pub fn recover<'a>(&self, env: &'a mut Env, db: &'a mut DB, inspector: &'a mut dyn Inspector<DB>) -> EVMImpl<'a, GSPEC, DB, INSPECT> {
        EVMImpl {
            data: EVMData {
                env,
                db,

                journaled_state: self.journaled_state.clone(),
                precompiles: self.precompiles.clone(),
                interpreters: self.interpreters.clone(),
                last_result: self.last_result,
                error: None,
            },
            inspector,
            _phantomdata: Default::default(),
        }
    }
}
