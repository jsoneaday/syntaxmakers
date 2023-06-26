use crate::common::repository::base::Repository;

pub struct AppState<T: Repository> {
    pub repo: T
}