use crate::models::circle::Circle;
use crate::services::automation_service::AutomationRule;
use crate::services::automation_service::NotificationAction;
use crate::services::automation_service::RuleAction;
use crate::services::automation_service::RuleTrigger;
use crate::services::automation_service::ScheduleTrigger;
use crate::utils::naming::CircleName;
use uuid::Uuid;

/// Create a personal circle for a user
/// This function is used to avoid borrow checker issues in the main app
pub fn create_personal_circle(user_id: Uuid, user_name: &str) -> (Circle, AutomationRule) {
    // Create a new circle ID
    let circle_id = Uuid::new_v4();
    let circle_name = format!("{}.personal", user_name);

    // Create the circle name
    let circle_name_obj = CircleName::new(&circle_name).unwrap_or_else(|_| {
        // Fallback if the name is invalid
        CircleName::new("default.personal").unwrap()
    });

    // Create the circle
    let mut circle = Circle::new(
        circle_name_obj.to_string(),
        crate::models::circle::CircleType::Personal,
        user_id,
    );
    circle.id = circle_id;

    // Create a rule for the circle
    let rule = AutomationRule {
        id: Uuid::new_v4(),
        name: "Daily Summary".to_string(),
        description: "Creates a daily summary of circle activity".to_string(),
        trigger: RuleTrigger::Schedule(ScheduleTrigger {
            cron: "0 0 * * *".to_string(), // Daily at midnight
            next_execution: chrono::Utc::now() + chrono::Duration::days(1),
        }),
        actions: vec![RuleAction::SendNotification(NotificationAction {
            title: "Daily Summary".to_string(),
            message: "Here's your daily summary of circle activity".to_string(),
            recipients: vec![],
        })],
        enabled: true,
        circle_id,
        created_by: user_id,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_executed: None,
    };

    (circle, rule)
}
