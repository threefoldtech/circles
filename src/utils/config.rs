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

/// Theme configuration for the application
///
/// This struct defines all the colors used throughout the application.
/// It provides a consistent way to style UI components and ensures
/// that the application has a cohesive look and feel.
#[derive(Clone)]
pub struct Theme {
    // Primary colors
    /// Main accent color used for primary actions, highlights, and active states
    pub accent: Color32,
    /// Primary background color for the application
    pub background: Color32,
    /// Background color for panels, cards, and other containers
    pub panel: Color32,
    /// Primary text color
    pub text: Color32,

    // Text colors
    /// Color for light text (usually on dark backgrounds)
    pub light_color: Color32,
    /// Color for dark text (usually on light backgrounds)
    pub dark_color: Color32,
    /// Color for header text
    pub header_text: Color32,
    /// Color for secondary or less important text
    pub secondary_text: Color32,
    /// Color for placeholder text in input fields
    pub placeholder_text: Color32,

    // Background variations
    /// Secondary background color for alternating sections
    pub secondary_background: Color32,
    /// Surface color for cards and elevated elements
    pub surface: Color32,

    // Interactive elements
    /// Color for active elements
    pub active: Color32,
    /// Color for elements in hover state
    pub hover: Color32,
    /// Color for focused elements
    pub focus: Color32,
    /// Color for disabled elements
    pub disabled: Color32,
    /// Color for primary buttons
    pub button_primary: Color32,
    /// Color for secondary buttons
    pub button_secondary: Color32,

    // Borders and dividers
    /// Color for borders
    pub border: Color32,
    /// Color for dividers
    pub divider: Color32,

    // Status colors
    /// Color for success states
    pub success: Color32,
    /// Color for warning states
    pub warning: Color32,
    /// Color for error states
    pub error: Color32,
    /// Color for info states
    pub info: Color32,

    // Special purpose colors
    /// Color for team-related elements
    pub team: Color32,
    /// Color for private/secure elements
    pub private: Color32,
    /// Color for shadows
    pub shadow: Color32,
    /// Color for today's date in calendar
    pub calendar_today: Color32,
    /// Color for video backgrounds
    pub video_background: Color32,
    /// Color for self-video border
    pub self_video_border: Color32,

    // Utility colors
    /// Pure white color
    pub white: Color32,
    /// Pure black color
    pub black: Color32,
    /// Transparent color
    pub transparent: Color32,
    /// Background color for icons
    pub icon_bg: Color32,
    /// Foreground color for icons
    pub icon_fg: Color32,
    /// Color for destructive actions (like delete)
    pub destructive: Color32,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            // Primary colors
            accent: Color32::from_rgb(59, 130, 246), // Royal blue
            background: Color32::from_rgb(248, 250, 252), // Warm cream
            panel: Color32::from_rgb(255, 255, 255), // White
            text: Color32::from_rgb(30, 41, 59),     // Dark slate

            // Text colors
            light_color: Color32::from_rgb(255, 255, 255), // White
            dark_color: Color32::from_rgb(30, 41, 59),     // Dark slate
            header_text: Color32::from_rgb(71, 85, 105),   // Slate blue
            secondary_text: Color32::from_rgb(100, 116, 139), // Soft gray
            placeholder_text: Color32::from_rgb(120, 130, 140), // Medium gray for placeholder text

            // Background variations
            secondary_background: Color32::from_rgb(226, 232, 240), // Light blue-gray
            surface: Color32::from_rgb(255, 255, 255),              // White surface

            // Interactive elements
            active: Color32::from_rgb(96, 165, 250), // Bright blue
            hover: Color32::from_rgb(239, 246, 255), // Pale blue
            focus: Color32::from_rgb(224, 242, 254), // Light blue focus
            disabled: Color32::from_rgb(203, 213, 225), // Light gray disabled
            button_primary: Color32::from_rgb(66, 133, 244), // Blue for primary buttons
            button_secondary: Color32::from_rgb(226, 232, 240), // Light gray for secondary buttons

            // Borders and dividers
            border: Color32::from_rgb(203, 213, 225), // Subtle gray
            divider: Color32::from_rgb(226, 232, 240), // Light divider

            // Status colors
            success: Color32::from_rgb(52, 211, 153), // Muted green
            warning: Color32::from_rgb(251, 191, 36), // Amber warning
            error: Color32::from_rgb(239, 68, 68),    // Soft red
            info: Color32::from_rgb(96, 165, 250),    // Info blue

            // Special purpose colors
            team: Color32::from_rgb(2, 132, 199), // Vibrant blue
            private: Color32::from_rgb(168, 85, 247), // Soft purple
            shadow: Color32::from_black_alpha(15), // Soft shadow
            calendar_today: Color32::from_rgb(234, 67, 53), // Red for calendar today highlight
            video_background: Color32::from_rgb(20, 20, 30), // Dark video background
            self_video_border: Color32::from_rgb(100, 100, 255), // Blue self video border

            // Utility colors
            white: Color32::from_rgb(255, 255, 255), // Pure white
            black: Color32::from_rgb(0, 0, 0),       // Pure black
            transparent: Color32::TRANSPARENT,       // Transparent
            icon_bg: Color32::from_rgb(230, 235, 240), // Light gray for icon backgrounds
            icon_fg: Color32::from_rgb(70, 80, 90),  // Dark gray for icon foregrounds
            destructive: Color32::from_rgb(220, 38, 38), // Red for destructive actions
        }
    }

    pub fn dark() -> Self {
        Self {
            // Primary colors
            accent: Color32::from_rgb(96, 165, 250), // Bright blue
            background: Color32::from_rgb(15, 23, 42), // Very dark slate
            panel: Color32::from_rgb(30, 41, 59),    // Dark slate
            text: Color32::from_rgb(241, 245, 249),  // Very light gray

            // Text colors
            light_color: Color32::from_rgb(241, 245, 249), // Very light gray
            dark_color: Color32::from_rgb(15, 23, 42),     // Very dark slate
            header_text: Color32::from_rgb(226, 232, 240), // Light gray
            secondary_text: Color32::from_rgb(148, 163, 184), // Medium gray
            placeholder_text: Color32::from_rgb(148, 163, 184), // Medium gray for placeholder text

            // Background variations
            secondary_background: Color32::from_rgb(30, 41, 59), // Darker slate
            surface: Color32::from_rgb(30, 41, 59),              // Dark surface

            // Interactive elements
            active: Color32::from_rgb(59, 130, 246), // Royal blue
            hover: Color32::from_rgb(51, 65, 85),    // Medium slate
            focus: Color32::from_rgb(30, 58, 138),   // Deep blue focus
            disabled: Color32::from_rgb(71, 85, 105), // Dark gray disabled
            button_primary: Color32::from_rgb(59, 130, 246), // Blue for primary buttons
            button_secondary: Color32::from_rgb(51, 65, 85), // Dark gray for secondary buttons

            // Borders and dividers
            border: Color32::from_rgb(51, 65, 85),  // Dark gray
            divider: Color32::from_rgb(51, 65, 85), // Dark divider

            // Status colors
            success: Color32::from_rgb(34, 197, 94),  // Green
            warning: Color32::from_rgb(251, 191, 36), // Amber warning
            error: Color32::from_rgb(239, 68, 68),    // Soft red
            info: Color32::from_rgb(96, 165, 250),    // Info blue

            // Special purpose colors
            team: Color32::from_rgb(2, 132, 199), // Vibrant blue
            private: Color32::from_rgb(168, 85, 247), // Soft purple
            shadow: Color32::from_black_alpha(50), // Darker shadow
            calendar_today: Color32::from_rgb(234, 67, 53), // Red for calendar today highlight
            video_background: Color32::from_rgb(10, 10, 20), // Darker video background
            self_video_border: Color32::from_rgb(100, 100, 255), // Blue self video border

            // Utility colors
            white: Color32::from_rgb(255, 255, 255), // Pure white
            black: Color32::from_rgb(0, 0, 0),       // Pure black
            transparent: Color32::TRANSPARENT,       // Transparent
            icon_bg: Color32::from_rgb(51, 65, 85),  // Darker slate for icon backgrounds
            icon_fg: Color32::from_rgb(180, 190, 200), // Light gray for icon foregrounds
            destructive: Color32::from_rgb(220, 38, 38), // Red for destructive actions
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

    /// Convert the theme to egui Visuals
    ///
    /// This method converts our theme to egui's Visuals struct,
    /// which is used by egui to style the UI.
    pub fn to_visuals(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::default();

        // Text colors
        visuals.override_text_color = Some(self.text);

        // Widget colors
        visuals.widgets.noninteractive.bg_fill = self.background;
        visuals.widgets.inactive.bg_fill = self.panel;
        visuals.widgets.hovered.bg_fill = self.hover;
        visuals.widgets.active.bg_fill = self.active;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.text);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, self.text);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, self.text);

        // Selection
        visuals.selection.bg_fill = self.accent;
        visuals.selection.stroke = egui::Stroke::new(1.0, self.accent);

        // Window and panel colors
        visuals.window_fill = self.panel;
        visuals.panel_fill = self.panel;
        visuals.faint_bg_color = self.secondary_background;
        visuals.extreme_bg_color = self.background;

        // Misc colors
        visuals.hyperlink_color = self.accent;
        visuals.warn_fg_color = self.warning;
        visuals.error_fg_color = self.error;

        visuals
    }
}
