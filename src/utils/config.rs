use egui::Color32;

use crate::app::ActiveFeature;

pub const NAV_ITEMS: &[(&str, &str, ActiveFeature)] = &[
    ("📧", "Mail", ActiveFeature::Mail),
    ("📅", "Calendar", ActiveFeature::Calendar),
    ("💬", "Chat", ActiveFeature::Chat),
    ("📄", "Documents", ActiveFeature::Documents),
    ("🤖", "AI Tools", ActiveFeature::AITools),
    ("📹", "Video", ActiveFeature::VideoConference),
    ("⚙️", "Settings", ActiveFeature::Settings),
];

pub struct LayoutConfig {
    pub spacing: f32,
    pub sidebar_width: f32,
    pub navbar_height: f32,
    // pub button_size: Vec2,
}

impl LayoutConfig {
    pub fn new() -> Self {
        Self {
            spacing: 12.0,
            sidebar_width: 300.0,
            navbar_height: 55.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Theme {
    pub accent: Color32,
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub light_color: Color32,
    pub dark_color: Color32,
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
    pub error: Color32,
    // Additional theme colors
    pub white: Color32,
    pub transparent: Color32,
    pub icon_bg: Color32,
    pub icon_fg: Color32,
    pub placeholder_text: Color32,
    pub button_primary: Color32,
    pub calendar_today: Color32,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            light_color: Color32::from_rgb(255, 255, 255), // White
            dark_color: Color32::from_rgb(30, 41, 59),     // Dark slate

            accent: Color32::from_rgb(59, 130, 246), // Royal blue
            background: Color32::from_rgb(248, 250, 252), // Warm cream
            panel: Color32::from_rgb(255, 255, 255), // White
            text: Color32::from_rgb(30, 41, 59),     // Dark slate
            header_text: Color32::from_rgb(71, 85, 105), // Slate blue
            secondary_text: Color32::from_rgb(100, 116, 139), // Soft gray
            secondary_background: Color32::from_rgb(226, 232, 240), // Light blue-gray
            border: Color32::from_rgb(203, 213, 225), // Subtle gray
            active: Color32::from_rgb(96, 165, 250), // Bright blue
            hover: Color32::from_rgb(239, 246, 255), // Pale blue
            shadow: Color32::from_black_alpha(15),   // Soft shadow
            success: Color32::from_rgb(52, 211, 153), // Muted green
            team: Color32::from_rgb(2, 132, 199),    // Vibrant blue
            private: Color32::from_rgb(168, 85, 247), // Soft purple
            error: Color32::from_rgb(239, 68, 68),   // Soft red

            // Additional theme colors
            white: Color32::from_rgb(255, 255, 255), // Pure white
            transparent: Color32::TRANSPARENT,       // Transparent
            icon_bg: Color32::from_rgb(230, 235, 240), // Light gray for icon backgrounds
            icon_fg: Color32::from_rgb(70, 80, 90),  // Dark gray for icon foregrounds
            placeholder_text: Color32::from_rgb(120, 130, 140), // Medium gray for placeholder text
            button_primary: Color32::from_rgb(66, 133, 244), // Blue for primary buttons
            calendar_today: Color32::from_rgb(234, 67, 53), // Red for calendar today highlight
        }
    }

    pub fn dark() -> Self {
        Self {
            light_color: Color32::from_rgb(241, 245, 249), // Very light gray
            dark_color: Color32::from_rgb(15, 23, 42),     // Very dark slate

            accent: Color32::from_rgb(96, 165, 250), // Bright blue
            background: Color32::from_rgb(15, 23, 42), // Very dark slate
            panel: Color32::from_rgb(30, 41, 59),    // Dark slate
            text: Color32::from_rgb(241, 245, 249),  // Very light gray
            header_text: Color32::from_rgb(226, 232, 240), // Light gray
            secondary_text: Color32::from_rgb(148, 163, 184), // Medium gray
            secondary_background: Color32::from_rgb(30, 41, 59), // Darker slate
            border: Color32::from_rgb(51, 65, 85),   // Dark gray
            active: Color32::from_rgb(59, 130, 246), // Royal blue
            hover: Color32::from_rgb(51, 65, 85),    // Medium slate
            shadow: Color32::from_black_alpha(50),   // Darker shadow
            success: Color32::from_rgb(34, 197, 94), // Green
            team: Color32::from_rgb(2, 132, 199),    // Vibrant blue
            private: Color32::from_rgb(168, 85, 247), // Soft purple
            error: Color32::from_rgb(239, 68, 68),   // Soft red

            // Additional theme colors
            white: Color32::from_rgb(255, 255, 255), // Pure white
            transparent: Color32::TRANSPARENT,       // Transparent
            icon_bg: Color32::from_rgb(51, 65, 85),  // Darker slate for icon backgrounds
            icon_fg: Color32::from_rgb(180, 190, 200), // Light gray for icon foregrounds
            placeholder_text: Color32::from_rgb(120, 130, 140), // Medium gray for placeholder text
            button_primary: Color32::from_rgb(59, 130, 246), // Blue for primary buttons
            calendar_today: Color32::from_rgb(234, 67, 53), // Red for calendar today highlight
        }
    }

    pub fn from_mode(mode: &crate::models::user::Theme, ctx: Option<&egui::Context>) -> Self {
        match mode {
            crate::models::user::Theme::Dark => Self::dark(),
            crate::models::user::Theme::Light => Self::light(),
            crate::models::user::Theme::System => {
                // Detect system theme if context is provided
                if let Some(ctx) = ctx {
                    if ctx.style().visuals.dark_mode {
                        Self::dark()
                    } else {
                        Self::light()
                    }
                } else {
                    // Fallback to OS detection without context
                    #[cfg(target_arch = "wasm32")]
                    {
                        // For web, we can use the prefers-color-scheme media query
                        let is_dark = web_sys::window()
                            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
                            .flatten()
                            .map(|m| m.matches())
                            .unwrap_or(false);

                        if is_dark { Self::dark() } else { Self::light() }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // For native, try to detect using the system
                        match dark_light::detect() {
                            dark_light::Mode::Dark => Self::dark(),
                            dark_light::Mode::Light | dark_light::Mode::Default => Self::light(),
                        }
                    }
                }
            }
        }
    }

    pub fn to_visuals(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::default();

        visuals.override_text_color = Some(self.text);
        visuals.widgets.noninteractive.bg_fill = self.background;
        visuals.widgets.inactive.bg_fill = self.panel;
        visuals.widgets.hovered.bg_fill = self.hover;
        visuals.widgets.active.bg_fill = self.active;
        visuals.selection.bg_fill = self.accent;

        // Set additional visual properties based on theme
        visuals.window_fill = self.panel;
        visuals.panel_fill = self.panel;
        visuals.faint_bg_color = self.secondary_background;
        visuals.extreme_bg_color = self.background;

        // Adjust widgets
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, self.text);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, self.text);

        visuals
    }
}
