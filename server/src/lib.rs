#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]
use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;

pub mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite().attach(SpaceHelmet::default()).mount(
        "/",
        routes![
            routes::user::user_list,
            routes::user::new_user,
            routes::user::info_user,
            routes::user::update_user,
            routes::user::delete_user
        ],
    )
}
