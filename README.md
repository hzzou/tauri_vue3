
## 记录

* tauri使用的是1.几的版本，tauri前端页面使用vue3，涉及到要和计算机底层交互的逻辑就使用Rust编写相应代码
* 在Rust项目上添加一个函数，如果要在前端调用，需要在函数前添加#[tauri::command]，然后前端invoke函数调用(来自@tauri-apps/api)
* 在添加allowlist相关功能属性时，需要在Cargo.toml文件的tauri版本的features添加对应的功能
* rodio是处理音频的Rust第三方库
* 本项目使用的公共目录是Mac或者windows下的Documents，使用documentDir函数(来自@tauri-apps/api)获取，在Documents新建Music目录存放音乐
* 监控visibilitychange事件，记录页面隐藏后台多少时间，使用Web Worker也不行，在它里面使用定时器时也会被优化

## 食用方式

* 本地安装Rust环境，使用cargo安装tauri-cli
* 使用cargo tauri build构建本地包
* 构建本地包时，相应系统的相应格式的包只能在对应系统上构建，windows的在windows系统上构建,mac系统的在mac上构建，linux的在Linux系统上构建

