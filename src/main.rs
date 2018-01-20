#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate hyper;
#[macro_use] extern crate num_derive;
extern crate env_logger;
extern crate kankyo;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;
extern crate rusqlite;
extern crate time;
extern crate num;

mod db;
mod models;
mod api;
mod commands;
mod services;

use serenity::client::Client;
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework, help_commands};
use serenity::model::event::ResumedEvent;
use serenity::model::Ready;
use serenity::model::ChannelId;
use serenity::http;
use std::collections::HashSet;
use std::env;
use std::thread;

struct Handler;
impl EventHandler for Handler 
{
    fn on_ready(&self, _: Context, ready: Ready)
    {
        info!("Connected as {}", ready.user.name);
    }

    fn on_resume(&self, _: Context, _: ResumedEvent)
    {
        info!("Resumed");
    }
}


fn main()
{
    kankyo::load().expect("Failed to load .env-file.");
    env_logger::init();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler);

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .command("help", |c| c.exec_help(help_commands::plain))
        .group("Streams", |g| g
            .command("addstream", |c| c
                .desc("Adds a stream notification to a discord channel.")
                .example("tewtal shaktool")
                .num_args(2)
                .exec(commands::streams::addstream))
            .command("delstream", |c| c
                .desc("Removes a stream notification from a discord channel.")
                .example("tewtal shaktool")
                .num_args(2)
                .exec(commands::streams::delstream))
            .command("list", |c| c
                .desc("Lists stream notifications for a discord channel.")
                .example("shaktool")
                .num_args(1)
                .exec(commands::streams::list))
            .command("live", |c| c
                .desc("List all current active streams.")
                .example("")
                .exec(commands::streams::live))
        )
        .group("Records", |g| g
            .command("top", |c| c
                .desc("Displays the top 10 records for the specified category.")
                .example("100%")
                .min_args(1)
                .exec(commands::records::top))
            .command("records", |c| c
                .desc("Displays all records for the specified person.")
                .example("total")
                .num_args(1)
                .exec(commands::records::records))
            .command("pb", |c| c
                .desc("Displays the personal best for the specified person and category.")
                .example("total any%")
                .min_args(2)
                .exec(commands::records::pb))
            .command("wr", |c| c
                .desc("Displays the world record for the specified category.")
                .example("any%")
                .min_args(1)
                .exec(commands::records::wr))
        )
        .group("Misc", |g| g
            .command("strat", |c| c
                .desc("Searches crocomi.re for one or more strategies.")
                .example("mockball")
                .min_args(1)
                .exec(commands::misc::strat))
            .command("version", |c| c
                .desc("Displays the version of the bot.")
                .example("")
                .exec(commands::misc::version))
        )
    );

    /* Start thread that polls twitch for stream notifications and reports to the correct channel */
    let stream_service = thread::spawn(move || services::streams::worker());

    if let Err(why) = client.start()
    {
        error!("Client error: {:?}", why);
    }

}