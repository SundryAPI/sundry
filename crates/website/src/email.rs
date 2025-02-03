use core::fmt;
use leptos::prelude::ServerFnError;
use leptos::prelude::*;
use sendgrid::v3::{Email as SendgridEmail, Message, Personalization, Sender, ASM};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Email<'a> {
    pub email: &'a str,
    pub name: Option<&'a str>,
    pub content: Option<&'a str>,
    pub template: SendgridTemplate,
}

impl<'a> Email<'a> {
    pub async fn send(&self) -> Result<(), leptos::prelude::Error> {
        let from = self.template.get_sender();
        let from = SendgridEmail::new(from.address()).set_name(from.to_string());

        let subject = self.template.get_subject();

        let mut data = HashMap::new();
        data.insert("subject".to_string(), subject.to_string());
        if let Some(body) = self.content {
            data.insert("content".to_string(), body.to_string());
        }
        if let Some(name) = self.name {
            data.insert("first_name".to_string(), name.to_string());
        }

        let recipient = self.email;

        let personalization = Personalization::new(SendgridEmail::new(recipient.to_string()))
            .add_dynamic_template_data(data);

        let personalization = if let Some(bcc) = self.template.get_bcc() {
            personalization.add_bcc(bcc)
        } else {
            personalization
        };

        let mail = Message::new(SendgridEmail::new(recipient.to_string()))
            .set_from(from.clone())
            .set_reply_to(from)
            .add_personalization(personalization)
            .set_asm(
                ASM::new()
                    .set_group_id(29928)
                    .set_groups_to_display(HashSet::from([29928]))
                    .map_err(|e| ServerFnError::new(e.to_string()))?,
            )
            .set_template_id(self.template.get_id());

        let api_key = std::env::var("SENDGRID_API_KEY")
            .map_err(|_| ServerFnError::new("No Sendgrid API key found.".to_string()))?;

        let sender = Sender::new(api_key, None);
        sender
            .send(&mail)
            .await
            .map_err(|_| ServerFnError::new("Faild to send email".to_string()))?;

        Ok(())
    }
}

pub enum SendgridSenders {
    Sundry,
    MontanaLow,
    Cassandra,
    Daniel,
}

// change these senders to sundry url when montana updates sendgrid
impl SendgridSenders {
    pub fn address(&self) -> String {
        match *self {
            SendgridSenders::Sundry => "team@getsundry.app".to_string(),
            SendgridSenders::MontanaLow => "montana@getsundry.app".to_string(),
            SendgridSenders::Cassandra => "cassandra@getsundry.app".to_string(),
            SendgridSenders::Daniel => "daniel@getsundry.app".to_string(),
        }
    }
}

impl fmt::Display for SendgridSenders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendgridSenders::Sundry => write!(f, "team @sundry"),
            SendgridSenders::MontanaLow => write!(f, "Montana Low"),
            SendgridSenders::Cassandra => write!(f, "Cassandra Stumer"),
            SendgridSenders::Daniel => write!(f, "Daniel Illenberger"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SendgridTemplate {
    Feedback,
    InviteUser,
}

impl SendgridTemplate {
    pub fn get_id(&self) -> &'static str {
        match self {
            SendgridTemplate::Feedback => "d-853538826c4b48c6a21f06e96b4f895d",
            SendgridTemplate::InviteUser => "d-505bf2598d604c9fa43d788b86f0bcaa",
        }
    }

    pub fn get_sender(&self) -> SendgridSenders {
        match self {
            SendgridTemplate::Feedback => SendgridSenders::Sundry,
            SendgridTemplate::InviteUser => SendgridSenders::Sundry,
        }
    }

    pub fn get_bcc(&self) -> Option<SendgridEmail> {
        match self {
            SendgridTemplate::Feedback => {
                Some(SendgridEmail::new("team@getsundry.app".to_string()))
            }
            _ => None,
        }
    }

    pub fn get_subject(&self) -> &'static str {
        match self {
            SendgridTemplate::Feedback => "Feedback",
            SendgridTemplate::InviteUser => "Invite to Sundry",
        }
    }
}
