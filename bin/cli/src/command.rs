// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// --- std ---
// use std::path::PathBuf;

use crate::cli::{Cli, Subcommand};
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
// use sc_service::PartialComponents;
use sp_core::crypto::Ss58AddressFormat;
// use lyra_primitives::{OpaqueBlock as Block};
use asterism_service::service::{
    hadron::{hadron_runtime, HadronExecutor},
    quark::{quark_runtime, QuarkExecutor},
    IdentifyVariant,
};
use log::info;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Asterism Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn executable_name() -> String {
        "asterism".into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/hercules-network/asterism/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        let id = if id.is_empty() {
            let n = get_exec_name().unwrap_or_default();
            ["lyr", "quark", "hadron"]
                .iter()
                .cloned()
                .find(|&chain| n.starts_with(chain))
                .unwrap_or("lyr")
        } else {
            id
        };

        Ok(match id {
            "dev" => Box::new(asterism_service::chain_spec::hadron_development_config()?),
            "" | "local" => Box::new(asterism_service::chain_spec::quark_local_testnet_config()?),
            "staging" => Box::new(asterism_service::chain_spec::staging_config()?),
            "lyr" => Box::new(asterism_service::chain_spec::quark_config()?),
            "quark" => Box::new(asterism_service::chain_spec::quark_config()?),
            "hadron_staging" => Box::new(asterism_service::chain_spec::hadron_staging_config()?),
            "hadron" => Box::new(asterism_service::chain_spec::hadron_config()?),
            "hadron_test" => Box::new(asterism_service::chain_spec::hadron_testnet_config()?),
            path => Box::new(
                asterism_service::chain_spec::QuarkChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?,
            ),
        })
    }

    fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        if spec.is_quark_network() {
            &asterism_service::service::quark::quark_runtime::VERSION
        } else if spec.is_hadron_network() {
            &asterism_service::service::hadron::hadron_runtime::VERSION
        } else {
            &asterism_service::service::quark::quark_runtime::VERSION
        }
    }
}

fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}

fn set_default_ss58_version(spec: &Box<dyn asterism_service::service::ChainSpec>) {
    let ss58_version = if spec.is_quark_network() {
        Ss58AddressFormat::SubstrateAccount
    } else if spec.is_hadron_network() {
        Ss58AddressFormat::SubstraTeeAccount
    } else {
        Ss58AddressFormat::SubstrateAccount
    };

    sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            if chain_spec.is_quark_network() {
                runner.async_run(|mut config| {
                    let (client, _, import_queue, task_manager) =
                        asterism_service::service::quark::new_chain_ops::<
                            quark_runtime::RuntimeApi,
                            QuarkExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, import_queue), task_manager))
                })
            } else if chain_spec.is_hadron_network() {
                runner.async_run(|mut config| {
                    let (client, _, import_queue, task_manager) =
                        asterism_service::service::hadron::new_chain_ops::<
                            hadron_runtime::RuntimeApi,
                            HadronExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, import_queue), task_manager))
                })
            } else {
                unreachable!()
            }
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            if chain_spec.is_quark_network() {
                runner.async_run(|mut config| {
                    let (client, _, _, task_manager) =
                        asterism_service::service::quark::new_chain_ops::<
                            quark_runtime::RuntimeApi,
                            QuarkExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, config.database), task_manager))
                })
            } else if chain_spec.is_hadron_network() {
                runner.async_run(|mut config| {
                    let (client, _, _, task_manager) =
                        asterism_service::service::hadron::new_chain_ops::<
                            hadron_runtime::RuntimeApi,
                            HadronExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, config.database), task_manager))
                })
            } else {
                unreachable!()
            }
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            if chain_spec.is_quark_network() {
                runner.async_run(|mut config| {
                    let (client, _, _, task_manager) =
                        asterism_service::service::quark::new_chain_ops::<
                            quark_runtime::RuntimeApi,
                            QuarkExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, config.chain_spec), task_manager))
                })
            } else if chain_spec.is_hadron_network() {
                runner.async_run(|mut config| {
                    let (client, _, _, task_manager) =
                        asterism_service::service::hadron::new_chain_ops::<
                            hadron_runtime::RuntimeApi,
                            HadronExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, config.chain_spec), task_manager))
                })
            } else {
                unreachable!()
            }
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            if chain_spec.is_quark_network() {
                runner.async_run(|mut config| {
                    let (client, _, import_queue, task_manager) =
                        asterism_service::service::quark::new_chain_ops::<
                            quark_runtime::RuntimeApi,
                            QuarkExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, import_queue), task_manager))
                })
            } else if chain_spec.is_hadron_network() {
                runner.async_run(|mut config| {
                    let (client, _, import_queue, task_manager) =
                        asterism_service::service::hadron::new_chain_ops::<
                            hadron_runtime::RuntimeApi,
                            HadronExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, import_queue), task_manager))
                })
            } else {
                unreachable!()
            }
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            if chain_spec.is_quark_network() {
                runner.async_run(|mut config| {
                    let (client, backend, _, task_manager) =
                        asterism_service::service::quark::new_chain_ops::<
                            quark_runtime::RuntimeApi,
                            QuarkExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, backend), task_manager))
                })
            } else if chain_spec.is_hadron_network() {
                runner.async_run(|mut config| {
                    let (client, backend, _, task_manager) =
                        asterism_service::service::hadron::new_chain_ops::<
                            hadron_runtime::RuntimeApi,
                            HadronExecutor,
                        >(&mut config)?;

                    Ok((cmd.run(client, backend), task_manager))
                })
            } else {
                unreachable!()
            }
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            let chain_spec = &runner.config().chain_spec;

            set_default_ss58_version(chain_spec);

            info!("by Hercules Network, 2018-2021");

            if chain_spec.is_quark_network() {
                runner.run_node_until_exit(|config| async move {
                    match config.role {
                        Role::Light => asterism_service::service::quark::quark_new_light(config),
                        _ => asterism_service::service::quark::quark_new_full(config)
                            .map(|(task_manager, _)| task_manager),
                    }
                    .map_err(sc_cli::Error::Service)
                })
            } else if chain_spec.is_hadron_network() {
                runner.run_node_until_exit(|config| async move {
                    match config.role {
                        Role::Light => asterism_service::service::hadron::hadron_new_light(config),
                        _ => asterism_service::service::hadron::hadron_new_full(config)
                            .map(|(task_manager, _)| task_manager),
                    }
                    .map_err(sc_cli::Error::Service)
                })
            } else {
                unreachable!()
            }
        }
    }
}