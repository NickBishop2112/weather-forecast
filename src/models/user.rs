use paperclip::actix::Apiv2Schema;

#[derive(serde::Serialize, serde::Deserialize, Apiv2Schema, Debug, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}