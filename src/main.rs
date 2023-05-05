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
                            //dbg!(read_children(child));
                        }
                    })
                }
            });
        }
    }
}

fn get_all_strings(node: &Value) -> Vec<Option<String>> {
    let mut v = vec![];
    v.push(read_children(node));

    return v;
}

fn read_children(node: &Value) -> Option<String> {
    let mut st = None;
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child);
            } else {
                //println!("{}", child["data"]["text"]);
                //return child["data"]["text"].to_string();
                st = Some(child["data"]["text"].to_string())
            }
        });
    }
    st
}
