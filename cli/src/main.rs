use clap::{Parser, Subcommand};
use commands::{hook, interactive};

pub mod commands;
pub mod ipc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Hook {
        #[command(subcommand)]
        action: HookCommands,
    },
    Namespace {
        #[command(subcommand)]
        action: NamespaceCommands,
    },
    Init {
        shell_name: String,
    },
}

#[derive(Subcommand)]
pub enum HookCommands {
    PreExec { command: String },
    PreCmd { exit_code: i32 },
}

#[derive(Subcommand)]
pub enum NamespaceCommands {
    Use { name: String },
    Exit,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hook { action } => hook::handle(action),
        Commands::Namespace { action } => interactive::handle_namespace(action),
        Commands::Init { shell_name } => {
            let script = match shell_name.as_str() {
                "zsh" => shell::generators::zsh::generate(),
                "bash" => shell::generators::bash::generate(),
                "powershell" => shell::generators::powershell::generate(),
                _ => String::new(),
            };
            print!("{}", script);
        }
    }
}
