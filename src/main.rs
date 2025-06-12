#[macro_use] extern crate rocket;

mod conversor;
mod db;
mod models;

use rocket::fs::FileServer;
use rocket::serde::{json::Json, json::json};
use rocket::State;
use surrealdb::{Surreal, engine::local::Db};
use chrono::Local;
use std::path::Path;

struct CORS;

#[rocket::async_trait]
impl rocket::fairing::Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "CORS Fairing",
            kind: rocket::fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r rocket::Request<'_>, response: &mut rocket::Response<'r>) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[post("/converter", data = "<req>")]
async fn converter(
    req: Json<models::ConversaoRequest>,
    db: &State<Surreal<Db>>
) -> Json<serde_json::Value> {
    let resultado = match conversor::converter_unidades(req.valor, &req.de, &req.para) {
        Ok(val) => val,
        Err(e) => return Json(json!({"success": false, "error": e.to_string()}))
    };

    
    let _: Result<Option<serde_json::Value>, surrealdb::Error> = db.create("conversao")
        .content(json!({
            "valor_original": req.valor,
            "valor_convertido": resultado,
            "de": req.de,
            "para": req.para,
            "timestamp": Local::now().to_rfc3339()
        }))
        .await;

    Json(json!({
        "original_value": req.valor,
        "converted_value": resultado,
        "from_unit": req.de,
        "to_unit": req.para
    }))
}

#[get("/health")]
fn health_check() -> &'static str {
    "OK"
}

#[launch]
async fn rocket() -> _ {
    
    let static_path = Path::new("static");
    if !static_path.exists() {
        panic!("Pasta 'static' não encontrada!");
    }

    
    let db = db::init_db().await.expect("Falha ao conectar ao banco");

    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/api", routes![converter, health_check])
        .mount("/", FileServer::from(static_path))
}
