use std::error::Error as StdError;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use project_knowledge::{
    Error, ReadModel, compile, compile_in_memory, default_db_path, load_records, rebuild,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(name = "pk", version, about = "Project Knowledge M0 kernel")]
struct Cli {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,

    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    Validate,
    Compile,
    Rebuild,
    Status,
    Resolve {
        #[arg(long)]
        subject: Uuid,
        #[arg(long)]
        concern: String,
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        context: Option<Uuid>,
    },
    Freshness {
        #[arg(long)]
        representation: Uuid,
    },
    Evidence {
        #[arg(long)]
        evaluation: Uuid,
    },
}

#[derive(Debug, Serialize)]
struct StatusOutput {
    root: PathBuf,
    enriched: bool,
    record_count: usize,
    observations: Vec<project_knowledge::SourceObservation>,
}

fn main() -> std::result::Result<(), Box<dyn StdError>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => {
            init(&cli.root)?;
            emit(
                &cli,
                &serde_json::json!({
                    "initialized": true,
                    "root": cli.root.display().to_string()
                }),
            )?;
        }
        Command::Validate => {
            let records = load_records(&cli.root)?;
            emit(
                &cli,
                &serde_json::json!({"valid": true, "record_count": records.len()}),
            )?;
        }
        Command::Compile => emit_serializable(&cli, &compile(&cli.root)?)?,
        Command::Rebuild => emit_serializable(&cli, &rebuild(&cli.root)?)?,
        Command::Status => {
            let (model, report) = compile_in_memory(&cli.root)?;
            let output = StatusOutput {
                root: cli.root.clone(),
                enriched: report.enriched,
                record_count: report.record_count,
                observations: model.observations()?,
            };
            emit_serializable(&cli, &output)?;
        }
        Command::Resolve {
            subject,
            concern,
            at,
            context,
        } => {
            ensure_compiled(&cli.root)?;
            let model = ReadModel::open(&default_db_path(&cli.root))?;
            emit_serializable(
                &cli,
                &model.resolve_current(subject, &concern, at.as_deref(), context)?,
            )?;
        }
        Command::Freshness { representation } => {
            ensure_compiled(&cli.root)?;
            let model = ReadModel::open(&default_db_path(&cli.root))?;
            emit(
                &cli,
                &serde_json::json!({
                    "representation": representation,
                    "freshness": model.representation_freshness(representation)?
                }),
            )?;
        }
        Command::Evidence { evaluation } => {
            ensure_compiled(&cli.root)?;
            let model = ReadModel::open(&default_db_path(&cli.root))?;
            emit(
                &cli,
                &serde_json::json!({
                    "evaluation": evaluation,
                    "state": model.evidence_state(evaluation)?
                }),
            )?;
        }
    }
    Ok(())
}

fn init(root: &Path) -> std::result::Result<(), Error> {
    fs::create_dir_all(root.join(".pk").join("records"))?;
    fs::create_dir_all(root.join(".pk").join("cache"))?;
    let ignore = root.join(".pk").join(".gitignore");
    if !ignore.exists() {
        fs::write(ignore, "cache/\n")?;
    }
    Ok(())
}

fn ensure_compiled(root: &Path) -> std::result::Result<(), Error> {
    compile(root)?;
    Ok(())
}

fn emit_serializable<T: Serialize>(
    cli: &Cli,
    value: &T,
) -> std::result::Result<(), Box<dyn StdError>> {
    let json = serde_json::to_value(value)?;
    emit(cli, &json)
}

fn emit(cli: &Cli, value: &serde_json::Value) -> std::result::Result<(), Box<dyn StdError>> {
    if cli.json {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else if let Some(object) = value.as_object() {
        for (key, value) in object {
            println!("{key}: {}", render(value));
        }
    } else {
        println!("{}", serde_json::to_string_pretty(value)?);
    }
    Ok(())
}

fn render(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        other => other.to_string(),
    }
}
