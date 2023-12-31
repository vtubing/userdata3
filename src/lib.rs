mod meta;
mod user_data;

pub use meta::Meta;
pub use user_data::UserData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct UserData3 {
  pub meta: Meta,
  pub user_data: UserData,
  pub version: u8,
}
