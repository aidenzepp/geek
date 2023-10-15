#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

// crate...
use crate::form::{Object, Record, Result, Error};


#[derive(Debug, clap::Subcommand)]
pub enum Command {
    //
    // Select
    //

    /// Returns all records from the database object.
    #[command(name = "select-all")]
    SelectAll {
        /// The name of the object to be selected.
        object: Object,
    },
    /// Returns the unique record from the database object.
    #[command(name = "select-one")]
    SelectOne {
        /// The object to be selected from.
        object: Object,

        /// The record to be selected.
        record: Record,
    },
    /// Returns the records from the database object matching the provided IDs.
    #[command(name = "select-ids")]
    SelectIds { 
        /// The object to be selected from.
        object: Object,

        /// The record ids to be selected.
        records: Vec<Record>,
    },
    /// ...
    #[command(name = "select-404")]
    Select404 { 
        /// The object to be selected from.
        object: Object,
    },

    //
    // Search
    //

    /// Returns all records from the database object matching the given criteria.
    #[command(name = "search-any")]
    SearchAny {
        /// The object to be searched from.
        object: Object,
    },
    /// Returns the unique record from the database object matching the given criteria.
    #[command(name = "search-one")]
    SearchOne {
        /// The object to be searched from.
        object: Object,
    },
    /// ...
    #[command(name = "search-404")]
    Search404 {
        /// The object to be searched from.
        object: Object,
    },
    /// ...
    #[command(name = "search-not")]
    SearchNot {
        /// The object to be searched from.
        object: Object,
    },

    //
    // Create
    //

    /// Creates and attempts to input the record into the database object.
    #[command(name = "create-all")]
    CreateAll {
        /// The object to be created into.
        object: Object,
    },
    #[command(name = "create-one")]
    CreateOne {
        /// The object to be created into.
        object: Object,

        /// The record to be created.
        record: Vec<Record>,
    },
    
    //
    // Update
    //

    #[command(name = "update-all")]
    UpdateAll {
        /// The object to be updated.
        object: Object,
    },
    #[command(name = "update-one")]
    UpdateOne {
        /// The object to be updated.
        object: Object,
    },

    //
    // Upsert
    //

    #[command(name = "upsert-all")]
    UpsertAll {
        /// The object to be upserted.
        object: Object,
    },
    #[command(name = "upsert-one")]
    UpsertOne {
        /// The object to be upserted.
        object: Object,
    },

    //
    // Expire
    //

    #[command(name = "expire-all")]
    ExpireAll {
        /// The object to be expired.
        object: Object,
    },
    #[command(name = "expire-one")]
    ExpireOne {
        /// The object to be expired.
        object: Object,
    },

    //
    // Delete
    //

    #[command(name = "delete-all")]
    DeleteAll {
        /// The object to be deleted.
        object: Object,
    },
    #[command(name = "delete-one")]
    DeleteOne {
        /// The object to be deleted.
        object: Object,
    },
}

#[derive(Clone, Debug, clap::Args)]
struct RecordP {
    key: String,
    val: String,
}

impl Command {
    const ADDR: &str = "http://localhost:8080/api/data/";

    pub async fn handle(client: &reqwest::Client, command: Command) -> Result<()> {
        let result = match command {
            //
            // Select
            //
    
            Command::SelectAll { object } => Self::select_all(&client, object).await,
            Command::SelectOne { object, record } => Self::select_one(&client, object, record).await,
            Command::SelectIds { object, records } => Self::select_ids(&client, object, records).await,
            Command::Select404 { object } => Self::select_404(&client, object).await,
        
            //
            // Search
            //
        
            Command::SearchAny { object } => Self::search_any(&client, object).await,
            Command::SearchOne { object } => Self::search_one(&client, object).await,
            Command::Search404 { object } => Self::search_404(&client, object).await,
            Command::SearchNot { object } => Self::search_not(&client, object).await,
        
            //
            // Create
            //
        
            Command::CreateAll { object } => Self::create_all(&client, object).await,
            Command::CreateOne { object, record } => Self::create_one(&client, object, record).await,
            
            //
            // Update
            //
        
            Command::UpdateAll { object } => Self::update_all(&client, object).await,
            Command::UpdateOne { object } => Self::update_one(&client, object).await,
        
            //
            // Upsert
            //
        
            Command::UpsertAll { object } => Self::upsert_all(&client, object).await,
            Command::UpsertOne { object } => Self::upsert_one(&client, object).await,
        
            //
            // Expire
            //
        
            Command::ExpireAll { object } => Self::expire_all(&client, object).await,
            Command::ExpireOne { object } => Self::expire_one(&client, object).await,
        
            //
            // Delete
            //
        
            Command::DeleteAll { object } => Self::delete_all(&client, object).await,
            Command::DeleteOne { object } => Self::delete_one(&client, object).await,
        };
    
        return result;
    }

    async fn select_all(client: &reqwest::Client, object: Object) -> Result<()> {
        let addr: String = Self::ADDR.to_string() + &object;
        let body: String = client.get(addr).send().await?.text().await?;
        let json: String = Self::stylify(body)?;
        
        println!("{}", json);

        return Ok(());
    }

    async fn select_one(client: &reqwest::Client, object: Object, record: Record) -> Result<()> {
        let addr: String = Self::ADDR.to_string() + &object + "/" + &record;
        let body: String = client.get(addr).send().await?.text().await?;
        let json: String = Self::stylify(body)?;

        println!("{}", json);

        // Implementation goes here
        return Ok(());
    }

    async fn select_ids(client: &reqwest::Client, object: Object, records: Vec<Record>) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn select_404(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn search_any(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn search_one(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn search_404(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn search_not(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn create_all(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn create_one(client: &reqwest::Client, object: Object, record: Vec<String>) -> Result<()> {
        let addr: String = Self::ADDR.to_string() + &object;
        let post: String = Self::jsonify(record)?;
        let body: String = client.post(addr).json(&post).send().await?.text().await?;
        let json: String = Self::stylify(body)?;
        
        println!("{}", json);

        return Ok(());
    }

    async fn update_all(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn update_one(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn upsert_all(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn upsert_one(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn expire_all(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn expire_one(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn delete_all(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    async fn delete_one(client: &reqwest::Client, object: Object) -> Result<()> {
        // Implementation goes here
        return Ok(());
    }

    fn hashify(args: Vec<String>) -> Result<HashMap<String, String>> {
        let mut map: HashMap<String, String> = HashMap::new();

        for arg in args {
            let pairs: Vec<&str> = arg.splitn(2, '=').collect();

            // Must be K=V pair.
            if pairs.len() != 2 {
                let message: String = "Each record must be in the format 'key=value'".to_string();

                return Err(Error::Arguments(message));
            }

            map.insert(
                pairs.get(0).unwrap().to_string(), 
                pairs.get(1).unwrap().to_string()
            );
        }

        return Ok(map);
    }
    
    fn jsonify(args: Vec<String>) -> Result<String> {
        let hash: HashMap<String, String> = Self::hashify(args)?;
        let temp: serde_json::Value = serde_json::json!(hash);
        let json: String = serde_json::to_string(&temp)?;

        return Ok(json);
    }

    fn stylify(body: String) -> Result<String> {
        let temp: serde_json::Value = serde_json::from_str(&body)?;
        let json: String = serde_json::to_string_pretty(&temp)?;

        return Ok(json);
    }
}
