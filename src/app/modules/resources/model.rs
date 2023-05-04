// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::resources;

// use crate::app::modules::slides::model::{NewSlide, Slide, UpdateSlide};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    pub id: i32,
    pub resource_type: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = resources)]
#[serde(crate = "rocket::serde")]
pub struct NewResource {
    pub resource_type: String,
    pub title: String,
    pub description: String,
}

impl From<Resource> for NewResource {
    fn from(resource: Resource) -> Self {
        NewResource {
            resource_type: resource.resource_type,
            title: resource.title,
            description: resource.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceContent {
    pub slides: Option<Vec<i32>>,
    pub form: Option<i32>,
    pub external: Option<i32>,
}

impl Default for ResourceContent {
    fn default() -> Self {
        ResourceContent {
            slides: None,
            form: None,
            external: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewResourceContent {
    pub slides: Option<Vec<i32>>,
    pub form: Option<i32>,
    pub external: Option<i32>,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct UpdateResourceContent {
//     pub slides: Option<Vec<UpdateSlide>>,
//     pub form: Option<String>,
//     pub external: Option<String>,
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct ResourceComplete {
//     pub id: i32,
//     pub resource_type: String,
//     pub title: String,
//     pub description: String,
//     pub content: Option<ResourceContent>,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }

// impl From<Resource> for ResourceComplete {
//     fn from(resource: Resource) -> Self {
//         ResourceComplete {
//             id: resource.id,
//             resource_type: resource.resource_type,
//             title: resource.title,
//             description: resource.description,
//             content: None,
//             created_at: resource.created_at,
//             updated_at: resource.updated_at,
//         }
//     }
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct NewResourceComplete {
//     pub resource_type: String,
//     pub title: String,
//     pub description: String,
//     pub content: Option<NewResourceContent>,
// }

// impl From<ResourceComplete> for NewResourceComplete {
//     fn from(resource: ResourceComplete) -> Self {
//         NewResourceComplete {
//             resource_type: resource.resource_type,
//             title: resource.title,
//             description: resource.description,
//             content: match resource.content {
//                 Some(content) => Some(NewResourceContent {
//                     slides: match content.slides {
//                         Some(slides) => {
//                             Some(slides.into_iter().map(|slide| slide.into()).collect())
//                         }
//                         None => None,
//                     },
//                     form: content.form,
//                     external: content.external,
//                 }),
//                 None => None,
//             },
//         }
//     }
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct UpdateResourceComplete {
//     pub resource_type: String,
//     pub title: String,
//     pub description: String,
//     pub content: Option<UpdateResourceContent>,
// }
