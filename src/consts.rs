pub const EXPORT_PATH_PROMPT: &str = "Export path: ";
pub const TITLE_BAR: &str = "1Password Export Tool";
pub const ARROW: &str = "--> ";
pub const CONTROLS: &str = r#"
-----------------------------------------*
Esc: Quit                                |
Enter: Save export file to path and quit |
Up / Down: Navigate through export data  |
Left / Right: Navigate through path      |
Space: Toggle export data entry          |
-----------------------------------------*
"#;
pub const LINES_PER_SECTION: usize = 10;
pub const INVALID_ARGUMENTS_MESSAGE: &str = "Invalid number of arguments. Provide no arguments for interactive menu, or 1 argument for direct exporting to json.";
