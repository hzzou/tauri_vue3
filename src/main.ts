import { createApp } from "vue";
import App from "./App.vue";
import "./style.scss";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";

createApp(App)
    .use(ElementPlus)
    .mount("#app");
