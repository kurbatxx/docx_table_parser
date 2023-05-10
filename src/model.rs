use std::ops::Not;

use serde::Serialize;

#[derive(Clone, Debug, Serialize, Default)]
pub struct District {
    num: String,
    address: String,
    pub candidate: String,
    streets: Vec<Street>,
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

        dbg!(row.len());

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

#[derive(Clone, Debug, Serialize)]
pub struct Street {
    name: String,
    numbers: Vec<String>,
}

impl Street {
    pub fn create(st: String) -> Self {
        let mut st = st.splitn(2, ":");
        let name = st.next().unwrap_or_default().trim().to_string();

        let numbers = st.next().unwrap_or_default().trim().to_string();
        let numbers: Vec<_> = numbers.split(",").map(|v| v.trim().to_string()).collect();

        dbg!(&name);
        dbg!(&numbers);

        Self {
            name: name.to_string(),
            numbers,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelController {
    district_store: Vec<District>,
}

// Constructor
impl ModelController {
    pub fn new() -> Self {
        Self {
            district_store: vec![],
        }
    }
}
