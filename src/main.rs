#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "tw"  => utils::twitter::direct_twitter_url(&cmd),
        "gh"  => utils::github::direct_github_url(&cmd),
        "osu" => utils::osu::construct_osu_profile_url(&cmd),
        _     => utils::google::construct_google_search_url(&cmd)
    };
    Redirect::to(redirect_url)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}

