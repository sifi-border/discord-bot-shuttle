use crate::{Context, Error};

/// Responds with "pong!"
#[poise::command(slash_command)]
pub async fn ping<'a>(ctx: Context<'a>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}
