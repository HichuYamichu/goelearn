<template>
  <v-app-bar density="compact" border flat>
    <!-- <v-app-bar-title>Class 1</v-app-bar-title> -->
    <h5 class="text-h5 mx-4 font-weight-medium">Class 1</h5>
    <v-tabs v-model="tabValue">
      <v-tab value="Chat">Chat</v-tab>
      <v-tab value="Files">Files</v-tab>
      <v-tab value="Meeting">Meeting</v-tab>
      <v-tab value="Settings">Settings</v-tab>
    </v-tabs>
  </v-app-bar>

  <v-window v-model="tabValue">
    <v-window-item value="Chat">
      <ClassChat></ClassChat>
    </v-window-item>
    <v-window-item value="Files" class="pa-4">
      <ClassFiles></ClassFiles>
    </v-window-item>
    <v-window-item value="Meeting"> </v-window-item>
    <v-window-item value="Settings"> </v-window-item>
  </v-window>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import ClassChat from "@/components/ClassChat.vue";
import ClassFiles from "@/components/ClassFiles.vue";

const tabValue = ref("Chat");
</script>
