use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use clap::{arg, command, value_parser, ArgAction, Command};

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    value: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    on: Value
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Effects {
    effects_list: Vec<String>,
    select: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PanelPosition {
    panel_id: Number,
    x: Number,
    y: Number,
    o: Number,
    shape_type: Number,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Layout {
    num_panels: Number,
    side_length: Number,
    position_data: Vec<PanelPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PanelLayout {
    // globalOrientation:
    layout: Layout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PanelInfo {
    name: String,
    serial_no: String,
    manufacturer: String,
    firmware_version: String,
    hardware_version: String,
    model: String,
    effects: Effects,
    // firmwareUpgrade: ???
    panel_layout: PanelLayout,
}

#[derive(Serialize, Debug)]
struct EffectCommand {
    write: Commands,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DisplayTempEffectSaved {
    
    anim_name: String,
}

#[derive(Serialize, Debug)]
#[serde(tag="command")]
enum Commands {
    displayTemp{ duration: u32, animName: String },
}

struct NanoleafClient {
    client: reqwest::blocking::Client,
    key: String,
    base_url: String,
}

impl NanoleafClient {
    fn new(
        client: reqwest::blocking::Client,
        key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> NanoleafClient {
        NanoleafClient {
            client: client,
            key: key.into(),
            base_url: base_url.into(),
        }
    }

    fn get_info(&self) -> PanelInfo {
        let body = self
            .client
            .get(format!("{}/{}", self.base_url, self.key))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let info: PanelInfo = serde_json::from_str(&body).unwrap();

        info
    }

    fn write_command(&self) {
        let d: EffectCommand = EffectCommand {
            write: Commands::displayTemp{
                duration: 5,
                animName: "Northern Lights".to_owned(),
            },
        };

        println!("{}", serde_json::to_string_pretty(&d).unwrap());

        let res = self
            .client
            .put(format!(
                "{}/{}/effects",
                self.base_url,
                self.key
            ))
            .json(&d)
            .send()
            .unwrap();

            println!("{:?}", res);
    }

    fn get_effect(&self) -> String {
        let effect: String = self
            .client
            .get(format!("{}/{}/effects/select", self.base_url, self.key))
            .send()
            .unwrap()
            .text()
            .unwrap();

        effect
    }

    fn turn_on( &self ) {
        self.client.put(format!("{}/{}/state", self.base_url, self.key))
            .json(&State{on: Value{ value:true }})
            .send()
            .unwrap();
    }

    fn turn_off( &self ) {
        self.client.put(format!("{}/{}/state", self.base_url, self.key))
            .json(&State{on: Value{ value:false }})
            .send()
            .unwrap();
    }
}

fn main() {
    let matches = Command::new("Nanoload Control")
        .arg(arg!(--api <API_KEY> "Nanoleaf API Key").required(true))
        .get_matches();

    let api_key = matches.get_one::<String>("api").expect("required");

    let client = reqwest::blocking::Client::new();
    let nl = NanoleafClient::new(
        client,
        api_key,
        "http://192.168.4.48:16021/api/v1",
    );
    
    println!("{:?}", nl.get_info());
    nl.turn_on();
    nl.write_command();

    println!("{}", nl.get_effect());
    
    nl.turn_off();
}
