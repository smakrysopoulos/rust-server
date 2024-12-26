use std::env;

use mongodb::{error::Error, results::InsertOneResult, Client, Collection};

use crate::models::build_metadata_model::BuildMetadata;



pub struct Database {
    build_metadata: Collection<BuildMetadata>
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI"){
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://admin:password@localhost:27017".to_string() //?directConnection=true
        };

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
        //let result = match self.build_metadata.insert_one(metadata).await {
        // Ok(result) => result,
        // Err(e) => {
        //     eprintln!("Error creating metadata: {:?}", e);
        //     // Optionally, return a default value, rethrow, or handle as needed.
        //     return Err(e); // Or handle accordingly
        //     }
        // };
        Ok(result)
    }


}