use crate::{check_output, say, Context};
use anyhow::Error;

#[derive(Debug, poise::ChoiceParameter)]
enum ModSource {
    #[name = "Modrinth"]
    Modrinth,
    #[name = "Curseforge (Stinky)"]
    Curseforge,
    #[name = "Direct download"]
    Url,
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub(crate) async fn install(
    ctx: Context<'_>,
    #[description = "The mod source. Required for non-URLs"] source: Option<ModSource>,
    #[description = "The mod slug. Required for URLs"] slug: Option<String>,
    #[description = "URL / slug to install. Will be ignored if slug is specified."] query: String,
) -> Result<(), Error> {
    match slug {
        Some(slug) => match source {
            Some(ModSource::Modrinth) => {
                check_output!(
                    "packwiz",
                    ["mr", "install", "-y", query.as_str()],
                    "install mod",
                    ctx
                );
            }
            Some(ModSource::Curseforge) => {
                check_output!(
                    "packwiz",
                    ["cf", "install", "-y", query.as_str()],
                    "install mod",
                    ctx
                );
            }
            Some(ModSource::Url) => check_output!(
                "packwiz",
                ["url", "add", slug.as_str(), query.as_str()],
                "install mod",
                ctx
            ),
            None => {
                say!(ctx, "You must specify a mod source when specifying a slug!");
            }
        },
        None => {
            match (
                query.contains("curseforge.com"),
                query.contains("modrinth.com"),
            ) {
                (true, false) => {
                    say!(
                        ctx,
                        "Query contains curseforge.com, attempting to install from Curseforge..."
                    );
                    check_output!(
                        "packwiz",
                        ["cf", "install", "-y", query.as_str()],
                        "install mod",
                        ctx
                    );
                }
                (false, true) => {
                    say!(
                        ctx,
                        "Query contains modrinth.com, attempting to install from Modrinth..."
                    );
                    check_output!(
                        "packwiz",
                        ["mr", "install", "-y", query.as_str()],
                        "install mod",
                        ctx
                    );
                }
                _ => {
                    say!(
                        ctx,
                        "Query seems to be a slug, attempting to install from modrinth anyways..."
                    );
                    check_output!(
                        "packwiz",
                        ["mr", "install", "-y", query.as_str()],
                        "install mod",
                        ctx
                    );
                }
            }
        }
    }

    Ok(())
}
