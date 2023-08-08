// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;

#[tauri::command]
fn read_csv(filename: &str) -> Option<Vec<Vec<String>>>{
  match fs::read_to_string(filename){
    Err(e) => {
      Some(vec![vec![String::from("cannot open the file")],vec![format!("{}", e)]])
    },
    Ok(text) => {
      let mut result = vec![vec![]];
      if let Some(character) = &text.chars().next(){
        if *character == '"' {
          let rows: Vec<&str> = text.split('\n').collect();
          for (row_index, row) in rows.iter().enumerate(){
            result.push(vec![]);
            let mut chars = row.chars();
            chars.next();
            chars.next_back();
            let row = chars.as_str();
            for element in row.split(r#"";""#){
              if !element.is_empty(){
                result[row_index].push(element.to_owned());
              }
            }
          }
        }else{
          let rows: Vec<&str> = text.split('\n').collect();
          for (row_index, row) in rows.iter().enumerate(){
            result.push(vec![]);
            for element in row.split(';'){
              if !element.is_empty() {
                result[row_index].push(element.to_owned());
              }
            }
          }
        }
        while let Some(vec) = result.last(){
          if vec.is_empty(){
            result.pop();
          }else{
            break;
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
