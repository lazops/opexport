use std::{thread, time::Duration};

use rustea::{
    command::{batch, quit},
    crossterm::event::{KeyCode, KeyEvent},
    view_helper::input::Input,
    App, Command, Message,
};

use crate::{
    consts::*,
    export::ExportOptions,
    format::ux::{ExportData, ExportDataEntry, UXExporter},
    op::OPError,
};

struct BumpLoadingIndicatorMessage;

pub struct Model {
    export_data: Option<ExportData>,
    flattened_export_data: Option<Vec<ExportDataEntry>>,
    viewable_entries: Vec<ExportDataEntry>,
    error: Option<OPError>,
    loading_indicator_count: usize,
    export_path_input: Input,
    export_options: ExportOptions,
    current_selection: usize,
}

impl Model {
    pub fn new() -> Self {
        Self {
            export_data: None,
            flattened_export_data: None,
            viewable_entries: Vec::new(),
            error: None,
            loading_indicator_count: 1,
            export_path_input: Input::new(),
            export_options: ExportOptions::new(),
            current_selection: 0,
        }
    }

    pub fn export_data_entry_view_line(&self, line: String, i: usize) -> String {
        let entry = &self.flattened_export_data.as_ref().unwrap()[i];

        format!(
            "{}{} {}\n",
            if i == self.current_selection {
                ARROW.to_owned() + " "
            } else {
                " ".repeat(ARROW.len() + 1)
            },
            if self.export_options.export_data_entry_is_excluded(entry) {
                "x"
            } else {
                "✓"
            },
            line
        )
    }

    pub fn is_entry_excluded(&self, idx: usize) -> bool {
        self.export_options
            .export_data_entry_is_excluded(&self.flattened_export_data.as_ref().unwrap()[idx])
    }

    pub fn cache_export_data(&mut self, export_data: ExportData) {
        self.flattened_export_data = Some(export_data.flat());
        self.export_data = Some(export_data);
    }

    pub fn process_current_entry(&mut self) {
        let entry = &self.flattened_export_data.as_ref().unwrap()[self.current_selection];
        self.export_options.process_export_data_entry(&entry);
    }

    pub fn cache_viewable_entries(&mut self) {
        let mut i = 0;
        let mut entries = Vec::new();

        for account in &self.export_data.as_ref().unwrap().accounts {
            entries.push(ExportDataEntry::Account(account.clone()));

            if self.is_entry_excluded(i) {
                continue;
            }

            i += 1;

            for vault in &account.vaults {
                entries.push(ExportDataEntry::Vault(vault.clone()));

                if self.is_entry_excluded(i) {
                    continue;
                }

                i += 1;

                for item in &vault.items {
                    entries.push(ExportDataEntry::Item(item.clone()));
                    i += 1;
                }
            }
        }

        self.viewable_entries = entries;
    }

    pub fn move_down(&mut self) {
        if self.current_selection < self.viewable_entries.len() - 1 {
            self.current_selection += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.current_selection > 0 {
            self.current_selection -= 1;
        }
    }

    pub fn bump_loading_indicator(&mut self) {
        if self.loading_indicator_count == 5 {
            self.loading_indicator_count = 1;
        } else if self.loading_indicator_count > 0 {
            self.loading_indicator_count += 1;
        }
    }

    pub fn process_key_event(&mut self, key_event: KeyEvent) -> Option<Command> {
        if let KeyCode::Esc = key_event.code {
            return Some(Box::new(quit));
        }

        if let Some(export_data) = &self.export_data {
            match key_event.code {
                KeyCode::Enter => {
                    let path = self.export_path_input.buffer();
                    self.export_path_input.clear();

                    if path.len() > 0 {
                        // TODO: handle this error
                        self.export_options.save(export_data.clone()).unwrap();

                        return Some(Box::new(quit));
                    }
                }
                KeyCode::Up => self.move_up(),
                KeyCode::Down => self.move_down(),
                KeyCode::Char(' ') => {
                    self.process_current_entry();
                    self.cache_viewable_entries();
                }
                _ => {
                    self.export_path_input.on_key_event(key_event);
                    self.export_options
                        .set_output_file(self.export_path_input.buffer());
                }
            }
        }

        None
    }
}

impl App for Model {
    fn init(&self) -> Option<Command> {
        Some(batch(vec![
            Box::new(move || match UXExporter::new().get_export_data() {
                Ok(export_data) => Some(Box::new(export_data)),
                Err(error) => Some(Box::new(error)),
            }),
            Box::new(move || Some(Box::new(BumpLoadingIndicatorMessage))),
        ]))
    }

    fn update(&mut self, msg: Message) -> Option<Command> {
        if msg.is::<ExportData>() {
            let export_data = msg.downcast::<ExportData>().unwrap();

            self.cache_export_data(*export_data);
            self.cache_viewable_entries();

            self.loading_indicator_count = 0;
        } else if msg.is::<OPError>() {
            let error = msg.downcast().unwrap();
            self.error = *error;

            self.loading_indicator_count = 0;
        } else if msg.is::<BumpLoadingIndicatorMessage>() {
            self.bump_loading_indicator();

            return Some(Box::new(|| {
                thread::sleep(Duration::from_millis(500));
                Some(Box::new(BumpLoadingIndicatorMessage))
            }));
        } else if msg.is::<KeyEvent>() {
            let key_event = msg.downcast::<KeyEvent>().unwrap();

            let maybe_cmd = self.process_key_event(*key_event);
            if maybe_cmd.is_some() {
                return maybe_cmd;
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut out = String::from(TITLE_BAR);
        out += CONTROLS;

        if self.loading_indicator_count > 0 {
            return out
                + &format!(
                    "Fetching all account data (this may take a while){}\n",
                    ".".repeat(self.loading_indicator_count)
                );
        }

        let viewable_entries_len = self.viewable_entries.len();
        if viewable_entries_len == 0 {
            if let Some(error) = &self.error {
                match error {
                    OPError::CommandError(err) => {
                        out.push_str(&format!("Error opening op process: {}", err))
                    }
                    OPError::DeserializeError(err) => out.push_str(&format!("JSON Error: {}", err)),
                    OPError::CLIError(err) => out.push_str(&format!("OP CLI Error: {}", err)),
                }
            }
        } else {
            out.push_str(&format!(
                "({}/{})\n",
                self.current_selection + 1,
                viewable_entries_len
            ));

            out.push_str(
                &self
                    .viewable_entries
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| match entry {
                        ExportDataEntry::Account(account) => self.export_data_entry_view_line(
                            format!("(Account) {}", account.attrs.name),
                            i,
                        ),
                        ExportDataEntry::Vault(vault) => self.export_data_entry_view_line(
                            format!("  (Vault) {}", vault.attrs.name),
                            i,
                        ),
                        ExportDataEntry::Item(item) => self.export_data_entry_view_line(
                            format!("    ({}) {}", item.category_uuid, item.overview.title),
                            i,
                        ),
                    })
                    .collect::<Vec<String>>()
                    .join(""),
            );

            out.push_str(&format!(
                "{}{}\n{}^",
                EXPORT_PATH_PROMPT,
                self.export_path_input.buffer(),
                " ".repeat(EXPORT_PATH_PROMPT.len() + self.export_path_input.pos())
            ));
        }

        out
    }
}
