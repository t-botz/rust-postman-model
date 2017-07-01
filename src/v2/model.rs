extern crate serde_json;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Collection {
    variables: Vec<::serde_json::Value>,
    info: Info,
    item: Vec<Item>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    item: Vec<Item2>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Item2 {
    name: String,
    #[serde(default)]
    event: Vec<Event>,
    request: Request,
    response: Vec<Response>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Response {
    id: String,
    name: String,
    #[serde(rename = "originalRequest")]
    original_request: OriginalRequest,
    status: String,
    code: i64,
    _postman_previewlanguage: String,
    _postman_previewtype: String,
    header: Vec<Header3>,
    cookie: Vec<Cookie>,
    #[serde(rename = "responseTime")]
    response_time: String,
    body: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Cookie {
    expires: String,
    #[serde(rename = "hostOnly")]
    host_only: Option<bool>,
    #[serde(rename = "httpOnly")]
    http_only: bool,
    domain: String,
    path: String,
    secure: bool,
    session: Option<bool>,
    value: String,
    key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Header3 {
    name: String,
    key: String,
    value: String,
    description: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct OriginalRequest {
    header: Vec<Header2>,
    body: Body2,
    url: Option<String>,
    method: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Body2 {
    mode: Option<String>,
    raw: Option<String>,
    #[serde(default)]
    formdata: Vec<Formdaum2>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Formdaum2 {
    key: String,
    value: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Header2 {
    key: String,
    name: Option<String>,
    value: String,
    #[serde(rename = "type")]
    type_field: Option<String>,
    enabled: Option<bool>,
    description: Option<String>,
    disabled: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Request {
    url: String,
    method: String,
    header: Vec<Header>,
    body: Body,
    description: String,
    auth: Option<Auth>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Auth {
    #[serde(rename = "type")]
    type_field: String,
    basic: Option<Basic>,
    oauth1: Option<Oauth1>,
    hawk: Option<Hawk>,
    digest: Option<Digest>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Digest {
    algorithm: String,
    username: String,
    realm: String,
    password: String,
    nonce: String,
    #[serde(rename = "nonceCount")]
    nonce_count: String,
    #[serde(rename = "clientNonce")]
    client_nonce: String,
    opaque: String,
    qop: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Hawk {
    #[serde(rename = "authId")]
    auth_id: String,
    #[serde(rename = "authKey")]
    auth_key: String,
    algorithm: String,
    user: String,
    #[serde(rename = "saveHelperData")]
    save_helper_data: bool,
    nonce: String,
    timestamp: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Oauth1 {
    #[serde(rename = "consumerKey")]
    consumer_key: String,
    #[serde(rename = "consumerSecret")]
    consumer_secret: String,
    token: String,
    #[serde(rename = "tokenSecret")]
    token_secret: String,
    #[serde(rename = "signatureMethod")]
    signature_method: String,
    timestamp: i64,
    nonce: String,
    version: String,
    realm: String,
    #[serde(rename = "addParamsToHeader")]
    add_params_to_header: bool,
    #[serde(rename = "addEmptyParamsToSign")]
    add_empty_params_to_sign: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Basic {
    username: String,
    password: String,
    #[serde(rename = "saveHelperData")]
    save_helper_data: bool,
    #[serde(rename = "showPassword")]
    show_password: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Body {
    mode: Option<String>,
    #[serde(default)]
    formdata: Vec<Formdaum>,
    raw: Option<String>,
    #[serde(default)]
    urlencoded: Vec<Urlencoded>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Urlencoded {
    key: String,
    value: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Formdaum {
    key: String,
    value: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Header {
    key: String,
    value: String,
    description: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Event {
    listen: String,
    script: Script,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Script {
    #[serde(rename = "type")]
    type_field: String,
    exec: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Info {
    name: String,
    _postman_id: String,
    description: String,
    schema: String,
}