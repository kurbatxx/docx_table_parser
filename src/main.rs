use anyhow::Ok;
use clap::{command, Parser};
use docx_rs::*;
use serde_json::Value;
use std::{fs, io::Read, ops::Not};

use crate::model::{District, Raion};

mod model;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let doc = parse_docx(&args.name)?;
    dbg!("-----------------------");

    let districts: Vec<_> = doc[0]
        .iter()
        .map(|row| District::create(row.to_vec()))
        .collect();

    //let row = &doc[0][1];
    //let dist = District::create(row.to_vec());

    let raion = Raion::create(districts);

    fs::write("resut.json", serde_json::to_string(&raion)?)?;
    Ok(())
}

fn parse_docx(file_name: &str) -> anyhow::Result<Vec<Vec<Vec<Vec<String>>>>> {
    let data: Value = serde_json::from_str(&read_docx(&read_to_vec(file_name)?)?.json())?;
    let mut v = vec![];

    if let Some(children) = data["document"]["children"].as_array() {
        dbg!(children.len());
        v = children
            .iter()
            .map(read_table)
            .filter(|x| x.is_empty().not())
            .collect::<_>();
    }

    Ok(v)
}

fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn read_table(node: &Value) -> Vec<Vec<Vec<String>>> {
    let mut table = vec![];
    if node["type"] == "table" {
        //fs::write("dbg.json", &node.to_string()).unwrap();

        if let Some(rows) = node["data"]["rows"].as_array() {
            dbg!(rows.len());
            rows.iter().for_each(|child| {
                let mut row_cell = vec![];
                if let Some(cells) = child["data"]["cells"].as_array() {
                    cells.iter().for_each(|child| {
                        //
                        if child["type"] == "tableCell" {
                            //println!("-------------------------------------------");
                            let mut t_cell = vec![];
                            read_children(child, &mut t_cell);

                            row_cell.push(t_cell);
                        }
                    })
                }

                table.push(row_cell);
            });
        }
    }
    table
}

fn read_children(node: &Value, t_cell: &mut Vec<String>) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child, t_cell);
            } else {
                let st = child["data"]["text"].to_string();
                t_cell.push(st);
            }
        });
    }
}
