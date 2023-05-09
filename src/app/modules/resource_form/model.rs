use serde::{Deserialize, Serialize};

use crate::database::schema::resource_form;

use crate::app::modules::resources::model::Resource;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Resource))]
#[diesel(table_name = resource_form)]
#[serde(crate = "rocket::serde")]
pub struct ResourceForm {
    pub id: i32,
    pub resource_id: i32,
    pub question_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = resource_form)]
#[serde(crate = "rocket::serde")]
pub struct NewResourceForm {
    pub resource_id: i32,
    pub question_id: i32,
}

impl From<ResourceForm> for NewResourceForm {
    fn from(resource_form: ResourceForm) -> Self {
        NewResourceForm {
            resource_id: resource_form.resource_id,
            question_id: resource_form.question_id,
        }
    }
}
