use eframe::egui;

use crate::app::CircleApp;
use crate::models::features::documents::{CreateType, ViewMode};
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::{button, context_menu};
use crate::utils::config::Theme;

/// Render the documents feature
pub fn render_documents(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    // Toolbar with actions
    let toolbar_frame = egui::Frame::new()
        .fill(theme.background)
        .inner_margin(8.0)
        .outer_margin(0.0)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(8.0);

    toolbar_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            // New button with dropdown
            let new_btn = ui.add(button::create_button("New", "➕", theme));
            if new_btn.clicked() {
                app.documents_state.create_dialog.open = true;
                app.documents_state.create_dialog.create_type = Some(CreateType::Document);
                app.documents_state.create_dialog.name = "Untitled Document".to_string();
            }

            ui.add_space(8.0);

            // Upload button
            if ui
                .add(button::create_button("Upload", "📤", theme))
                .clicked()
            {
                // TODO: Implement upload
            }

            ui.add_space(8.0);

            // Sort button
            if ui.add(button::create_button("Sort", "🔄", theme)).clicked() {
                // TODO: Implement sorting
            }

            // View toggle buttons
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Style the buttons
                ui.style_mut().visuals.widgets.inactive.bg_fill = theme.secondary_background;
                ui.style_mut().visuals.widgets.active.bg_fill = theme.accent;
                ui.style_mut().visuals.widgets.hovered.bg_fill = theme.hover;

                let list_text = egui::RichText::new("List View").color(theme.white);

                let list_btn = ui.add(
                    egui::Button::new(list_text)
                        .selected(app.documents_state.view_mode == ViewMode::List),
                );

                if list_btn.clicked() {
                    app.documents_state.view_mode = ViewMode::List;
                }

                ui.add_space(4.0);

                let grid_text = egui::RichText::new("Grid View").color(theme.white);

                let grid_btn = ui.add(
                    egui::Button::new(grid_text)
                        .selected(app.documents_state.view_mode == ViewMode::Grid),
                );

                if grid_btn.clicked() {
                    app.documents_state.view_mode = ViewMode::Grid;
                }

                ui.add_space(8.0);
                ui.label("View: ");
            });
        });
    });

    ui.add_space(16.0);

    // Main content area
    create_content_frame(theme).show(ui, |ui| {
        // Breadcrumbs navigation
        let mut navigate_to_root = false;
        let mut navigate_to_breadcrumb = None;

        let breadcrumbs_frame = egui::Frame::new()
            .fill(theme.secondary_background)
            .inner_margin(8.0)
            .outer_margin(0.0)
            .corner_radius(4.0);

        breadcrumbs_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Home button with icon
                let home_text = egui::RichText::new("📁 Documents")
                    .color(theme.accent)
                    .size(16.0);
                if ui.add(egui::Button::new(home_text).frame(false)).clicked() {
                    navigate_to_root = true;
                }

                // Clone the breadcrumbs to avoid borrowing issues
                let breadcrumbs: Vec<_> =
                    app.documents_state.current_path.iter().cloned().collect();

                // Show breadcrumb trail
                for (i, breadcrumb) in breadcrumbs.iter().enumerate() {
                    ui.label(egui::RichText::new(" > ").color(theme.secondary_text));

                    let crumb_text = egui::RichText::new(&breadcrumb.name).color(theme.accent);
                    if ui.add(egui::Button::new(crumb_text).frame(false)).clicked() {
                        navigate_to_breadcrumb = Some((breadcrumb.id, i));
                    }
                }
            });
        });

        // Handle navigation after UI rendering to avoid borrowing issues
        if navigate_to_root {
            app.documents_state.current_folder_id = None;
            app.documents_state.current_path.clear();
        } else if let Some((id, index)) = navigate_to_breadcrumb {
            app.documents_state.current_folder_id = Some(id);
            let breadcrumbs: Vec<_> = app.documents_state.current_path.iter().cloned().collect();
            app.documents_state.current_path = breadcrumbs[0..=index].to_vec();
        }

        ui.add_space(16.0);

        // Enhanced search bar
        let search_frame = egui::Frame::new()
            .fill(theme.secondary_background)
            .inner_margin(8.0)
            .outer_margin(0.0)
            .corner_radius(20.0);

        search_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(16.0));
                ui.add_space(4.0);

                let search_response = ui.add(
                    egui::TextEdit::singleline(&mut app.documents_state.search_query)
                        .hint_text("Search documents and folders...")
                        .desired_width(ui.available_width() - 30.0),
                );

                if search_response.changed() {
                    // Filter items based on search query
                }

                if !app.documents_state.search_query.is_empty() {
                    if ui.button("✖").clicked() {
                        app.documents_state.search_query.clear();
                    }
                }
            });
        });

        ui.add_space(16.0);

        // Document and folder listing
        if let Some(active_data) = &app.active_feature_data {
            let documents = &active_data.document_data.documents;
            let folders = &active_data.document_data.folders;

            // Filter items based on current folder and search query
            let current_folder_id = app.documents_state.current_folder_id;
            let search_query = &app.documents_state.search_query.to_lowercase();

            let filtered_folders: Vec<_> = folders
                .iter()
                .filter(|folder| {
                    search_query.is_empty() || folder.name.to_lowercase().contains(search_query)
                })
                .collect();

            let filtered_documents: Vec<_> = documents
                .iter()
                .filter(|doc| {
                    let folder_match = match current_folder_id {
                        Some(folder_id) => doc.folder_id == Some(folder_id),
                        None => doc.folder_id.is_none(),
                    };

                    folder_match
                        && (search_query.is_empty()
                            || doc.name.to_lowercase().contains(search_query)
                            || doc.content.to_lowercase().contains(search_query))
                })
                .collect();

            // Show message if no items found
            if filtered_folders.is_empty() && filtered_documents.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label("No items found");
                    ui.add_space(40.0);
                });
                return;
            }

            // Display items based on view mode
            if app.documents_state.view_mode == ViewMode::Grid {
                // Grid view
                let available_width = ui.available_width();
                let item_width = 120.0;
                let item_spacing = 16.0;
                let items_per_row = (available_width / (item_width + item_spacing))
                    .floor()
                    .max(1.0) as usize;

                egui::Grid::new("documents_grid")
                    .spacing([item_spacing, item_spacing])
                    .show(ui, |ui| {
                        let mut item_count = 0;
                        let mut folder_to_navigate = None;

                        // Render folders first
                        for folder in &filtered_folders {
                            let folder_id = folder.id;
                            // Clone folder name once and reuse it
                            let folder_name = folder.name.clone();
                            let folder_name_for_context = folder.name.clone();

                            // Create a frame for the folder item
                            let item_frame = egui::Frame::new()
                                .fill(theme.secondary_background)
                                .stroke(egui::Stroke::new(1.0, theme.border))
                                .corner_radius(8.0)
                                .inner_margin(8.0);

                            let response = item_frame.show(ui, |ui| {
                                ui.set_width(item_width);
                                ui.vertical_centered(|ui| {
                                    // Folder icon
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new("📁").size(32.0));
                                    ui.add_space(4.0);

                                    // Folder name
                                    let folder_text =
                                        egui::RichText::new(&folder_name).strong().size(14.0);

                                    if ui
                                        .add(egui::Button::new(folder_text).frame(false))
                                        .clicked()
                                    {
                                        // Store folder to navigate to
                                        folder_to_navigate = Some((folder_id, folder_name.clone()));
                                    }

                                    ui.add_space(4.0);
                                });
                            }).response;
                            
                            // Add context menu
                            response.context_menu(|ui| {
                                let menu_items = vec![
                                    crate::ui::components::context_menu::MenuItem::new("Open").with_icon("📁"),
                                    crate::ui::components::context_menu::MenuItem::new("Rename").with_icon("✏️"),
                                    crate::ui::components::context_menu::MenuItem::new("Move to").with_icon("📦"),
                                    crate::ui::components::context_menu::MenuItem::new("Share").with_icon("👥"),
                                    crate::ui::components::context_menu::MenuItem::new("Delete").with_icon("🗑️").destructive(),
                                ];
                                
                                let config = crate::ui::components::context_menu::ContextMenuConfig::new(theme)
                                    .with_min_width(180.0);
                                
                                let clicked_indices = crate::ui::components::context_menu::render_context_menu(ui, &menu_items, &config);
                                
                                for &index in &clicked_indices {
                                    match index {
                                        0 => { // Open
                                            folder_to_navigate = Some((folder_id, folder_name.clone()));
                                        },
                                        1 => { // Rename
                                            app.documents_state.rename_dialog.open = true;
                                            app.documents_state.rename_dialog.item = Some(
                                                crate::models::features::documents::SelectedItem::Folder(folder_id)
                                            );
                                            app.documents_state.rename_dialog.current_name = folder_name_for_context.clone();
                                            app.documents_state.rename_dialog.new_name = folder_name_for_context.clone();
                                        },
                                        2 => { // Move to
                                            // TODO: Implement move to
                                        },
                                        3 => { // Share
                                            // TODO: Implement share
                                        },
                                        4 => { // Delete
                                            app.documents_state.delete_dialog.open = true;
                                            app.documents_state.delete_dialog.item = Some(
                                                crate::models::features::documents::SelectedItem::Folder(folder_id)
                                            );
                                            app.documents_state.delete_dialog.item_name = folder_name_for_context.clone();
                                        },
                                        _ => {}
                                    }
                                }
                            });

                            item_count += 1;
                            if item_count % items_per_row == 0 {
                                ui.end_row();
                            }
                        }

                        // Handle folder navigation after the loop
                        if let Some((id, name)) = folder_to_navigate {
                            app.documents_state.current_folder_id = Some(id);
                            app.documents_state.current_path.push(
                                crate::models::features::documents::FolderBreadcrumb { id, name },
                            );
                        }

                        // Render documents
                        for doc in &filtered_documents {
                            // Create a frame for the document item
                            let item_frame = egui::Frame::new()
                                .fill(theme.secondary_background)
                                .stroke(egui::Stroke::new(1.0, theme.border))
                                .corner_radius(8.0)
                                .inner_margin(8.0);

                            let response = item_frame.show(ui, |ui| {
                                ui.set_width(item_width);
                                ui.vertical_centered(|ui| {
                                    // Document icon
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new("📄").size(32.0));
                                    ui.add_space(4.0);

                                    // Document name
                                    let doc_text =
                                        egui::RichText::new(&doc.name).strong().size(14.0);

                                    if ui.add(egui::Button::new(doc_text).frame(false)).clicked() {
                                        // TODO: Open document
                                    }

                                    // Last edited
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Last edited: {}",
                                            doc.updated_at.format("%b %d")
                                        ))
                                        .size(12.0)
                                        .color(theme.secondary_text),
                                    );

                                    // Owner
                                    ui.label(
                                        egui::RichText::new(&doc.created_by)
                                            .size(12.0)
                                            .color(theme.secondary_text),
                                    );

                                    ui.add_space(4.0);
                                });
                            }).response;
                            
                            // Add context menu
                            response.context_menu(|ui| {
                                let menu_items = vec![
                                    crate::ui::components::context_menu::MenuItem::new("Open").with_icon("📄"),
                                    crate::ui::components::context_menu::MenuItem::new("Rename").with_icon("✏️"),
                                    crate::ui::components::context_menu::MenuItem::new("Move to").with_icon("📦"),
                                    crate::ui::components::context_menu::MenuItem::new("Share").with_icon("👥"),
                                    crate::ui::components::context_menu::MenuItem::new("Delete").with_icon("🗑️").destructive(),
                                ];
                                
                                let config = crate::ui::components::context_menu::ContextMenuConfig::new(theme)
                                    .with_min_width(180.0);
                                
                                let clicked_indices = crate::ui::components::context_menu::render_context_menu(ui, &menu_items, &config);
                                
                                for &index in &clicked_indices {
                                    match index {
                                        0 => { // Open
                                            // TODO: Open document
                                        },
                                        1 => { // Rename
                                            app.documents_state.rename_dialog.open = true;
                                            app.documents_state.rename_dialog.item = Some(
                                                crate::models::features::documents::SelectedItem::Document(doc.id)
                                            );
                                            app.documents_state.rename_dialog.current_name = doc.name.clone();
                                            app.documents_state.rename_dialog.new_name = doc.name.clone();
                                        },
                                        2 => { // Move to
                                            // TODO: Implement move to
                                        },
                                        3 => { // Share
                                            // TODO: Implement share
                                        },
                                        4 => { // Delete
                                            app.documents_state.delete_dialog.open = true;
                                            app.documents_state.delete_dialog.item = Some(
                                                crate::models::features::documents::SelectedItem::Document(doc.id)
                                            );
                                            app.documents_state.delete_dialog.item_name = doc.name.clone();
                                        },
                                        _ => {}
                                    }
                                }
                            });

                            item_count += 1;
                            if item_count % items_per_row == 0 {
                                ui.end_row();
                            }
                        }
                    });
            } else {
                // List view with enhanced styling
                let header_frame = egui::Frame::new()
                    .fill(theme.secondary_background)
                    .inner_margin(8.0)
                    .outer_margin(0.0)
                    .corner_radius(4.0);

                header_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Name").strong().size(14.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("Owner").strong().size(14.0));
                            ui.add_space(16.0);
                            ui.label(egui::RichText::new("Last Modified").strong().size(14.0));
                        });
                    });
                });

                ui.add_space(8.0);

                // Render folders first
                let mut folder_to_navigate = None;

                for folder in &filtered_folders {
                    let folder_id = folder.id;
                    let folder_name = folder.name.clone();

                    let row_frame = egui::Frame::new()
                        .fill(theme.transparent)
                        .inner_margin(8.0)
                        .outer_margin(0.0)
                        .corner_radius(4.0);

                    let row_response = row_frame
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new("📁").size(16.0));
                                ui.add_space(4.0);

                                let name_text = egui::RichText::new(&folder_name).size(14.0);
                                if ui.add(egui::Button::new(name_text).frame(false)).clicked() {
                                    // Store folder to navigate to
                                    folder_to_navigate = Some((folder_id, folder_name));
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new("-").color(theme.secondary_text),
                                        );
                                        ui.add_space(16.0);
                                        ui.label(
                                            egui::RichText::new("-").color(theme.secondary_text),
                                        );
                                    },
                                );
                            });
                        })
                        .response;

                    // Highlight on hover
                    if row_response.hovered() {
                        ui.painter()
                            .rect_filled(row_response.rect, 0.0, theme.hover);
                    }
                }

                // Handle folder navigation after the loop
                if let Some((id, name)) = folder_to_navigate {
                    app.documents_state.current_folder_id = Some(id);
                    app.documents_state
                        .current_path
                        .push(crate::models::features::documents::FolderBreadcrumb { id, name });
                }

                // Render documents
                for doc in &filtered_documents {
                    let row_frame = egui::Frame::new()
                        .fill(theme.transparent)
                        .inner_margin(8.0)
                        .outer_margin(0.0)
                        .corner_radius(4.0);

                    let row_response = row_frame
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new("📄").size(16.0));
                                ui.add_space(4.0);

                                let name_text = egui::RichText::new(&doc.name).size(14.0);
                                if ui.add(egui::Button::new(name_text).frame(false)).clicked() {
                                    // TODO: Open document
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new(&doc.created_by)
                                                .color(theme.secondary_text),
                                        );
                                        ui.add_space(16.0);
                                        ui.label(
                                            egui::RichText::new(
                                                doc.updated_at.format("%b %d, %Y").to_string(),
                                            )
                                            .color(theme.secondary_text),
                                        );
                                    },
                                );
                            });
                        })
                        .response;

                    // Highlight on hover
                    if row_response.hovered() {
                        ui.painter()
                            .rect_filled(row_response.rect, 0.0, theme.hover);
                    }
                }
            }
        } else {
            ui.label("No documents available");
        }
    });
}

/// Render rename dialog
fn render_rename_dialog(app: &mut CircleApp, ctx: &egui::Context, theme: &Theme) {
    if !app.documents_state.rename_dialog.open {
        return;
    }
    
    let mut result = None;
    
    egui::Window::new("Rename")
        .fixed_size([400.0, 150.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Rename Item");
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("New name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut app.documents_state.rename_dialog.new_name)
                            .desired_width(250.0)
                    );
                });
                
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        result = Some(false);
                    }
                    
                    if ui.button("Rename").clicked() {
                        result = Some(true);
                    }
                });
            });
        });
    
    // Handle the result
    if let Some(confirmed) = result {
        if confirmed {
            // Implement rename logic here
            if let Some(item) = &app.documents_state.rename_dialog.item {
                let new_name = app.documents_state.rename_dialog.new_name.clone();
                
                if let Some(active_data) = &mut app.active_feature_data {
                    match item {
                        crate::models::features::documents::SelectedItem::Folder(id) => {
                            if let Some(folder) = active_data.document_data.folders.iter_mut().find(|f| f.id == *id) {
                                folder.name = new_name;
                            }
                        }
                        crate::models::features::documents::SelectedItem::Document(id) => {
                            if let Some(doc) = active_data.document_data.documents.iter_mut().find(|d| d.id == *id) {
                                doc.name = new_name;
                            }
                        }
                    }
                }
            }
        }
        
        // Reset the dialog state
        app.documents_state.rename_dialog = crate::models::features::documents::RenameDialogState::default();
    }
}

/// Render delete confirmation dialog
fn render_delete_dialog(app: &mut CircleApp, ctx: &egui::Context, theme: &Theme) {
    if !app.documents_state.delete_dialog.open {
        return;
    }
    
    let item_name = app.documents_state.delete_dialog.item_name.clone();
    let body_text = format!(
        "Are you sure you want to delete \"{}\"?\nThis action cannot be undone.",
        item_name
    );
    
    let mut result = None;
    
    egui::Window::new("Delete Item?")
        .fixed_size([400.0, 200.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("Delete Item?").color(theme.error));
                ui.add_space(10.0);
                ui.label(body_text);
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        result = Some(false);
                    }
                    
                    if ui.button("Delete").clicked() {
                        result = Some(true);
                    }
                });
            });
        });
    
    // Handle the result
    if let Some(confirmed) = result {
        if confirmed {
            // Implement delete logic here
            if let Some(item) = &app.documents_state.delete_dialog.item {
                if let Some(active_data) = &mut app.active_feature_data {
                    match item {
                        crate::models::features::documents::SelectedItem::Folder(id) => {
                            active_data.document_data.folders.retain(|f| f.id != *id);
                            // Also move any documents in this folder to root
                            for doc in &mut active_data.document_data.documents {
                                if doc.folder_id == Some(*id) {
                                    doc.folder_id = None;
                                }
                            }
                        }
                        crate::models::features::documents::SelectedItem::Document(id) => {
                            active_data.document_data.documents.retain(|d| d.id != *id);
                        }
                    }
                }
            }
        }
        
        // Reset the dialog state
        app.documents_state.delete_dialog = crate::models::features::documents::DeleteDialogState::default();
    }
}

/// Render create dialog for new document or folder
fn render_create_dialog(app: &mut CircleApp, ctx: &egui::Context, theme: &Theme) {
    if !app.documents_state.create_dialog.open {
        return;
    }
    
    let create_type = match app.documents_state.create_dialog.create_type {
        Some(CreateType::Document) => "Document",
        Some(CreateType::Folder) => "Folder",
        None => return,
    };
    
    let mut result = None;
    
    egui::Window::new(format!("Create New {}", create_type))
        .fixed_size([400.0, 150.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(format!("Create New {}", create_type));
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut app.documents_state.create_dialog.name)
                            .desired_width(250.0)
                    );
                });
                
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        result = Some(false);
                    }
                    
                    if ui.button("Create").clicked() {
                        result = Some(true);
                    }
                });
            });
        });
    
    // Handle the result
    if let Some(confirmed) = result {
        if confirmed {
            // Implement create logic here
            let name = app.documents_state.create_dialog.name.clone();
            let create_type = app.documents_state.create_dialog.create_type.unwrap();
            
            if let Some(active_data) = &mut app.active_feature_data {
                match create_type {
                    CreateType::Folder => {
                        // Create a new folder
                        let folder = crate::models::dummy_data::DocFolder {
                            id: uuid::Uuid::new_v4(),
                            name,
                        };
                        active_data.document_data.folders.push(folder);
                    }
                    CreateType::Document => {
                        // Create a new document
                        let document = crate::models::dummy_data::Document {
                            id: uuid::Uuid::new_v4(),
                            name,
                            content: String::new(),
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                            created_by: "Me".to_string(),
                            folder_id: app.documents_state.current_folder_id,
                        };
                        active_data.document_data.documents.push(document);
                    }
                }
            }
        }
        
        // Reset the dialog state
        app.documents_state.create_dialog = crate::models::features::documents::CreateDialogState::default();
    }
}
