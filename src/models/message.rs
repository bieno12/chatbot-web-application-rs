use actix_web::{Responder, body::BoxBody, http::header::HttpDate, HttpResponse};
use rusqlite::params;
use crate::models;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Message
{
	pub id : i32,
	pub created_at : String,
	pub content : String,
	pub conv_id : i32,
	pub sender : String,
}

impl Responder for Message {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl Message
{
    pub fn get(id: i32) -> Result<Self, crate::Error> {

        let conn = models::db().get_connection();

        let mut statement: rusqlite::Statement<'_> =
            conn.prepare("SELECT id, created_at, content, conv_id, sender FROM messages WHERE id = ?")?;

        let mut rows = statement.query(params![id])?;
        let row = rows.next()?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;


        Ok(Message {
            id: row.get(0)?,
            created_at: row.get(1)?,
            content: row.get(2)?,
            conv_id: row.get(3)?,
            sender: row.get(4)?,
        })
    }

    pub fn new(content: String, conv_id: i32, sender: String) -> Result<Self, crate::Error>
    {
        let conn = models::db().get_connection();
        let mut statement = conn.prepare("
            INSERT INTO messages (content, conv_id, sender)
            VALUES (?, ?, ?)
        ")?;
        let rowid = statement.insert(params![content, conv_id, sender])?;
        let new_message = conn.query_row("
            SELECT id, created_at, content, conv_id, sender FROM messages WHERE rowid = ?
        ", [rowid], |row| {
            let created_at: String = row.get(1)?;
            Ok(Self
            {
                id: row.get(0)?,
                created_at,
                content: row.get(2)?,
                conv_id: row.get(3)?,
                sender: row.get(4)?,
            })
        });
        new_message.map_err(|x| x.into())
    } 

	pub fn save(&self) -> Result<(), crate::Error> {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare(
            "INSERT INTO messages (id, created_at, content, conv_id, sender)
             VALUES (?, ?, ?, ?, ?)",
        )?;

        statement.execute(params![
            self.id,
            self.created_at,
            self.content,
            self.conv_id,
            self.sender,
        ])?;

        Ok(())
    }
    pub fn delete(id: i32) -> Result<(), crate::Error> {
        let conn = models::db().get_connection();

        let mut statement = conn.prepare("DELETE FROM messages WHERE id = ?")?;
        statement.execute(params![id])?;

        Ok(())
    }
}