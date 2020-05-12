use log::info;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult, CheckResult,
    macros::{check, command},
};

use crate::database::{save_database};

#[command]
pub fn clear(ctx: &mut Context, msg: &Message) -> CommandResult
{
	msg.channel_id.say(&ctx, "clearing! not")?;

	Ok(())
}

#[command]
pub fn stop(ctx: &mut Context, msg: &Message) -> CommandResult
{
	msg.channel_id.say(&ctx, "Saving database and shutting down...")?;

	save_database(ctx);

	info!("Bot was stopped");
	std::process::exit(0);
}

#[command]
pub fn save(ctx: &mut Context, msg: &Message) -> CommandResult
{
	save_database(ctx);
	msg.channel_id.say(&ctx, "Saved database")?;

	info!("Database was forcefully saved");
	Ok(())
}

#[check]
#[name = "Owner"]
pub fn owner_check(_: &mut Context, msg: &Message) -> CheckResult {
    if msg.author.id == 325444141469270016 {
    	true.into()
    } else {
    	info!("{} ({}) tried to use a owner command", msg.author.id, msg.author.name);
    	false.into()
    }
}
