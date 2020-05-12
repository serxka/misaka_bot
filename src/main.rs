// Standard Library
use std::env;
use std::collections::{HashMap, HashSet};
use std::fs::File;

// Crates
use log::{info, debug, error};
use serenity::{
  prelude::*,
  model::prelude::*,
  model::gateway::Ready,
  Client,
  framework::standard::{
  		StandardFramework,
        macros::group
    }
};

// Modules
mod database;
use database::CoinDatabase;
mod commands;
use commands::{
	meta::*,
	general::*,
	admin::*,
};

// Command groups
#[group]
#[commands(ping, hello)]
struct Meta;

#[group]
#[commands(coin, balance)]
struct General;

#[group]
#[checks(Owner)]
#[commands(clear, save, stop)]
struct Owner;

struct Handler;
impl EventHandler for Handler {
	fn ready(&self, _: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);
	}
	fn message(&self, _ctx: Context, msg: Message) {
		debug!("{}({}) sent a message to {} with the content: {}", msg.author.id, msg.author.name, msg.channel_id, msg.content);
	}
}

fn main() {
	// Load the token from the .env file
	dotenv::dotenv().ok();
	env_logger::init();
	let discord_token = env::var("TOKEN").expect("Couldn't find TOKEN in enviroment");
	
	// Create the client
	let mut client = Client::new(&discord_token, Handler).expect("Couldn't create the bot");

	// Open database
	let coin_database: HashMap<u64, u64>;
	if std::path::Path::new("database.ron").exists() {
		let database_file = File::open("database.ron").expect("Couldn't open database.ron");
		coin_database = ron::de::from_reader(database_file).expect("Couldn't parse RON database");
	} else { coin_database = HashMap::default(); }

	// Add CoinDatabase to the client data
	{
		let mut data = client.data.write();
		data.insert::<CoinDatabase>(coin_database);
	}

	// Setup owners
	let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
		Ok(info) => {
			let mut owners = HashSet::new();
			owners.insert(info.owner.id);

			(owners, info.id)
		},
		Err(e) => {panic!("Could not access application info: {:?}", e);},
	};

	// Setup command framework
	client.with_framework(StandardFramework::new()
		.configure(|c| c
			.with_whitespace(true)
			.on_mention(Some(bot_id))
			.prefix("~")
			.owners(owners)
			.delimiters(vec![", ", ","]))
		.group(&META_GROUP)
		.group(&GENERAL_GROUP)
		.group(&OWNER_GROUP)
	);

	// Start the client
	if let Err(e) = client.start() {
		error!("Client error: {:?}", e);
	}
}