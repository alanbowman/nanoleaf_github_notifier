use crate::nanoleaf::{EffectCommand, NanoleafClient, WriteCommand, HSB};

pub trait Notification {
    fn notify(&self);
}

impl Notification for NanoleafClient {
    fn notify(&self) {
        let d = WriteCommand {
            write: EffectCommand::DisplayTemp {
                duration: 5,
                anim_type: crate::nanoleaf::AnimType::Solid,
                palette: vec![HSB {
                    hue: 10,
                    saturation: 100,
                    brightness: 100,
                }],
                color_type: crate::nanoleaf::ColorType::HSB,
            },
        };
        self.write_command(&d);
    }
}
