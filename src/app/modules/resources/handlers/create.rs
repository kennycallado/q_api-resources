use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::resources::model::{ NewResource, Resource };
use crate::app::modules::resources::services::repository as resources_repository;

pub async fn post_create_admin(db: &Db, _admin: UserInClaims, new_resource: NewResource) -> Result<Json<Resource>, Status> {
    let resource = resources_repository::create(db, new_resource).await;

    match resource {
        Ok(resource) => Ok(Json(resource)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// async fn post_create_admin_old(db: &Db, _admin: UserInClaims, new_resource: NewResource) -> Result<Json<Resource>, Status> {
//     // Determinate kind of resource (slide, form, external)
//     // Create content
//     //  - Create slides
//     //  - Create form
//     //  - Create external
//     // Create resource
//     // Assign slides to resource
//     // Return ResourceComplete

//     let temp_new_resource = NewResource {
//         title: new_resource.title,
//         description: new_resource.description,
//         resource_type: new_resource.resource_type.clone(),
//     };
//     let resource = resources_repository::create(db, temp_new_resource).await;

//     unimplemented!();

//     match resource {
//         Ok(resource) => {
//             // Crea el contenido
//             let content = match new_resource.content {
//                 Some(new_content) => {
//                     create_content(&db, new_content, new_resource.resource_type.as_str()).await
//                 }
//                 None => ResourceContent {
//                     form: None,
//                     slides: None,
//                     external: None,
//                 },
//             };

//             // Asigna los slides al recurso
//             match content.slides.clone() {
//                 Some(slides) => {
//                     for slide in slides {
//                         let new_resource_slide = NewResourceSlides {
//                             resource_id: resource.id,
//                             slide_id: slide.id,
//                         };
//                         match resource_slides_repository::add_slides(db, new_resource_slide)
//                             .await
//                         {
//                             Ok(_) => {}
//                             Err(_) => return Err(Status::InternalServerError),
//                         }
//                     }
//                 }
//                 None => {}
//             }

//             // Crea el recurso completo
//             let resource_complete = ResourceComplete {
//                 id: resource.id,
//                 title: resource.title,
//                 description: resource.description,
//                 resource_type: resource.resource_type,
//                 content: Some(content),
//                 created_at: resource.created_at,
//                 updated_at: resource.updated_at,
//             };

//             // Devuelve el recurso completo
//             Ok(Json(resource_complete))
//         }
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

// TODO: actually save in database
// async fn create_content(
//     db: &Db,
//     new_content: NewResourceContent,
//     resource_type: &str,
// ) -> ResourceContent {
//     match resource_type {
//         "slides" => {
//             let slides = match new_content.slides {
//                 Some(slides) => {
//                     let new_slides =
//                         resource_slides_repository::create_slides(db, slides).await;

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
//                 form: None,
//                 slides,
//                 external: None,
//             }
//         }
//         "form" => match new_content.form {
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
//             form: None,
//             slides: None,
//             external: None,
//         },
//     }
// }