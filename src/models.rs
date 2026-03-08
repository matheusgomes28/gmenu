use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Title {
    // Name and title for the menu
    pub name: String,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct MenuItem {
    // Unique name for this item
    pub name: String,
    // Text that appears on the menu
    pub text: String,
    // Command to execute when this item is selected
    pub command: String,
    // Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Deserialize, Default)]
pub struct MenuConfig {
    // Title section
    pub title: Title,
    // Available items displayed in order
    pub items: Vec<MenuItem>,
}
