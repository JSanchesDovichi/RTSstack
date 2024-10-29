use rocket::{form::Form, fs::FileServer, get, launch, post, routes, FromForm};
use rocket_dyn_templates::{Template, context};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { firstname: "hello", lastname: "world!" })
}

#[derive(FromForm)]
struct Task<'criacao_conta> {
    name: &'criacao_conta str,
    key2: &'criacao_conta str,
    another: &'criacao_conta str,
}

#[post("/post_test", data = "<task>")]
fn conta_nova<'criacao_conta>(task: Form<Task<'criacao_conta>>) -> &'criacao_conta str {
    "Conta criada com sucesso!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![hello, conta_nova])
    .mount("/index", routes![index])
    .mount("/", FileServer::new("static/", rocket::fs::Options::None))
    .attach(Template::fairing())
}
