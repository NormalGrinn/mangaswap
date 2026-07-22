use std::clone;

use crate::{Context, Error, database, types::UserInfo, utilities::ensure_host_role};
use poise::CreateReply;
use rusqlite::Result;
use serenity::all::{CreateAttachment, CreateMessage};
use tokio::{fs::{self, OpenOptions}, io::AsyncWriteExt};

const PATH: &str = "status.txt";

fn print_if_exists(letter: Option<String>) -> String {
    match letter {
        Some(l) => l,
        None => "This user does not have a letter".to_owned(),
    }
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn status(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !ensure_host_role(&ctx, ctx.author()).await? {return Ok(())}

    let users_res = database::get_all_users().await;
    let mut users: Vec<UserInfo> = Vec::new();

    match users_res {
        Ok(u) => users = u,
        Err(e) => {
            ctx.send(CreateReply::default().content("Error fetching users").ephemeral(true)).await?;
            eprintln!("Error fetching users: {}", e);
            return Ok(());
        },
    }
    
    let letter_less: Vec<(u64, String)> = users.clone().into_iter()
        .filter(|u| u.letter.is_none())
        .map(|u| (u.discord_id, u.username))
        .collect();

    let unmatched: Vec<(u64, String)> = users.clone().into_iter()
        .filter(|u| u.giftee_id.is_none())
        .map(|u| (u.discord_id, u.username))
        .collect();

    let submission_less: Vec<(u64, String)> = users.clone().into_iter()
        .filter(|u| u.submission.is_none())
        .map(|u| (u.discord_id, u.username))
        .collect();

    let submissions: Vec<(u64, String, Option<String>)> = users.into_iter()
        .map(|u| (u.discord_id, u.username, u.submission))
        .collect();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(PATH)
        .await?;
    let mut buffer = String::new();

    buffer.push_str("Users who have not yet written a letter:\n");
    for (id, name) in letter_less { buffer.push_str(&format!("{}, {}\n", name, id)) }
    buffer.push_str("\nUsers who are not yet matched:\n");
    for (id, name) in unmatched { buffer.push_str(&format!("{}, {}\n", name, id)) }
    buffer.push_str("\nUsers without submissions:\n");
    for (id, name) in submission_less { buffer.push_str(&format!("{}, {}\n", name, id)) }
    buffer.push_str("\nAll the submissions of the users:\n");
    for (id, name, submission) in submissions { buffer.push_str(&format!("{}, {}\n{}\n", name, id, print_if_exists(submission))) }

    fs::write(PATH, buffer.as_bytes()).await?;
    file.flush().await?;

    let attachment = CreateAttachment::path(PATH).await?;
    let builder = CreateMessage::new().add_file(attachment);

    match ctx.author().direct_message(ctx.http(), builder).await {
        Ok(_) => {
            ctx.send(CreateReply::default().content("Successfully sent status").ephemeral(true)).await?;
        },
        Err(_) => {
            ctx.send(CreateReply::default().content("Error sending status").ephemeral(true)).await?;
        },
    };

    let _ = tokio::fs::remove_file(PATH).await; // Ignore error if the file doesn't exist

    Ok(())
}