use crate::{Context, Error, database, types::Phase, utilities::ensure_host_role};
use poise::CreateReply;
use rusqlite::Result;
use serenity::all::User;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ban_user(
    ctx: Context<'_>,
    #[description = "User"]
    user: User,
) -> Result<(), Error> {
    if !ensure_host_role(&ctx, ctx.author()).await? {return Ok(())}
    
    let current_phase = database::get_phase()?;
    
    let res = database::ban_user(user.id.get());
    match res {
        Ok(_) => {
            if current_phase == Phase::Join {
                let message = format!("Banned user: {}", user.name);
                ctx.send(CreateReply::default()
                    .content(message)
                    .ephemeral(true)).await?;
                } else {
                
            }
        },
        Err(e) => {
            let message = format!("Error banning user: {}", user.name);
            ctx.send(CreateReply::default()
                .content(message)
                .ephemeral(true)).await?;
                eprintln!("Error banning user: {} {}", user.name, e);
        }
    }


    Ok(())
}