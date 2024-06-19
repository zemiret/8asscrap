use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, IndexModel};

const DB_NAME: &str = "8asscrap";
const ASCENTS_COLLECTION_NAME: &str = "ascents";

pub struct MongoStore {
    client: Client,
}

pub async fn new_mongo(connection_str: &str) -> Result<MongoStore, mongodb::error::Error> {
    let client_options = ClientOptions::parse(connection_str).await?;
    let client = Client::with_options(client_options)?;

    // create indexes right away
    let db = client.database(DB_NAME);
    let collection = db.collection::<serde_json::Value>(ASCENTS_COLLECTION_NAME);
    collection.create_index(IndexModel::builder().keys(doc! { "userSlug": 1 }).build(), None).await?;
    collection.create_index(IndexModel::builder().keys(doc! { "countryName": 1 }).build(), None).await?;
    collection.create_index(IndexModel::builder().keys(doc! { "userSlug": 1, "date": -1 }).build(), None).await?;

    Ok(MongoStore {
        client: client,
    })
}

impl MongoStore {
    pub async fn replace_ascents(&self, user: &str, ascents: Vec<serde_json::Value>) -> Result<(), mongodb::error::Error> {
        let client = self.client.clone();
        let db = client.database(DB_NAME);
        let collection = db.collection::<serde_json::Value>(ASCENTS_COLLECTION_NAME);

        collection.delete_many(doc! { "userSlug": user }, None).await?;
        collection.insert_many(ascents, None).await?;

        Ok(())
    }

    pub async fn get_user_ascents(&self, user: &str) -> Result<Vec<serde_json::Value>, mongodb::error::Error> {
        let client = self.client.clone();
        let db = client.database(DB_NAME);
        let collection = db.collection::<serde_json::Value>(ASCENTS_COLLECTION_NAME);

        collection.find(doc! { "userSlug": user }, None).await?.try_collect().await
    }
}
