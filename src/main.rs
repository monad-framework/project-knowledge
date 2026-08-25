use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use project_knowledge::{
    AuthoringIntent, Error, ReadModel, Result, apply_capture_plan, build_capture_plan, compile,
    compile_in_memory, default_db_path, load_plan, load_records, parse_intent, rebuild, render_plan,
    save_plan,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(name = "pk", version, about = "Project Knowledge local semantic runtime")]
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
    Capture {
        #[command(subcommand)]
        command: Option<CaptureCommand>,
    },
}

#[derive(Debug, Subcommand)]
enum CaptureCommand {
    Plan {
        #[arg(long)]
        intent: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Apply {
        #[arg(long)]
        plan: PathBuf,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Debug, Serialize)]
struct StatusOutput {
    root: PathBuf,
    enriched: bool,
    record_count: usize,
    observations: Vec<project_knowledge::SourceObservation>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code());
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
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
                &model.resolve_current(*subject, concern, at.as_deref(), *context)?,
            )?;
        }
        Command::Freshness { representation } => {
            ensure_compiled(&cli.root)?;
            let model = ReadModel::open(&default_db_path(&cli.root))?;
            emit(
                &cli,
                &serde_json::json!({
                    "representation": representation,
                    "freshness": model.representation_freshness(*representation)?
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
                    "state": model.evidence_state(*evaluation)?
                }),
            )?;
        }
        Command::Capture { command: None } => interactive_capture(&cli)?,
        Command::Capture {
            command: Some(CaptureCommand::Plan { intent, out }),
        } => capture_plan(&cli, intent.as_deref(), out.as_deref())?,
        Command::Capture {
            command: Some(CaptureCommand::Apply { plan, yes }),
        } => capture_apply(&cli, plan, *yes)?,
    }
    Ok(())
}

fn capture_plan(cli: &Cli, source: Option<&str>, out: Option<&Path>) -> Result<()> {
    let intent = read_intent(source)?;
    let plan = build_capture_plan(&cli.root, &intent)?;
    if let Some(path) = out {
        save_plan(path, &plan)?;
    }
    if cli.json {
        emit_serializable(cli, &plan)
    } else {
        print!("{}", render_plan(&plan));
        Ok(())
    }
}

fn capture_apply(cli: &Cli, path: &Path, yes: bool) -> Result<()> {
    let plan = load_plan(path)?;
    if !yes {
        print!("{}", render_plan(&plan));
        if !confirm("Apply exactly this capture plan? [y/N] ")? {
            return Err(Error::AuthoringInput("capture apply cancelled".to_string()));
        }
    }
    let result = apply_capture_plan(&cli.root, &plan)?;
    emit_serializable(cli, &result)
}

fn interactive_capture(cli: &Cli) -> Result<()> {
    let intent = read_intent(None)?;
    let plan = build_capture_plan(&cli.root, &intent)?;
    print!("{}", render_plan(&plan));
    if plan.has_blockers() {
        return Err(Error::BlockedPlan(
            "review the blockers above and create a new plan".to_string(),
        ));
    }
    if !confirm("Apply exactly this capture plan? [y/N] ")? {
        return Err(Error::AuthoringInput("capture cancelled".to_string()));
    }
    let result = apply_capture_plan(&cli.root, &plan)?;
    emit_serializable(cli, &result)
}

fn read_intent(source: Option<&str>) -> Result<AuthoringIntent> {
    let text = match source {
        Some("-") => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;
            text
        }
        Some(path) => fs::read_to_string(path)?,
        None => {
            eprint!("Authoring Intent path (or '-' for stdin): ");
            io::stderr().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let source = input.trim();
            if source.is_empty() {
                return Err(Error::AuthoringInput(
                    "an Authoring Intent path is required".to_string(),
                ));
            }
            if source == "-" {
                let mut text = String::new();
                io::stdin().read_to_string(&mut text)?;
                text
            } else {
                fs::read_to_string(source)?
            }
        }
    };
    parse_intent(&text)
}

fn confirm(prompt: &str) -> Result<bool> {
    eprint!("{prompt}");
    io::stderr().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes"))
}

fn init(root: &Path) -> Result<()> {
    fs::create_dir_all(root.join(".pk").join("records"))?;
    fs::create_dir_all(root.join(".pk").join("cache"))?;
    let ignore = root.join(".pk").join(".gitignore");
    if !ignore.exists() {
        fs::write(ignore, "cache/\n")?;
    }
    Ok(())
}

fn ensure_compiled(root: &Path) -> Result<()> {
    compile(root)?;
    Ok(())
}

fn emit_serializable<T: Serialize>(cli: &Cli, value: &T) -> Result<()> {
    let json = serde_json::to_value(value)?;
    emit(cli, &json)
}

fn emit(cli: &Cli, value: &serde_json::Value) -> Result<()> {
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
