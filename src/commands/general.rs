use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use crate::database::{inc_coincount, get_coincount};

#[command]
pub fn coin(ctx: &mut Context, msg: &Message) -> CommandResult
{
	inc_coincount(ctx, *msg.author.id.as_u64(), 1);
	msg.channel_id.say(&ctx, ":new_moon_with_face:")?;

	Ok(())
}

#[command]
#[aliases("bal")]
pub fn balance(ctx: &mut Context, msg: &Message) -> CommandResult
{
	let coins = get_coincount(ctx, *msg.author.id.as_u64());

	msg.channel_id.send_message(&ctx, |m| {
		m.embed(|e| {
			e.title("Balance");
			e.description(format!(":new_moon_with_face: {}", coins));
			e});
	m})?;

	Ok(())
}