use crate::db_helper::Db;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
pub struct CreateInfo {
    #[field(name = "user")]
    pub username: String,
    pub email: String,
    #[field(name = "pass")]
    pub password: String,
    #[field(name = "pass-repeat")]
    pub password_check: String,
}

#[get("/signup")]
fn signup_index() -> Template {
    Template::render("signup", context! {})
}

#[derive(Responder)]
enum FormResponse {
    Redirect(Redirect),
    Template(Template),
}

#[post("/signup", data = "<info>")]
async fn signup_post(db: &State<Db>, info: Form<CreateInfo>) -> FormResponse {
    let username = &info.username;
    let email = &info.email;
    let password = &info.password;
    let password_check = &info.password_check;

    let data = vec![username, email, password, password_check];

    let count = data.iter().filter(|field| field == &&"").count();

    if count > 0 {
        return FormResponse::Template(
            Template::render("signup", context! {
                error: "incorrect field"
            }))
    }
    
    if password != password_check {
        return FormResponse::Template(Template::render(
            "signup",
            context! {
                error: "Password Don't match",
            },
        ));
    }

    let new_user = db.create_user(username, email, password).await;
    match &new_user {
        Ok(_n) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }

    FormResponse::Redirect(Redirect::to("/"))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.mount("/", routes![signup_index, signup_post])
    })
}
