use actix_web::{web, get, post, Responder, http::header::HttpDate, error::ErrorHttpVersionNotSupported, HttpResponse};

use crate::models::{Message, Conversation, message};

pub fn scope() -> actix_web::Scope
{
	web::scope("{conv_id}/messages")
		.service(get_messages)
		.service(add_message)
		.service(bot_response)
}

#[get("/")]
async fn get_messages(path : web::Path<(i32,)>) -> Result<impl Responder, crate::Error>
{
	let (conv_id,) = path.into_inner();
	let messages = Conversation::get(conv_id)?.get_messages()?;
	Ok(HttpResponse::Ok().json(messages))
}
#[derive(serde::Deserialize)]
struct AddMessageRequest
{
	content: String,
}
#[post("/")]
async fn add_message(path: web::Path<(i32,)>, json: web::Json<AddMessageRequest>) -> Result<impl Responder, crate::Error>
{
	let info = json.into_inner();
	let (conv_id,) = path.into_inner();
	Message::new(info.content, conv_id, "user".to_string())
}

#[get("/bot_response")]
async fn bot_response(path : web::Path<(i32,)>) -> impl Responder
{
	let (conv_id,) = path.into_inner();
	let responses = ["This is so cool", "this is nice"];
	let message = Message::new(responses[0].to_string(), conv_id, "bot".to_string());
	message
}