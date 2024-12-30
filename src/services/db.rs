use std::env;

use mongodb::{bson::{doc, Document}, error::Error, results::InsertOneResult, Client, Collection};
use crate::models::build_metadata_model::BuildMetadata;
use futures_util::TryStreamExt;




pub struct Database {
    build_metadata: Collection<BuildMetadata>
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI"){
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://admin:password@localhost:27017".to_string() //?directConnection=true
        };
        
        println!("{}", uri);
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("build_metadata");
        
        let build_metadata = db.collection("build_metadata");

        Database{
            build_metadata,
        }
    }

    pub async fn create_metadata(&self, metadata: BuildMetadata) -> Result<InsertOneResult, Error>{
        
       let result = self
           .build_metadata
           .insert_one(metadata)
           .await
           .ok()
           .expect("Error creating metadata");
        Ok(result)
    }

    pub async fn get_metadata(
        &self,
        version: &str,
        branch: &str,
        image_name: &str,
    ) -> Result<Option<BuildMetadata>, Error> {
        let filter = doc! {
            "version": version,
            "branch": branch,
            "image_name": image_name,
        };
        self
            .build_metadata
            .find_one(filter)
            .await
    }

    pub async fn get_metadata_list(
        &self,
        filter: Document,
    ) -> Result<Vec<BuildMetadata>, mongodb::error::Error> {
        let mut cursor = self.build_metadata.find(filter).await?;
        let mut results = Vec::new();
        while let Some(document) = cursor.try_next().await? {
            results.push(document);
        }
        Ok(results)
    }
}
