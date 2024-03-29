use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::resources::model::Resource;
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn get_index_admin(
    db: &Db,
    _admin: UserInClaims,
) -> Result<Json<Vec<Resource>>, Status> {
    let resources = resources_repository::get_all(db).await;

    match resources {
        Ok(slides) => Ok(Json(slides)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}
