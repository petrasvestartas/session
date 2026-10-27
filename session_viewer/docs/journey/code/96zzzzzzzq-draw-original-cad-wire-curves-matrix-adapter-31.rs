
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextLabel {
    #[serde(default)]
    pub bare: bool,
    pub id: u32,
    pub object: Option<TextObject>,
    pub text: String,
