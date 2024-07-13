use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions},
    Client, Collection, IndexModel,
};

const DB_NAME: &str = "8asscrap";
const ASCENTS_COLLECTION_NAME: &str = "ascents";

pub struct Mongo {
    ascents_col: Collection<serde_json::Value>,
}

// TODO: For now only sport climbing. Will have to make a separate collection for bouldering
impl Mongo {
    pub async fn new(connection_str: &str) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(connection_str).await?;
        let client = Client::with_options(client_options)?;

        // create indexes right away
        let db = client.database(DB_NAME);
        let collection = db.collection::<serde_json::Value>(ASCENTS_COLLECTION_NAME);
        collection
            .create_index(
                IndexModel::builder().keys(doc! { "userSlug": 1 }).build(),
                None,
            )
            .await?;
        collection
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "countryName": 1 })
                    .build(),
                None,
            )
            .await?;
        collection
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "userSlug": 1, "date": -1 })
                    .build(),
                None,
            )
            .await?;

        Ok(Mongo {
            ascents_col: collection,
        })
    }

    pub async fn user_replace_ascents(
        &self,
        user: &str,
        ascents: Vec<serde_json::Value>,
    ) -> Result<(), mongodb::error::Error> {
        self.ascents_col
            .delete_many(doc! { "userSlug": user }, None)
            .await?;
        self.ascents_col.insert_many(ascents, None).await?;
        Ok(())
    }

    pub async fn user_get_ascents(
        &self,
        user: &str,
    ) -> Result<Vec<serde_json::Value>, mongodb::error::Error> {
        self.ascents_col
            .find(
                doc! { "userSlug": user },
                FindOptions::builder().sort(doc! { "date": -1 }).build(),
            )
            .await?
            .try_collect()
            .await
    }

    pub async fn user_peek_ascents(
        &self,
        user: &str,
        count: u32,
    ) -> Result<Vec<serde_json::Value>, mongodb::error::Error> {
        self.ascents_col
            .find(
                doc! { "userSlug": user },
                FindOptions::builder()
                    // TODO: Turns out date for the same day is the same. Have to figure out what do they sort by in 2nd order?
                    .sort(doc! { "date": -1 })
                    .limit(Some(i64::from(count)))
                    .build(),
            )
            .await?
            .try_collect()
            .await
    }
}
