use super::components::circle_dialog;
use super::features::settings;
use crate::app::{ActiveFeature, CircleApp};
use crate::ui::features::{
    ai_tools, bot_channel, calendar, chat, documents, mail, video_conf, welcome,
};
use eframe::egui::{CentralPanel, Context, Frame};

// Feature content rendering
pub fn render_feature_content(app: &mut CircleApp, ctx: &Context, app_layout: &Frame) {
    // Render the circle dialog if open
    render_circle_dialog(app, ctx);
    CentralPanel::default()
        .frame(app_layout.clone())
        .show(ctx, |ui| match app.active_feature {
            ActiveFeature::Mail => mail::render_mail(app, ui),
            ActiveFeature::Calendar => calendar::render_calendar(app, ui),
            ActiveFeature::Chat => chat::render_chat(app, ui),
            ActiveFeature::Documents => documents::render_documents(app, ui),
            ActiveFeature::AITools => ai_tools::render_ai_tools(app, ui),
            ActiveFeature::VideoConference => video_conf::render_video_conference(app, ui),
            ActiveFeature::Settings => settings::render_settings(app, ui),
            ActiveFeature::Welcome => welcome::render_welcome_screen(app, ui),
            ActiveFeature::BotChannel => bot_channel::render_bot_channel(app, ui),
        });
}

fn render_circle_dialog(app: &mut CircleApp, ctx: &Context) {
    if app.circle_dialog_state.is_open {
        if let Some(user) = &app.user {
            if let Some(new_circle) =
                circle_dialog::render_circle_dialog(&mut app.circle_dialog_state, ctx, user.id)
            {
                app.add_circle(new_circle.clone());

                // Set the newly created circle as the active circle
                app.set_active_circle(new_circle.id);

                // Ensure the navbar is displayed by setting is_first_time to false
                app.is_first_time = false;

                // Set the active feature to the default (Mail) instead of Welcome
                app.set_active_feature(crate::app::ActiveFeature::default());
            }
        }
    }
}
