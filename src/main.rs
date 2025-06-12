#[macro_use] extern crate rocket;

mod conversor;
mod db;
mod models;

use rocket::fs::FileServer;
use rocket::serde::{json::Json, json::json};
use rocket::State;
use std::path::Path;
use surrealdb::{Surreal, engine::local::Db};
use chrono::Local;
use models::{ConversaoRequest, RegistroConversao};

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
    req: Json<ConversaoRequest>,
    db: &State<Surreal<Db>>
) -> Result<Json<serde_json::Value>, Json<String>> {
    let resultado = match conversor::converter_temperatura(req.valor, &req.de, &req.para) {
        Ok(val) => val,
        Err(e) => return Err(Json(e.to_string()))
    };

    let registro: Option<RegistroConversao> = db
        .create("conversao")
        .content(RegistroConversao {
            id: None,
            data: models::Conversao {
                valor: req.valor,
                de: req.de.clone(),
                para: req.para.clone(),
                resultado,
                usuario: None,
            },
            created_at: Local::now().to_rfc3339(),
        })
        .await
        .map_err(|e| Json(e.to_string()))?;

    Ok(Json(json!({
    "valor_original": req.valor,
    "valor_convertido": resultado,
    "de": req.de,
    "para": req.para,
    "registro_id": registro.unwrap().id.unwrap().id.to_string()
})))
}

#[options("/converter")]
fn options_handler() -> &'static str {
    ""
}

#[get("/historico?<limite>")]
async fn historico(
    limite: Option<u32>,
    db: &State<Surreal<Db>>
) -> Result<Json<Vec<RegistroConversao>>, Json<String>> {
    let mut query = db
        .query("SELECT * FROM conversao ORDER BY created_at DESC LIMIT $limite")
        .bind(("limite", limite.unwrap_or(10)))
        .await
        .map_err(|e| Json(e.to_string()))?;

    let historico: Vec<RegistroConversao> = query.take(0).map_err(|e| Json(e.to_string()))?;
    Ok(Json(historico))
}

#[launch]
async fn rocket() -> _ {
    let static_path = Path::new("static");
    if !static_path.exists() || !static_path.is_dir() {
        panic!("Pasta 'static' não encontrada ou não é um diretório!");
    }

    let db = db::init_db().await.expect("Falha ao conectar ao banco de dados");

    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/api", routes![converter, options_handler, historico])
        .mount("/", FileServer::from(static_path))
}
