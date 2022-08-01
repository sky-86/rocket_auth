use rocket_dyn_templates::{context, Template};
use rocket::form::Form;
use rocket::fairing::AdHoc;

#[derive(FromForm)]
struct CreateInfo {
    username: String,
    email: String,
    password: String,
}

#[get("/signup")]
fn signup_index() -> Template {
    Template::render("signup", context! {})
}

#[post("/signup", data="<info>")]
fn signup_post(info: Form<CreateInfo>) -> Template {
    Template::render("signup", context! {})
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.mount("/", routes![signup_index, signup_post])
    })
}
