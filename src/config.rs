#[derive(clap::Parser)]
pub struct Config { 
    #[clap(long, env)]
    pub postgres_user: String,
    
    #[clap(long, env)]
    pub postgres_password: String,
    
    #[clap(long, env)]
    pub postgres_db: String,
    
    #[clap(long, env)]
    pub postgres_port: String,
    
    #[clap(long, env)]
    pub hmac_key: String,
}
