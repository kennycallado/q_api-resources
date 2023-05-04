use diesel::prelude::*;

use rocket::http::Status;

use crate::config::database::Db;
use crate::database::schema::resource_slides;

use crate::app::providers::interfaces::helpers::claims::{Claims, UserInClaims};
use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::slide::PubSlide;

pub async fn get_slide_ids_by_resource_id(db: &Db,id: i32) -> Result<Vec<i32>, diesel::result::Error> {
    let slide_ids = db
        .run(move |conn| resource_slides::table
            .filter(resource_slides::resource_id.eq(id))
            .select(resource_slides::slide_id)
            .load::<i32>(conn))
        .await;

    slide_ids
}

pub async fn get_multiple_slides(ids: Vec<i32>) -> Result<Vec<PubSlide>, Status> {
    // Prepare token for robot
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    // Prepare url for slide
    let slide_url = ConfigGetter::get_entity_url("slide").unwrap_or("http://localhost:8021/api/v1/slide".to_string())
        + "/multiple";

    // Request slides
    let client = reqwest::Client::new();
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

pub async fn get_slide_by_id(id: i32) -> Result<PubSlide, Status> {
    // Prepare token for robot
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    // Prepare url for slide
    let slide_url = ConfigGetter::get_entity_url("slide").unwrap_or("http://localhost:8021/api/v1/slide".to_string())
        + "/"
        + &id.to_string();

    // Request slide
    let client = reqwest::Client::new();
    let res = client
        .get(&slide_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<PubSlide>().await.unwrap())
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
