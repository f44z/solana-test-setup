//! SolanaTestInitializer Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod init;

use self::init::InitCmd;
use crate::config::SolanaTestInitializerConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// SolanaTestInitializer Configuration Filename
pub const CONFIG_FILE: &str = "solana_test_initializer.toml";

/// SolanaTestInitializer Subcommands
/// Subcommands need to be listed in an enum.
#[derive(Command, Debug, Parser, Runnable)]
pub enum SolanaTestInitializerCmd {
    /// The `start` subcommand
    Init(InitCmd),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: SolanaTestInitializerCmd,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<SolanaTestInitializerConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
            .config
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.into());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(
        &self,
        config: SolanaTestInitializerConfig,
    ) -> Result<SolanaTestInitializerConfig, FrameworkError> {
        match &self.cmd {
            SolanaTestInitializerCmd::Init(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
        }
    }
}