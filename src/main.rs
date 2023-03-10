#[macro_use]
extern crate rocket;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use rocket::serde::{json::Json, Serialize};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct File {
    name: String,
    dir: bool,
    size: u64,
}

impl From<walkdir::DirEntry> for File {
    fn from(entry: walkdir::DirEntry) -> Self {
        let name = entry.file_name().to_string_lossy().to_string();
        let dir = entry.file_type().is_dir();
        let size = entry.metadata().map(|m| m.len()).unwrap_or_default();
        Self { name, dir, size }
    }
}

#[get("/content/<path..>")]
fn content(path: PathBuf) -> Json<Vec<File>> {
    let r = walkdir::WalkDir::new(Path::new("/media/simon").join(path))
        .max_depth(1).min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into())
        .collect::<Vec<_>>();
    Json(r)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, content])
}
