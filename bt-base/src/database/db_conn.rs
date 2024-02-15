use crate::prelude::*;

use mongodb::{bson::doc, sync::{Client, Collection, Database}};

pub struct DatabaseConn {
    client: Client,
    database: Option<Database>,
    collection: Option<Collection<Games>>,
}

impl DatabaseConn {
    pub fn connect(user: &str, pass: &str, host: &str, port: &str, auth_db: &str) -> Self {
        let client = Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
            .expect("Unable to connect to MongoDB");

        if let Err(err) = client.list_database_names(None, None) {
            panic!("{}", err.to_string());
        }

        DatabaseConn {
            client,
            database: None,
            collection: None,
        }
    }

    pub fn connect_db(user: &str, pass: &str, host: &str, port: &str, auth_db: &str, db: &str) -> Self {
        let client = Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
            .expect("Unable to connect to MongoDB");

        let database = client.database(db);
        if let Err(err) = database.list_collection_names(None) {
            panic!("{}", err.to_string());
        }

        DatabaseConn {
            client,
            database: Some(database),
            collection: None,
        }
    }

    pub fn connect_full(user: &str, pass: &str, host: &str, port: &str, auth_db: &str, db: &str, coll: &str) -> Self {
        let client = Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
            .expect("Unable to connect to MongoDB");

        let database = client.database(db);
        if let Err(err) = database.list_collection_names(None) {
            panic!("{}", err.to_string());
        }

        let collection = database.collection(coll);
        if let Err(err) = collection.list_index_names() {
            panic!("{}", err.to_string());
        }

        DatabaseConn {
            client,
            database: Some(database),
            collection: Some(collection),
        }
    }

    // TODO: Make functions return Result<(), Error> with a custom error type depending on what the error is
    // Possible errors:
    // Database not set, etc.

    pub fn set_database(&mut self, name: &str) -> bool {
        let database = self.client.database(name);
        // Check that the user has access to the database being set
        if database.list_collection_names(None).is_err() {
            return false
        }

        // Check that the database being set is not the same
        if let Some(db) = &self.database {
            if database.name() == db.name() {
                return false;
            }
        }

        self.database = Some(self.client.database(name));
        self.collection = None;

        true
    }

    pub fn unset_database(&mut self) -> bool {
        // Check that there is a database set
        if self.database.is_none() {
            return false;
        }

        self.database = None;
        self.collection = None;

        true
    }

    pub fn set_collection(&mut self, name: &str) -> bool {
        let collection = match &self.database {
            Some(database) => database.collection::<Games>(name),
            None => return false,
        };
        if collection.list_index_names().is_err() {
            return false
        }

        self.collection = Some(collection);

        true
    }

    pub fn unset_collection(&mut self) -> bool {
        if self.collection.is_none() {
            return false;
        }

        self.collection = None;

        true
    }

    pub fn add_game(&self, item: &Games) -> bool {
        // TODO: Check the case for when the key exists, then append to existing entry

        if self.collection.is_none() {
            return false;
        }

        let coll = self.collection.as_ref().unwrap();


        // TODO: deal with error case and check if it is a WriteError for a duplicate key, then use
        if coll.insert_one(item, None).is_err() {

            let date = item.date();

            if coll.update_one(
                doc! {
                    "_id.year": Into::<i32>::into(date.year()),
                    "_id.month": Into::<i32>::into(date.month()),
                    "_id.day": Into::<i32>::into(date.day()),
                },
                doc! {

                },
                None
            ).is_err() {
                return false
            }
        }

        true
    }

    // pub fn get_game(&self, item: Date) -> Result<Game, ()> {
    //     if self.database.is_none() || self.collection.is_none() {
    //         return Err(())
    //     }

    //     let mut find_result = self.collection.as_ref().unwrap().find(None, None).unwrap();

    //     let game = find_result.next().unwrap().unwrap();

    //     println!("{:?}", game);

    //     Ok(game)
    // }

    // pub fn remove_game(&self, item: Date) -> bool {
    //     false
    // }
}
