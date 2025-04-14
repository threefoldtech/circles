use egui::Color32;

#[allow(dead_code)]
pub struct Theme {
    pub accent: Color32,
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub header_text: Color32,
    pub secondary_text: Color32,
    pub secondary_background: Color32,
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
            accent: Color32::from_rgb(59, 130, 246),      // Royal blue
            background: Color32::from_rgb(248, 250, 252), // Warm cream
            panel: Color32::from_rgb(255, 255, 255),      // White
            text: Color32::from_rgb(30, 41, 59),          // Dark slate
            header_text: Color32::from_rgb(71, 85, 105),  // Slate blue
            secondary_text: Color32::from_rgb(100, 116, 139), // Soft gray
            secondary_background: Color32::from_rgb(226, 232, 240), // Light blue-gray
            border: Color32::from_rgb(203, 213, 225),     // Subtle gray
            active: Color32::from_rgb(96, 165, 250),      // Bright blue
            hover: Color32::from_rgb(239, 246, 255),      // Pale blue
            shadow: Color32::from_black_alpha(15),        // Soft shadow
            success: Color32::from_rgb(52, 211, 153),     // Muted green
            team: Color32::from_rgb(2, 132, 199),         // Vibrant blue
            private: Color32::from_rgb(168, 85, 247),     // Soft purple
        }
    }
}
