<template>
  <v-app class="h">
    <default-bar />
    <default-view />
  </v-app>
</template>

<script lang="ts" setup>
import DefaultBar from "./AppBar.vue";
import DefaultView from "./View.vue";
</script>

<style scoped>
.h {
  display: flex;
  flex-direction: column;
  height: 100vh !important;
}
</style>
