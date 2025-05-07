use clap::{Parser, Subcommand};
mod ai;
mod resume;
mod utils;
use crate::logger;

#[derive(Parser)]
#[clap(name = "phaseops")]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: CLICommand,
}

#[derive(Subcommand)]
pub enum CLICommand {
    /// Extract metadata from a job description file
    ParseJd {
        /// Path to the JD text file
        path: String,
    },
    /// Generate a tailored resume PDF
    GenResume {
        /// Path to the extracted metadata JSON
        jd_path: String,
    },
    /// Log an application shot
    LogApp {
        /// Company name
        company: String,
        /// Role title
        role: String,
    },
}
// src/cli/mod.rs

pub fn run() {
    // 1. Grab the raw argv: [ "phaseops", "cli", "parse-jd", "./file" ]
    let mut raw: Vec<String> = std::env::args().collect();

    // 2. Remove the "cli" prefix at index 1 (if present)
    if raw.len() > 1 && raw[1] == "cli" {
        raw.remove(1);
    }
    // Now raw is [ "phaseops", "parse-jd", "./file" ]

    // 3. Parse with Clap
    let args = CliArgs::parse_from(raw);

    // 4. Dispatch as before
    match args.command {
        CLICommand::ParseJd { path } => {
            logger::info("▶️ Running ParseJd");
            ai::extract_metadata(&path);
        }
        CLICommand::GenResume { jd_path } => {
            logger::info("▶️ Running GenResume");
            resume::generate_resume(&jd_path);
        }
        CLICommand::LogApp { company, role } => {
            logger::info("▶️ Running LogApp");
            utils::log_application(&company, &role);
        }
    }
}
