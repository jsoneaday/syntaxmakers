use fake::Fake;
use fake::faker::internet::en::{Username, SafeEmail};
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::developers::models::{NewDeveloper, UpdateDeveloper};
use syntaxmakers_server::common::repository::developers::repo::{InsertDeveloperFn, QueryAllDevelopersFn, QueryDeveloperByEmailFn, QueryDeveloperFn, UpdateDeveloperFn};
use syntaxmakers_server::common_test::fixtures::{ get_fake_email, get_fake_fullname, init_fixtures, LANGUAGES};

#[tokio::test]
async fn test_create_developer_and_get_back() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let primary_lang_id = 1;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }).await.unwrap();
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().primary_lang_id == primary_lang_id);
}

#[tokio::test]
async fn test_create_developer_and_get_back_by_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let primary_lang_id = 1;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }).await.unwrap();
    let get_result = repo.query_developer_by_email(email.clone()).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().primary_lang_id == primary_lang_id);
}

#[tokio::test]
async fn test_create_two_developers_and_get_all() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let create_result1 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        password: "test123".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }).await.unwrap();
    let create_result2 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        password: "test123".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }).await.unwrap();

    let get_all_result = repo.query_all_developers(10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}

#[tokio::test]
async fn test_update_developer_fails_on_old_password() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test123".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }).await.unwrap();

    let update_result = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: get_fake_fullname(), 
        email: get_fake_email(), 
        old_password: "fake_old".to_string(), // no match should fail
        new_password: "test456".to_string(), 
        primary_lang_id: LANGUAGES.get().unwrap()[1].id,
        secondary_lang_id: Some(LANGUAGES.get().unwrap()[2].id)
    }).await;
    assert!(update_result.is_err());
}

#[tokio::test]
async fn test_update_developer_fails_on_new_password() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test123".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }).await.unwrap();

    let update_result = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: get_fake_fullname(), 
        email: get_fake_email(), 
        old_password: old_password.clone(), 
        new_password: "test456".to_string(), // not at least 8 characters should fail
        primary_lang_id: LANGUAGES.get().unwrap()[1].id,
        secondary_lang_id: Some(LANGUAGES.get().unwrap()[2].id)
    }).await;
    assert!(update_result.is_err());
}

#[tokio::test]
async fn test_update_developer_updates_fields() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test123".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    let new_secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);
    let update_result = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        old_password: old_password.clone(), 
        new_password: "test4567".to_string(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: new_secondary_lang_id
    }).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(update_result.is_ok());
    assert!(get_result.clone().full_name == new_full_name);
    assert!(get_result.clone().email == new_email);
    assert!(get_result.clone().primary_lang_id == new_primary_lang_id);
    assert!(get_result.clone().secondary_lang_id == new_secondary_lang_id);
}

#[tokio::test]
async fn test_update_developer_updates_secondary_lang() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test123".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;
    let secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id
    }).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    let new_secondary_lang_id = Some(LANGUAGES.get().unwrap()[4].id);
    _ = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        old_password, 
        new_password: "test4567".to_string(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: new_secondary_lang_id
    }).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    println!("id: {}", get_result.id);
    assert!(get_result.clone().secondary_lang_id.unwrap() == new_secondary_lang_id.unwrap());
}

#[tokio::test]
async fn test_update_developer_succeeds_on_remove_new_secondary_lang() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test123".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;
    let secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id
    }).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    _ = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        old_password, 
        new_password: "test4567".to_string(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: None
    }).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    println!("id: {}", get_result.id);
    assert!(get_result.clone().secondary_lang_id == None);
}