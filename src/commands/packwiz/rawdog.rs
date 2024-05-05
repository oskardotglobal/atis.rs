use std::env::var;
use std::process::Command;

use anyhow::Error;

use crate::commands::packwiz::check_for_admin;
use crate::commands::packwiz::help::{help_impl, HELP_SUBCOMMANDS};
use crate::utils::fatal;
use crate::{say, Context};

const DISALLOWED_SUBCOMMANDS: [&str; 4] = ["init", "completion", "utils", "serve"];

#[poise::command(slash_command, prefix_command, check = "check_for_admin")]
pub(crate) async fn rawdog(
    ctx: Context<'_>,
    #[description = "Arguments to pass to packwiz directly"]
    #[rest]
    args: String,
) -> Result<(), Error> {
    rawdog_impl(ctx, args).await
}

pub(crate) async fn rawdog_impl(ctx: Context<'_>, args: String) -> Result<(), Error> {
    let mut cmd = Command::new("packwiz");

    let cwd = var("PACKWIZ_REPO_PATH")
        .unwrap_or_else(|e| fatal("PACKWIZ_REPO_PATH not found in env!", e));

    cmd.current_dir(cwd);

    let subcommand = match args.contains(' ') {
        true => {
            let split: Vec<&str> = args.split_whitespace().collect();
            cmd.args(split.clone());

            split[0]
        }
        false => {
            cmd.arg(args.clone());
            args.as_str()
        }
    };

    if HELP_SUBCOMMANDS.contains(&subcommand) {
        return help_impl(ctx).await;
    }

    if DISALLOWED_SUBCOMMANDS.contains(&subcommand) {
        say!(ctx, "That command is disabled");
        return Ok(());
    }

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if stdout.is_empty() && stderr.is_empty() {
                say!(ctx, "Command ran with no output");
                return Ok(());
            }

            // Don't send the output if it's just the help message
            // that would be too long anyways
            if stdout.contains("Available Commands:") {
                return help_impl(ctx).await;
            }

            if !stderr.is_empty() {
                say!(ctx, "```\n{}```", stderr);
            }

            say!(ctx, "```\n{}```", stdout);
        }
        Err(_) => say!(ctx, "Error running packwiz"),
    }

    Ok(())
}
