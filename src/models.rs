use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversaoRequest {
    pub valor: f64,
    pub de: String,
    pub para: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversao {
    pub valor: f64,
    pub de: String,
    pub para: String,
    pub resultado: f64,
    pub usuario: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistroConversao {
    pub id: Option<Thing>,
    pub data: Conversao,
    pub created_at: String,
}
