use async_trait::async_trait;
use pumpkin_config::BasicConfiguration;
use pumpkin_util::text::TextComponent;
use crate::{
    command::{
        args::ConsumedArgs, tree::CommandTree, CommandError, CommandExecutor, CommandSender,
    },
};
use crate::command::args::{Arg, GetCloned};
use crate::command::args::gamemode::GamemodeArgumentConsumer;
use crate::command::dispatcher::CommandError::InvalidConsumption;
use crate::command::tree::builder::argument;

const NAMES: [&str; 1] = ["defaultgamemode"];

const DESCRIPTION: &str = "Change the default gamemode";

pub const ARG_GAMEMODE: &str = "gamemode";

struct DefaultGamemodeExecutor;

#[async_trait]
impl CommandExecutor for DefaultGamemodeExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {

        let Some(Arg::GameMode(mut gamemode)) = args.get_cloned(&ARG_GAMEMODE) else {
            return Err(InvalidConsumption(Some(ARG_GAMEMODE.into())));
        };

        if BasicConfiguration::default().force_gamemode {

            for player in server.get_all_players().await {
                player.set_gamemode(gamemode).await;
            }
        }


        let gamemode_string = format!("{gamemode:?}").to_lowercase();
        let gamemode_string = format!("gameMode.{gamemode_string}");

        sender
            .send_message(TextComponent::translate(
                "commands.defaultgamemode.success", [
                    TextComponent::translate(gamemode_string, [].into()),
                ]
                    .into(),
            ))
            .await;

        //TODO
        // Set the new default gamemode in the configuration.toml

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        argument(ARG_GAMEMODE, GamemodeArgumentConsumer)
            .execute(DefaultGamemodeExecutor))
}
