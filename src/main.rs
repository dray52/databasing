/*
By: <Draydon Levesque>
Date: 2025-06-13
Program Details: <Program Description Here>
*/

mod modules;

use crate::modules::database::{create_database_client, DatabaseClient, DatabaseTable};
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "databasing".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let btn_text = TextButton::new(300.0, 400.0, 200.0, 60.0, "Create", BLUE, RED, 30);
    let btn_text2 = TextButton::new(100.0, 400.0, 200.0, 60.0, "Login", BLUE, RED, 30);
    let btn_text3 = TextButton::new(500.0, 400.0, 200.0, 60.0, "SAVE", BLUE, RED, 30);
    let level = TextButton::new(300.0, 700.0, 200.0, 60.0, "Level Up", BLUE, GOLD, 30);
    let mut txtuser = TextInput::new(250.0, 150.0, 300.0, 40.0, 25.0);
    let mut txtpassword = TextInput::new(250.0, 250.0, 300.0, 40.0, 25.0);
    let mut lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    txtuser.set_prompt("Enter Username");
    txtuser.set_prompt_color(DARKGRAY);
    txtpassword.set_prompt("Enter Password");
    txtpassword.set_prompt_color(DARKGRAY);
    let client = create_database_client();
    let mut score = 0;
    let mut new_record = DatabaseTable {
        id: None, // Will be auto-generated
        username: "".to_string(),
        password: "".to_string(),
        level: 1,
    };
    loop {
        use_virtual_resolution(1024.0, 768.0);
        clear_background(RED);

        draw_rectangle(100.0, 100.0, 500.0, 400.0, GREEN);
        if btn_text.click() {
           
            new_record.username = txtuser.get_text();
            new_record.password = txtpassword.get_text();
              let records: Vec<DatabaseTable> = client.fetch_table("draysTable").await.unwrap();
            for record in records {
            if record.username == new_record.username && record.password == new_record.password {
                    
                    lbl_out.set_text(format!("user already exists"));
                }
            }
            else{
            new_record.level = 1;
            let _inserted: Vec<DatabaseTable> = client.insert_record("draysTable", &new_record).await.unwrap();
            lbl_out.set_text(format!("level: {}", new_record.level));}
        };

        if btn_text2.click() {
            let records: Vec<DatabaseTable> = client.fetch_table("draysTable").await.unwrap();
            for record in records {
                if record.username == txtuser.get_text() && record.password == txtpassword.get_text() {
                    new_record = record;
                    lbl_out.set_text(format!("level: {}", new_record.level));
                }
            }
        }
        if btn_text3.click() {
            
             let _result = client
        .update_records("draysTable", &format!("username=eq.{}&password=eq.{}", new_record.username, new_record.password), &new_record)
        .await.unwrap();
    
        }
        if level.click() {
            new_record.level += 1;
            lbl_out.set_text(format!("level: {}", new_record.level));
        }
        lbl_out.draw();
        txtpassword.draw();
        txtuser.draw();
        next_frame().await;
    }
}
