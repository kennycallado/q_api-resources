use diesel::prelude::*;

use rocket::http::Status;
use rocket::State;

use crate::database::connection::Db;
use crate::database::schema::resource_module;

use crate::app::providers::config_getter::ConfigGetter;
use crate::app::providers::models::slide::PubSlide;
use crate::app::providers::services::claims::{Claims, UserInClaims};
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::resource_module::model::NewResourceModule;

pub async fn get_slide_ids_by_resource_id(db: &Db, id:i32) -> Result<Vec<i32>, diesel::result::Error> {
    let slide_ids = db
        .run(move |conn| resource_module::table
            .select(resource_module::slide_id)
            .filter(resource_module::resource_id.eq(id))
            .load::<i32>(conn))
        .await;

    slide_ids
}

pub async fn get_multiple_slides(fetch: &State<Fetch>, ids: Vec<i32>) -> Result<Vec<PubSlide>, Status> {
    // Prepare token for robot
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    // Prepare url for slide
    let slide_url = ConfigGetter::get_entity_url("slide").unwrap_or("http://localhost:8021/api/v1/slide/".to_string())
        + "multiple";

    // Request slides
    let client = fetch.client.lock().await;
    let res = client
        .post(&slide_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&ids)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<Vec<PubSlide>>().await.unwrap())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

async fn robot_token_generator() -> Result<String, Status> {
    let mut claims: Claims = Claims::from(UserInClaims::default());

    let access_token = claims.enconde_for_robot();
    if let Err(_) = access_token {
        return Err(Status::InternalServerError);
    }

    match access_token {
        Ok(access_token) => Ok(access_token),
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}

pub async fn add_slides(db: &Db, resouce_id: i32, slides: Vec<i32>) -> Result<usize, diesel::result::Error> {
    let mut new_slides: Vec<NewResourceModule> = Vec::new();

    for slide in slides {
        new_slides.push(NewResourceModule {
            resource_id: resouce_id,
            slide_id: slide,
        });
    }

    let inserted_slides = db
        .run(move |conn| {
            diesel::insert_into(resource_module::table)
                .values(&new_slides)
                .execute(conn)
        })
        .await;

    inserted_slides
}
