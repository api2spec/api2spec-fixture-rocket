#[macro_use] extern crate rocket;

use api2spec_fixture_rocket::rocket;

#[launch]
fn launch() -> _ {
    rocket()
}
