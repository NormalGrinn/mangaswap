use crate::database;
use crate::types::Phase::{self, Join, Read, Swap};
use crate::{utilities::ensure_host_role, Context, Error};
use poise::CreateReply;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn set_phase(
    ctx: Context<'_>,
    #[description = "The phase you want to set it to"]
    phase: Phase,
) -> Result<(), Error> {
    if !ensure_host_role(&ctx, ctx.author()).await? { return Ok(()) };

    let current_phase = database::get_phase()?;
    if current_phase == phase {
        let message = format!("You are already in this phase!");
        ctx.send(CreateReply::default()
            .content(message)
            .ephemeral(true)).await?;
    }

    match phase {
        Phase::Join => {
            if current_phase == Swap {
                let message = format!("You cannot change from the swap to the join phase.");
                ctx.send(CreateReply::default()
                    .content(message)
                    .ephemeral(true)).await?;
            } else {
                // Reset everything
                database::set_phase(phase)?;
            }
        },
        Phase::Swap => {
            if current_phase == Read {
                let message = format!("You cannot change from the read to the swap phase.");
                ctx.send(CreateReply::default()
                    .content(message)
                    .ephemeral(true)).await?;
            } else {
                // Match users
                database::set_phase(phase)?;
            }
        },
        Phase::Read => {
            if current_phase == Join {
                let message = format!("You cannot change from the join to the read phase.");
                ctx.send(CreateReply::default()
                    .content(message)
                    .ephemeral(true)).await?;
            } else {
                database::set_phase(phase)?;
                let message = format!("Changed phase to read");
                ctx.send(CreateReply::default()
                    .content(message)
                    .ephemeral(true)).await?;
            }
        },
    }
    Ok(())
}