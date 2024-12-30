use actix_web::{get, post, web::{Data, Json, Query}, HttpResponse};
use mongodb::bson::Document;

use crate::{models::build_metadata_model::{BuildMetadata, BuildMetadataRequest, BuildMetadataQuery, BuildMetadataListQuery}, services::db::Database};

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
    match db.get_metadata(&version, &branch, &image_name).await {
        Ok(Some(build_metadata)) => HttpResponse::Ok().json(build_metadata),
        Ok(None) => HttpResponse::NotFound().body("Metadata not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/build_metadata_list")]
pub async fn get_metadata_list(
    db: Data<Database>,
    query: Query<BuildMetadataListQuery>,
) -> HttpResponse {
    let mut filter = Document::new();
    if let Some(version) = &query.version {
        filter.insert("version", version);
    }
    if let Some(branch) = &query.branch {
        filter.insert("branch", branch);
    }
    if let Some(commit_hash) = &query.commit_hash {
        filter.insert("commit_hash", commit_hash);
    }
    if let Some(repo) = &query.repo {
        filter.insert("repo", repo);
    }
    if let Some(image_name) = &query.image_name {
        filter.insert("image_name", image_name);
    }
    if filter.is_empty() {
        return HttpResponse::BadRequest().body("At least one query parameter must be provided.");
    }
    match db.get_metadata_list(filter).await {
        Ok(metadata_list) => HttpResponse::Ok().json(metadata_list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}