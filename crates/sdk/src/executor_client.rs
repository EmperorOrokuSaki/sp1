pub struct ExecutorClient {}

use sp1_primitives::io::SP1PublicValues;
use std::env;
use sp1_core_machine::io::SP1Stdin;
use sp1_prover::components::DefaultProverComponents;
use sp1_core_executor::{ExecutionReport, SP1ContextBuilder};
use sp1_prover::SP1Prover;
use cfg_if::cfg_if;

impl ExecutorClient {
    #[cfg(feature = "execution-logic")]
    pub fn run_execute<'a>(
        &'a self,
        elf: &'a [u8],
        stdin: SP1Stdin,
    ) -> anyhow::Result<(SP1PublicValues, ExecutionReport)> {
        #[allow(unreachable_code)]
        let prover: SP1Prover<DefaultProverComponents> = match env::var("SP1_PROVER").unwrap_or("local".to_string()).to_lowercase().as_str() {
            "mock" => SP1Prover::new(),
            "local" => {
                cfg_if! {
                if #[cfg(not(feature = "cuda"))] {
                    SP1Prover::new()
                } else {
                    SP1CudaProver::new()
                }
            }},
            "network" => {
                cfg_if! {
                    if #[cfg(feature = "network-v2")] {
                        Self {
                            prover: Box::new(NetworkProverV2::new()),
                        }
                    } else if #[cfg(feature = "network")] {
                        Self {
                            prover: Box::new(NetworkProverV1::new()),
                        }
                    } else {
                        panic!("network feature is not enabled")
                    }
                }
            }
            _ => panic!(
                "invalid value for SP1_PROVER environment variable: expected 'local', 'mock', or 'network'"
            ),
        };

        let mut context_builder = SP1ContextBuilder::default();
        let context = context_builder.build();
        Ok(prover.execute(elf, &stdin, context)?)
    }
}
