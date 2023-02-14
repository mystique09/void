use serenity::{
    async_trait,
    model::{
        prelude::{Activity, Ready},
        user::OnlineStatus,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity: Option<Activity> = Some(Activity::playing("NeoVim"));

        ctx.set_presence(activity, OnlineStatus::Online).await;
        println!("{} is now open.", &ready.user.name);
    }
}
