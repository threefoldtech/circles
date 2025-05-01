use crate::utils::config::Theme;
use eframe::egui::{Button, CursorIcon, RichText, Ui, Vec2};

/// Represents a single menu item in the context menu
pub struct MenuItem {
    /// The label text to display
    pub label: String,
    /// Optional icon to display before the label (emoji or text)
    pub icon: Option<String>,
    /// Whether this is a destructive action (will be styled differently)
    pub is_destructive: bool,
    /// Optional ID for this menu item (useful for identifying which item was clicked)
    #[allow(dead_code)]
    pub id: Option<String>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            is_destructive: false,
            id: None,
        }
    }

    /// Add an icon to the menu item
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark this menu item as destructive (will be styled in error color)
    pub fn destructive(mut self) -> Self {
        self.is_destructive = true;
        self
    }

    /// Add an ID to the menu item
    #[allow(dead_code)]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Get the full label with icon if present
    fn display_text(&self) -> String {
        if let Some(icon) = &self.icon {
            format!("{} {}", icon, self.label)
        } else {
            self.label.clone()
        }
    }
}

/// Configuration for the context menu
pub struct ContextMenuConfig<'a> {
    /// The theme to use for styling
    pub theme: &'a Theme,
    /// The minimum width of the menu
    pub min_width: f32,
    /// Whether to show separators between menu items
    pub show_separators: bool,
    /// The corner radius of the menu
    pub corner_radius: f32,
    /// The padding around menu items
    pub item_padding: Vec2,
}

// We don't implement Default for ContextMenuConfig since it requires a theme reference

impl<'a> ContextMenuConfig<'a> {
    /// Create a new context menu configuration with the given theme
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            min_width: 200.0,
            show_separators: true,
            corner_radius: 8.0,
            item_padding: Vec2::new(10.0, 10.0),
        }
    }

    /// Set the minimum width of the menu
    pub fn with_min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Set whether to show separators between menu items
    #[allow(dead_code)]
    pub fn with_separators(mut self, show_separators: bool) -> Self {
        self.show_separators = show_separators;
        self
    }

    /// Set the corner radius of the menu
    pub fn with_corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    /// Set the padding around menu items
    #[allow(dead_code)]
    pub fn with_item_padding(mut self, padding: Vec2) -> Self {
        self.item_padding = padding;
        self
    }
}

/// Render a context menu directly within the UI
///
/// Returns a vector of indices of clicked menu items (usually empty or with one item)
pub fn render_context_menu(
    ui: &mut Ui,
    items: &[MenuItem],
    config: &ContextMenuConfig,
) -> Vec<usize> {
    // Apply styling
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = config.theme.hover;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = config.theme.hover;
    ui.style_mut().spacing.indent = 16.0;
    ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 4.0);
    ui.style_mut().spacing.button_padding = config.item_padding;
    ui.set_min_width(config.min_width);

    let mut clicked_indices = Vec::new();

    for (i, item) in items.iter().enumerate() {
        let button = Button::new(RichText::new(item.display_text()).size(14.0).color(
            if item.is_destructive {
                config.theme.error
            } else {
                config.theme.text
            },
        ))
        .min_size(Vec2::new(config.min_width - 20.0, 32.0));

        let button_response = ui.add(button).on_hover_cursor(CursorIcon::PointingHand);

        if button_response.clicked() {
            clicked_indices.push(i);
            ui.memory_mut(|mem| mem.close_popup());
        }

        if config.show_separators && i < items.len() - 1 {
            ui.separator();
        }
    }

    clicked_indices
}
