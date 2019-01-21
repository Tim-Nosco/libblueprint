
#[derive(Serialize, Debug, Deserialize)]
pub struct Position {
    //https://wiki.factorio.com/Blueprint_string_format#Position_object
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ItemFilter {
    //https://wiki.factorio.com/Blueprint_string_format#Item_filter_object
    pub name: String,
    pub index: u64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Entity {
    //https://wiki.factorio.com/Blueprint_string_format#Entity_object
    pub entity_number: u64,
    pub name: String,
    pub position: Position,
    #[serde(default)]
    pub direction: u64,
    #[serde(default)]
    pub recipe: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub input_priority: String,
    #[serde(default)]
    pub output_priority: String,
    #[serde(default)]
    pub filter: String,
    #[serde(default)]
    pub filters: Vec<ItemFilter>,
}