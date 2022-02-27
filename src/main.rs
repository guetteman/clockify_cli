use clap::{Parser, Subcommand};
use commands::{timesheet, list_tasks};

mod commands;

#[derive(Parser)]
#[clap(name = "clockify")]
#[clap(version = "0.1.0")]
#[clap(about = "Clockify CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Timesheet table of the selected month")]
    Timesheet {
        #[clap(name = "month")]
        #[clap(help = "3 letters month. Ex: JAN")]
        month: String,

        #[clap(name = "working days")]
        #[clap(help = "Working days for the specified month. Ex: 20")]
        working_days: i32,
    },
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Timesheet table of the selected month")]
    ListTasks {
        #[clap(name = "month")]
        #[clap(help = "3 letters month. Ex: JAN")]
        month: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Timesheet { month, working_days } => {
           timesheet::handle(month, working_days).await;
        },
        Commands::ListTasks { month } => {
            list_tasks::handle(month).await;
        }
    }
}
