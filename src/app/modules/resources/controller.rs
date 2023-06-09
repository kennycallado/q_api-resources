use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::resources::handlers::{ create, index, show, update };

use crate::app::modules::resources::model::{ NewResource, Resource, ResourceComplete, NewResourceWithNewContent, ResourceWithContent };

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/<_..>")]
pub async fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Resource>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(&db, claims.0.user).await,
        _ => Err(Status::Unauthorized),
    }
}

#[get("/", rank = 2)]
pub fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(fetch: &State<Fetch> , db: Db, claims: AccessClaims, id: i32) -> Result<Json<ResourceComplete>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(fetch, &db, claims.0.user, id).await,
        "robot" => show::get_show_admin(fetch, &db, claims.0.user, id).await,
        _ => Err(Status::Unauthorized),
    }
}

#[get("/<_id>", rank = 102)]
pub fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_resource>", rank = 1)]
pub async fn post_create(db: Db, claims: AccessClaims, new_resource: Json<NewResourceWithNewContent>) -> Result<Json<ResourceWithContent>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(&db, claims.0.user, new_resource.into_inner()).await,
        _ => Err(Status::Unauthorized),
    }
}

#[post("/", data = "<_new_resource>", rank = 2)]
pub async fn post_create_none(_new_resource: Json<NewResource>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_resource>", rank = 101)]
pub async fn put_update(db: Db, claims: AccessClaims, id: i32, new_resource: Json<NewResourceWithNewContent>) -> Result<Json<ResourceWithContent>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(&db, claims.0.user, id, new_resource.into_inner()).await,
        _ => Err(Status::Unauthorized),
    }
}

#[put("/<_id>", data = "<_new_resource>", rank = 102)]
pub async fn put_update_none(_id: i32, _new_resource: Json<NewResource>) -> Status {
    Status::Unauthorized
}
