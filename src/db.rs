use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use std::time::Duration;

pub async fn init_db() -> Result<Surreal<surrealdb::engine::local::Db>, surrealdb::Error> {
    
    let config = surrealdb::opt::Config::default()
        .query_timeout(Duration::from_secs(3));

    
    let db = match Surreal::new::<RocksDb>(("file://data/conversor.db", config)).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Tentando reconexão sem configurações especiais...");
            Surreal::new::<RocksDb>("file://data/conversor.db").await?
        }
    };

    
    db.use_ns("conversor").use_db("conversor").await?;
    
    Ok(db)
}
