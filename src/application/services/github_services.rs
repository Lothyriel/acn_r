use mongodb::Database;

#[derive(Clone)]
pub struct GithubServices {}

impl GithubServices {
    pub fn new(_database: &Database) -> Self {
        Self {}
    }
}
