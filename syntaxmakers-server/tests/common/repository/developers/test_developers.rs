use fake::Fake;
use fake::faker::internet::en::Username;
use syntaxmakers_server::common::repository::base::{ConnGetter, DbRepo};
use syntaxmakers_server::common::repository::developers::models::NewDeveloper;
use syntaxmakers_server::common::repository::developers::repo::{GetDeveloperFn, GetAllDevelopersFn, CreateDeveloperFn};
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, get_fake_fullname};

#[tokio::test]
async fn test_create_developer_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let primary_lang_id = 1;

    let create_result = repo.create_developer(conn, NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        primary_lang_id
    }).await.unwrap();
    let get_result = repo.get_developer(conn, create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().primary_lang_id == primary_lang_id);
}

#[tokio::test]
async fn test_create_two_developers_and_get_back_both() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();

    let create_result1 = repo.create_developer(conn, NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        primary_lang_id: 1
    }).await.unwrap();
    let create_result2 = repo.create_developer(conn, NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        primary_lang_id: 1
    }).await.unwrap();

    let get_all_result = repo.get_all_developers(conn, 10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}