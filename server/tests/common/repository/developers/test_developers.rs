use fake::Fake;
use fake::faker::internet::en::{Username, SafeEmail};
use syntaxmakers_server::common::emailer::emailer::Emailer;
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::developers::models::{NewDeveloper, UpdateDeveloper};
use syntaxmakers_server::common::repository::developers::repo::{
    InsertDeveloperFn, ConfirmEmailFn, QueryAllDevelopersFn, QueryDeveloperByEmailFn, QueryDeveloperFn, UpdateDeveloperFn, QueryLatestValidEmailConfirmFn
};
use syntaxmakers_server::common::repository::user::models::{ChangePassword, DeveloperOrEmployer};
use syntaxmakers_server::common::repository::user::repo::ChangePasswordFn;
use syntaxmakers_server::common_test::fixtures::{ get_fake_dev_desc, get_fake_email, get_fake_fullname, get_fake_user_name, init_fixtures, MockEmailer, LANGUAGES};


#[tokio::test]
async fn test_create_developer_and_get_back() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let primary_lang_id = 1;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().primary_lang_id == primary_lang_id);
}

#[tokio::test]
async fn test_create_developers_and_check_does_not_allow_duplicate_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let primary_lang_id = 1;

    _ = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();
    let create_result = repo.insert_developer(NewDeveloper {
        user_name: get_fake_user_name(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await;
    
    assert!(create_result.is_err());
    assert!(create_result.err().unwrap().as_database_error().unwrap().is_unique_violation());
}

#[tokio::test]
async fn test_create_developers_and_check_does_not_allow_duplicate_user_name() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let primary_lang_id = 1;

    _ = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();
    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name,
        full_name: full_name,
        email: get_fake_email(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await;
    
    assert!(create_result.is_err());
    assert!(create_result.err().unwrap().as_database_error().unwrap().is_unique_violation());
}

#[tokio::test]
async fn test_create_developer_and_get_back_by_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let primary_lang_id = 1;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();
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
    let emailer = MockEmailer;
    
    let create_result1 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }, &emailer).await.unwrap();
    let create_result2 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let get_all_result = repo.query_all_developers(10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}

#[tokio::test]
async fn test_change_dev_password_fails_on_invalid_old_password() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let update_result = repo.change_password(ChangePassword { 
        id: create_result.id, 
        old_password: "fake_old".to_string(),
        new_password: "test4567".to_string(),
        dev_or_emp: DeveloperOrEmployer::Developer
    }).await;
    assert!(update_result.is_err());
}

#[tokio::test]
async fn test_change_dev_password_fails_on_invalid_new_password() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = Emailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let update_result = repo.change_password(ChangePassword { 
        id: create_result.id, 
        old_password: old_password.clone(), 
        new_password: "test456".to_string(),
        dev_or_emp: DeveloperOrEmployer::Developer
    }).await;
    assert!(update_result.is_err());
}

#[tokio::test]
async fn test_change_dev_password_succeeds_on_new_password() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let update_result = repo.change_password(ChangePassword { 
        id: create_result.id, 
        old_password: old_password.clone(), 
        new_password: "test4567".to_string(),
        dev_or_emp: DeveloperOrEmployer::Developer
    }).await;
    assert!(update_result.is_ok());
}

#[tokio::test]
async fn test_update_developer_updates_fields() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let old_email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: old_email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    let new_secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);
    let update_result = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        description: get_fake_dev_desc(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: new_secondary_lang_id
    }, &emailer).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(update_result.is_ok());
    assert!(get_result.clone().full_name == new_full_name);
    // updating user does not immediately update email, as it requires confirmation
    assert!(get_result.clone().email == old_email); 
    assert!(get_result.clone().primary_lang_id == new_primary_lang_id);
    assert!(get_result.clone().secondary_lang_id == new_secondary_lang_id);
}

#[tokio::test]
async fn test_update_developer_updates_secondary_lang() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;
    let secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id
    }, &emailer).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    let new_secondary_lang_id = Some(LANGUAGES.get().unwrap()[4].id);
    _ = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        description: get_fake_dev_desc(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: new_secondary_lang_id
    }, &emailer).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().secondary_lang_id.unwrap() == new_secondary_lang_id.unwrap());
}

#[tokio::test]
async fn test_update_developer_succeeds_on_remove_new_secondary_lang() {
    let repo = DbRepo::init().await;
    let emailer = MockEmailer;
    init_fixtures().await;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let old_password = "test1234".to_string();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;
    let secondary_lang_id = Some(LANGUAGES.get().unwrap()[3].id);

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: old_password.clone(),
        primary_lang_id,
        secondary_lang_id
    }, &emailer).await.unwrap();

    let new_full_name = get_fake_fullname();
    let new_email = get_fake_email();
    let new_primary_lang_id = LANGUAGES.get().unwrap()[2].id;
    _ = repo.update_developer(UpdateDeveloper { 
        id: create_result.id, 
        full_name: new_full_name.clone(), 
        email: new_email.clone(), 
        description: get_fake_dev_desc(),
        primary_lang_id: new_primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await;    
    let get_result = repo.query_developer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().secondary_lang_id == None);
}

#[tokio::test]
async fn test_insert_dev_and_confirm_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    let email = get_fake_email();
    
    // insert_developer should create a new email confirm
    let create_result1 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let email_confirm = repo.query_latest_valid_email_confirm(create_result1.id).await.unwrap().unwrap();

    match repo.confirm_email(email, create_result1.id, email_confirm.unique_key.to_string()).await {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }
}

#[tokio::test]
async fn test_update_dev_email_and_confirm_it() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    let old_email = get_fake_email();
    
    let create_result1 = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: old_email.clone(),
        description: get_fake_dev_desc(),
        password: "test1234".to_string(),
        primary_lang_id: 1,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let new_email = get_fake_email();
    _ = repo.update_developer(UpdateDeveloper {
        id: create_result1.id,
        full_name: get_fake_fullname(),
        email: new_email.clone(),
        description: get_fake_dev_desc(),
        primary_lang_id: 2,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let email_confirm = repo.query_latest_valid_email_confirm(create_result1.id).await.unwrap().unwrap();
    
    match repo.confirm_email(new_email.clone(), create_result1.id, email_confirm.unique_key.to_string()).await {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }

    match repo.query_developer(create_result1.id).await {
        Ok(dev) => match dev {
            Some(dev) => dev.email == new_email,
            None => panic!("Developer's email does not match after email confirm")
        },
        Err(e) => panic!("{}", e)
    };
}