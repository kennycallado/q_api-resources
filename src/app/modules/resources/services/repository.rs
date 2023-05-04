use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::resources;

use crate::app::modules::resources::model::{NewResource, Resource};

pub async fn get_all(db: &Db) -> Result<Vec<Resource>, diesel::result::Error> {
    let resources = db
        .run(move |conn| resources::table.load::<Resource>(conn))
        .await;

    resources
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Resource, diesel::result::Error> {
    let resource = db
        .run(move |conn| resources::table.find(id).first::<Resource>(conn))
        .await;

    resource
}

pub async fn create(
    db: &Db,
    new_resource: NewResource,
) -> Result<Resource, diesel::result::Error> {
    let resource = db
        .run(move |conn| {
            diesel::insert_into(resources::table)
                .values(&new_resource)
                .get_result::<Resource>(conn)
        })
        .await;

    resource
}

pub async fn update(
    db: &Db,
    id: i32,
    new_resource: NewResource,
) -> Result<Resource, diesel::result::Error> {
    let resource = db
        .run(move |conn| {
            diesel::update(resources::table.find(id))
                .set(&new_resource)
                .get_result::<Resource>(conn)
        })
        .await;

    resource
}
