use ping::ping;
use std::net::IpAddr;
use std::time::Duration;
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command,
};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
pub async fn ping_default(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong !").await?;
    Ok(())
}

#[command]
pub async fn ping_ip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let ip: IpAddr = args.rest().parse().unwrap();
    let duration: Option<Duration> = Some(Duration::from_secs(1));

    println!("{}", args.rest());
    match ping(ip, duration, None, None, None, None) {
        Ok(response) => {
            msg.reply(ctx, format!("Ping to {} succeeded.", ip)).await?;
            Ok(())
        }
        Err(_err) => {
            msg.reply(ctx, format!("Probleme: {:?}", _err)).await?;
            Ok(())
        }
    }
}

#[command]
pub async fn pick_vpn(ctx: &Context, msg: &Message) -> CommandResult {
    //send corresponding .ovpn file corresponding to the user (Private Message if possible)
    msg.reply(ctx, "pick Vpn !").await?;
    Ok(())
}

#[command]
pub async fn pc_on(ctx: &Context, msg: &Message) -> CommandResult {
    //restrict this Command to administrator
    //call api GET from DIY WOL
    msg.reply(ctx, "pc On !").await?;
    Ok(())
}

#[command]
pub async fn pc_off(ctx: &Context, msg: &Message) -> CommandResult {
    //restrict this Command to administrator
    //call api GET from DIY WOL
    msg.reply(ctx, "pc Off !").await?;
    Ok(())
}