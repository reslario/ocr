use {
    reqwest::blocking::Client,
    crate::{
        args::Language,
        file::{ImageFile, Filetype}
    }
};

const KEY: &str = include_str!("../key.txt");

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct FormData {
    base64_image: String,
    language: Language,
    filetype: Filetype,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub parsed_results: Vec<Parsed>,
    #[serde(rename = "IsErroredOnProcessing")]
    pub error: bool,
    pub error_message: Option<String>,
    pub error_details: Option<String>
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Parsed {
    #[serde(rename = "ParsedText")]
    pub text: Option<String>,
    pub error_message: Option<String>,
    pub error_details: Option<String>
}

pub fn send_request(client: &Client, image: ImageFile, language: Language) -> reqwest::Result<Response> {
    let form_data = FormData {
        base64_image: image.base64,
        filetype: image.filetype,
        language,
        ..<_>::default()
    };

    client
        .post("https://api.ocr.space/parse/image")
        .header("apikey", KEY)
        .form(&form_data)
        .send()?
        .json::<Response>()
}