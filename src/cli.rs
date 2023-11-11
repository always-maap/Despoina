use std::path::PathBuf;

use clap::{command, Parser};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub email: EmailConfig,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub subject: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    pub config: PathBuf,

    #[arg(short, long, value_name = "FILE", required = true)]
    pub file: PathBuf,
}
