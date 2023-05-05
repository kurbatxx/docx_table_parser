use clap::{command, Parser};
use docx_rs::*;
use serde_json::{json, Value};
use std::{fs, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    parse_docx(&args.name)?;
    Ok(())
}

fn parse_docx(file_name: &str) -> anyhow::Result<()> {
    let data: Value = serde_json::from_str(&read_docx(&read_to_vec(file_name)?)?.json())?;

    // if let Some(children) = data["document"]["children"].as_array() {
    //     children.iter().for_each(read_children);
    // }

    if let Some(children) = data["document"]["children"].as_array() {
        dbg!(children.len());
        children.iter().for_each(read_table);
    }

    Ok(())
}

fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn read_table(node: &Value) {
    if node["type"] == "table" {
        //fs::write("dbg.json", &node.to_string()).unwrap();

        if let Some(rows) = node["data"]["rows"].as_array() {
            dbg!(rows.len());
            rows.iter().for_each(|child| {
                //
                if let Some(cells) = child["data"]["cells"].as_array() {
                    cells.iter().for_each(|child| {
                        //
                        if child["type"] == "tableCell" {
                            println!("-------------------------------------------");
                            let mut t_cell = vec![];
                            read_children(child, &mut t_cell);
                            dbg!(t_cell);
                        }
                    })
                }
            });
        }
    }
}

fn read_children(node: &Value, t_cell: &mut Vec<String>) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child, t_cell);
            } else {
                let st = child["data"]["text"].to_string();
                t_cell.push(st.clone());
            }
        });
    }
}
