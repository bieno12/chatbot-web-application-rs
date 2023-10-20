use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use std::path::PathBuf;

pub async fn home(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".into();
    Ok(NamedFile::open(path)?)
}

pub async fn serve_static(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    let full_path = PathBuf::from("./static/").join(path);
    Ok(NamedFile::open(full_path)?)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(actix_web::web::resource("/").route(actix_web::web::get().to(home)))
        .service(web::resource("/static/{filename:.*}").route(web::get().to(serve_static)));
}
