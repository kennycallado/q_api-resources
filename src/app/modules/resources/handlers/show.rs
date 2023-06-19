use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::resource_slides::services::repository as rs_repository;
use crate::app::modules::resource_module::services::repository as rm_repository;
use crate::app::modules::resource_form::services::repository   as rf_repository;

use crate::app::modules::resources::model::{ContentComplete, ResourceComplete};
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn get_show_admin(fetch: &State<Fetch>, db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ResourceComplete>, Status> {
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

                    match rs_repository::get_multiple_slides(fetch, ids).await {
                        Ok(slides) => {
                            let content = ContentComplete {
                                slides: Some(slides),
                                form: None,
                                module: None,
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
        },
        "module" => {
            match rm_repository::get_slide_ids_by_resource_id(db, id).await {
                Ok(ids) => {

                    match rm_repository::get_multiple_slides(fetch, ids).await {
                        Ok(slides) => {
                            let content = ContentComplete {
                                slides: Some(slides),
                                form: None,
                                module: None,
                            };

                            let mut resource_complete: ResourceComplete = resource.into();
                            resource_complete.content = Some(content);

                            return Ok(Json(resource_complete));
                        },
                        Err(_) => return Err(Status::InternalServerError),
                    }
                },
                Err(_) => return Err(Status::InternalServerError),
            }
        },
        "form" => {
            match rf_repository::get_question_ids_by_resource_id(db, id).await {
                Ok(ids) => {

                    match rf_repository::get_multiple_questions(fetch, ids).await {
                        Ok(questions) => {
                            let content = ContentComplete {
                                slides: None,
                                form: Some(questions),
                                module: None,
                            };

                            let mut resource_complete: ResourceComplete = resource.into();
                            resource_complete.content = Some(content);

                            return Ok(Json(resource_complete));
                        },
                        Err(_) => return Err(Status::InternalServerError),
                    }
                },
                Err(_) => return Err(Status::InternalServerError),
            }
        },
        "external" => {
            let content = ContentComplete {
                slides: None,
                form: None,
                module: None,
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
