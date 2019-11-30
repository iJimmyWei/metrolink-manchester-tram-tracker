extern crate reqwest;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Data {
    pub Id: i32,
    pub Line: String,
    pub TLAREF: String,
    pub PIDREF: String,
    pub StationLocation: String,
    pub AtcoCode: String,
    pub Direction: String,
    pub Dest0: String,
    pub Carriages0: String,
    pub Status0: String,
    pub Wait0: String,
    pub Dest1: String,
    pub Carriages1: String,
    pub Status1: String,
    pub Wait1: String,
    pub Dest2: String,
    pub Carriages2: String,
    pub Status2: String,
    pub Wait2: String,
    pub Dest3: String,
    pub Carriages3: String,
    pub Status3: String,
    pub Wait3: String,
    pub MessageBoard: String,
    pub LastUpdated: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseDto {
    #[serde(rename="@odata.context")]
    pub context: String,
    pub value: Vec<Data>,
}

pub fn get(client: Client) -> Result<ResponseDto, reqwest::Error> {
    let api_key = "";
    let request_url = "https://api.tfgm.com/odata/Metrolinks";

    let mut res = client.get(request_url)
        .header("Ocp-Apim-Subscription-Key", api_key)
        .send()?
        .error_for_status()?;

    let body: ResponseDto = res.json()?;

    Ok(body)
}

pub fn handler(e: reqwest::Error) {
    if e.is_client_error() {
        println!("We fudged up - {}", e);
    }

    if e.is_server_error() {
        println!("Server Error - {}", e)
    }

    if e.is_serialization() {
        let serde_error = match e.get_ref() {
            None => return,
            Some(err) => err
        };

        println!("Problem parsing information {}", serde_error);
    }
}