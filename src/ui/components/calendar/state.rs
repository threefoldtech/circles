use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use std::collections::BTreeMap;
use uuid::Uuid;

use super::event::Event;

#[derive(Debug, Clone, PartialEq)]
pub enum CalendarViewMode {
    Year,
    Month,
    Week,
    Day,
}

impl Default for CalendarViewMode {
    fn default() -> Self {
        Self::Month
    }
}

#[derive(Debug, Clone)]
pub struct CalendarState {
    pub event_map: BTreeMap<NaiveDate, Vec<Event>>,
    pub all_events: Vec<Event>,
    pub selected_date: DateTime<Local>,
    pub view_mode: CalendarViewMode,
    pub selected_event: Option<Uuid>,
    pub show_event_dialog: bool,
    pub new_event: Option<Event>,
    pub selected_time_slot: Option<(NaiveDate, NaiveTime)>,
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            event_map: BTreeMap::new(),
            all_events: Vec::new(),
            selected_date: Local::now(),
            view_mode: CalendarViewMode::default(),
            selected_event: None,
            show_event_dialog: false,
            new_event: None,
            selected_time_slot: None,
        }
    }
}
