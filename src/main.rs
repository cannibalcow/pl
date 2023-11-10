use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

use clap::{command, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PipelineStep {
    name: String,
    command: String,
}

#[derive(Deserialize, Serialize)]
struct Pipeline {
    steps: Vec<PipelineStep>,
}

#[derive(Parser, Debug)]
struct PipelineArgs {
    #[command(subcommand)]
    pub pl_cmd: PipelineCommand,
}

#[derive(Subcommand, Debug)]
pub enum PipelineCommand {
    Create {
        #[arg(short = 'f', long, default_value_t = String::from("pipeline.yml"), help ="Runs pipeline") ]
        filename: String,
    },

    Run {
        #[arg(short = 'f', long, default_value_t = String::from("pipeline.yml"), help ="Runs pipeline") ]
        filename: String,
    },
}

fn main() {
    let args = PipelineArgs::parse();

    match args.pl_cmd {
        PipelineCommand::Create { filename } => {
            match File::create(filename) {
                Ok(mut f) => f
                    .write_all(b"steps:\n    - name: Build\n      command: echo 'hello'")
                    .unwrap(),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
        }
        PipelineCommand::Run { filename } => {
            run_pipeline(&filename);
        }
    }
    return;
}

fn run_pipeline(filename: &String) {
    let pfile = fs::read_to_string(filename).unwrap();

    println!("piplinefile: {}", filename);

    let pipeline: Pipeline = serde_yaml::from_str(&pfile).unwrap();

    for step in pipeline.steps {
        let cmd = Command::new("bash").args(["-c", &step.command]).output();

        match cmd {
            Ok(o) => {
                if o.status.success() {
                    println!("Success: {}", step.name);
                    let out = String::from_utf8_lossy(&o.stdout);
                    if !out.is_empty() {
                        println!("{}", out);
                    }
                } else {
                    eprintln!("Failed: {}", String::from_utf8_lossy(&o.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
                return;
            }
        }
    }
}
