use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime};
use egui::Ui;

use crate::ui::components::calendar::event::Event;
use crate::ui::components::calendar::render::{
    render_day_view, render_event_dialog, render_month_view, render_navigation_header,
    render_toolbar, render_week_view, render_year_view,
};
use crate::ui::components::calendar::state::{CalendarState, CalendarViewMode};
use crate::utils::config::Theme;

#[derive(Clone)]
pub struct Calendar {
    state: CalendarState,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            state: CalendarState::default(),
        }
    }

    pub fn with_events(events: Vec<Event>) -> Self {
        let mut calendar = Self::new();
        for event in events {
            calendar.add_event(event);
        }
        calendar
    }

    pub fn add_event(&mut self, event: Event) {
        let date = event.start_time.date_naive();
        self.state
            .event_map
            .entry(date)
            .or_insert_with(Vec::new)
            .push(event.clone());
        self.state.all_events.push(event);
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.state.all_events
    }

    pub fn render(&mut self, ui: &mut Ui, theme: &Theme) {
        render_toolbar(ui, &mut self.state, theme);
        ui.add_space(16.0);

        egui::Frame::new()
            .fill(theme.background)
            .show(ui, |ui| match self.state.view_mode {
                CalendarViewMode::Year => render_year_view(ui, &mut self.state, theme),
                CalendarViewMode::Month => render_month_view(ui, &mut self.state, theme),
                CalendarViewMode::Week => render_week_view(ui, &mut self.state, theme),
                CalendarViewMode::Day => render_day_view(ui, &mut self.state, theme),
            });

        if self.state.show_event_dialog {
            render_event_dialog(ui, &mut self.state, theme);
        }
    }

    pub fn navigate(&mut self, forward: bool) {
        let duration = match self.state.view_mode {
            CalendarViewMode::Year => Duration::days(365),
            CalendarViewMode::Month => Duration::days(30),
            CalendarViewMode::Week => Duration::days(7),
            CalendarViewMode::Day => Duration::days(1),
        };

        self.state.selected_date = if forward {
            self.state.selected_date + duration
        } else {
            self.state.selected_date - duration
        };
    }

    // Static method to navigate a calendar state directly
    pub fn navigate_state(state: &mut CalendarState, forward: bool) {
        let duration = match state.view_mode {
            CalendarViewMode::Year => Duration::days(365),
            CalendarViewMode::Month => Duration::days(30),
            CalendarViewMode::Week => Duration::days(7),
            CalendarViewMode::Day => Duration::days(1),
        };

        state.selected_date = if forward {
            state.selected_date + duration
        } else {
            state.selected_date - duration
        };
    }

    pub fn handle_time_slot_click(&mut self, date: NaiveDate, hour: u32) {
        let naive_time = NaiveTime::from_hms_opt(hour, 0, 0).unwrap();
        self.state.selected_time_slot = Some((date, naive_time));
        self.state.new_event = Some(Event::default());
        self.state.show_event_dialog = true;
    }

    // Static method to add an event to a calendar state directly
    pub fn add_event_to_state(state: &mut CalendarState, event: Event) {
        let date = event.start_time.date_naive();
        state
            .event_map
            .entry(date)
            .or_insert_with(Vec::new)
            .push(event.clone());
        state.all_events.push(event);
    }

    // Static method to handle time slot click on a calendar state directly
    pub fn handle_time_slot_click_state(state: &mut CalendarState, date: NaiveDate, hour: u32) {
        let naive_time = NaiveTime::from_hms_opt(hour, 0, 0).unwrap();
        state.selected_time_slot = Some((date, naive_time));
        state.new_event = Some(Event::default());
        state.show_event_dialog = true;
    }
}
