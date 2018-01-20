command!(addstream(_ctx, msg)
{
    let _ = msg.channel_id.say("Addstream");

});

command!(delstream(_ctx, msg)
{
    let _ = msg.channel_id.say("Delstream");

});

command!(list(_ctx, msg)
{
    let _ = msg.channel_id.say("List");

});

command!(live(_ctx, msg)
{
    let _ = msg.channel_id.say("Live");

});
