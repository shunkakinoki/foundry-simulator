// mod config;
// use anyhow::{Ok, Result};
// use bytes::Bytes;
// pub use config::NodeConfig;
// use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend,
// Executor, ExecutorBuilder}; use primitive_types::{H160, H256, U256};
// use std::str::FromStr;

// pub async fn spawn(config: NodeConfig) -> Executor {
//     let evm_opts = EvmOpts { fork_url: Some(config.fork_url.clone()),
// ..Default::default() };

//     let env = evm_opts.evm_env_blocking().unwrap();
//     let builder = ExecutorBuilder::default().set_tracing(true);

//     let fork_opts =
//         Some(CreateFork { url: config.fork_url.clone(), enable_caching: true,
// env, evm_opts });     let db = Backend::spawn(fork_opts);
//     let executor = builder.build(db);

//     return executor;
// }

// pub async fn simulate(mut executor: Executor) -> Result<()> {
//     let r = executor.call_raw_committing(
//         H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").
// unwrap(),         H160::from_str("0x225E9B54F41F44F42150b6aAA730Da5f2d23FAf2"
// ).unwrap(),         Bytes::from(""),
//         U256::from(0),
//     );
//     println!("{:?}", r);

//     Ok(())
// }
