pub struct Config {
    pub db_url: String,
    pub db_pool_min: Option<u32>,
    pub db_pool_max: u32,
    pub server_port: u32,
    pub server_bind: String
}


impl Config {
    pub fn new() -> Config {
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let db_pool_min = std::env::var("DB_POOL_MIN")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let db_pool_max = std::env::var("DB_POOL_MAX")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())    
            .unwrap_or(10);
            
        let server_port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())    
            .unwrap_or(7000);
            
        let server_bind = std::env::var("BIND")
            .unwrap_or("0.0.0.0".to_string());

        Config {
            db_pool_max,
            db_pool_min,
            db_url,
            server_port,
            server_bind
        }
    }
}
