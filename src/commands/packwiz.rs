use anyhow::Error;

use crate::utils::check_for_any_role;
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
    check = "check_for_admin",
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

pub(crate) async fn check_for_admin(ctx: Context<'_>) -> Result<bool, Error> {
    check_for_any_role(
        ctx,
        vec![
            1072249811048341545, // Admin
            906816241052811325,  // Nightmare
            933819147362648115,  // Brightmare
        ],
    )
    .await
}
