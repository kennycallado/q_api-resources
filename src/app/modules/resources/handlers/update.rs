use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::resources::model::{ Resource, NewResource, NewResourceWithNewContent, NewContent, ResourceWithContent, Content };

use crate::app::modules::resources::services::repository as resources_repository;
use crate::app::modules::resource_slides::services::repository as rs_repository;
use crate::app::modules::resource_form::services::repository as rf_repository;
use crate::app::modules::resource_module::services::repository as rm_repository;

pub async fn put_update_admin(db: &Db, _admin: UserInClaims, id: i32, new_resource: NewResourceWithNewContent) -> Result<Json<ResourceWithContent>, Status> {
    let content = match new_resource.content.clone() {
        Some(content) => NewContent {
            slides: content.slides,
            form: content.form,
            module: content.module,
        },
        None => NewContent {
            slides: None,
            form: None,
            module: None,
        },
    };

    let resource = resources_repository::update(db, id, new_resource.into()).await;

    match resource {
        Ok(resource) => {
            if let Some(slides) = &content.slides {
                let _ = rs_repository::update(db, id, slides).await;
            }

            if let Some(form) = &content.form {
                let _ = rf_repository::update(db, id, form).await;
            }

            if let Some(module) = &content.module {
                let _ = rm_repository::update(db, id, module).await;
            }

            let reso_cont = ResourceWithContent {
                id: resource.id,
                title: resource.title,
                description: resource.description,
                resource_type: resource.resource_type,
                content: Some(Content {
                    slides: content.slides,
                    form: content.form,
                    module: content.module,
                }),
            };

            Ok(Json(reso_cont))
        },
        Err(_) => Err(Status::InternalServerError),
    }

    // match resource {
    //     Ok(resource) => {
    //         // Actualiza contenido
    //         let content = match new_resource.content {
    //             Some(content) => update_content(db, content, &new_resource.resource_type).await,
    //             None => ResourceContent {
    //                 slides: None,
    //                 form: None,
    //                 external: None,
    //             },
    //         };

    //         let resource: ResourceComplete = ResourceComplete {
    //             id: resource.id,
    //             title: resource.title,
    //             description: resource.description,
    //             resource_type: resource.resource_type,
    //             content: Some(content),
    //         };
    //         Ok(Json(resource))
    //     }
    //     Err(_) => Err(Status::InternalServerError),
    // }
}

// pub async fn put_update_admin(
//     db: &Db,
//     _admin: UserInClaims,
//     id: i32,
//     new_resource: UpdateResourceComplete,
// ) -> Result<Json<ResourceComplete>, Status> {
//     let temp_new_resource = NewResource {
//         title: new_resource.title,
//         description: new_resource.description,
//         resource_type: new_resource.resource_type.clone(),
//     };
//     let resource = resources_repository::update(db, id, temp_new_resource).await;

//     match resource {
//         Ok(resource) => {
//             // Actualiza contenido
//             let content = match new_resource.content {
//                 Some(content) => update_content(db, content, &new_resource.resource_type).await,
//                 None => ResourceContent {
//                     slides: None,
//                     form: None,
//                     external: None,
//                 },
//             };

//             let resource: ResourceComplete = ResourceComplete {
//                 id: resource.id,
//                 title: resource.title,
//                 description: resource.description,
//                 resource_type: resource.resource_type,
//                 content: Some(content),
//                 created_at: resource.created_at,
//                 updated_at: resource.updated_at,
//             };
//             Ok(Json(resource))
//         }
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

// async fn update_content(
//     db: &Db,
//     content: UpdateResourceContent,
//     resource_type: &str,
// ) -> ResourceContent {
//     match resource_type {
//         "slides" => {
//             let slides = match content.slides {
//                 Some(slides) => {
//                     let new_slides =
//                         resource_slides_repository::update_slides(db, slides).await;

//                     if let Err(_) = new_slides {
//                         return ResourceContent {
//                             form: None,
//                             slides: None,
//                             external: None,
//                         };
//                     }
//                     let new_slides = new_slides.unwrap();

//                     Some(new_slides)
//                 }
//                 None => None,
//             };
//             ResourceContent {
//                 slides,
//                 form: None,
//                 external: None,
//             }
//         }
//         "form" => match content.form {
//             Some(form) => ResourceContent {
//                 form: Some(String::from(form)),
//                 slides: None,
//                 external: None,
//             },
//             None => ResourceContent {
//                 form: Some(String::new()),
//                 slides: None,
//                 external: None,
//             },
//         },
//         // "external" => {},
//         _ => ResourceContent {
//             slides: None,
//             form: None,
//             external: None,
//         },
//     }
// }
