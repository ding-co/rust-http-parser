use super::method::HttpMethod;

pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    query_string: Option<String>,
}
