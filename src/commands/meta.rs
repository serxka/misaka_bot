use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult
{
	msg.channel_id.say(&ctx, "Pong!")?;

	Ok(())
}

#[command]
#[aliases("hi", "hey")]
pub fn hello(ctx: &mut Context, msg: &Message) -> CommandResult
{
	msg.channel_id.say(&ctx, format!("Hi {}!", msg.author.name))?;

	Ok(())
}