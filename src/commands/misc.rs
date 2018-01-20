use std::env;
use api::crocomire;

command!(version(ctx, msg) {
    let version = &env::var("CARGO_PKG_VERSION").unwrap_or(String::from("<unknown>"));
    let _ = msg.channel_id.say(format!("Shaktool™ by total - Version: {}", version));
});

command!(strat(ctx, msg, args) {
    let strat = args.full();
    if strat.len() > 0
    {
        if let Ok(strats) = crocomire::Strategy::find(&strat)
        {
            if strats.len() > 0
            {
                let mut output = String::new();
                for s in strats
                {
                    output.push_str(&format!("**{}** *({}/{})* :: http://crocomi.re/{}\n", s.name, s.area_name, s.room_name, s.id));            
                }
                let _ = msg.channel_id.say(output);
            } else {
                let _ = msg.channel_id.say("No results found.");
            }
        }
    } else {
        let _ = msg.channel_id.say("You need to specify a search string.");
    }
});