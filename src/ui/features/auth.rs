// File: src/ui/features/auth.rs
use crate::app::CircleApp;
use crate::models::user::{User, UserPreferences};
use crate::utils::config::Theme;
use chrono::Utc;
use eframe::egui::{self, Button, Frame, Margin, RichText, TextEdit, Ui, Vec2};
use egui_phosphor::regular::{EYE, EYE_SLASH};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::PathBuf;
use uuid::Uuid;

// Credentials structure to be saved to file
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub email: String,
    pub password_hash: String, // In a real app, this would be a properly hashed password
}

// Auth tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthTab {
    SignUp,
    SignIn,
}

// Auth screen state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub active_tab: AuthTab,
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub show_password: bool,
    pub show_confirm_password: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            active_tab: AuthTab::SignIn,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            error_message: None,
            success_message: None,
            show_password: false,
            show_confirm_password: false,
        }
    }
}

// Get the path to the credentials file
fn get_credentials_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".config");
    fs::create_dir_all(&path).ok(); // Create .config directory if it doesn't exist
    path.push("circles.json");
    path
}

// Load credentials from file
pub fn load_credentials() -> Result<Credentials, io::Error> {
    let path = get_credentials_path();
    let content = fs::read_to_string(path)?;
    serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// Save credentials to file
fn save_credentials(credentials: &Credentials) -> Result<(), io::Error> {
    let path = get_credentials_path();
    let content = serde_json::to_string_pretty(credentials)?;
    fs::write(path, content)
}

// Create a user from credentials
fn create_user_from_credentials(credentials: &Credentials) -> User {
    User {
        id: Uuid::new_v4(),
        name: credentials.name.clone(),
        email: credentials.email.clone(),
        created_at: Utc::now(),
        preferences: UserPreferences::default(),
    }
}

// Validate email format
fn is_valid_email(email: &str) -> bool {
    // Basic email validation using regex
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

// Validate password strength
fn is_strong_password(password: &str) -> bool {
    // Password must be at least 8 characters long
    if password.len() < 8 {
        return false;
    }

    // Password must contain at least one digit
    let has_digit = password.chars().any(|c| c.is_digit(10));
    if !has_digit {
        return false;
    }

    // Password must contain at least one special character
    let special_chars = "!@#$%^&*()_-+={}[]|:;<>,.?/~`";
    let has_special = password.chars().any(|c| special_chars.contains(c));
    if !has_special {
        return false;
    }

    // Password must contain at least one uppercase letter
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    if !has_uppercase {
        return false;
    }

    true
}

// Validate name
fn is_valid_name(name: &str) -> bool {
    // Name must not be empty
    if name.trim().is_empty() {
        return false;
    }

    // Name must not contain special characters except spaces, hyphens, and apostrophes
    let name_regex = Regex::new(r"^[a-zA-Z\s\-']+$").unwrap();
    name_regex.is_match(name)
}

// Thread-local storage for auth state to persist between renders
thread_local! {
    static AUTH_STATE: RefCell<AuthState> = RefCell::new(AuthState::default());
}

// Render the auth screen
pub fn render_auth_screen(app: &mut CircleApp, ui: &mut Ui, theme: &Theme) {
    // Get the auth state from thread-local storage
    let auth_state = AUTH_STATE.with(|state| state.borrow().clone());

    // Create local variables for convenience
    let mut name = auth_state.name.clone();
    let mut email = auth_state.email.clone();
    let mut password = auth_state.password.clone();
    let mut confirm_password = auth_state.confirm_password.clone();
    let error_message = auth_state.error_message.clone();
    let success_message = auth_state.success_message.clone();
    let mut active_tab = auth_state.active_tab;

    let available_size = ui.available_size();
    let available_height = available_size.y;
    let available_width = available_size.x;

    // Create a centered frame for the auth screen
    let auth_frame = Frame::new()
        .fill(theme.background)
        .inner_margin(Margin::same(20))
        .outer_margin(Margin::same(0));

    auth_frame.show(ui, |ui| {
        ui.set_min_height(available_height);
        ui.vertical_centered(|ui| {
            ui.add_space(available_height * 0.1);

            // App title
            ui.heading(
                RichText::new("Circles")
                    .size(32.0)
                    .strong()
                    .color(theme.accent),
            );
            ui.add_space(10.0);
            ui.label(
                RichText::new("Your collaboration platform")
                    .size(18.0)
                    .color(theme.text),
            );
            ui.add_space(40.0);

            // Auth form container
            let form_width = available_width.min(400.0);

            // Create a panel for the form with the theme's panel color
            let form_frame = Frame::new()
                .fill(theme.panel)
                .inner_margin(Margin::same(30))
                .corner_radius(8);

            form_frame.show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(form_width - 40.0, 0.0), // Account for inner margin
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Tab buttons
                        ui.horizontal(|ui| {
                            let tab_button_style = |ui: &mut Ui, selected: bool| {
                                let style = ui.style_mut();
                                if selected {
                                    style.visuals.widgets.active.bg_fill = theme.accent;
                                    style.visuals.widgets.active.fg_stroke.color = theme.white;
                                } else {
                                    style.visuals.widgets.active.bg_fill =
                                        theme.secondary_background;
                                    style.visuals.widgets.active.fg_stroke.color = theme.text;
                                }
                            };

                            // Sign In tab
                            {
                                let selected = active_tab == AuthTab::SignIn;
                                tab_button_style(ui, selected);
                                if ui
                                    .add_sized(
                                        [form_width / 2.0, 40.0],
                                        Button::new(
                                            RichText::new("Sign In")
                                                .size(16.0)
                                                .color(theme.light_color),
                                        )
                                        .fill(theme.button_primary),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    active_tab = AuthTab::SignIn;
                                    AUTH_STATE.with(|state| {
                                        let mut state = state.borrow_mut();
                                        state.error_message = None;
                                    });
                                }
                            }

                            // Sign Up tab
                            {
                                let selected = active_tab == AuthTab::SignUp;
                                tab_button_style(ui, selected);
                                if ui
                                    .add_sized(
                                        [form_width / 2.0, 40.0],
                                        Button::new(
                                            RichText::new("Sign Up")
                                                .size(16.0)
                                                .color(theme.light_color),
                                        )
                                        .fill(theme.button_primary),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    active_tab = AuthTab::SignUp;
                                    AUTH_STATE.with(|state| {
                                        let mut state = state.borrow_mut();
                                        state.error_message = None;
                                    });
                                }
                            }
                        });

                        ui.add_space(30.0);

                        // Form fields
                        match active_tab {
                            AuthTab::SignUp => {
                                // Name field
                                ui.label(RichText::new("Name").size(14.0).color(theme.text));
                                ui.add(
                                    TextEdit::singleline(&mut name)
                                        .hint_text("Enter your name")
                                        .desired_width(form_width - 40.0)
                                        .margin(Vec2::new(10.0, 8.0)),
                                );
                                ui.add_space(15.0);

                                // Email field
                                ui.label(RichText::new("Email").size(14.0).color(theme.text));
                                ui.add(
                                    TextEdit::singleline(&mut email)
                                        .hint_text("Enter your email")
                                        .desired_width(form_width - 40.0)
                                        .margin(Vec2::new(10.0, 8.0)),
                                );
                                ui.add_space(15.0);

                                // Password field with toggle visibility
                                ui.label(RichText::new("Password").size(14.0).color(theme.text));
                                ui.horizontal(|ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut password)
                                            .password(!auth_state.show_password)
                                            .hint_text("Enter your password")
                                            .desired_width(form_width - 80.0)
                                            .margin(Vec2::new(10.0, 8.0)),
                                    );
                                    
                                    // Eye icon to toggle password visibility
                                    let eye_icon = if auth_state.show_password { EYE } else { EYE_SLASH };
                                    if ui.add(Button::new(eye_icon.to_owned()).frame(false)).clicked() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.show_password = !state.show_password;
                                        });
                                    }
                                });
                                ui.add_space(15.0);

                                // Confirm password field with toggle visibility
                                ui.label(
                                    RichText::new("Confirm Password")
                                        .size(14.0)
                                        .color(theme.text),
                                );
                                ui.horizontal(|ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut confirm_password)
                                            .password(!auth_state.show_confirm_password)
                                            .hint_text("Confirm your password")
                                            .desired_width(form_width - 80.0)
                                            .margin(Vec2::new(10.0, 8.0)),
                                    );
                                    
                                    // Eye icon to toggle password visibility
                                    let eye_icon = if auth_state.show_confirm_password { EYE } else { EYE_SLASH };
                                    if ui.add(Button::new(eye_icon.to_owned()).frame(false)).clicked() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.show_confirm_password = !state.show_confirm_password;
                                        });
                                    }
                                });
                                ui.add_space(30.0);

                                // Sign Up button
                                if ui
                                    .add_sized(
                                        Vec2::new(form_width, 40.0),
                                        Button::new(
                                            RichText::new("Sign Up")
                                                .size(16.0)
                                                .color(theme.light_color),
                                        )
                                        .fill(theme.button_primary),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    // Validate inputs
                                    if name.trim().is_empty() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Name is required".to_string());
                                        });
                                    } else if !is_valid_name(&name) {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Name can only contain letters, spaces, hyphens, and apostrophes".to_string());
                                        });
                                    } else if email.trim().is_empty() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Email is required".to_string());
                                        });
                                    } else if !is_valid_email(&email) {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Please enter a valid email address".to_string());
                                        });
                                    } else if password.trim().is_empty() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Password is required".to_string());
                                        });
                                    } else if !is_strong_password(&password) {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Password must be at least 8 characters and include uppercase, digit, and special character".to_string());
                                        });
                                    } else if password != confirm_password {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Passwords do not match".to_string());
                                        });
                                    } else {
                                        // Create credentials
                                        let credentials = Credentials {
                                            name: name.clone(),
                                            email: email.clone(),
                                            password_hash: password.clone(), // In a real app, this would be hashed
                                        };

                                        // Save credentials
                                        match save_credentials(&credentials) {
                                            Ok(_) => {
                                                // Create user
                                                let user =
                                                    create_user_from_credentials(&credentials);
                                                app.user = Some(user);

                                                // Set first time to false to show normal app layout
                                                app.is_first_time = false;

                                                // Set success message
                                                AUTH_STATE.with(|state| {
                                                    let mut state = state.borrow_mut();
                                                    state.success_message =
                                                        Some("Sign up successful!".to_string());
                                                    state.error_message = None;

                                                    // Clear form
                                                    state.name = String::new();
                                                    state.email = String::new();
                                                    state.password = String::new();
                                                    state.confirm_password = String::new();
                                                });

                                                // Switch to default feature
                                                app.set_active_feature(
                                                    crate::app::ActiveFeature::default(),
                                                );
                                            }
                                            Err(_) => {
                                                AUTH_STATE.with(|state| {
                                                    let mut state = state.borrow_mut();
                                                    state.error_message = Some(
                                                        "Failed to save credentials".to_string(),
                                                    );
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            AuthTab::SignIn => {
                                // Email field
                                ui.label(RichText::new("Email").size(14.0).color(theme.text));
                                ui.add(
                                    TextEdit::singleline(&mut email)
                                        .hint_text("Enter your email")
                                        .desired_width(form_width - 40.0)
                                        .margin(Vec2::new(10.0, 8.0)),
                                );
                                ui.add_space(15.0);

                                // Password field with toggle visibility
                                ui.label(RichText::new("Password").size(14.0).color(theme.text));
                                ui.horizontal(|ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut password)
                                            .password(!auth_state.show_password)
                                            .hint_text("Enter your password")
                                            .desired_width(form_width - 80.0)
                                            .margin(Vec2::new(10.0, 8.0)),
                                    );
                                    
                                    // Eye icon to toggle password visibility
                                    let eye_icon = if auth_state.show_password { EYE } else { EYE_SLASH };
                                    if ui.add(Button::new(eye_icon.to_owned()).frame(false)).clicked() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.show_password = !state.show_password;
                                        });
                                    }
                                });
                                ui.add_space(30.0);

                                // Sign In button
                                if ui
                                    .add_sized(
                                        Vec2::new(form_width, 40.0),
                                        Button::new(
                                            RichText::new("Sign In")
                                                .size(16.0)
                                                .color(theme.light_color),
                                        )
                                        .fill(theme.button_primary),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    // Validate inputs
                                    if email.trim().is_empty() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Email is required".to_string());
                                        });
                                    } else if !is_valid_email(&email) {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Please enter a valid email address".to_string());
                                        });
                                    } else if password.trim().is_empty() {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Password is required".to_string());
                                        });
                                    } else if !is_strong_password(&password) {
                                        AUTH_STATE.with(|state| {
                                            let mut state = state.borrow_mut();
                                            state.error_message =
                                                Some("Password must be at least 8 characters and include uppercase, digit, and special character".to_string());
                                        });
                                    } else {
                                        // Load credentials
                                        match load_credentials() {
                                            Ok(credentials) => {
                                                // Check if email and password match
                                                if credentials.email == email
                                                    && credentials.password_hash == password
                                                {
                                                    // Create user
                                                    let user =
                                                        create_user_from_credentials(&credentials);
                                                    app.user = Some(user);

                                                    // Set first time to false to show normal app layout
                                                    app.is_first_time = false;

                                                    // Set success message
                                                    AUTH_STATE.with(|state| {
                                                        let mut state = state.borrow_mut();
                                                        state.success_message =
                                                            Some("Sign in successful!".to_string());
                                                        state.error_message = None;

                                                        // Clear form
                                                        state.email = String::new();
                                                        state.password = String::new();
                                                    });

                                                    // Switch to default feature
                                                    app.set_active_feature(
                                                        crate::app::ActiveFeature::default(),
                                                    );
                                                } else {
                                                    AUTH_STATE.with(|state| {
                                                        let mut state = state.borrow_mut();
                                                        state.error_message = Some(
                                                            "Invalid email or password".to_string(),
                                                        );
                                                    });
                                                }
                                            }
                                            Err(_) => {
                                                AUTH_STATE.with(|state| {
                                                    let mut state = state.borrow_mut();
                                                    state.error_message = Some(
                                                        "No account found. Please sign up."
                                                            .to_string(),
                                                    );
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        ui.add_space(10.0);

                        // Error message
                        if let Some(error) = &error_message {
                            ui.colored_label(theme.error, error);
                        }

                        // Success message
                        if let Some(success) = &success_message {
                            ui.colored_label(theme.success, success);
                        }
                    },
                );
            });
        });
    });

    // Update the auth state with the form values
    AUTH_STATE.with(|state| {
        let mut state_mut = state.borrow_mut();
        state_mut.name = name;
        state_mut.email = email;
        state_mut.password = password;
        state_mut.confirm_password = confirm_password;
        state_mut.active_tab = active_tab;
    });
}
