use anyhow::Error;

use crate::Context;

mod bulkinstall;
mod help;
mod install;
mod rawdog;

/// Run packwiz commands.  
/// Should be safe to run outside a container, but be cautious. Might allow RCE.
#[poise::command(
    slash_command,
    prefix_command,
    owners_only,
    subcommands(
        "rawdog::rawdog",
        "bulkinstall::bulkinstall",
        "help::help",
        "install::install"
    )
)]
pub async fn packwiz(
    ctx: Context<'_>,
    #[description = "Arguments to pass to packwiz directly"]
    #[rest]
    args: String,
) -> Result<(), Error> {
    rawdog::rawdog_impl(ctx, args).await
}
