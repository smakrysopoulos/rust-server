use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::{models::build_metadata_model::{BuildMetadata, BuildMetadataRequest}, services::db::Database};

#[post("/build_metadata")]
pub async fn create_metadata(db: Data<Database>, request: Json<BuildMetadataRequest>) -> HttpResponse {
    match db
        .create_metadata(
            BuildMetadata::try_from(BuildMetadataRequest {
                version: request.version.clone(),
                branch: request.branch.clone(),
                commit_hash: request.commit_hash.clone(),
                repo: request.repo.clone(),
                image_name: request.image_name.clone()
            })
            .expect("Error creating BuildMetadataRequest to BuildMetadata")
        ).await {
            Ok(build_metadata) => HttpResponse::Ok().json(build_metadata),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
}