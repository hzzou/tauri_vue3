// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::sync::Arc;
use tokio::sync::Mutex;
use rodio::{Decoder, Sink, Source};
use tauri::Manager;
use tokio::sync::broadcast::Sender;
use crate::audio::{AudioEvent, AudioFile, AudioService};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// 扫描音乐文件
#[tauri::command]
fn scan_audio_sync(path: &str) ->Vec<AudioFile> {
    use std::fs;

    let mut counter: i32 = 1;

    match fs::read_dir(path){
        Ok(data) => {
            data.filter_map(|result|{
                result.ok().and_then(|entry| {
                    println!("en: {:?}", entry);
                    let file = entry.file_name().into_string().ok()
                        .map(|file_name| AudioFile{
                            name: file_name,
                            id: counter
                        });

                    // 判断Option中是否有一个Some实际值
                    if file.is_some(){
                        counter += 1;
                    }

                    file
                })
            }).collect()
        }
        Err(_) => {
            eprintln!("Failed to read directory '{}'", path);
            Vec::new()
        }
    }
}

// 播放控制
#[tauri::command]
fn handle_event(sender: tauri::State<Sender<AudioEvent>>, event: String) -> f64{
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();

    println!("sender: {:?}", sender);
    println!("event: {}", event);
    let mut source_time = 0.0;
    if let Some(action) = event["action"].as_str(){
        match action{
            "play" => {
                event["path"].as_str().map(|path| {
                    let file = BufReader::new(File::open(path).unwrap()); // 异步读取
                    let source = Decoder::new(file).unwrap(); //解析
                    source_time = source.total_duration().unwrap().as_secs_f64();
                    sender.send(AudioEvent::Play(path.to_owned()))
                });
            },
            "pause" => {
                Some(sender.send(AudioEvent::Pause));
            },
            "recovery" => {
                Some(sender.send(AudioEvent::Recovery));
            }
            "volume" => {
                // 此处没有as_f32
                event["volume"].as_f64().map(|volume| {
                    sender.send(AudioEvent::Volume(volume as f32))
                });
            }
            "seek" => {
                event["seek"].as_f64().map(|seek| {
                    sender.send(AudioEvent::Seek(seek))
                });
            }
            _ => {}
        }
    }
    // 返回歌曲总时间
    source_time
}

// 判断音轨是否是空
#[tauri::command]
async fn sink_empty(sink: tauri::State<'_, Arc<Mutex<Sink>>>) -> Result<bool, String>{
    let sink_clone = Arc::clone(&sink);
    let sink_data = sink_clone.try_lock().unwrap();  // Mutex互斥是tokio中的

    let is_empty = sink_data.empty();

    Ok(is_empty)
}

#[tokio::main]
async fn main() {
    let audio_service = AudioService::new();
    tauri::Builder::default()
        .setup(|app|{
            #[cfg(debug_assertions)] // 后面的代码只在debug模式下生效
            {
                let windows = app.get_window("main").unwrap();
                windows.open_devtools();
                windows.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_audio_sync, handle_event,
            sink_empty
        ])
        .manage(audio_service.event_sender) // 管理事件
        .manage(audio_service.sink) // 管理音轨
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
