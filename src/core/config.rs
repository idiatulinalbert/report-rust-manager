use clap::Parser;





#[derive(Parser)]
struct env_settings{
    #[arg(long,env = "DATABASE_URL")]
     DB_URL:String,

}


pub struct AppConfig;

impl AppConfig {
    pub async fn build(&self) -> anyhow::Result<env_settings>{
        let env_data = env_settings::parse();
        anyhow::Ok(env_data)
    }
}