mod device;

use crate::config::XossUtilConfig;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Args, Debug)]
pub struct MgaUpdateOptions {
    /// Do not try to update the MGA data
    ///
    /// May be useful if you have no Internet Connection
    #[clap(long)]
    pub mga_offline: bool,
    /// Force update of the MGA data
    #[clap(long)]
    pub mga_force_update: bool,
}

#[derive(Args, Debug)]
pub struct SyncOptions {
    #[clap(flatten)]
    mga_update: MgaUpdateOptions,
}

#[derive(Subcommand, Debug)]
pub enum DeviceCommand {
    /// Synchronize the device with the computer.
    ///
    /// Set the time, upload new MGA (satellite) data, download tracks
    Sync(SyncOptions),
    /// Shows various information about the device.
    Info,
    /// Download a file from the device.
    Pull {
        device_filename: String,
        output_filename: Option<Utf8PathBuf>,
    },
    /// Upload a file to the device.
    Push {
        input_filename: Utf8PathBuf,
        device_filename: Option<String>,
    },
    /// Delete a file from the device.
    ///
    /// NOTE: don't delete .json files, not all of them are regenerated by the device.
    Delete { device_filename: String },
}

#[derive(Args, Debug)]
pub struct DeviceCli {
    // TODO: include options for selecting the device
    #[clap(subcommand)]
    subcommand: DeviceCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    /// Interact with the device.
    Dev(DeviceCli),
    /// Make sure the MGA data is up to date.
    UpdateMga(MgaUpdateOptions),
}

impl Cli {
    pub async fn run(self, config: Option<XossUtilConfig>) -> Result<()> {
        match self.command {
            CliCommand::Dev(dev) => {
                let device = crate::locate_util::find_device_from_config(&config)
                    .await
                    .context("Failed to find the device")?;

                let result = dev.run(&device, config).await;

                // let disconnect_result = device
                //     .disconnect()
                //     .await
                //     .context("Failed to disconnect from the device");

                result.context("Failed to run the device subcommand")
                // .and(disconnect_result)
            }
            CliCommand::UpdateMga(mga_update) => {
                let config = config.context("Config is required for update-mga subcommand")?;
                crate::mga::get_mga_data(&config.mga, &mga_update).await?;
                Ok(())
            }
        }
    }
}
