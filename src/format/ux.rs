use serde::{self, Serialize};

use crate::op::OPError;

use super::cli;

#[derive(Clone)]
pub struct UXExporter {
    listed_vaults: Vec<cli::ListedVault>,
    listed_items: Vec<cli::ListedItem>,
}

impl UXExporter {
    pub fn new() -> UXExporter {
        UXExporter {
            listed_vaults: Vec::new(),
            listed_items: Vec::new(),
        }
    }

    pub fn get_export_data(&mut self) -> Result<ExportData, OPError> {
        let listed_accounts = cli::get_listed_accounts()?;
        self.listed_vaults = cli::get_listed_vaults()?;
        self.listed_items = cli::get_listed_items()?;

        Ok(ExportData {
            accounts: listed_accounts
                .iter()
                .map(|acc| self.get_account(acc))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn get_account(&self, listed_account: &cli::ListedAccount) -> Result<Account, OPError> {
        let account = cli::get_account(&listed_account.user_uuid)?;

        Ok(Account {
            attrs: AccountAttributes {
                name: account.name,
                email: listed_account.email.to_owned(),
                uuid: account.id,
                domain: account.domain,
            },
            vaults: self
                .listed_vaults
                .iter()
                .map(|vault| self.get_vault(vault))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn get_vault(&self, listed_vault: &cli::ListedVault) -> Result<Vault, OPError> {
        let vault = cli::get_vault(&listed_vault.id)?;

        Ok(Vault {
            attrs: VaultAttributes {
                uuid: listed_vault.id.to_owned(),
                name: listed_vault.name.to_owned(),
                type_: vault.type_,
            },
            items: self
                .listed_items
                .iter()
                .filter_map(|item| {
                    if item.vault.id == listed_vault.id {
                        Some(self.get_item(&item))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn get_item(&self, listed_item: &cli::ListedItem) -> Result<Item, OPError> {
        let item = cli::get_item(&listed_item.id)?;

        let urls = item.urls.unwrap_or_default();

        Ok(Item {
            uuid: listed_item.id.to_owned(),
            created_at: listed_item.created_at.to_owned(),
            updated_at: listed_item.updated_at.to_owned(),
            category_uuid: listed_item.category.to_owned(),
            overview: Overview {
                title: listed_item.title.to_owned(),
                url: match urls.iter().find(|url| url.primary.unwrap_or_default()) {
                    Some(url) => Some(url.href.clone().unwrap_or("".to_owned()).to_owned()),
                    None => None,
                },
                urls: urls
                    .iter()
                    .map(|url| URL {
                        url: url.href.clone().unwrap_or("".to_owned()).to_owned(),
                    })
                    .collect(),
                tags: listed_item.tags.as_ref().unwrap_or(&Vec::new()).to_owned(),
            },
            details: ItemDetails {
                login_fields: item
                    .fields
                    .unwrap_or_default()
                    .iter()
                    .map(|field| LoginField {
                        value: field.value.to_owned(),
                        name: field.label.to_owned(),
                        type_: field.type_.to_owned(),
                        designation: field.purpose.to_owned(),
                    })
                    .collect(),
            },
        })
    }
}

/*
#[derive(Clone)]
pub struct Document {
    id: String,
    title: String,
}

pub fn get_documents() -> Result<Vec<Document>, OPError> {
    let documents = run::<Vec<cli::Document>>(&["document", "list"])?
        .iter()
        .map(|doc| Document {
            id: doc.id.clone(),
            title: doc.title.clone(),
        })
        .collect::<Vec<Document>>();

    Ok(documents)
}

pub fn download_document(id: String) -> Result<(), OPError> {
    run::<cli::Document>(&["document", "get", &id])?;

    Ok(())
}
*/

#[derive(Default, Serialize, Clone)]
pub struct ExportData {
    pub accounts: Vec<Account>,
}

pub enum ExportDataEntry {
    Account(Account),
    Vault(Vault),
    Item(Item),
}

impl ExportData {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn flatten(&self) -> Vec<ExportDataEntry> {
        let mut entries = Vec::new();

        for acc in &self.accounts {
            entries.push(ExportDataEntry::Account(acc.clone()));
            for vault in &acc.vaults {
                entries.push(ExportDataEntry::Vault(vault.clone()));
                for item in &vault.items {
                    entries.push(ExportDataEntry::Item(item.clone()));
                }
            }
        }

        entries
    }
}

pub fn unflatten(entries: Vec<ExportDataEntry>) -> ExportData {
    let mut accounts = Vec::new();
    let mut vaults = Vec::new();
    let mut items = Vec::new();

    for entry in entries {
        match entry {
            ExportDataEntry::Account(acc) => accounts.push(acc),
            ExportDataEntry::Vault(vault) => vaults.push(vault),
            ExportDataEntry::Item(item) => items.push(item),
        }
    }

    ExportData { accounts }
}

#[derive(Serialize, Clone)]
pub struct Account {
    pub attrs: AccountAttributes,
    pub vaults: Vec<Vault>,
}

#[derive(Serialize, Clone)]
pub struct AccountAttributes {
    #[serde(rename = "accountName")]
    pub name: String,
    pub email: String,
    pub uuid: String,
    pub domain: String,
}

#[derive(Serialize, Clone)]
pub struct Vault {
    pub attrs: VaultAttributes,
    pub items: Vec<Item>,
}

#[derive(Serialize, Clone)]
pub struct VaultAttributes {
    pub uuid: String,
    pub name: String,
    // TODO: deserialize this into an enum of P, E, or U
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Clone)]
pub struct Item {
    pub uuid: String,
    // #[serde(rename = "favIndex")]
    // fav_index: u32,
    // TODO: make these u32s
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    // TODO: implement this and deserialize this into an enum of Y or N
    // trashed: String,
    #[serde(rename = "categoryUuid")]
    pub category_uuid: String,
    pub overview: Overview,
    pub details: ItemDetails,
}

#[derive(Serialize, Clone)]
pub struct Overview {
    pub title: String,
    pub url: Option<String>,
    pub urls: Vec<URL>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct URL {
    // label: String,
    pub url: String,
}

#[derive(Serialize, Clone)]
pub struct ItemDetails {
    #[serde(rename = "loginFields")]
    pub login_fields: Vec<LoginField>,
}

#[derive(Serialize, Clone)]
pub struct LoginField {
    pub value: Option<String>,
    pub name: Option<String>,
    // TODO: deserialize this into an enum of T, E, U, N, P, A, TEL
    #[serde(rename = "type")]
    pub type_: String,
    pub designation: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Field {
    pub title: String,
    pub id: String,
    pub value: String,
}

#[derive(Serialize, Clone)]
pub struct PreviousPassword {
    pub value: String,
    pub time: u32,
}
