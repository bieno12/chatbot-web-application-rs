use crate::models::Conversation;
use crate::models::Message;
use actix_web::put;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub fn scope() -> actix_web::Scope {
    web::scope("/conversations")
        .service(get_conversations)
        .service(new_conversation)
        .service(get_conversation)
        .service(del_conversation)
        .service(update_conversation)
}

#[get("/")]
async fn get_conversations() -> impl Responder {
    let convs = Conversation::get_all();
    match convs {
        Ok(convs) => HttpResponse::Ok().json(convs),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e:#?}")),
    }
}
#[derive(serde::Deserialize)]
struct ConversationInfo {
    name: String,
}
#[post("/")]
async fn new_conversation(json: web::Json<ConversationInfo>) -> impl Responder {
    let name = json.into_inner().name;
    let conv = Conversation::new(name);
    match conv {
        Ok(conv) => HttpResponse::Ok().json(conv),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e:#?}")),
    }
}

#[get("{conv_id}")]
async fn get_conversation(path: web::Path<(i32,)>) -> impl Responder {
    let (id,) = path.into_inner();
    let conv = Conversation::get(id);
    match conv {
        Ok(conv) => HttpResponse::Ok().json(conv),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e:#?}")),
    }
}

#[put("{conv_id}")]
async fn update_conversation(
    path: web::Path<(i32,)>,
    json: web::Json<ConversationInfo>,
) -> actix_web::Result<impl Responder> {
    let (conv_id,) = path.into_inner();
    let conv_info = json.into_inner();
    let mut conv = Conversation::get(conv_id)?;
    conv.name = conv_info.name;
    conv.save();
    Ok(conv)
}

#[delete("{conv_id}")]
async fn del_conversation(path: web::Path<(i32,)>) -> impl Responder {
    let (id,) = path.into_inner();
    Conversation::delete(id).map(|_| HttpResponse::Ok().body("deleted successfully"))
}
