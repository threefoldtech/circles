use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Dummy mail data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailData {
    pub emails: Vec<Email>,
    pub folders: Vec<Folder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: Uuid,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
    pub folder_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub unread_count: usize,
}

/// Dummy calendar data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarData {
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
}

/// Dummy chat data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatData {
    pub conversations: Vec<Conversation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub name: String,
    pub participants: Vec<String>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
}

/// Dummy document data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    pub documents: Vec<Document>,
    pub folders: Vec<DocFolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub folder_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocFolder {
    pub id: Uuid,
    pub name: String,
}

/// Dummy video conference data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfData {
    pub meetings: Vec<Meeting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: Uuid,
    pub title: String,
    pub scheduled_time: DateTime<Utc>,
    pub duration_minutes: u32,
    pub participants: Vec<String>,
    pub meeting_link: String,
}

/// Dummy AI tools data for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIToolsData {
    pub tools: Vec<AITool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITool {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub usage_count: u32,
}

/// Dummy data for all features in a circle
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CircleFeatureData {
    pub mail_data: MailData,
    pub calendar_data: CalendarData,
    pub chat_data: ChatData,
    pub document_data: DocumentData,
    pub video_conf_data: VideoConfData,
    pub ai_tools_data: AIToolsData,
}

/// Generate dummy data for a circle based on its type and name
pub fn generate_dummy_data_for_circle(
    circle_id: Uuid,
    circle_name: &str,
    circle_type: crate::models::circle::CircleType,
) -> CircleFeatureData {
    // Special handling for default circles
    if circle_name == "Welcome to Circles" {
        return generate_welcome_circle_data(circle_id, circle_name);
    } else if circle_name == "Circles Bot Channel" {
        return generate_bot_circle_data(circle_id, circle_name);
    }

    // Regular handling based on circle type
    match circle_type {
        crate::models::circle::CircleType::Personal => {
            generate_personal_circle_data(circle_id, circle_name)
        }
        crate::models::circle::CircleType::Team => {
            generate_team_circle_data(circle_id, circle_name)
        }
        crate::models::circle::CircleType::Private => {
            generate_private_circle_data(circle_id, circle_name)
        }
    }
}

/// Generate data for the welcome circle with instructions
fn generate_welcome_circle_data(circle_id: Uuid, circle_name: &str) -> CircleFeatureData {
    // Start with basic private circle data
    let mut data = generate_private_circle_data(circle_id, circle_name);

    // Create welcome document
    let welcome_doc = Document {
        id: Uuid::new_v4(),
        name: "welcome_guide.md".to_string(),
        content: "# Welcome to Circles\n\n\
                 ## Getting Started\n\n\
                 Circles is a collaboration platform that helps you organize your work and personal life.\n\n\
                 ### Key Features:\n\n\
                 - **Mail**: Send and receive messages within your circles\n\
                 - **Calendar**: Schedule events and meetings\n\
                 - **Chat**: Real-time communication with circle members\n\
                 - **Documents**: Create and share documents\n\
                 - **Video Conference**: Hold virtual meetings\n\
                 - **AI Tools**: Use AI to enhance your productivity\n\n\
                 ### Creating Circles\n\n\
                 Click the + button in the Circles panel to create a new circle.\n\
                 You can create different types of circles:\n\n\
                 - **Personal**: For individual use or one-on-one collaboration\n\
                 - **Team**: For group collaboration\n\
                 - **Private**: For your personal content\n\n\
                 Enjoy using Circles!".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: "Circles System".to_string(),
        folder_id: None,
    };

    // Add welcome document to documents
    data.document_data.documents.push(welcome_doc);

    data
}

/// Generate data for the bot circle with system messages
fn generate_bot_circle_data(circle_id: Uuid, circle_name: &str) -> CircleFeatureData {
    // Start with basic private circle data
    let mut data = generate_private_circle_data(circle_id, circle_name);

    // Create bot conversation
    let bot_conversation = Conversation {
        id: Uuid::new_v4(),
        name: "Circles Bot".to_string(),
        participants: vec!["Circles Bot".to_string(), "Me".to_string()],
        messages: vec![
            Message {
                id: Uuid::new_v4(),
                sender: "Circles Bot".to_string(),
                content: "Welcome to the Circles Bot Channel! You'll receive system updates and announcements here.".to_string(),
                timestamp: Utc::now(),
                read: true,
            },
            Message {
                id: Uuid::new_v4(),
                sender: "Circles Bot".to_string(),
                content: "🔔 System Update: Circles v1.0 has been released with new collaboration features!".to_string(),
                timestamp: Utc::now().checked_add_signed(chrono::Duration::minutes(5)).unwrap(),
                read: true,
            },
        ],
    };

    // Replace existing conversations with our bot conversation
    data.chat_data.conversations = vec![bot_conversation];

    data
}

fn generate_personal_circle_data(_: Uuid, _: &str) -> CircleFeatureData {
    // Generate inbox folder
    let inbox_id = Uuid::new_v4();
    let sent_id = Uuid::new_v4();
    let drafts_id = Uuid::new_v4();
    let trash_id = Uuid::new_v4();

    // Generate emails
    let emails = vec![
        Email {
            id: Uuid::new_v4(),
            sender: "friend@example.com".to_string(),
            recipients: vec!["me@example.com".to_string()],
            subject: "Coffee this weekend?".to_string(),
            content: "Hey, would you like to grab coffee this weekend? Let me know!".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 14, 30, 0).unwrap(),
            read: false,
            folder_id: inbox_id,
        },
        Email {
            id: Uuid::new_v4(),
            sender: "family@example.com".to_string(),
            recipients: vec!["me@example.com".to_string()],
            subject: "Family dinner next week".to_string(),
            content: "Don't forget we have family dinner next Tuesday at 7pm!".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 8, 10, 15, 0).unwrap(),
            read: true,
            folder_id: inbox_id,
        },
        Email {
            id: Uuid::new_v4(),
            sender: "me@example.com".to_string(),
            recipients: vec!["colleague@example.com".to_string()],
            subject: "Personal project ideas".to_string(),
            content: "I've been thinking about some personal project ideas. Let's discuss!"
                .to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 7, 9, 0, 0).unwrap(),
            read: true,
            folder_id: sent_id,
        },
    ];

    // Generate folders
    let folders = vec![
        Folder {
            id: inbox_id,
            name: "Inbox".to_string(),
            unread_count: 1,
        },
        Folder {
            id: sent_id,
            name: "Sent".to_string(),
            unread_count: 0,
        },
        Folder {
            id: drafts_id,
            name: "Drafts".to_string(),
            unread_count: 0,
        },
        Folder {
            id: trash_id,
            name: "Trash".to_string(),
            unread_count: 0,
        },
    ];

    // Generate calendar events
    let events = vec![
        Event {
            id: Uuid::new_v4(),
            title: "Coffee with Friend".to_string(),
            description: "Meeting at the local coffee shop".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 13, 10, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 13, 11, 30, 0).unwrap(),
            location: Some("Local Coffee Shop".to_string()),
            attendees: vec!["Friend".to_string()],
        },
        Event {
            id: Uuid::new_v4(),
            title: "Family Dinner".to_string(),
            description: "Weekly family dinner".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 14, 19, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 14, 21, 0, 0).unwrap(),
            location: Some("Parents' House".to_string()),
            attendees: vec!["Mom".to_string(), "Dad".to_string(), "Sister".to_string()],
        },
    ];

    // Generate chat conversations
    let conversations = vec![Conversation {
        id: Uuid::new_v4(),
        name: "Best Friend".to_string(),
        participants: vec!["Me".to_string(), "Best Friend".to_string()],
        messages: vec![
            Message {
                id: Uuid::new_v4(),
                sender: "Best Friend".to_string(),
                content: "Hey, how's it going?".to_string(),
                timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 9, 30, 0).unwrap(),
                read: true,
            },
            Message {
                id: Uuid::new_v4(),
                sender: "Me".to_string(),
                content: "Not bad! Just working on some personal projects.".to_string(),
                timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 9, 35, 0).unwrap(),
                read: true,
            },
            Message {
                id: Uuid::new_v4(),
                sender: "Best Friend".to_string(),
                content: "Cool! Want to hang out this weekend?".to_string(),
                timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 9, 40, 0).unwrap(),
                read: false,
            },
        ],
    }];

    // Generate documents
    let personal_folder_id = Uuid::new_v4();
    let documents = vec![
        Document {
            id: Uuid::new_v4(),
            name: "Personal Goals.txt".to_string(),
            content: "1. Learn a new language\n2. Travel to Japan\n3. Run a marathon".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 3, 15, 14, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 4, 1, 10, 30, 0).unwrap(),
            created_by: "Me".to_string(),
            folder_id: Some(personal_folder_id),
        },
        Document {
            id: Uuid::new_v4(),
            name: "Shopping List.txt".to_string(),
            content: "- Milk\n- Eggs\n- Bread\n- Apples\n- Coffee".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 4, 5, 9, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 4, 5, 9, 0, 0).unwrap(),
            created_by: "Me".to_string(),
            folder_id: None,
        },
    ];

    let doc_folders = vec![DocFolder {
        id: personal_folder_id,
        name: "Personal".to_string(),
    }];

    // Generate meetings
    let meetings = vec![Meeting {
        id: Uuid::new_v4(),
        title: "Virtual Coffee with Friend".to_string(),
        scheduled_time: Utc.with_ymd_and_hms(2025, 4, 15, 10, 0, 0).unwrap(),
        duration_minutes: 60,
        participants: vec!["Me".to_string(), "Friend".to_string()],
        meeting_link: "https://meet.example.com/personal-123".to_string(),
    }];

    // Generate AI tools
    let ai_tools = vec![
        AITool {
            id: Uuid::new_v4(),
            name: "Personal Assistant".to_string(),
            description: "AI assistant for managing personal tasks and reminders".to_string(),
            usage_count: 15,
        },
        AITool {
            id: Uuid::new_v4(),
            name: "Writing Helper".to_string(),
            description: "AI tool to help with creative writing and journaling".to_string(),
            usage_count: 8,
        },
    ];

    CircleFeatureData {
        mail_data: MailData { emails, folders },
        calendar_data: CalendarData { events },
        chat_data: ChatData { conversations },
        document_data: DocumentData {
            documents,
            folders: doc_folders,
        },
        video_conf_data: VideoConfData { meetings },
        ai_tools_data: AIToolsData { tools: ai_tools },
    }
}

fn generate_team_circle_data(_: Uuid, _: &str) -> CircleFeatureData {
    // Generate inbox folder
    let inbox_id = Uuid::new_v4();
    let sent_id = Uuid::new_v4();
    let drafts_id = Uuid::new_v4();
    let trash_id = Uuid::new_v4();

    // Generate emails
    let emails = vec![
        Email {
            id: Uuid::new_v4(),
            sender: "manager@company.com".to_string(),
            recipients: vec!["team@company.com".to_string()],
            subject: "Team Meeting Tomorrow".to_string(),
            content: "Hi team, reminder that we have our weekly meeting tomorrow at 10am. Please prepare your updates.".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 15, 0, 0).unwrap(),
            read: false,
            folder_id: inbox_id,
        },
        Email {
            id: Uuid::new_v4(),
            sender: "colleague@company.com".to_string(),
            recipients: vec!["me@company.com".to_string()],
            subject: "Project Update".to_string(),
            content: "Here's the latest update on the project. We're making good progress!".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 8, 11, 30, 0).unwrap(),
            read: true,
            folder_id: inbox_id,
        },
    ];

    // Generate folders
    let folders = vec![
        Folder {
            id: inbox_id,
            name: "Inbox".to_string(),
            unread_count: 1,
        },
        Folder {
            id: sent_id,
            name: "Sent".to_string(),
            unread_count: 0,
        },
        Folder {
            id: drafts_id,
            name: "Drafts".to_string(),
            unread_count: 0,
        },
        Folder {
            id: trash_id,
            name: "Trash".to_string(),
            unread_count: 0,
        },
    ];

    // Generate calendar events
    let events = vec![
        Event {
            id: Uuid::new_v4(),
            title: "Weekly Team Meeting".to_string(),
            description: "Regular team sync-up meeting".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 10, 10, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 10, 11, 0, 0).unwrap(),
            location: Some("Conference Room A".to_string()),
            attendees: vec![
                "Manager".to_string(),
                "Colleague 1".to_string(),
                "Colleague 2".to_string(),
                "Me".to_string(),
            ],
        },
        Event {
            id: Uuid::new_v4(),
            title: "Project Deadline".to_string(),
            description: "Final submission for the current project phase".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 15, 17, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 15, 17, 0, 0).unwrap(),
            location: None,
            attendees: vec!["Team".to_string()],
        },
    ];

    // Generate chat conversations
    let conversations = vec![Conversation {
        id: Uuid::new_v4(),
        name: "Team Chat".to_string(),
        participants: vec![
            "Manager".to_string(),
            "Colleague 1".to_string(),
            "Colleague 2".to_string(),
            "Me".to_string(),
        ],
        messages: vec![
            Message {
                id: Uuid::new_v4(),
                sender: "Manager".to_string(),
                content: "Good morning team! How's everyone doing today?".to_string(),
                timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 9, 0, 0).unwrap(),
                read: true,
            },
            Message {
                id: Uuid::new_v4(),
                sender: "Colleague 1".to_string(),
                content: "Morning! Working on the frontend updates.".to_string(),
                timestamp: Utc.with_ymd_and_hms(2025, 4, 9, 9, 5, 0).unwrap(),
                read: true,
            },
        ],
    }];

    // Generate documents
    let project_folder_id = Uuid::new_v4();
    let documents = vec![
        Document {
            id: Uuid::new_v4(),
            name: "Project Plan.docx".to_string(),
            content: "# Project Plan\n\n## Objectives\n- Deliver feature X by end of Q2\n- Improve performance by 20%".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 3, 25, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 4, 2, 15, 30, 0).unwrap(),
            created_by: "Manager".to_string(),
            folder_id: Some(project_folder_id),
        },
    ];

    let doc_folders = vec![DocFolder {
        id: project_folder_id,
        name: "Project Files".to_string(),
    }];

    // Generate meetings
    let meetings = vec![Meeting {
        id: Uuid::new_v4(),
        title: "Weekly Team Meeting".to_string(),
        scheduled_time: Utc.with_ymd_and_hms(2025, 4, 10, 10, 0, 0).unwrap(),
        duration_minutes: 60,
        participants: vec![
            "Manager".to_string(),
            "Colleague 1".to_string(),
            "Colleague 2".to_string(),
            "Me".to_string(),
        ],
        meeting_link: "https://meet.example.com/team-123".to_string(),
    }];

    // Generate AI tools
    let ai_tools = vec![AITool {
        id: Uuid::new_v4(),
        name: "Code Assistant".to_string(),
        description: "AI tool for code generation and review".to_string(),
        usage_count: 42,
    }];

    CircleFeatureData {
        mail_data: MailData { emails, folders },
        calendar_data: CalendarData { events },
        chat_data: ChatData { conversations },
        document_data: DocumentData {
            documents,
            folders: doc_folders,
        },
        video_conf_data: VideoConfData { meetings },
        ai_tools_data: AIToolsData { tools: ai_tools },
    }
}

fn generate_private_circle_data(_: Uuid, _: &str) -> CircleFeatureData {
    // Generate inbox folder
    let inbox_id = Uuid::new_v4();
    let sent_id = Uuid::new_v4();
    let drafts_id = Uuid::new_v4();
    let trash_id = Uuid::new_v4();

    // Generate emails
    let emails = vec![
        Email {
            id: Uuid::new_v4(),
            sender: "me@example.com".to_string(),
            recipients: vec!["me@example.com".to_string()],
            subject: "Note to Self: Project Ideas".to_string(),
            content: "Here are some project ideas I want to work on:\n1. Personal website redesign\n2. Mobile app for habit tracking\n3. Smart home automation system".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 8, 20, 15, 0).unwrap(),
            read: true,
            folder_id: inbox_id,
        },
        Email {
            id: Uuid::new_v4(),
            sender: "me@example.com".to_string(),
            recipients: vec!["me@example.com".to_string()],
            subject: "Books to Read".to_string(),
            content: "Books I want to read:\n- The Pragmatic Programmer\n- Clean Code\n- Design Patterns\n- Refactoring".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 4, 5, 18, 30, 0).unwrap(),
            read: true,
            folder_id: inbox_id,
        },
    ];

    // Generate folders
    let folders = vec![
        Folder {
            id: inbox_id,
            name: "Inbox".to_string(),
            unread_count: 0,
        },
        Folder {
            id: sent_id,
            name: "Sent".to_string(),
            unread_count: 0,
        },
        Folder {
            id: drafts_id,
            name: "Drafts".to_string(),
            unread_count: 0,
        },
        Folder {
            id: trash_id,
            name: "Trash".to_string(),
            unread_count: 0,
        },
    ];

    // Generate calendar events
    let events = vec![
        Event {
            id: Uuid::new_v4(),
            title: "Gym Session".to_string(),
            description: "Workout at the gym".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 11, 18, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 11, 19, 30, 0).unwrap(),
            location: Some("Local Gym".to_string()),
            attendees: vec!["Me".to_string()],
        },
        Event {
            id: Uuid::new_v4(),
            title: "Study Time".to_string(),
            description: "Study new programming language".to_string(),
            start_time: Utc.with_ymd_and_hms(2025, 4, 12, 10, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 4, 12, 12, 0, 0).unwrap(),
            location: None,
            attendees: vec!["Me".to_string()],
        },
    ];

    // Generate chat conversations (empty for private circle)
    let conversations = vec![];

    // Generate documents
    let projects_folder_id = Uuid::new_v4();
    let journal_folder_id = Uuid::new_v4();

    let documents = vec![
        Document {
            id: Uuid::new_v4(),
            name: "Journal - April 9.txt".to_string(),
            content: "Today I made good progress on my personal project. I'm excited about the direction it's taking.".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 4, 9, 22, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 4, 9, 22, 0, 0).unwrap(),
            created_by: "Me".to_string(),
            folder_id: Some(journal_folder_id),
        },
        Document {
            id: Uuid::new_v4(),
            name: "Project Ideas.md".to_string(),
            content: "# Project Ideas\n\n## Website Redesign\n- Modern UI/UX\n- Portfolio section\n- Blog integration".to_string(),
            created_at: Utc.with_ymd_and_hms(2025, 4, 7, 19, 30, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 4, 8, 20, 15, 0).unwrap(),
            created_by: "Me".to_string(),
            folder_id: Some(projects_folder_id),
        },
    ];

    let doc_folders = vec![
        DocFolder {
            id: projects_folder_id,
            name: "Projects".to_string(),
        },
        DocFolder {
            id: journal_folder_id,
            name: "Journal".to_string(),
        },
    ];

    // Generate meetings (minimal for private circle)
    let meetings = vec![Meeting {
        id: Uuid::new_v4(),
        title: "Self-reflection Session".to_string(),
        scheduled_time: Utc.with_ymd_and_hms(2025, 4, 13, 9, 0, 0).unwrap(),
        duration_minutes: 30,
        participants: vec!["Me".to_string()],
        meeting_link: "https://meet.example.com/private-123".to_string(),
    }];

    // Generate AI tools
    let ai_tools = vec![
        AITool {
            id: Uuid::new_v4(),
            name: "Journal Assistant".to_string(),
            description: "AI tool for journaling and self-reflection".to_string(),
            usage_count: 25,
        },
        AITool {
            id: Uuid::new_v4(),
            name: "Learning Companion".to_string(),
            description: "AI tool to help with learning new skills".to_string(),
            usage_count: 18,
        },
    ];

    CircleFeatureData {
        mail_data: MailData { emails, folders },
        calendar_data: CalendarData { events },
        chat_data: ChatData { conversations },
        document_data: DocumentData {
            documents,
            folders: doc_folders,
        },
        video_conf_data: VideoConfData { meetings },
        ai_tools_data: AIToolsData { tools: ai_tools },
    }
}
