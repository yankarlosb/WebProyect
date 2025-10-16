#[macro_use]
extern crate rocket;
use WebProyect::run;

#[launch]
async fn rocket() -> _ {
    run().await
}
