<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { fetch, ResponseType } from "@tauri-apps/api/http";
import { convert } from "html-to-text";

const url = ref(
  "https://www.zhihu.com/market/paid_column/1616029167866060800/section/1616029675813314560?is_share_data=true&vp_share_title=0"
);
const html = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const handleInput = () => {
  fetch(url.value, {
    method: "GET",
    timeout: 30,
    responseType: ResponseType.Text,
  })
    .then((res) => {
      const d = convert(res.data, {
        wordwrap: 20,
        baseElements: {
          selectors: ["p"],
        },
      });
      console.log(d);
      html.value = d;
      console.log(html.value);
    })
    .catch((err) => {
      console.log(err);
    });
};

const handleGen = () => {
  invoke("handle_text_to_png", { html: html.value })
    .then((message) => console.log(message))
    .catch((error) => {
      console.log(error);
    });
};
</script>

<template>
  <el-row justify="center">
    <el-col :span="14"
      ><el-input v-model="url" placeholder="输入知乎链接" />
    </el-col>
    <el-col :span="3" :offset="1">
      <el-button type="primary" @click="handleInput">请求</el-button>
    </el-col>
  </el-row>

  <el-row justify="center" :span="24" class="mt20">
    <el-input
      v-model="html"
      type="textarea"
      :rows="20"
      placeholder="搜索结果"
      resize="none"
    />
  </el-row>

  <el-row class="mt20">
    <el-col :span="3">
      <el-button type="primary" @click="handleGen">生成图片</el-button>
    </el-col>
  </el-row>
</template>

<style scoped></style>
