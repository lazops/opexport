use serde::{Deserialize, Serialize};

use crate::op::{run, OPError};

// helper struct used for fields that only contain an ID
#[derive(Deserialize, Serialize, Clone)]
pub struct OnlyID {
    pub id: String,
}

// op document get <id>
#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub version: usize,
    pub vault: OnlyID,
    pub last_edited_by: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "overview.ainfo")]
    pub overview_ainfo: Option<String>,
}

// op account list
#[derive(Deserialize, Serialize, Clone)]
pub struct ListedAccount {
    pub url: String,
    pub email: String,
    pub user_uuid: String,
    pub shorthand: Option<String>,
}

pub fn get_listed_accounts() -> Result<Vec<ListedAccount>, OPError> {
    run::<Vec<ListedAccount>>(&["account", "list"])
}

// op account get <id>
#[derive(Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub domain: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub state: String,
    pub created_at: String,
}

pub fn get_account(id: &str) -> Result<Account, OPError> {
    run::<Account>(&["account", "get", "--account", id])
}

// op vault list
#[derive(Deserialize, Serialize, Clone)]
pub struct ListedVault {
    pub id: String,
    pub name: String,
}

pub fn get_listed_vaults() -> Result<Vec<ListedVault>, OPError> {
    run::<Vec<ListedVault>>(&["vault", "list"])
}

// op vault get <id>
#[derive(Deserialize, Serialize, Clone)]
pub struct Vault {
    #[serde(flatten)]
    pub listed_vault: ListedVault,
    pub attribute_version: usize,
    pub content_version: usize,
    pub items: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn get_vault(id: &str) -> Result<Vault, OPError> {
    run::<Vault>(&["vault", "get", id])
}

// op item list
#[derive(Deserialize, Serialize, Clone)]
pub struct ListedItem {
    pub id: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub version: usize,
    pub vault: OnlyID,
    pub category: String,
    pub last_edited_by: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn get_listed_items() -> Result<Vec<ListedItem>, OPError> {
    run::<Vec<ListedItem>>(&["item", "list"])
}

// op item get <id>
#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    #[serde(flatten)]
    pub listed_item: ListedItem,
    pub sections: Vec<OnlyID>,
    pub fields: Vec<Field>,
    pub urls: Option<Vec<URL>>,
}

pub fn get_item(id: &str) -> Result<Item, OPError> {
    run::<Item>(&["item", "get", id])
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub purpose: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub entropy: Option<f64>,
    pub password_details: Option<PasswordDetails>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PasswordDetails {
    pub entropy: Option<usize>,
    pub generated: Option<bool>,
    pub strength: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct URL {
    pub primary: bool,
    pub href: String,
}
