use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serenity::prelude::*;

pub struct CoinDatabase;

impl TypeMapKey for CoinDatabase {
	type Value = HashMap<u64, u64>;
}

pub fn _set_coincount(ctx: &mut Context, userid: u64, amount: u64)
{
	let mut data = ctx.data.write();
	let database = data.get_mut::<CoinDatabase>().expect("Unable to get CoinDatabase");

	database.insert(userid, amount);
}

pub fn get_coincount(ctx: &mut Context, userid: u64) -> u64
{
	let mut data = ctx.data.write();
	let database = data.get_mut::<CoinDatabase>().expect("Unable to get CoinDatabase");

	*database.entry(userid).or_insert(0)
}

pub fn inc_coincount(ctx: &mut Context, userid: u64, amount: u64) -> u64
{
	let mut data = ctx.data.write();
	let database = data.get_mut::<CoinDatabase>().expect("Unable to get CoinDatabase");
	let entry = database.entry(userid).or_insert(0);
	*entry += amount;

	*entry
}

pub fn save_database(ctx: &mut Context)
{
	let mut file = File::create("database.ron").expect("Unable to open database file");

	let data = ctx.data.write();
	let database = data.get::<CoinDatabase>().expect("Unable to get CoinDatabase");

	let ron = ron::ser::to_string(database).expect("Unable to parse CoinDatabase to RON");
	file.write_all(ron.as_bytes()).expect("Unable to write out database");
}