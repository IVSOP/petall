use clap::Parser;


#[derive(Parser)]
pub struct Config {
    #[arg(long, env)]
    pub pg_host: String,
    #[arg(long, env, default_value_t = 5432)]
    pub pg_port: u16,
    #[arg(long, env)]
    pub pg_user: String,
    #[arg(long, env)]
    pub pg_password: String,
    #[arg(long, env)]
    pub pg_database: String,
    #[arg(long, env)]
    pub bind_ip: String,
    #[arg(long, env, default_value_t = 8080)]
    pub bind_port: u16,
}
