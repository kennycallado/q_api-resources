// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{database::schema::resources, app::providers::interfaces::slide::PubSlide};

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

impl From<NewResourceWithNewContent> for NewResource {
    fn from(resource: NewResourceWithNewContent) -> Self {
        NewResource {
            resource_type: resource.resource_type,
            title: resource.title,
            description: resource.description,
        }
    }
}

pub struct NewResourceWithNewContent {
    pub resource_type: String,
    pub title: String,
    pub description: String,
    pub content: Option<NewResourceContent>,
}

impl From<Resource> for NewResourceWithNewContent {
    fn from(resource: Resource) -> Self {
        NewResourceWithNewContent {
            resource_type: resource.resource_type,
            title: resource.title,
            description: resource.description,
            content: None,
        }
    }
}

impl From<NewResource> for NewResourceWithNewContent {
    fn from(resource: NewResource) -> Self {
        NewResourceWithNewContent {
            resource_type: resource.resource_type,
            title: resource.title,
            description: resource.description,
            content: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceComplete {
    pub id: i32,
    pub resource_type: String,
    pub title: String,
    pub description: String,
    pub content: Option<ResourceContentComplete>,
}

impl From<Resource> for ResourceComplete {
    fn from(resource: Resource) -> Self {
        ResourceComplete {
            id: resource.id,
            resource_type: resource.resource_type,
            title: resource.title,
            description: resource.description,
            content: None,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceContentComplete {
    pub slides: Option<Vec<PubSlide>>,
    pub form: Option<i32>,
    pub external: Option<i32>,
}
