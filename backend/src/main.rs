use std::{path::{PathBuf, Path}, io::{ErrorKind, Error}};

use rocket::{fs::{FileServer, relative, NamedFile}};

use progressly::routes::{projects::*, trackings::*};

#[macro_use] extern crate rocket;

fn try_file(path: PathBuf, base_path: &Path) -> Result<PathBuf, Error> {
    let file_path = base_path.join(path);

    match file_path.exists() && file_path.is_file() {
        true => Ok(file_path),
        false => Err(Error::new(ErrorKind::NotFound, "File does not exist: {}")),
    }
}

#[get("/<file..>")]
async fn index(file: PathBuf) -> Option<NamedFile> {
    let mut base_path = PathBuf::new();
    base_path.push(relative!("static"));

    let named_file = match try_file(file, &*base_path) {
        Ok(path) => NamedFile::open(path).await,
        Err(_) => NamedFile::open(Path::new(relative!("static/index.html"))).await,
    };

    named_file.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_projects, post_project, start_tracking, end_tracking])
        .mount("/", routes![index])
}
