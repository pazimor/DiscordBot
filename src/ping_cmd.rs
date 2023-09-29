use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong !").await?;
    Ok(())
}

#[command]
pub async fn ping_ip(ctx: &Context, msg: &Message) -> CommandResult {
    //add some metrics and ping a computer
    msg.reply(ctx, "PongIp !").await?;
    Ok(())
}

#[command]
pub async fn pick_vpn(ctx: &Context, msg: &Message) -> CommandResult {
    //send corresponding .ovpn file corresponding to the user (Private Message if possible)
    msg.reply(ctx, "pickVpn !").await?;
    Ok(())
}

#[command]
pub async fn pc_on(ctx: &Context, msg: &Message) -> CommandResult {
    //restrict this Command to administrator
    //call api GET from DIY WOL
    msg.reply(ctx, "pcOn !").await?;
    Ok(())
}

#[command]
pub async fn pc_off(ctx: &Context, msg: &Message) -> CommandResult {
    //restrict this Command to administrator
    //call api GET from DIY WOL
    msg.reply(ctx, "pcOff !").await?;
    Ok(())
}