use serde::{Deserialize, Serialize};

use crate::database::schema::resource_module;

use crate::app::modules::resources::model::Resource;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Resource))]
#[diesel(table_name = resource_module)]
#[serde(crate = "rocket::serde")]
pub struct ResourceModule {
    pub id: i32,
    pub resource_id: i32,
    pub slide_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = resource_module)]
#[serde(crate = "rocket::serde")]
pub struct NewResourceModule {
    pub resource_id: i32,
    pub slide_id: i32,
}

impl From<ResourceModule> for NewResourceModule {
    fn from(resource_module: ResourceModule) -> Self {
        NewResourceModule {
            resource_id: resource_module.resource_id,
            slide_id: resource_module.slide_id,
        }
    }
}
