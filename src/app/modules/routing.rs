use crate::app::modules::resources::controller::routes as resources_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/resource", resources_routes())
    })
}
