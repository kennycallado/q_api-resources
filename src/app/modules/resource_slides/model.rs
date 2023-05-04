use serde::{Deserialize, Serialize};

use crate::database::schema::resource_slides;

use crate::app::modules::resources::model::Resource;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Resource))]
#[diesel(table_name = resource_slides)]
#[serde(crate = "rocket::serde")]
pub struct ResourceSlide {
    pub id: i32,
    pub resource_id: i32,
    pub slide_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = resource_slides)]
#[serde(crate = "rocket::serde")]
pub struct NewResourceSlide {
    pub resource_id: i32,
    pub slide_id: i32,
}

impl From<ResourceSlide> for NewResourceSlide {
    fn from(resource_slides: ResourceSlide) -> Self {
        NewResourceSlide {
            resource_id: resource_slides.resource_id,
            slide_id: resource_slides.slide_id,
        }
    }
}
