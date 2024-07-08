use actix_web::web::Data;
use crate::{
    app_state::AppState, common::{
        authentication::auth_keys_service::Authenticator, emailer::emailer::EmailerService, repository::{base::Repository, languages::repo::QueryAllLanguagesFn}
    }, routes::user_error::UserError
};
use super::models::{LanguageResponders, LanguageResponder};

pub async fn get_all_languages<T: QueryAllLanguagesFn + Repository, E: EmailerService, U: Authenticator>(app_data: Data<AppState<T, E, U>>) -> Result<LanguageResponders, UserError> {
    let result = app_data.repo.query_all_languages().await;

    match result {
        Ok(languages) => {
            let responders = languages.iter().map(|lang| {
                LanguageResponder {
                    id: lang.id,
                    updated_at: lang.updated_at,
                    name: lang.name.to_owned()
                }
            })
            .collect::<Vec<LanguageResponder>>();
            Ok(LanguageResponders(responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{authentication::auth_keys_service::AuthenticationError, repository::languages::models::Language}, 
        common_test::fixtures::{get_app_data, MockDbRepo, MockEmailer}
    };
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::internet::en::Username, Fake};
    use jsonwebtoken::DecodingKey;

    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl QueryAllLanguagesFn for MockDbRepo {
        async fn query_all_languages(&self) -> Result<Vec<Language>, sqlx::Error> {
            Ok(vec![
                Language {
                    id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    name: Username().fake::<String>()
                }
            ])
        }
    }

    #[tokio::test]
    async fn test_get_all_languages_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let result = get_all_languages(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}