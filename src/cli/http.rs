use std::path::PathBuf;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{process::http::process_http, CmdExector};

use super::verify_path;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(about = "通过HTTP服务文件")]
    Serve(HttpServeOpts),
}

// impl CmdExector for HttpSubCommand {
//     async fn execute(&self) -> anyhow::Result<()> {
//         match self {
//             HttpSubCommand::Serve(opts) => opts.execute().await,
//         }
//     }
// }

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http(self.dir.clone(), self.port).await?;
        Ok(())
    }
}
