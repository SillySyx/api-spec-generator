use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ApiEntry {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,

    pub request_method: String,
    pub request_url: String,
    pub request_headers: Option<HashMap<String, String>>,
    pub request_body: Option<Value>,

    pub response: Option<HashMap<String, String>>,
    pub response_body: Option<Value>,
}

impl ApiEntry {
    pub fn from(json: &Value) -> Option<ApiEntry> {
        let name = parse_name(json);
        let request_method = parse_request_method(json);
        let request_url = parse_request_url(json);

        let description = parse_description(json);
        let permissions = parse_permissions(json);
        let request_headers = parse_request_headers(json);
        let request_body = parse_request_body(json);
        let response = parse_response(json);
        let response_body = parse_response_body(json);

        Some(ApiEntry {
            name,
            request_method,
            request_url,
            description,
            permissions,
            request_headers,
            request_body,
            response,
            response_body,
        })
    }

    pub fn generate_output(&self) -> String {
        let mut output = generate_header(self);
        
        if let Some(_) = self.permissions {
            output.push_str(&generate_permissions(self));
        }

        output.push_str(&generate_request(self));

        if let Some(_) = self.request_headers {
            output.push_str(&generate_request_headers(self));
        }

        if let Some(_) = self.request_body {
            output.push_str(&generate_request_body(self));
        }

        if let Some(_) = self.response {
            output.push_str(&generate_response(self));
        }

        if let Some(_) = self.response_body {
            output.push_str(&generate_response_body(self));
        }

        output.push_str("\n\n");
        output
    }
}

fn parse_name(json: &Value) -> String {
    json["name"]
        .as_str()
        .unwrap()
        .to_string()
}

fn parse_description(json: &Value) -> Option<String> {
    if let Some(description) = json["description"].as_str() {
        if description.is_empty() {
            return None;
        }
        
        return Some(description.to_string());
    }

    None
}

fn parse_permissions(json: &Value) -> Option<Vec<String>> {
    if let Some(permissions) = json["permissions"].as_array() {
        let permissions = permissions
            .iter()
            .fold(Vec::new(), |mut list, permission| {
                if let Some(value) = permission.as_str() {
                    list.push(value.to_string());
                }

                list
            });

        return Some(permissions)
    }

    None
}

fn parse_request_method(json: &Value) -> String {
    json["request_method"]
        .as_str()
        .unwrap()
        .to_string()
}

fn parse_request_url(json: &Value) -> String {
    json["request_url"]
        .as_str()
        .unwrap()
        .to_string()
}

fn parse_request_headers(json: &Value) -> Option<HashMap<String, String>> {
    if let Some(headers) = json["request_headers"].as_object() {
        let mut request_headers: HashMap<String, String> = HashMap::new();
        
        for item in headers {
            let value = item.1.as_str().unwrap().to_string();

            request_headers.insert(item.0.to_owned(), value);
        }

        return Some(request_headers);
    }

    None
}

fn parse_request_body(json: &Value) -> Option<Value> {
    if !json["request_body"].is_null() {
        return Some(json["request_body"].to_owned());
    }

    None
}

fn parse_response(json: &Value) -> Option<HashMap<String, String>> {
    if let Some(response) = json["response"].as_object() {
        let mut response_map: HashMap<String, String> = HashMap::new();
        
        for item in response {
            let value = item.1.as_str().unwrap().to_string();

            response_map.insert(item.0.to_owned(), value);
        }

        return Some(response_map);
    }

    None
}

fn parse_response_body(json: &Value) -> Option<Value> {
    if !json["response_body"].is_null() {
        return Some(json["response_body"].to_owned());
    }

    None
}

fn generate_header(entry: &ApiEntry) -> String {
    let mut output = format!("# {}\n", &entry.name);

    if let Some(description) = &entry.description {
        output.push_str(&description);
        output.push_str("  \n");
    }

    output.push_str("\n");
    output
}

fn generate_permissions(entry: &ApiEntry) -> String {
    let mut output = format!("## Permissions\n");

    if let Some(permissions) = &entry.permissions {
        for permission in permissions {
            output.push_str(&format!("* {}\n", &permission));
        }
    }

    output.push_str("\n");
    output
}

fn generate_request(entry: &ApiEntry) -> String {
    let mut output = format!("## HTTP request\n");
    output.push_str("``` \n");
    output.push_str(format!("{} {} \n", &entry.request_method, &entry.request_url).as_str());
    output.push_str("``` \n\n");
    output
}

fn generate_request_headers(entry: &ApiEntry) -> String {
    let mut output = format!("## Request headers\n");
    output.push_str("|Name|Value| \n");
    output.push_str("|-|-| \n");

    if let Some(headers) = &entry.request_headers {
        for header in headers {
            output.push_str(format!("|{}|{}| \n", header.0, header.1).as_str());
        }
    }

    output.push_str("\n");
    output
}

fn generate_request_body(entry: &ApiEntry) -> String {
    let mut output = format!("## Request body\n");
    output.push_str("``` \n");

    if let Some(request_body) = &entry.request_body {
        output.push_str(&serde_json::to_string_pretty(request_body).unwrap());
    }
    output.push_str("\n``` \n\n");
    output
}

fn generate_response(entry: &ApiEntry) -> String {
    let mut output = format!("## Response\n");
    
    if let Some(responses) = &entry.response {
        for response in responses {
            output.push_str(format!("`{}` {}  \n", response.0, response.1).as_str());
        }
    }

    output.push_str("\n");
    output
}

fn generate_response_body(entry: &ApiEntry) -> String {
    let mut output = format!("## Response body\n");
    output.push_str("``` \n");

    if let Some(response_body) = &entry.response_body {
        output.push_str(&serde_json::to_string_pretty(response_body).unwrap());
    }
    output.push_str("\n``` \n\n");
    output
}