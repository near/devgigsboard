use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

pub type CommunityHandle = String;

pub type CommunityAddOnId = String;

pub type CommunityAddOnConfigId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityInputs {
    pub handle: CommunityHandle,
    pub name: String,
    pub tag: String,
    pub description: String,
    pub logo_url: String,
    pub banner_url: String,
    pub bio_markdown: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityMetadata {
    pub admins: Vec<AccountId>,
    pub handle: CommunityHandle,
    pub name: String,
    pub tag: String,
    pub description: String,
    pub logo_url: String,
    pub banner_url: String,
    pub bio_markdown: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityFeatureFlags {
    pub telegram: bool,
    pub github: bool,
    pub board: bool,
    pub wiki: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WikiPage {
    name: String,
    content_markdown: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityAddOnConfig {
    pub config_id: CommunityAddOnConfigId,
    pub addon_id: CommunityAddOnId,
    pub parameters: String,
    pub enabled: bool,
    pub name: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityAddOn {
    pub id: CommunityAddOnId,
    pub title: String,
    pub description: String,
    pub viewer: String,
    pub configurator: String,
    pub icon: String,
}

impl CommunityAddOn {
    pub fn validate(&self) {
        if !matches!(self.id.chars().count(), 3..=60) {
            panic!("Add-on id must contain 3 to 60 characters");
        }

        if !matches!(self.title.chars().count(), 3..=60) {
            panic!("Add-on title must contain 3 to 60 characters");
        }

        if !matches!(self.description.chars().count(), 3..=60) {
            panic!("Add-on description must contain 3 to 60 characters");
        }

        if !matches!(self.viewer.chars().count(), 6..=60) {
            panic!("Add-on description must contain 6 to 60 characters");
        }

        if !matches!(self.configurator.chars().count(), 0..=60) {
            panic!("Add-on configurator must contain 0 to 60 characters");
        }
        if !matches!(self.icon.chars().count(), 6..=60) {
            panic!("Add-on icon must contain 6 to 60 characters");
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Community {
    pub admins: Vec<AccountId>,
    pub handle: CommunityHandle,
    pub name: String,
    pub tag: String,
    pub description: String,
    pub logo_url: String,
    pub banner_url: String,
    pub bio_markdown: Option<String>,
    pub github_handle: Option<String>,
    pub telegram_handle: Vec<String>,
    pub twitter_handle: Option<String>,
    pub website_url: Option<String>,
    /// JSON string of github board configuration
    pub github: Option<String>,
    /// JSON string of kanban board configuration
    pub board: Option<String>,
    pub wiki1: Option<WikiPage>,
    pub wiki2: Option<WikiPage>,
    pub features: CommunityFeatureFlags,
    pub addon_list: Vec<CommunityAddOnConfig>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FeaturedCommunity {
    pub handle: CommunityHandle,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityPermissions {
    pub can_configure: bool,
    pub can_delete: bool,
}

impl Community {
    pub fn validate(&self) {
        if !matches!(self.handle.chars().count(), 3..=40) {
            panic!("Community handle must contain 3 to 40 characters");
        }

        if !matches!(self.name.chars().count(), 3..=30) {
            panic!("Community name must contain 3 to 30 characters");
        }

        if !matches!(self.tag.chars().count(), 3..=30) {
            panic!("Community tag must contain 3 to 30 characters");
        }

        if !matches!(self.description.chars().count(), 6..=60) {
            panic!("Community description must contain 6 to 60 characters");
        }

        if self.bio_markdown.is_some()
            && !matches!(
                self.bio_markdown.to_owned().map_or(0, |text| text.chars().count()),
                3..=200
            )
        {
            panic!("Community bio must contain 3 to 200 characters");
        }
    }

    pub fn add_addon(&mut self, addon_config: CommunityAddOnConfig) {
        self.addon_list.push(addon_config);
    }

    pub fn remove_addon(&mut self, config_id: String) {
        self.addon_list.retain(|config| config.config_id != config_id);
    }

    pub fn set_default_admin(&mut self) {
        if self.admins.is_empty() {
            self.admins = vec![env::predecessor_account_id()];
        }
    }
}
