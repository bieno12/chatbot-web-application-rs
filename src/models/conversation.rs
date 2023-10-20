use actix_web::{Responder, body::BoxBody, HttpResponse};
use rusqlite::params;
use crate::models;
use models::Message;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Conversation
{
	pub id: i32,
	pub created_at: String,
	pub name: String,

}

impl Responder for Conversation {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl Conversation {
    pub fn get(id: i32) -> Result<Self, crate::Error> {
        let conn = models::db().get_connection();

        let mut statement =
            conn.prepare("SELECT id, created_at, name FROM conversations WHERE id = ?")?;

        let mut rows = statement.query(params![id])?;
        let row = rows.next()?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;

        let created_at: String = row.get(1)?;

        Ok(Conversation {
			id: row.get(0)?,
			created_at,
			name: row.get(2)?
		})
    }

    pub fn get_all() -> Result<Vec<Self>, crate::Error>
    {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare(
            "SELECT id, created_at, name
             FROM conversations
             ORDER BY created_at",
        )?;

        let rows = statement.query_map([], |row| {
			let created_at: String = row.get(1)?;
            Ok(Self {
                id: row.get(0)?,
                created_at,
                name: row.get(2)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows
        {
            result.push(row?)
        }

        Ok(result)
    }

    pub fn new(name : String) -> Result<Self, crate::Error>
    {
        let conn = models::db().get_connection();
        let mut statement = conn.prepare("
            INSERT INTO conversations (name)
            VALUES (?)
        ")?;

        let rowid = statement.insert(params![name])?;
        let new_conv = conn.query_row("select * from conversations where rowid=?", params![rowid], |row|{
            let created_at: String = row.get(1)?;
            Ok(Self
            {
                id: row.get(0)?,
                created_at,
                name: row.get(2)?,
            })
        });
        new_conv.map_err(|x|x.into())
    } 
    pub fn save(&self) -> Result<(), crate::Error> {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare(
            "INSERT INTO conversations (id, created_at, name)
             VALUES (?, ?, ?)",
        )?;

        statement.execute(params![
            self.id,
            self.created_at,
            self.name,
        ])?;
        
        Ok(())
    }

    pub fn delete(id : i32) -> Result<(), crate::Error> {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare("DELETE FROM conversations WHERE id = ?")?;
        statement.execute(params![id])?;

        // Delete related messages
        let mut message_statement = conn.prepare("DELETE FROM messages WHERE conv_id = ?")?;
        message_statement.execute(params![id])?;

        Ok(())
    }

    pub fn get_messages(&self) -> Result<Vec<Message>, crate::Error> {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare(
            "SELECT id, created_at, content, conv_id, sender
             FROM messages
             WHERE conv_id = ?
             ORDER BY created_at",
        )?;

        let rows = statement.query_map(params![self.id], |row| {
			let created_at: String = row.get(1)?;
            Ok(Message {
                id: row.get(0)?,
                created_at,
                content: row.get(2)?,
                conv_id: row.get(3)?,
                sender: row.get(4)?,
            })
        })?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    }
}
