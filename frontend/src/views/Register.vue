<template>
  <v-sheet width="500" class="mx-auto my-10">
    <form>
      <v-text-field label="Username" required focused></v-text-field>
      <v-text-field label="Email" required></v-text-field>
      <v-text-field label="Password" required></v-text-field>
      <v-btn class="me-4 bg-primary"> submit </v-btn>
      <v-btn @click="clear"> clear </v-btn>
    </form>
  </v-sheet>
</template>

<script lang="ts" setup>
const clear = () => {};
</script>
