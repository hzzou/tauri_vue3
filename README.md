
## 记录

* 在Rust项目上添加一个函数，如果要在前端调用，需要在函数前添加#[tauri::command]，然后前端invoke函数调用(来自@tauri-apps/api)
* 在添加allowlist相关功能属性时，需要在Cargo.toml文件的tauri版本的features添加对应的功能
* rodio是处理音频的Rust第三方库
