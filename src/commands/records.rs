use models::records::{Category, Runner, Record};

command!(top(_ctx, msg, args)
{
    let category_name = args.full();
    let category = Category::from_name(&category_name);
    if let Ok(records) = Record::get_top(category)
    {
        if records.len() > 0
        {
            let mut output = String::new();
            let mut i = 1;
            output.push_str(&format!("Top records for: **{}**\n", category));
            for r in records
            {
                let runner = Runner::from_id(r.runner_id).unwrap();
                output.push_str(&format!("({}) **{}** by **{}** :: {}\n", i, r.realtime_str() , runner.name, r.video));
                i += 1;
            }
            let _ = msg.channel_id.say(output);
            return Ok(());
        }
    }    
    let _ = msg.channel_id.say("Could not find any records for the specified category.");
});

command!(wr(_ctx, msg, args)
{
    let category_name = args.full();
    let category = Category::from_name(&category_name);
    if let Ok(record) = Record::get_wr(category)
    {
        let runner = Runner::from_id(record.runner_id).unwrap();
        let _ = msg.channel_id.say(format!("WR for *{}* is **{}** by **{}** :: *{}* :: {}", category, record.realtime_str(), runner.name, record.comment, record.video));
    } else {
        let _ = msg.channel_id.say("Could not find any records for the specified category.");
    }
});

command!(records(_ctx, msg, args)
{
    let runner_name = args.full();
    if let Ok(runner) = Runner::from_name(&runner_name)
    {
        if let Ok(records) = Record::get_records(runner.id)
        {
            if records.len() > 0
            {
                let mut output = String::new();
                output.push_str(&format!("Current records for: **{}**\n", runner.name));
                for r in records
                {
                    output.push_str(&format!("**{}** ({}) **{}** :: {} :: {}\n", r.category, r.get_rank(), r.realtime_str(), r.comment, r.video));
                }
                let _ = msg.channel_id.say(output);
            } else {
                let _ = msg.channel_id.say("No records found for this runner.");
            }
        } else {
            let _ = msg.channel_id.say("No records found for this runner.");
        }
    } else {
        let _ = msg.channel_id.say("No records found for this runner.");
    }
});

command!(pb(ctx, msg, args) {
    let runner_name = args.single::<String>();
    let category_name = args.full();

    if runner_name.is_ok()
    {
        let category = Category::from_name(&category_name);
        if category != Category::Unknown
        {
            let runner = Runner::from_name(&runner_name.unwrap());
            if let Ok(r) = runner
            {
                let pb = Record::get_pb(r.id, category);
                if let Ok(p) = pb
                {                
                    let _ = msg.channel_id.say(format!("Personal best for **{}** in *{}*:\n({}) **{}** :: {}", r.name, category, p.get_rank(), p.realtime_str(), p.video));
                } else {
                    let _ = msg.channel_id.say("No personal best found for this category.");
                }                                
            } else {
                let _ = msg.channel_id.say("The specified runner does not have any records.");
            }
        } else {
            let _ = msg.channel_id.say("The specified category does not exist.");
        }
    } else {
        let _ = msg.channel_id.say("You must specify a runner and a category.");
    }
});