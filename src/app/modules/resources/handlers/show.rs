use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::app::providers::models::question::PubQuestion;
use crate::database::connection::Db;

use crate::app::providers::models::slide::PubSlide;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::resource_slides::services::repository as rs_repository;
use crate::app::modules::resource_module::services::repository as rm_repository;
use crate::app::modules::resource_form::services::repository   as rf_repository;

use crate::app::modules::resources::model::{ContentComplete, ResourceComplete};
use crate::app::modules::resources::services::repository as resources_repository;

fn sort_slides(slides: &mut Vec<PubSlide>, ids: Vec<i32>) {
    let mut sorted_slides = Vec::new();

    for id in ids {
        for slide in slides.clone() {
            if slide.id == id {
                sorted_slides.push(slide);
            }
        }
    }

    slides.clear();
    slides.append(&mut sorted_slides);
}

fn sort_form(questions: &mut Vec<PubQuestion>, ids: Vec<i32>) {
    let mut sorted_questions = Vec::new();

    for id in ids {
        for question in questions.clone() {
            if question.id == id {
                sorted_questions.push(question);
            }
        }
    }

    questions.clear();
    questions.append(&mut sorted_questions);
}

pub async fn get_show_admin(fetch: &State<Fetch>, db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ResourceComplete>, Status> {
    let resource = resources_repository::get_by_id(db, id).await;

    let resource = match resource {
        Ok(resource) => resource,
        Err(_) => return Err(Status::NotFound),
    };

    match resource.resource_type.as_str() {
        "slides" => {
            match rs_repository::get_slide_ids_by_resource_id(db, id).await {
                Ok(ids) => {

                    match rs_repository::get_multiple_slides(fetch, ids.clone()).await {
                        Ok(mut slides) => {
                            sort_slides(&mut slides, ids);

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

                    match rm_repository::get_multiple_slides(fetch, ids.clone()).await {
                        Ok(mut slides) => {
                            sort_slides(&mut slides, ids);

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

                    match rf_repository::get_multiple_questions(fetch, ids.clone()).await {
                        Ok(mut questions) => {
                            sort_form(&mut questions, ids);

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
