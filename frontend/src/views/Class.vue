<template>
  <v-app-bar density="compact" border flat>
    <v-skeleton-loader v-if="loading" type="heading"> </v-skeleton-loader>
    <h5 v-else class="text-h5 mx-4 font-weight-medium">
      {{ name }}
    </h5>
    <v-tabs v-model="tabValue">
      <v-tab value="Chat">Chat</v-tab>
      <v-tab value="Files">Files</v-tab>
      <v-tab value="Meeting">Meeting</v-tab>
      <v-tab value="Settings">Settings</v-tab>
    </v-tabs>
  </v-app-bar>

  <v-window v-model="tabValue">
    <v-window-item value="Chat">
      <ClassChat :loading="loading" :class_="class_"></ClassChat>
    </v-window-item>
    <v-window-item value="Files" class="pa-4">
      <ClassFiles></ClassFiles>
    </v-window-item>
    <v-window-item value="Meeting"> </v-window-item>
    <v-window-item value="Settings"> </v-window-item>
  </v-window>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import ClassChat from "@/components/ClassChat/ClassChat.vue";
import ClassFiles from "@/components/ClassFiles.vue";
import { graphql, useFragment } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const tabValue = ref("Chat");

const ClassQuery = graphql(/* GraphQL */ `
  query ClassClassByIdQuery($id: ID!) {
    classById(id: $id) {
      name
      ...ChatFragment
    }
  }
`);

const { result, loading, onResult, refetch } = useQuery(
  ClassQuery,
  () => ({
    id: route.params.classId as string,
  }),
  {
    fetchPolicy: "network-only",
  }
);

const class_ = computed(() => result.value?.classById ?? null);
const name = computed(() => class_.value?.name ?? "");
</script>
