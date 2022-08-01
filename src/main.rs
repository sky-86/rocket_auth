#[macro_use]
extern crate rocket;

mod db_helper;
mod signup;

//use rocket::form::Form;
use rocket::fairing::AdHoc;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::postgres::PgPool;

use db_helper::Db;

#[get("/")]
async fn index(db: &State<Db>) -> Template {
    let users = db.select_all().await;

    match users {
        Ok(u) => Template::render("index", context! {name: &u[0].username}),
        Err(_e) => Template::render("index", context! {}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Db::connect())
        .attach(Template::fairing())
        .attach(signup::stage())
        .mount("/", routes![index])
}
