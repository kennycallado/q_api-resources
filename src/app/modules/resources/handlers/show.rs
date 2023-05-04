use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::resource_slides::services::repository as rs_repository;

use crate::app::modules::resources::model::{ResourceContent, ResourceComplete};
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ResourceComplete>, Status> {
    // get the resource
    // get the resouce type
    // - get the slides
    // - get the form
    // - get the external
    // return the resource with the content

    let resource = resources_repository::get_by_id(db, id).await;
    if let Err(_) = resource {
        return Err(Status::NotFound);
    }
    let resource = resource.unwrap();

    match resource.resource_type.as_str() {
        "slides" => {
            match rs_repository::get_slide_ids_by_resource_id(db, id).await {
                Ok(ids) => {

                    match rs_repository::get_multiple_slides(ids).await {
                        Ok(slides) => {
                            let content = ResourceContent {
                                slides: Some(slides),
                                form: None,
                                external: None,
                            };

                            let mut resource_complete: ResourceComplete = resource.into();
                            resource_complete.content = Some(content);

                            return Ok(Json(resource_complete));
                        },// Algo fué mal ??
                        Err(_) => return Err(Status::InternalServerError),
                    }
                },// Algo fué mal ??
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        "form" => {
            let content = ResourceContent {
                slides: None,
                form: None,
                external: None,
            };

            let mut resource_complete: ResourceComplete = resource.into();
            resource_complete.content = Some(content);

            return Ok(Json(resource_complete));
        },
        "external" => {
            let content = ResourceContent {
                slides: None,
                form: None,
                external: None,
            };

            let mut resource_complete: ResourceComplete = resource.into();
            resource_complete.content = Some(content);

            return Ok(Json(resource_complete));
        },
        _ => {
            println!("Resource type not handled");
            return Err(Status::BadRequest);
        }
    }
}
