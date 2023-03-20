<template>
  <v-app-bar>
    <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
    <v-spacer></v-spacer>

    <v-container class="fill-height d-flex align-center">
      <v-btn
        v-for="link in links"
        :key="link.target"
        variant="text"
        :to="link.target"
        exact
      >
        {{ link.text }}
      </v-btn>

      <v-spacer></v-spacer>

      <v-responsive max-width="260">
        <v-text-field
          density="compact"
          hide-details
          variant="solo"
        ></v-text-field>
      </v-responsive>
    </v-container>

    <v-btn> login </v-btn>
  </v-app-bar>

  <v-navigation-drawer v-model="drawer" temporary>
    <v-list nav>
      <v-list-item v-for="link in links" :key="link.target">{{
        link.text
      }}</v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const links = [
  { text: "Home", target: "/" },
  { text: "Classes", target: "/classes" },
  { text: "Calendar", target: "/calendar" },
  { text: "Assignments", target: "/assignments" },
];

const drawer = ref(false);
</script>
