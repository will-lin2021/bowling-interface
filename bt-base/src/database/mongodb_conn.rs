use crate::{
    base::{Date, Game, Games},
    util::error::Error,
};

use mongodb::{
    bson::{doc, to_bson},
    sync::{Client, Collection, Database},
};

pub struct DatabaseConn {
    client: Client,
    database: Option<Database>,
    collection: Option<Collection<Games>>,
}

impl DatabaseConn {
    pub fn connect(user: &str, pass: &str, host: &str, port: &str, auth_db: &str) -> Self {
        let client =
            Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
                .expect("Unable to connect to MongoDB");

        if let Err(err) = client.list_database_names(None, None) {
            panic!("{}", err.to_string());
        };

        DatabaseConn {
            client,
            database: None,
            collection: None,
        }
    }

    pub fn connect_db(
        user: &str,
        pass: &str,
        host: &str,
        port: &str,
        auth_db: &str,
        db: &str,
    ) -> Self {
        let client =
            Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
                .expect("Unable to connect to MongoDB");

        let database = client.database(db);
        if let Err(err) = database.list_collection_names(None) {
            panic!("{}", err.to_string());
        };

        DatabaseConn {
            client,
            database: Some(database),
            collection: None,
        }
    }

    pub fn connect_full(
        user: &str,
        pass: &str,
        host: &str,
        port: &str,
        auth_db: &str,
        db: &str,
        coll: &str,
    ) -> Self {
        let client =
            Client::with_uri_str(format!("mongodb://{user}:{pass}@{host}:{port}/{auth_db}"))
                .expect("Unable to connect to MongoDB");

        let database = client.database(db);
        if let Err(err) = database.list_collection_names(None) {
            panic!("{}", err.to_string());
        };

        let collection = database.collection(coll);
        if let Err(err) = collection.list_index_names() {
            panic!("{}", err.to_string());
        };

        DatabaseConn {
            client,
            database: Some(database),
            collection: Some(collection),
        }
    }

    pub fn set_database(&mut self, name: &str) -> Result<(), Error> {
        let database = self.client.database(name);
        // Check that the user has access to the database being set
        if database.list_collection_names(None).is_err() {
            return Err(Error::DBConnError);
        };

        // Check that the database being set is not the same
        if let Some(db) = &self.database {
            if database.name() == db.name() {
                return Err(Error::DBConnError);
            }
        };

        self.database = Some(self.client.database(name));
        self.collection = None;

        Ok(())
    }

    pub fn unset_database(&mut self) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        self.database = None;
        self.collection = None;

        Ok(())
    }

    pub fn set_collection(&mut self, name: &str) -> Result<(), Error> {
        let collection = match &self.database {
            None => return Err(Error::DBConnError),
            Some(database) => database.collection::<Games>(name),
        };

        if collection.list_index_names().is_err() {
            return Err(Error::DBConnError);
        };

        self.collection = Some(collection);

        Ok(())
    }

    pub fn unset_collection(&mut self) -> Result<(), Error> {
        if self.collection.is_none() {
            return Err(Error::DBConnError);
        };

        self.collection = None;

        Ok(())
    }

    pub fn add_game(&self, date: Date, game: &Game) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        // Check for existing games
        match coll.find_one(filter, None)? {
            None => {
                coll.insert_one(Games::build_with(date, game.clone()), None)?;
            }
            Some(_) => {
                coll.update_one(
                    doc! {
                        "_id": to_bson(&date).unwrap()
                    },
                    doc! {
                        "$push": doc! {
                            "games": to_bson(game).unwrap()
                        }
                    },
                    None,
                )?;
            }
        };

        Ok(())
    }

    pub fn add_games(&self, games: &Games) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&games.date()).unwrap(),
        };

        // Check for existing games
        let result = coll.find_one(filter, None)?;

        match result {
            Some(_) => return Err(Error::DBConnError),
            None => {
                coll.insert_one(games, None)?;
            }
        };

        Ok(())
    }

    pub fn get_game(&self, date: Date, game_num: u8) -> Result<Option<Game>, Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        let result = coll.find_one(filter, None)?;

        let games = match result {
            None => return Ok(None),
            Some(games) => games,
        };

        let game = games
            .games()
            .iter()
            .find(|g| g.game_num() == game_num)
            .cloned();

        Ok(game)
    }

    pub fn get_games(&self, date: Date) -> Result<Option<Games>, Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        Ok(coll.find_one(filter, None)?)
    }

    pub fn num_games(&self, date: Date) -> Result<u8, Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        let result = coll.find_one(filter, None)?;

        Ok(match result {
            None => 0,
            Some(games) => games.games().len() as u8,
        })
    }

    pub fn modify_game(&self, date: Date, game: &Game) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let query = doc! {
            "_id": to_bson(&date).unwrap(),
            "games.game_num": to_bson(&game.game_num()).unwrap(),
        };

        let update = doc! {
            "$set": doc! {
                "games.$.frames": to_bson(&game.frames()).unwrap(),
            },
        };

        if coll.update_one(query, update, None)?.matched_count == 0 {
            return Err(Error::DBConnError);
        };

        Ok(())
    }

    pub fn modify_games(&self, games: &Games) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&games.date()).unwrap(),
        };

        if coll
            .find_one_and_replace(filter, games.clone(), None)?
            .is_none()
        {
            return Err(Error::DBConnError);
        }

        Ok(())
    }

    pub fn remove_game(&self, date: Date, game_num: u8) -> Result<Option<Game>, Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        if game_num == 0 || game_num > self.num_games(date)? {
            return Ok(None);
        };

        let game = self.get_game(date, game_num)?.unwrap();

        let query = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        let update = doc! {
            "$pull": doc! {
                "games": doc! {
                    "game_num": doc! {
                        "$eq": to_bson(&game_num).unwrap(),
                    },
                },
            },
        };

        if coll.update_one(query, update, None)?.matched_count == 0 {
            return Err(Error::DBConnError);
        }

        Ok(Some(game))
    }

    pub fn remove_games(&self, date: Date) -> Result<Option<Games>, Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        let filter = doc! {
            "_id": to_bson(&date).unwrap(),
        };

        Ok(coll.find_one_and_delete(filter, None)?)
    }

    pub fn drop_all(&self) -> Result<(), Error> {
        if self.database.is_none() {
            return Err(Error::DBConnError);
        };

        let coll = match &self.collection {
            None => return Err(Error::DBConnError),
            Some(coll) => coll,
        };

        coll.delete_many(doc! {}, None)?;

        Ok(())
    }
}
