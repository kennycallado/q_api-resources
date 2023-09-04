use serde::{Deserialize, Serialize};

use crate::database::schema::resources;

use crate::app::providers::models::question::PubQuestion;
use crate::app::providers::models::slide::PubSlide;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    pub id: i32,
    pub resource_type: String,
    pub sort_order: Vec<Option<i32>>,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = resources)]
#[serde(crate = "rocket::serde")]
pub struct NewResource {
    pub resource_type: String,
    pub sort_order: Option<Vec<Option<i32>>>,
    pub title: String,
    pub description: String,
}

impl From<Resource> for NewResource {
    fn from(resource: Resource) -> Self {
        NewResource {
            resource_type: resource.resource_type,
            sort_order: Some(resource.sort_order),
            title: resource.title,
            description: resource.description,
        }
    }
}

impl From<NewResourceWithNewContent> for NewResource {
    fn from(resource: NewResourceWithNewContent) -> Self {
        NewResource {
            resource_type: resource.resource_type,
            sort_order: resource.sort_order,
            title: resource.title,
            description: resource.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewResourceWithNewContent {
    pub resource_type: String,
    pub sort_order: Option<Vec<Option<i32>>>,
    pub title: String,
    pub description: String,
    pub content: Option<NewContent>,
}

impl From<Resource> for NewResourceWithNewContent {
    fn from(resource: Resource) -> Self {
        NewResourceWithNewContent {
            resource_type: resource.resource_type,
            sort_order: Some(resource.sort_order),
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
            sort_order: resource.sort_order,
            title: resource.title,
            description: resource.description,
            content: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceComplete {
    pub id: i32,
    pub resource_type: String,
    pub sort_order: Vec<Option<i32>>,
    pub title: String,
    pub description: String,
    pub content: Option<ContentComplete>,
}

impl From<Resource> for ResourceComplete {
    fn from(resource: Resource) -> Self {
        ResourceComplete {
            id: resource.id,
            resource_type: resource.resource_type,
            sort_order: resource.sort_order,
            title: resource.title,
            description: resource.description,
            content: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceWithContent {
    pub id: i32,
    pub resource_type: String,
    pub sort_order: Vec<Option<i32>>,
    pub title: String,
    pub description: String,
    pub content: Option<Content>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Content {
    pub slides: Option<Vec<i32>>,
    pub form: Option<Vec<i32>>,
    pub module: Option<Vec<i32>>,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            slides: None,
            form: None,
            module: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewContent {
    pub slides: Option<Vec<i32>>,
    pub form: Option<Vec<i32>>,
    pub module: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ContentComplete {
    pub slides: Option<Vec<PubSlide>>,
    pub form: Option<Vec<PubQuestion>>,
    pub module: Option<Vec<i32>>,
}
