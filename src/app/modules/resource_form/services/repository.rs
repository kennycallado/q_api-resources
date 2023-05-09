use diesel::prelude::*;

use rocket::http::Status;

use crate::config::database::Db;
use crate::database::schema::resource_form;

use crate::app::providers::interfaces::helpers::claims::{Claims, UserInClaims};
use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;

use crate::app::providers::interfaces::question::PubQuestion;

use crate::app::modules::resource_form::model::{NewResourceForm, ResourceForm};

pub async fn get_question_ids_by_resource_id(db: &Db, id: i32) -> Result<Vec<i32>, diesel::result::Error> {
    let question_ids = db
        .run(move |conn| resource_form::table
            .select(resource_form::question_id)
            .filter(resource_form::resource_id.eq(id))
            .load::<i32>(conn)
        ).await;

    question_ids
}

pub async fn get_multiple_questions(ids: Vec<i32>) -> Result<Vec<PubQuestion>, Status> {
    // Prepare token for robot
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    // Prepare url for request
    let question_url = ConfigGetter::get_entity_url("question").unwrap_or("http://localhost:8011/api/v1/question".to_string()) + "/multiple";

    // Request questions
    let client = reqwest::Client::new();
    let res = client
        .post(&question_url)
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

            Ok(res.json::<Vec<PubQuestion>>().await.unwrap())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

pub async fn add_questions(db: &Db, resource_id: i32, question_ids: Vec<i32>) -> Result<usize, diesel::result::Error> {
    let mut new_questions: Vec<NewResourceForm> = Vec::new();

    for question_id in question_ids {
        new_questions.push(NewResourceForm {
            resource_id,
            question_id,
        });
    }

    let inserted_questions = db
        .run(move |conn| diesel::insert_into(resource_form::table)
            .values(new_questions)
            .execute(conn)
        ).await;

    inserted_questions
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
