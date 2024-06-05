use std::io::BufReader;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink, Source};
use serde::Serialize;
use std::fs::File;
use std::ops::Div;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, Mutex};

#[derive(Serialize, Debug)]
pub struct AudioFile{
    pub id: i32,
    pub name: String
}

#[derive(Debug, Clone)]
pub enum AudioEvent{
    Play(String),
    Recovery,
    Pause,
    Volume(f32),
    Seek(f64)
}

pub struct AudioService{
    pub event_sender: Sender<AudioEvent>,
    stream: OutputStream,
    pub sink: Arc<Mutex<Sink>>
}

impl AudioService{
    pub fn new() -> Self{
        // tokio中的异步广播模式
        let (tx, mut rx) = broadcast::channel(100);
        let (stream, handle) = OutputStream::try_default().unwrap();

        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));
        let sink_clone = Arc::clone(&sink);

        tokio::spawn(async move{
            while let Ok(event) = rx.recv().await {
                let sink = sink_clone.lock().await;

                match event {
                    AudioEvent::Play(path) => {
                        let file = BufReader::new(File::open(path).unwrap()); // 异步读取
                        let source = Decoder::new(file).unwrap(); //解析

                        sink.clear(); //清空音轨
                        if sink.is_paused(){
                            sink.append(source);
                            sink.play();
                        }
                    }
                    AudioEvent::Pause => {
                        sink.pause();
                    }
                    AudioEvent::Recovery => {
                        sink.play();
                    }
                    AudioEvent::Volume(volume) => {
                        sink.set_volume(volume / 50.0); // 设置sink上的音量, 需要的类型是f32
                    }
                    AudioEvent::Seek(seek) => {
                        sink.try_seek(Duration::from_secs_f64(seek)).unwrap();
                    }
                }
            }
        });

        Self{
            event_sender: tx,
            stream,
            sink
        }
    }
}

#[cfg(test)]
mod test{
    use rodio::Source;
    use tokio::test;
    // Debug模式运行会报错
    #[test]
    async fn test_radio(){
        use rodio::{Decoder, OutputStream, Sink};
        use std::fs::File;
        use std::io::BufReader;

        let handle = tokio::spawn(async move{
            let (stream, handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&handle).unwrap();

            // mac笔记本上
            let file = File::open("/Users/hzlzh/Documents/Music/大海.mp3");
            println!("f: {:?}", file);
            let reader = BufReader::new(file.unwrap());
            let source = Decoder::new(reader).unwrap();

            sink.append(source);

            println!("sink: {}", sink.empty());

            sink.sleep_until_end(); // 阻塞进程，等待播放完才执行后面的代码

            println!("end: {}", sink.empty());
        });

        handle.await.unwrap();
    }
}