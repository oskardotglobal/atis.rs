use anyhow::Error;

use crate::commands::packwiz::check_for_admin;
use crate::{check_output, Context};

#[poise::command(slash_command, prefix_command, check = "check_for_admin")]
pub(crate) async fn bulkinstall(
    ctx: Context<'_>,
    #[description = "URLs of mods to install, one per line"]
    #[rest]
    args: String,
) -> Result<(), Error> {
    let lines: Vec<&str> = args.split('\n').collect();

    for line in lines {
        if line.starts_with('#') {
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if line.contains("modrinth.com") {
            check_output!("packwiz", ["mr", "install", "-y", line], "install mod", ctx);
        }

        if line.contains("curseforge.com") {
            check_output!("packwiz", ["cf", "install", "-y", line], "install mod", ctx);
        }
    }

    Ok(())
}
