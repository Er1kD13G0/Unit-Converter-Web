use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

pub async fn init_db() -> Result<Surreal<surrealdb::engine::local::Db>, surrealdb::Error> {
    let db = Surreal::new::<RocksDb>("file://data/conversor.db").await?;

    db.use_ns("conversor").use_db("conversor").await?;

    Ok(db)
}
