use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::resources::model::{ Resource, NewResource };
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn put_update_admin(db: &Db, _admin: UserInClaims, id: i32, new_resource: NewResource) -> Result<Json<Resource>, Status> {
    let resource = resources_repository::update(db, id, new_resource).await;

    match resource {
        Ok(resource) => Ok(Json(resource)),
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
