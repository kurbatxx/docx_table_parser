use std::ops::Not;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct District {
    pub num: String,
    pub address: String,
    pub candidate: String,
    pub streets: Vec<Street>,
}

impl District {
    fn cell_to_string(cell: Vec<String>) -> String {
        let cell = cell.join(" ");
        let cell = cell.replace("\"", "");
        let cell: Vec<_> = cell.split_whitespace().collect();
        cell.join(" ")
    }

    fn streets(st: String) -> Vec<Street> {
        let streets: Vec<_> = st.split([';'].as_ref()).collect();
        let streets: Vec<_> = streets
            .iter()
            .filter(|s| s.to_string().is_empty().not())
            .map(|street| Street::create(street.to_string()))
            .collect();

        streets
    }

    pub fn create(row: Vec<Vec<String>>) -> Self {
        let mut dist = Self {
            ..Default::default()
        };

        for (i, item) in row.iter().enumerate() {
            let content = Self::cell_to_string(item.to_vec());

            match i {
                1 => dist.num = content,

                2 => dist.address = content,

                3 => dist.streets = Self::streets(content),
                4 => dist.candidate = content,
                _ => {}
            }
        }

        dist
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Street {
    pub name: String,
    // no show null
    #[serde(skip_serializing_if = "Option::is_none")]
    numbers: Option<Vec<String>>,
}

impl Street {
    pub fn create(st: String) -> Self {
        let mut st = st.splitn(2, ":");
        let name = st.next().unwrap_or_default().trim().to_string();

        let numbers = st.next().unwrap_or_default().trim().to_string();
        let numbers: Vec<_> = numbers.split(",").map(|v| v.trim().to_string()).collect();

        let numbers = if numbers[0].is_empty() {
            None
        } else {
            Some(numbers)
        };

        Self {
            name: name.to_string(),
            numbers,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Raion {
    name: String,
    pub districts: Vec<District>,
}

impl Raion {
    pub fn create(vec: Vec<District>) -> Self {
        Self {
            name: "".to_string(),
            districts: vec,
        }
    }
}
