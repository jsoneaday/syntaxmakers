use fake::Fake;
use fake::faker::internet::en::{Username, SafeEmail};
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::developers::models::NewDeveloper;
use syntaxmakers_server::common::repository::developers::repo::{QueryDeveloperFn, QueryAllDevelopersFn, InsertDeveloperFn};
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, get_fake_fullname};

#[tokio::test]
async fn test_create_developer_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let primary_lang_id = 1;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        primary_lang_id
    }).await.unwrap();
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().primary_lang_id == primary_lang_id);
}

#[tokio::test]
async fn test_create_two_developers_and_get_all() {
    init_fixtures();
    let repo = DbRepo::init().await;
    
    let create_result1 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        primary_lang_id: 1
    }).await.unwrap();
    let create_result2 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        primary_lang_id: 1
    }).await.unwrap();

    let get_all_result = repo.query_all_developers(10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}