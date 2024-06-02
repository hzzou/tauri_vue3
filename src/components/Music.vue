<template>
  <div class="music">
    <div class="top">
      <el-table
          :data="tableData"
          size="small"
          :show-header="false"
      >
        <el-table-column props="id" width="40px" />
        <el-table-column prop="name" />
        <el-table-column width="100">
          <template #default="{row}" >
              <div v-if="row.id === currAudioId">
                  <el-button v-if="!isPlaying" link type="primary" @click="playAudio(row)">
                      <el-icon size="20px"><VideoPlay /></el-icon>
                  </el-button>
                  <el-button v-else link type="primary" @click="pauseAudio">
                      <el-icon size="20px"><VideoPause /></el-icon>
                  </el-button>
              </div>
              <div v-else>
                  <el-button link type="primary" @click="playAudio(row)">
                      <el-icon size="20px"><VideoPlay /></el-icon>
                  </el-button>
              </div>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <footer class="bottom">
      <el-row :gutter="20" align="middle">
        <el-col :span="1"></el-col>
        <el-col :span="6">
          <div style="margin-top: 10px">
            <el-text class="music-name" size="small">
              <el-icon><Headset /></el-icon>
              {{currAudioName}}
            </el-text>
          </div>
        </el-col>
        <el-col :span="1"></el-col>
        <el-col :span="4">
          <el-button link type="primary" class="no-shadow" @click="preAudio">
            <el-icon><ArrowLeftBold /></el-icon>
          </el-button>
          <el-button link type="primary" v-if="!isPlaying" class="no-shadow" @click="recoveryAudio()">
            <el-icon size="35px"><VideoPlay /></el-icon>
          </el-button>
          <el-button v-else link type="primary" class="no-shadow" @click="pauseAudio()">
            <el-icon size="35px"><VideoPause /></el-icon>
          </el-button>
          <el-button link type="primary" class="no-shadow" @click="nextAudio()">
            <el-icon><ArrowRightBold /></el-icon>
          </el-button>
        </el-col>
        <el-col :span="8">
            <div class="slider-block">
                <el-slider v-model="musicSeek" :min="0" :max="musicTime" :step="1" size="small" height="140px"
                           @mouseup="changeMusicSeek" @touchend="changeMusicSeek">
                </el-slider>
            </div>
        </el-col>
        <el-col :span="4">
          <div style="margin-top: 15px">
            <el-popover placement="top" trigger="hover">
              <template #reference>
                <el-button link type="primary" class="no-shadow" @click="toggleMute">
                  <el-icon v-if="!isMuted"><Bell /></el-icon>
                  <el-icon v-else><MuteNotification /></el-icon>
                </el-button>
              </template>
              <div class="slider-block">
                <el-slider v-model="volume" :min="0" :max="100" :step="2" size="small" height="140px"
                @mouseup="changeVolume" @touchend="changeVolume">
                </el-slider>
              </div>
            </el-popover>
            <el-popover placement="top" trigger="hover">
              <template #reference>
                <el-button link type="primary" class="no-shadow">
                  <span v-if="radio === 'L'">L</span>
                  <span v-else>S</span>
                </el-button>
              </template>
              <div>
                <el-radio-group v-model="radio">
                  <el-radio size="small" label="List loop" value="L"></el-radio>
                  <el-radio size="small" label="Single loop" value="S"></el-radio>
                </el-radio-group>
              </div>
            </el-popover>
          </div>
        </el-col>
      </el-row>
    </footer>
  </div>
</template>
<script setup lang="ts">
    import {
        ArrowLeftBold, ArrowRightBold, Bell,
        Headset, MuteNotification, VideoPause, VideoPlay
    } from "@element-plus/icons-vue";
    import {onBeforeMount, onMounted, onUnmounted, ref, watch} from "vue";
    import {CustomPlayEvent, InterAudio, PlayMode} from "./useMusic.ts";
    import {documentDir} from "@tauri-apps/api/path";
    import {invoke} from "@tauri-apps/api";
    import {ElMessage} from "element-plus";
    const currAudioName = ref(""), // 当前歌曲名称
        isPlaying = ref(false), // 是否播放
        isMuted = ref(false), // 是否静音
        volume = ref(20), // 音量
        musicSeek = ref(0), // 播放量控制
        musicTime = ref(0), // 音乐总时间
        originVolume = ref(0), // 静音时存储原始音量
        radio = ref(PlayMode.ListLoop),
        currAudioId = ref(0),
        docPath = await documentDir(),  // 计算机的documents目录
        tableData = ref([]);

    let intervalId = ref(), // 定时器id,控制播放循环
    seekId = ref(); // 定时器id, 控制播放进度

    onBeforeMount(()=>{
        getList().then();
        changeVolume().then(); //设置默认音量
    });

    onMounted(()=>{
        intervalId.value = setInterval(playControl, 1000);
    });

    onUnmounted(()=>{
        clearInterval(intervalId.value);
        clearInterval(seekId.value);
    })

    // 获取播放列表
    // 此处Mac和Windows下都基于当前的Documents
    const getList = async() => {
        tableData.value = await invoke("scan_audio_sync", {path: docPath + "Music"});
    };

    // 播放
    const playAudio = async (row: InterAudio) => {
        isPlaying.value = true;
        currAudioName.value = row.name;
        currAudioId.value = row.id;
        const file_path = docPath+"Music/"+row.name;
        const event: CustomPlayEvent = {action: "play", path: file_path};
        musicTime.value = await invoke("handle_event", {event: JSON.stringify(event)}).catch((error)=>ElMessage.error(error)) as number;
        musicSeek.value = 0;
        seekId.value = setInterval(changeMusicProcess, 1000);
    };

    // 恢复播放
    const recoveryAudio = async () => {
        if(currAudioName.value === "") return;
        isPlaying.value = true;
        const event: CustomPlayEvent = {action: "recovery"};
        await invoke("handle_event", {event: JSON.stringify(event)}).catch((error)=>ElMessage.error(error));
    };

    // 暂停播放
    const pauseAudio = async() => {
        isPlaying.value = false;
        const event: CustomPlayEvent = {action: "pause"};
        await invoke("handle_event", {event: JSON.stringify(event)}).catch((error)=>ElMessage.error(error));
    };

    // 前一首
    const preAudio = () => {
        if(currAudioId.value >= 2){
            let row = tableData.value[currAudioId.value - 2];
            row && playAudio(row);
        }
    };

    // 下一首
    const nextAudio = () => {
        // id是从1开始,index从0开始
        let row = tableData.value[currAudioId.value];
        row && playAudio(row);
    };

    // 静音
    const toggleMute = () => {
        if(!isMuted.value) {
            originVolume.value = volume.value;
        }
        isMuted.value = !isMuted.value;
        // ?? 如果前面的是null或者undefined则返回第二个
        volume.value = isMuted.value ? 0 : originVolume.value ?? volume.value;

        changeVolume(); // 主要是触发Rust后面的声音设置
    };

    // 改变声音
    const changeVolume = async () => {
        const event: CustomPlayEvent = {action: "volume", volume: volume.value};
        await invoke("handle_event", {event: JSON.stringify(event)}).catch((error)=>ElMessage.error(error));
    };

    // 自动改变进度条
    const changeMusicProcess = async () => {
        if(isPlaying.value){
            musicSeek.value += 1;
        }
    };

    const changeMusicSeek = async () => {
        const event: CustomPlayEvent = {action: "seek", seek: musicSeek.value};
        await invoke("handle_event", {event: JSON.stringify(event)}).catch((error)=>ElMessage.error(error));
    }


    // 播放控制,判断它是否播放完，再根据播放模式进行判断和播放
    const playControl = () => {
        invoke("sink_empty").then((is_empty)=>{
            if(is_empty){
                isPlaying.value = false;
                if(currAudioId.value === 0) return;

                let index = 0;
                switch(radio.value){
                    case PlayMode.ListLoop:
                        index = currAudioId.value === tableData.value.length ? 0 : currAudioId.value;
                        break;
                    case PlayMode.SingleLoop:
                        index = currAudioId.value - 1;
                        break;
                    default:
                        console.warn(`Unsupported play mode ${radio.value}`);
                        break;
                }
                playAudio(tableData.value[index]);
            }
        });
    }
</script>
<style lang="scss">
    .music{
        width: 100%;
        padding: 20px;
        box-sizing: border-box;
        .top{

        }
        .bottom{
            position: fixed;
            bottom: 0; left: 0;
            margin-top: 50px;
            width: 100%;
            height: 50px;
            background: #fff;
            border-top: 2px solid #bbb;
            z-index: 1000;
            .music-name{
                white-space: nowrap;
                text-overflow: ellipsis;
                overflow: hidden;
            }
    }
    .no-shadow{
        box-shadow: none!important;
    }

    .el-table{
        tr:hover{
            .hover-visible-button{
                display: inline-block;
        }
      }
    }

    .hover-visible-button{
        display: none;
    }

    .slider-block{
        max-width: 150px;
        display: flex;
        align-items: center;
        .el-slider{
            margin-top: 0;
        }
    }
  }
</style>


