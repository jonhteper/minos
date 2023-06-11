use super::file::File;

pub struct Container {
    id: String,
    description: String,
    files: Vec<File>,
}
