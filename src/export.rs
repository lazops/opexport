use std::{fs, io};

use crate::format::ux::{unflatten, ExportData, ExportDataEntry};

pub struct ExportOptions {
    pub output_file: String,
    pub excluded_accounts: Vec<String>,
    pub excluded_vaults: Vec<String>,
    pub excluded_items: Vec<String>,
}

impl ExportOptions {
    pub fn new() -> ExportOptions {
        ExportOptions {
            output_file: String::new(),
            excluded_accounts: Vec::new(),
            excluded_vaults: Vec::new(),
            excluded_items: Vec::new(),
        }
    }

    pub fn export_data_entry_is_excluded(&self, export_data_entry: &ExportDataEntry) -> bool {
        match export_data_entry {
            ExportDataEntry::Account(account) => {
                self.excluded_accounts.contains(&account.attrs.uuid)
            }
            ExportDataEntry::Vault(vault) => self.excluded_vaults.contains(&vault.attrs.uuid),
            ExportDataEntry::Item(item) => self.excluded_items.contains(&item.uuid),
        }
    }

    pub fn process_export_data_entry(&mut self, export_data_entry: &ExportDataEntry) {
        match export_data_entry {
            ExportDataEntry::Account(account) => {
                let index = self
                    .excluded_accounts
                    .iter()
                    .position(|account_uuid| account_uuid == &account.attrs.uuid);
                if let Some(index) = index {
                    self.excluded_accounts.remove(index);
                } else {
                    self.excluded_accounts.push(account.attrs.uuid.clone());
                }
            }
            ExportDataEntry::Vault(vault) => {
                let index = self
                    .excluded_vaults
                    .iter()
                    .position(|vault_uuid| vault_uuid == &vault.attrs.uuid);
                if let Some(index) = index {
                    self.excluded_vaults.remove(index);
                } else {
                    self.excluded_vaults.push(vault.attrs.uuid.clone());
                }
            }
            ExportDataEntry::Item(item) => {
                let index = self
                    .excluded_items
                    .iter()
                    .position(|item_uuid| item_uuid == &item.uuid);
                if let Some(index) = index {
                    self.excluded_items.remove(index);
                } else {
                    self.excluded_items.push(item.uuid.clone());
                }
            }
        }
    }

    fn filter_export_data(&self, export_data: ExportData) -> ExportData {
        unflatten(
            export_data
                .flatten()
                .into_iter()
                .filter(|entry| match entry {
                    ExportDataEntry::Account(account) => {
                        !self.excluded_accounts.contains(&account.attrs.uuid)
                    }
                    ExportDataEntry::Vault(vault) => {
                        !self.excluded_vaults.contains(&vault.attrs.uuid)
                    }
                    ExportDataEntry::Item(item) => !self.excluded_items.contains(&item.uuid),
                })
                .collect(),
        )
    }

    pub fn save(&self, export_data: ExportData) -> io::Result<()> {
        let export_data = self.filter_export_data(export_data);
        fs::write(&self.output_file, export_data.to_json().unwrap())
    }

    pub fn set_output_file(&mut self, output_file: String) {
        self.output_file = output_file;
    }
}
