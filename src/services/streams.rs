use std::thread;
use serenity::model::ChannelId;
use models::streams;
use api::twitch;

pub fn worker()
{
    /* Wait for discord to connect */
    thread::sleep_ms(5000);

    /* Do any initialization if needed */
}