mod filter;
mod obsidian;
mod settings;
mod task;
mod todoist;
use clap::{Parser, Subcommand};
use colored::Colorize;
use settings::Settings;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Obsidian {
        #[command(subcommand)]
        command: ObsidianCommands,
    },
    Todoist {
        #[command(subcommand)]
        command: TodoistCommands,
    },
}

#[derive(Subcommand, Debug)]
enum ObsidianCommands {
    Tasks {
        #[arg(short, long)]
        state: Option<Vec<filter::FilterState>>,

        #[arg(short, long)]
        today: bool,
    },
}

#[derive(Subcommand, Debug)]
enum TodoistCommands {
    Tasks {
        #[arg(short, long)]
        project: Option<String>,

        #[arg(short, long)]
        state: Option<Vec<filter::FilterState>>,

        #[arg(short, long)]
        today: bool,
    },
    Projects {},
}

fn due_to_str(t: Option<task::DateTimeUtc>) -> String {
    if let Some(d) = t {
        if d.time() == chrono::NaiveTime::default() {
            return d.format("%Y-%m-%d").to_string();
        }

        return d.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    String::from("-")
}

fn print_tasks<T: task::Task>(tasks: &Vec<T>) {
    for t in tasks {
        println!(
            "- [{}] {} ({}) ({})",
            t.state(),
            t.text(),
            format!("due: {}", due_to_str(t.due())).blue(),
            t.place().green()
        );
    }
}

async fn print_obsidian_task_list(
    cfg: Settings,
    f: &filter::Filter,
) -> Result<(), Box<dyn std::error::Error>> {
    let obs = obsidian::Obsidian::new(cfg.obsidian.path.as_str());
    let tasks = obs.tasks(f).await?;
    print_tasks(&tasks);

    Ok(())
}

async fn print_todoist_task_list(
    cfg: Settings,
    project: &Option<String>,
    f: &filter::Filter,
) -> Result<(), Box<dyn std::error::Error>> {
    let td = todoist::Todoist::new(&cfg.todoist.api_key);
    let tasks = td.tasks(project, f).await?;
    print_tasks(&tasks);

    Ok(())
}

async fn print_todoist_project_list(cfg: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let td = todoist::Todoist::new(&cfg.todoist.api_key);
    let projects = td.projects().await?;

    for p in projects {
        println!("{}: {}", p.id, p.name);
    }

    Ok(())
}

fn state_to_filter(state: &Option<Vec<filter::FilterState>>) -> Vec<filter::FilterState> {
    match state {
        Some(st) => st.to_vec(),
        None => vec![filter::FilterState::Uncompleted],
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Settings::load("settings.toml")?;

    let cli = Cli::parse();
    match &cli.command {
        Commands::Obsidian { command } => match command {
            ObsidianCommands::Tasks { state, today } => {
                print_obsidian_task_list(
                    cfg,
                    &filter::Filter {
                        states: state_to_filter(state),
                        today: *today,
                    },
                )
                .await?
            }
        },
        Commands::Todoist { command } => match command {
            TodoistCommands::Tasks {
                project,
                state,
                today,
            } => {
                print_todoist_task_list(
                    cfg,
                    project,
                    &filter::Filter {
                        states: state_to_filter(state),
                        today: *today,
                    },
                )
                .await?
            }
            TodoistCommands::Projects {} => print_todoist_project_list(cfg).await?,
        },
    };
    Ok(())
}
