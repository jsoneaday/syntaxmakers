use actix_http::header::HeaderMap;

pub fn get_header_strings(headers: &HeaderMap) -> Vec<(&str, &str)> {
    headers.iter().map(|header| {
        let header_val_result = header.1.to_str();
        let header_val_str = match header_val_result {
            Ok(header_val) => header_val,
            Err(_) => ""
        };

        (header.0.as_str(), header_val_str)
    })
    .collect::<Vec<(&str, &str)>>()
}