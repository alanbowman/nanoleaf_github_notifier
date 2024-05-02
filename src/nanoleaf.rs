use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ShapeType {
    Hexagon = 7,
    Triangle = 8,
    MiniTriangle = 9,
    ShapesController = 12,
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    on: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Effects {
    effects_list: Vec<String>,
    select: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PanelPosition {
    pub panel_id: Number,
    pub x: Number,
    pub y: Number,
    pub o: Number,
    pub shape_type: ShapeType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub num_panels: u32,
    pub side_length: u32,
    pub position_data: Vec<PanelPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PanelLayout {
    // globalOrientation:
    pub layout: Layout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PanelInfo {
    pub name: String,
    pub serial_no: String,
    pub manufacturer: String,
    pub firmware_version: String,
    pub hardware_version: String,
    pub model: String,
    pub effects: Effects,
    // firmwareUpgrade: ???
    pub panel_layout: PanelLayout,
}

#[derive(Serialize, Debug)]
pub struct HSB {
    pub hue: u8,
    pub saturation: u8,
    pub brightness: u8,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WriteCommand {
    pub write: EffectCommand,
}

#[derive(Serialize, Debug)]
pub enum ColorType {
    HSB,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AnimType {
    Solid,
    Static,
    Wheel,
    ExtControl,
    Random,
    Flow,
    Fade,
    Highlight,
    Custom,
    Plugin,
}

#[derive(Serialize, Debug)]
#[serde(
    rename_all_fields = "camelCase",
    rename_all = "camelCase",
    tag = "command"
)]
pub enum EffectCommand {
    DisplayTemp {
        duration: i32,
        anim_type: AnimType,
        palette: Vec<HSB>,
        color_type: ColorType,
    },
}

pub struct NanoleafClient {
    client: reqwest::blocking::Client,
    key: String,
    base_url: String,
}

impl NanoleafClient {
    pub fn new(key: impl Into<String>, base_url: impl Into<String>) -> NanoleafClient {
        NanoleafClient {
            client: reqwest::blocking::Client::new(),
            key: key.into(),
            base_url: base_url.into(),
        }
    }

    pub fn get_info(&self) -> PanelInfo {
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

    pub fn write_command(&self, command: &WriteCommand) {
        println!("{}", serde_json::to_string_pretty(&command).unwrap());

        let response = self
            .client
            .put(format!("{}/{}/effects", self.base_url, self.key))
            .json(command)
            .send()
            .unwrap();

        println!("{:?}", response);
    }

    pub fn get_effect(&self) -> String {
        let effect: String = self
            .client
            .get(format!("{}/{}/effects/select", self.base_url, self.key))
            .send()
            .unwrap()
            .text()
            .unwrap();

        effect
    }

    pub fn turn_on(&self) {
        self.client
            .put(format!("{}/{}/state", self.base_url, self.key))
            .json(&State {
                on: Value { value: true },
            })
            .send()
            .unwrap();
    }

    pub fn turn_off(&self) {
        self.client
            .put(format!("{}/{}/state", self.base_url, self.key))
            .json(&State {
                on: Value { value: false },
            })
            .send()
            .unwrap();
    }
}
