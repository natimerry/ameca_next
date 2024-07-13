use serde::{Deserialize, Serialize};
use serenity::all::GuildId;
use crate::db::database::Database;

use chrono::{DateTime, Utc};
use tracing::{debug, error, info};
use surrealdb::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub guild_id: String,
    pub time_of_join: DateTime<Utc>,
}

pub trait GuildData{
    async fn joined_guild(db:&Database,members: u64,guild_id:GuildId);
    async fn get_all_guilds(db:&Database) -> Option<Vec<Guild>>;
}


impl GuildData for Database {
    async fn joined_guild(db:&Database,members: u64,guild_id:GuildId){
        info!("Registering new GUILD in the database");
        let created_guild: Result<Option<Guild>, surrealdb::Error> = db.client
            .create(("guild",guild_id.get()))
            .content(Guild{
                guild_id: guild_id.to_string(),
                time_of_join: Utc::now(),
            }).await;
        debug!("{:?}",created_guild);
    }

    async fn get_all_guilds(db: &Database) -> Option<Vec<Guild>> {
        let query = "SELECT * FROM guild;";
        let mut response = db.db_query(query).await.expect("Unable to query database for guilds");
        debug!("{response:#?}");
        let guilds :Result<Vec<Guild>,Error> = response.take(0);
        return match guilds{
            Ok(found_guilds) => {
                debug!("{:#?}",found_guilds);
                Some(found_guilds)
            },
            Err(e) => {
                error!("Unable to query database for guilds!");
                None
            }
        }
    }
}