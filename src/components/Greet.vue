<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { fetch, ResponseType } from "@tauri-apps/api/http";

const url = ref("");
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
      html.value = res.data;
    })
    .catch((err) => {
      console.log(err);
    });
};
</script>

<template>
  <div class="card">
    <input v-model="url" placeholder="输入知乎链接" />
    <button type="button" @click="handleInput">请求</button>
  </div>

  <p>{{ html }}</p>
</template>
