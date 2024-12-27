use actix_web::{get, post, web::{Data, Json, Query}, HttpResponse};

use crate::{models::build_metadata_model::{BuildMetadata, BuildMetadataRequest, BuildMetadataQuery}, services::db::Database};

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

#[get("/build_metadata")]
pub async fn get_metadata(
    db: Data<Database>, 
    query: Query<BuildMetadataQuery>
) -> HttpResponse {
    let version = query.version.clone();
    let branch = query.branch.clone();
    let image_name = query.image_name.clone();
    println!("{} {} {}", version, branch, image_name);
    match db.get_metadata(&version, &branch, &image_name).await {
        Ok(Some(build_metadata)) => HttpResponse::Ok().json(build_metadata),
        Ok(None) => HttpResponse::NotFound().body("Metadata not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}