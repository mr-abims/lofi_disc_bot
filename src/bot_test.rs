#[cfg(test)]
mod tests {
    use serenity::model::gateway::Ready;
    use serenity::model::channel::Message;
    use serenity::client::{Context, EventHandler};

    struct MockHandler;

    #[async_trait]
    impl EventHandler for MockHandler {
        async fn ready(&self, _: Context, ready: Ready) {
            assert_eq!(ready.user.name, "MockBot");
        }

        async fn message(&self, _: Context, msg: Message) {
            assert_eq!(msg.content, "!play");
            assert_eq!(msg.guild_id.unwrap().to_string(), "123456789"); // Replace with an actual guild ID
        }
    }

    #[tokio::test]
    async fn test_bot_handler() {
        let handler = MockHandler;
        let ctx = Context::builder().build().await.unwrap();
        let ready = Ready {
            user: serenity::model::user::CurrentUser::default(),
            guilds: Vec::new(),
            session_id: String::new(),
            shard: None,
        };
        let message = Message {
            id: serenity::model::id::MessageId(0),
            channel_id: serenity::model::id::ChannelId(0),
            guild_id: Some(serenity::model::id::GuildId(123456789)), // Replace with an actual guild ID
            author: serenity::model::user::User::default(),
            content: "!play".to_string(),
            timestamp: String::new(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: Vec::new(),
            mention_roles: Vec::new(),
            mention_channels: None,
            attachments: Vec::new(),
            embeds: Vec::new(),
            reactions: None,
            nonce: None,
            pinned: false,
            webhook_id: None,
            activity: None,
            application: None,
            message_reference: None,
            flags: None,
            referenced_message: None,
            interaction: None,
        };

        handler.ready(ctx.clone(), ready).await;
        handler.message(ctx, message).await;
    }
}