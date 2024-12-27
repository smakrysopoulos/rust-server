use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub _id: ObjectId,
    pub version: String,
    pub branch: String,
    pub commit_hash: String,
    pub repo: String,
    pub image_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildMetadataQuery {
    pub version: String,
    pub branch: String,
    pub image_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildMetadataRequest {
    pub version: String,
    pub branch: String,
    pub commit_hash: String,
    pub repo: String,
    pub image_name: String,
}

impl TryFrom<BuildMetadataRequest> for BuildMetadata {
    type Error = Box<dyn std::error::Error>;
    fn try_from(item: BuildMetadataRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            version: item.version,
            branch: item.branch,
            commit_hash: item.commit_hash,
            repo: item.repo,
            image_name: item.image_name
        })
    }
}