use serenity::all::{ActivityData, Context, CreateBotAuthParameters, Permissions, Scope};

pub async fn handle(ctx: Context) {
    // Generate an invite link
    let builder = CreateBotAuthParameters::new()
        .permissions(Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES | Permissions::CONNECT)
        .scopes(&[Scope::Bot])
        .auto_client_id(&ctx.http)
        .await
        .expect("Error generating invite link");

    ctx.set_activity(Some(ActivityData::custom("!help for commands")));
    println!("Invite Link: {}", builder.build());
}
