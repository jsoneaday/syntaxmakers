use crate::common::repository::base::Repository;

#[derive(Debug)]
pub struct AppState<T: Repository> {
    pub repo: T
}