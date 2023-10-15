#![allow(unused_variables)]
#![allow(dead_code)]

// thiserror...
use thiserror::Error;

// crate...
use crate::form::{Object, Record};

pub type Success = ();

#[derive(Debug, Error)]
pub enum Failure {
    #[error("")]
    Server,

    #[error("")]
    Database,

    #[error("")]
    OpenAI,
}

pub type Result = std::result::Result<Success, Failure>;

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
        record: Record,
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

impl Command {
    pub fn handle(command: Command) -> Result {
        let result = match command {
            //
            // Select
            //
    
            Command::SelectAll { object } => Self::select_all(object),
            Command::SelectOne { object, record } => Self::select_one(object, record),
            Command::SelectIds { object, records } => Self::select_ids(object, records),
            Command::Select404 { object } => Self::select_404(object),
        
            //
            // Search
            //
        
            Command::SearchAny { object } => Self::search_any(object),
            Command::SearchOne { object } => Self::search_one(object),
            Command::Search404 { object } => Self::search_404(object),
            Command::SearchNot { object } => Self::search_not(object),
        
            //
            // Create
            //
        
            Command::CreateAll { object } => Self::create_all(object),
            Command::CreateOne { object, record } => Self::create_one(object, record),
            
            //
            // Update
            //
        
            Command::UpdateAll { object } => Self::update_all(object),
            Command::UpdateOne { object } => Self::update_one(object),
        
            //
            // Upsert
            //
        
            Command::UpsertAll { object } => Self::upsert_all(object),
            Command::UpsertOne { object } => Self::upsert_one(object),
        
            //
            // Expire
            //
        
            Command::ExpireAll { object } => Self::expire_all(object),
            Command::ExpireOne { object } => Self::expire_one(object),
        
            //
            // Delete
            //
        
            Command::DeleteAll { object } => Self::delete_all(object),
            Command::DeleteOne { object } => Self::delete_one(object),
        };
    
        return result;
    }

    fn select_all(object: Object) -> Result {
        println!("{:?}", object);

        // Implementation goes here
        return Ok(());
    }

    fn select_one(object: Object, record: Record) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn select_ids(object: Object, records: Vec<Record>) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn select_404(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn search_any(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn search_one(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn search_404(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn search_not(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn create_all(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn create_one(object: Object, record: Record) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn update_all(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn update_one(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn upsert_all(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn upsert_one(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn expire_all(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn expire_one(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn delete_all(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }

    fn delete_one(object: Object) -> Result {
        // Implementation goes here
        return Ok(());
    }
}