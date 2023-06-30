use lazy_static::lazy_static;
use crate::common::components::select::SelectOption;

pub fn prim_languages() -> Vec<SelectOption> {
    lazy_static!{
        static ref PRIM_LANGUAGES: Vec<SelectOption> = {
            vec![
                SelectOption { id: 1, label: "Rust".to_string()},
                SelectOption { id: 7, label: "Go".to_string()},
                SelectOption { id: 10, label: "Ruby".to_string()},
                SelectOption { id: 11, label: "Swift".to_string()},
                SelectOption { id: 12, label: "Kotlin".to_string()},
                SelectOption { id: 13, label: "Scala".to_string()},
                SelectOption { id: 14, label: "Elixir".to_string()}
            ]
        };
    }
    (*PRIM_LANGUAGES).to_vec()
}

pub fn sec_languages() -> Vec<SelectOption> {
    lazy_static!{
        static ref SEC_LANGUAGES: Vec<SelectOption> = {
            let mut sec_languages = prim_languages().clone();
            sec_languages.insert(0, SelectOption { id: 0, label: "Optional".to_string() });
            sec_languages
        };
    }
    (*SEC_LANGUAGES).to_vec()
}

pub fn industries() -> Vec<SelectOption> {
    lazy_static!{
        static ref INDUSTRIES: Vec<SelectOption> = {
            let industries = vec![
                SelectOption { id: 1, label: "Finance".to_string()},
                SelectOption { id: 2, label: "Crypto".to_string()},
                SelectOption { id: 3, label: "AI/ML".to_string()},
                SelectOption { id: 5, label: "Video Games".to_string()},
            ];
            industries
        };
    }
    (*INDUSTRIES).to_vec()
}

pub fn salaries() -> Vec<SelectOption> {
    lazy_static!{
        static ref SALARIES: Vec<SelectOption> = {
            let salaries = vec![
                SelectOption { id: 1, label: "$200,000+".to_string()},
                SelectOption { id: 2, label: "$300,000+".to_string()},
                SelectOption { id: 3, label: "$400,000+".to_string()},
            ];
            salaries
        };
    }
    (*SALARIES.clone()).to_vec()
}

pub fn locations() -> Vec<SelectOption> {
    lazy_static!{
        static ref LOCATIONS: Vec<SelectOption> = {
            let locations = vec![
                SelectOption { id: 1, label: "Remote".to_string()},
                SelectOption { id: 2, label: "New York, United States".to_string()},
                SelectOption { id: 3, label: "San Francisco, United States".to_string()},
                SelectOption { id: 3, label: "Other".to_string()},
            ];
            locations
        };
    }
    (*LOCATIONS.clone()).to_vec()
}