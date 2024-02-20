use std::path::{Path, PathBuf};

use rocket::{fs::relative, fs::NamedFile, get};

#[get("/static/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("./src/features/shared/static/")).join(file))
        .await
        .ok()
}
