use std::path::Path;

use async_trait::async_trait;
use pumpkin_util::{
    PermissionLvl,
    text::{TextComponent, color::NamedColor, hover::HoverEvent},
};

use crate::{
    PLUGIN_MANAGER,
    command::{
        CommandError, CommandExecutor, CommandSender,
        args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
        tree::{
            CommandTree,
            builder::{argument, literal, require},
        },
    },
};

use crate::command::CommandError::InvalidConsumption;

const NAMES: [&str; 1] = ["plugin"];

const DESCRIPTION: &str = "Manage plugins.";

const PLUGIN_NAME: &str = "plugin_name";

struct ListExecutor;

#[async_trait]
impl CommandExecutor for ListExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &crate::server::Server,
        _args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let plugins = PLUGIN_MANAGER.active_plugins().await;

        let message_text = if plugins.is_empty() {
            "There are no loaded plugins.".to_string()
        } else if plugins.len() == 1 {
            "There is 1 plugin loaded:\n".to_string()
        } else {
            format!("There are {} plugins loaded:\n", plugins.len())
        };
        let mut message = TextComponent::text(message_text);

        for (i, metadata) in plugins.clone().into_iter().enumerate() {
            let fmt = if i == plugins.len() - 1 {
                metadata.name.to_string()
            } else {
                format!("{}, ", metadata.name)
            };
            let hover_text = format!(
                "Version: {}\nAuthors: {}\nDescription: {}",
                metadata.version, metadata.authors, metadata.description
            );
            let component = TextComponent::text(fmt)
                .color_named(NamedColor::Green)
                .hover_event(HoverEvent::show_text(TextComponent::text(hover_text)));

            message = message.add_child(component);
        }

        sender.send_message(message).await;

        Ok(())
    }
}

struct LoadExecutor;

#[async_trait]
impl CommandExecutor for LoadExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(plugin_name)) = args.get(PLUGIN_NAME) else {
            return Err(InvalidConsumption(Some(PLUGIN_NAME.into())));
        };

        if PLUGIN_MANAGER.is_plugin_active(plugin_name).await {
            sender
                .send_message(
                    TextComponent::text(format!("Plugin {plugin_name} is already loaded"))
                        .color_named(NamedColor::Red),
                )
                .await;
            return Ok(());
        }

        let result = PLUGIN_MANAGER.try_load_plugin(Path::new(plugin_name)).await;

        match result {
            Ok(()) => {
                sender
                    .send_message(
                        TextComponent::text(format!("Plugin {plugin_name} loaded successfully"))
                            .color_named(NamedColor::Green),
                    )
                    .await;
            }
            Err(e) => {
                sender
                    .send_message(
                        TextComponent::text(format!("Failed to load plugin {plugin_name}: {e}"))
                            .color_named(NamedColor::Red),
                    )
                    .await;
            }
        }

        Ok(())
    }
}

struct UnloadExecutor;

#[async_trait]
impl CommandExecutor for UnloadExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(plugin_name)) = args.get(PLUGIN_NAME) else {
            return Err(InvalidConsumption(Some(PLUGIN_NAME.into())));
        };

        if !PLUGIN_MANAGER.is_plugin_active(plugin_name).await {
            sender
                .send_message(
                    TextComponent::text(format!("Plugin {plugin_name} is not loaded"))
                        .color_named(NamedColor::Red),
                )
                .await;
            return Ok(());
        }

        let result = PLUGIN_MANAGER.unload_plugin(plugin_name).await;

        match result {
            Ok(()) => {
                sender
                    .send_message(
                        TextComponent::text(format!("Plugin {plugin_name} unloaded successfully",))
                            .color_named(NamedColor::Green),
                    )
                    .await;
            }
            Err(e) => {
                sender
                    .send_message(
                        TextComponent::text(format!("Failed to unload plugin {plugin_name}: {e}"))
                            .color_named(NamedColor::Red),
                    )
                    .await;
            }
        }

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        require(|sender| sender.has_permission_lvl(PermissionLvl::Three))
            .then(
                literal("load")
                    .then(argument(PLUGIN_NAME, SimpleArgConsumer).execute(LoadExecutor)),
            )
            .then(
                literal("unload")
                    .then(argument(PLUGIN_NAME, SimpleArgConsumer).execute(UnloadExecutor)),
            )
            .then(literal("list").execute(ListExecutor)),
    )
}
