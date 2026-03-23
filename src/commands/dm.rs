use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct DmSendResult {
    username: String,
    success: bool,
}

impl Tableable for DmSendResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Action", "To", "Status"]);
        table.add_row(vec![
            "send_dm",
            self.username.as_str(),
            if self.success { "Sent" } else { "Failed" },
        ]);
        table
    }
}

#[derive(Serialize)]
struct InboxList {
    conversations: Vec<ConversationRow>,
}

#[derive(Serialize)]
struct ConversationRow {
    id: String,
    participants: String,
}

impl Tableable for InboxList {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Conversation ID", "Participant IDs"]);
        for c in &self.conversations {
            table.add_row(vec![c.id.as_str(), c.participants.as_str()]);
        }
        table
    }
}

#[derive(Serialize)]
struct MessageList {
    messages: Vec<MessageRow>,
}

#[derive(Serialize)]
struct MessageRow {
    sender: String,
    text: String,
    date: String,
}

impl Tableable for MessageList {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Sender", "Text", "Date"]);
        for m in &self.messages {
            table.add_row(vec![m.sender.as_str(), m.text.as_str(), m.date.as_str()]);
        }
        table
    }
}

pub async fn send(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    username: &str,
    text: &str,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let user = api.get_user_by_username(username).await?;
    api.send_dm(&user.id, text).await?;
    let display = DmSendResult {
        username: username.to_string(),
        success: true,
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn inbox(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let convos = api.get_dm_conversations(count).await?;
    let display = InboxList {
        conversations: convos.into_iter().map(|c| ConversationRow {
            id: c.id,
            participants: c.participant_ids.join(", "),
        }).collect(),
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn thread(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    id: &str,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let messages = api.get_dm_messages(id, count).await?;
    let display = MessageList {
        messages: messages.into_iter().map(|m| MessageRow {
            sender: m.sender_id.unwrap_or_default(),
            text: m.text.unwrap_or_default(),
            date: m.created_at.unwrap_or_default(),
        }).collect(),
    };
    output::render(format, &display, None);
    Ok(())
}
