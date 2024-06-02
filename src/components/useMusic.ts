import {BaseDirectory, readBinaryFile, readTextFile, readDir} from "@tauri-apps/api/fs";

export interface InterAudio{
	id: number;
	name: string;
}

export interface CustomPlayEvent{
	action: "play" | "pause" | "recovery" | "volume" | "seek";
	path?: string;
	volume?: number;
	seek?: number;
}

// 测试访问本地文件
export const useMusic = async ()=>{
	const data = await readTextFile("服务器相关.txt", {dir: BaseDirectory.Document});
	console.log("data:",data);

	const data_1 = await readBinaryFile("邹兴鸿_前端.doc", {dir: BaseDirectory.Document});
	console.log("data_1:", data_1);

	const data_2 = await readDir("Music", {dir: BaseDirectory.Document});
	console.log("data_2:", data_2);

};

// 播放模式
export enum PlayMode{
	ListLoop = "L", // 列表循环
	SingleLoop = "S" // 单曲循环
}
