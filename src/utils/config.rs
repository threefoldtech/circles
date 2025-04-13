// Configuration utilities will be implemented here

use egui::Color32;

#[allow(dead_code)]
// Constants for styling and layout
pub struct Theme {
    pub accent: Color32,
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub border: Color32,
    pub active: Color32,
    pub hover: Color32,
    pub shadow: Color32,
    pub success: Color32,
    pub team: Color32,
    pub private: Color32,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            accent: Color32::from_rgb(66, 133, 244),
            background: Color32::from_rgb(245, 247, 250),
            panel: Color32::WHITE,
            text: Color32::from_rgb(40, 50, 60),
            border: Color32::from_rgb(230, 235, 240),
            active: Color32::from_rgb(220, 230, 240),
            hover: Color32::from_rgb(235, 240, 245),
            shadow: Color32::from_black_alpha(20),
            success: Color32::from_rgb(76, 175, 80),
            team: Color32::from_rgb(33, 150, 243),
            private: Color32::from_rgb(156, 39, 176),
        }
    }
}
