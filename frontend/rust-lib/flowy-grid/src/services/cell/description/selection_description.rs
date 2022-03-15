use crate::impl_from_and_to_type_option;
use crate::services::row::CellDataSerde;
use crate::services::util::*;
use flowy_derive::ProtoBuf;
use flowy_error::FlowyError;
use flowy_grid_data_model::entities::{FieldMeta, FieldType};
use serde::{Deserialize, Serialize};

// Single select
#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct SingleSelectDescription {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_from_and_to_type_option!(SingleSelectDescription, FieldType::SingleSelect);

impl CellDataSerde for SingleSelectDescription {
    fn deserialize_cell_data(&self, data: String) -> String {
        data
    }

    fn serialize_cell_data(&self, data: &str) -> Result<String, FlowyError> {
        Ok(select_option_id_from_data(data.to_owned(), true))
    }
}

// Multiple select
#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct MultiSelectDescription {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_from_and_to_type_option!(MultiSelectDescription, FieldType::MultiSelect);
impl CellDataSerde for MultiSelectDescription {
    fn deserialize_cell_data(&self, data: String) -> String {
        data
    }

    fn serialize_cell_data(&self, data: &str) -> Result<String, FlowyError> {
        Ok(select_option_id_from_data(data.to_owned(), false))
    }
}

fn select_option_id_from_data(data: String, is_single_select: bool) -> String {
    if !is_single_select {
        return data;
    }
    let select_option_ids = data.split(',').collect::<Vec<&str>>();
    if select_option_ids.is_empty() {
        return "".to_owned();
    }

    select_option_ids.split_first().unwrap().0.to_string()
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct SelectOption {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub color: String,
}

impl SelectOption {
    pub fn new(name: &str) -> Self {
        SelectOption {
            id: uuid(),
            name: name.to_owned(),
            color: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::cell::{MultiSelectDescription, SingleSelectDescription};
    use crate::services::row::CellDataSerde;

    #[test]
    fn selection_description_test() {
        let description = SingleSelectDescription::default();
        assert_eq!(description.serialize_cell_data("1,2,3").unwrap(), "1".to_owned());

        let description = MultiSelectDescription::default();
        assert_eq!(description.serialize_cell_data("1,2,3").unwrap(), "1,2,3".to_owned());
    }
}
