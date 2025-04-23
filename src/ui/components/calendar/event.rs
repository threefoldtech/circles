use chrono::{DateTime, Duration, Timelike, Utc};
use egui::Color32;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub color: Color32,
}

impl Event {
    pub fn duration_minutes(&self) -> i64 {
        (self.end_time - self.start_time).num_minutes()
    }

    pub fn overlaps_with(&self, other: &Event) -> bool {
        (self.start_time <= other.end_time) && (self.end_time >= other.start_time)
    }
}

impl Default for Event {
    fn default() -> Self {
        let now = Utc::now();
        let minutes = now.minute();
        let (rounded_minutes, hour_adjustment) = if minutes < 30 { (30, 0) } else { (0, 1) };
        let start_time = now
            .with_minute(rounded_minutes)
            .and_then(|t| t.with_second(0))
            .and_then(|t| t.with_nanosecond(0))
            .unwrap()
            + Duration::hours(hour_adjustment);

        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            start_time,
            end_time: start_time + Duration::hours(1),
            location: None,
            attendees: Vec::new(),
            color: Color32::from_rgb(66, 133, 244),
        }
    }
}
