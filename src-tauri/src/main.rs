// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;

#[tauri::command]
fn read_csv(filename: &str) -> Option<Vec<Vec<String>>>{
  match fs::read_to_string(filename){
    Err(_e) => {
      Some(vec![vec![String::from("cannot open the file")]])
    },
    Ok(text) => {
      let mut result = vec![vec![]];
      if let Some(character) = &text.chars().next(){
        if *character == '"' {
          let mut rows: Vec<&str> = text.split('\n').collect();
          for (row_index, row) in rows.iter().enumerate(){
            result.push(vec![]);
            let mut chars = row.chars();
            chars.next();
            chars.next_back();
            let row = chars.as_str();
            for element in row.split(r#"";""#){
              result[row_index].push(element.to_owned());
            }
          }
        }else{
          let mut rows: Vec<&str> = text.split('\n').collect();
          for (row_index, row) in rows.iter().enumerate(){
            result.push(vec![]);
            for element in row.split(';'){
              result[row_index].push(element.to_owned());
            }
          }
        }
        Some(result)
      }else{
        None
      }
    }
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_csv])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
