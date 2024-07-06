pub mod common {
    pub mod authentication{
        pub mod test_auth_keys_service;
    }
    pub mod repository {
        pub mod application {
            pub mod test_application;
        }
        pub mod companies {
            pub mod test_companies;
        }
        pub mod countries {
            pub mod test_countries;
        }
        pub mod industries {
            pub mod test_industries;
        }
        pub mod languages {
            pub mod test_languages;
        }
        pub mod salaries {
            pub mod test_salaries;
        }
        pub mod developers {
            pub mod test_developers;
        }
        pub mod employers {
            pub mod test_employers;
        }
        pub mod jobs {
            pub mod test_jobs;
        }
        pub mod user {
            pub mod test_user;
        }
    }
    pub mod routes {
        pub mod authentication {
            pub mod test_route_authentication;
        }
    }
}