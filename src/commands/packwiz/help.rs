use crate::commands::packwiz::check_for_admin;
use crate::{say, Context};
use anyhow::Error;

pub(crate) const HELP_SUBCOMMANDS: [&str; 2] = ["help", "--help"];

const HELP_1: &str = r###"
```
A command line tool for creating Minecraft modpacks

Usage:
  packwiz [command]

Available Commands:
  completion  Generate the autocompletion script for the specified shell
  curseforge  Manage curseforge-based mods
  help        Help about any command
  list        List all the mods in the modpack
  migrate     Migrate your Minecraft and loader versions to newer versions.
  modrinth    Manage modrinth-based mods
  pin         Pin a file so it does not get updated automatically
  refresh     Refresh the index file
  remove      Remove an external file from the modpack; equivalent to manually removing the file and running packwiz refresh
  settings    Manage pack settings
  unpin       Unpin a file so it receives updates
  update      Update an external file (or all external files) in the modpack
  url         Add external files from a direct download link, for sites that are not directly supported by packwiz
```
"###;

const HELP_2: &str = r###"
```
Flags:
      --cache string              The directory where packwiz will cache downloaded mods (default "/Users/oskar/Library/Caches/packwiz/cache")
      --config string             The config file to use (default "/Users/oskar/Library/Application Support/packwiz/.packwiz.toml")
  -h, --help                      help for packwiz
      --meta-folder string        The folder in which new metadata files will be added, defaulting to a folder based on the category (mods, resourcepacks, etc; if the category is unknown the current directory is used)
      --meta-folder-base string   The base folder from which meta-folder will be resolved, defaulting to the current directory (so you can put all mods/etc in a subfolder while still using the default behaviour) (default ".")
      --pack-file string          The modpack metadata file to use (default "pack.toml")
  -y, --yes                       Accept all prompts with the default or "yes" option (non-interactive mode) - may pick unwanted options in search results

Use "packwiz [command] --help" for more information about a command.
```
"###;

#[poise::command(slash_command, prefix_command, check = "check_for_admin")]
pub(crate) async fn help(ctx: Context<'_>) -> Result<(), Error> {
    help_impl(ctx).await
}

pub(crate) async fn help_impl(ctx: Context<'_>) -> Result<(), Error> {
    say!(ctx, "{}", HELP_1);
    say!(ctx, "{}", HELP_2);

    Ok(())
}
