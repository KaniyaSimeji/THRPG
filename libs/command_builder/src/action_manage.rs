use std::{time::Duration, sync::Arc};

use serenity::{
    builder::{CreateActionRow, CreateButton, CreateInputText, CreateSelectMenu},
    model::prelude::{ReactionType, Message, interaction::message_component::MessageComponentInteraction}, prelude::Context,
};
use tokio::sync::mpsc::Sender;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionKind {
    Button,
    Buttons(u8),
    TextInput,
    SelectMenu,
}

/// Structure for managing actions
#[derive(Clone)]
pub struct ActionManage<'a> {
    kind: ActionKind,
    actions: CreateActionRow,
    send_channel: Option<Sender<Arc<MessageComponentInteraction>>>,
    messages: Option<Vec<&'a Message>>,
    ctx: Option<Context>
}

impl<'a> ActionManage<'a> {
    pub fn new(kind: ActionKind) -> Self {
        Self {
            kind,
            actions: CreateActionRow::default(),
            send_channel: None,
            messages: None,
            ctx: None
        }
    }

    pub fn button<F>(&mut self, button: F) -> &mut Self
    where
        F: FnOnce(&mut CreateButton) -> &mut CreateButton,
    {
        if self.kind == ActionKind::Button {
            let mut init_button = CreateButton::default();
            button(&mut init_button);
            self.actions.add_button(init_button);
        }
        self
    }

    pub fn standard_button<T: ToString>(
        &mut self,
        label: Option<T>,
        emoji: Option<ReactionType>,
    ) -> &mut Self {
        let mut button = CreateButton::default();
        if let Some(label_string) = label {
            button.label(label_string);
        }
        if let Some(some_emoji) = emoji {
            button.emoji(some_emoji);
        }
        self.actions.add_button(button);
        self
    }

    pub fn buttons(&mut self, buttons: Vec<CreateButton>) -> &mut Self {
        if self.kind == ActionKind::Buttons(buttons.len() as u8) {
            for button in buttons {
                self.actions.add_button(button);
            }
        }
        self
    }

    pub fn select_menu<F>(&mut self, menu: F) -> &mut Self
    where
        F: FnOnce(&mut CreateSelectMenu) -> &mut CreateSelectMenu,
    {
        if self.kind == ActionKind::SelectMenu {
            let mut init_menu = CreateSelectMenu::default();
            menu(&mut init_menu);
            self.actions.add_select_menu(init_menu);
        }
        self
    }

    pub fn text_input<F>(&mut self, input: F) -> &mut Self
    where
        F: FnOnce(&mut CreateInputText) -> &mut CreateInputText,
    {
        if self.kind == ActionKind::TextInput {
            let mut init_input = CreateInputText::default();
            input(&mut init_input);
            self.actions.add_input_text(init_input);
        }
        self
    }

    pub fn context(&self) -> CreateActionRow {
        self.actions.clone()
    }

    pub fn reaction_channel(&mut self, channel: Sender<Arc<MessageComponentInteraction>>) -> &mut Self {
        self.send_channel = Some(channel);
        self
    }

    pub fn add_watch_message(&mut self, messages: Vec<&'a Message>) -> &mut Self {
        self.messages = Some(messages);
        self
    }

    pub fn add_discord_ctx(&mut self, ctx: Context) -> &mut Self {
        self.ctx = Some(ctx);
        self
    }

    pub async fn event_loop(&mut self) -> anyhow::Result<()> {
        if self.send_channel.is_some() && self.messages.is_some() && self.ctx.is_some() {
            for message in self.messages.as_ref().unwrap() {
                if let Some(component) = 
                    message.await_component_interaction(self.ctx.as_ref().unwrap())
                        .timeout(Duration::from_secs(10))
                        .author_id(message.author.id.0.clone())
                        .await
                {
                    self.send_channel.as_ref().unwrap().send(component).await?;
                }
            }
        }
        Ok(())
    }
}
