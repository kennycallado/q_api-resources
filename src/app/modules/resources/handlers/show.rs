use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::resources::model::Resource;
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<Resource>, Status> {
    let resource = resources_repository::get_by_id(db, id).await;

    match resource {
        Ok(resource) => {
            Ok(Json(resource))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
