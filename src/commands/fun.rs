use crate::utils::check_for_any_role;
use crate::{say, Context};
use anyhow::Error;
use log::warn;
use std::str::from_utf8;

const DEFAULT_YAWN: &str = r"
Hello everyone! Just wanted to say good morning/good afternoon/good evening/good night/hello/good bye to all of you! Also, well pole.
";

/// Sets a users yawn
#[poise::command(slash_command, prefix_command, check = "check")]
pub async fn setyawn(
    ctx: Context<'_>,
    #[description = "Arguments"] yawn: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    match tree.insert(format!("--yawn-{}", ctx.author().id), yawn.as_str()) {
        Ok(_) => say!(ctx, "Yawn set"),
        Err(e) => {
            warn!("Error inserting yawn into database: {:?}", e);
            say!(ctx, "Error inserting yawn into database");
        }
    };

    Ok(())
}

/// Gets the users yawn
#[poise::command(slash_command, prefix_command)]
pub async fn yawn(ctx: Context<'_>) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    match tree.get(format!("--yawn-{}", ctx.author().id)) {
        Ok(Some(response)) => {
            let message = from_utf8(&response).unwrap_or_else(|e| {
                warn!("Error converting yawn to string: {:?}", e);
                "Error converting yawn to string"
            });

            say!(ctx, "{}", message);
        }
        Ok(None) => say!(ctx, "{}", DEFAULT_YAWN),
        Err(e) => {
            warn!("Error getting yawn from database: {:?}", e);
            say!(ctx, "Error getting yawn from database");
        }
    }

    Ok(())
}

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    check_for_any_role(
        ctx,
        vec![
            945886621931286538,  // Booster
            933819407707291698,  // Mod
            1009504345055830097, // Origins Developer
            984130473913622549,  // Artist
            1015637432873525329, // Mod Developer
            1072249811048341545, // Admin
            906816241052811325,  // Nightmare
            933819147362648115,  // Brightmare
        ],
    )
    .await
}
