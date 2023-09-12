<template>
  <v-app-bar density="compact" border flat>
    <v-skeleton-loader v-if="loading" type="heading"> </v-skeleton-loader>
    <h5 v-else class="text-h5 mx-4 pa-3 font-weight-medium d-none d-sm-flex">
      {{ name }}
    </h5>
    <v-divider vertical></v-divider>
    <v-tabs v-model="tabValue">
      <v-tab value="Chat">Chat</v-tab>
      <v-tab value="Files">Files</v-tab>
      <v-tab value="Assignments">Assignments</v-tab>
      <v-tab value="Meeting">Meeting</v-tab>
      <v-tab value="Settings">Settings</v-tab>
    </v-tabs>
  </v-app-bar>

  <v-container fluid class="fill-height pa-0">
    <v-row class="fill-height" no-gutters>
      <v-col cols="12" class="fill-height">
        <v-window v-model="tabValue" class="fill-height">
          <v-window-item value="Chat" class="fill-height">
            <ClassChat :loading="loading" :class_="class_"></ClassChat>
          </v-window-item>
          <v-window-item value="Files" class="fill-height">
            <ClassFiles :loading="loading" :class_="class_"></ClassFiles>
          </v-window-item>
          <v-window-item value="Assignments" class="fill-height">
            <ClassAssignments
              :loading="loading"
              :class_="class_"
            ></ClassAssignments>
          </v-window-item>
          <v-window-item value="Meeting">
            <ClassMeeting :loading="loading" :class_="class_"></ClassMeeting>
          </v-window-item>
          <v-window-item value="Settings">
            <ClassSettings :loading="loading" :class_="class_"></ClassSettings>
          </v-window-item>
        </v-window>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import ClassChat from "@/components/ClassChat/ClassChat.vue";
import ClassFiles from "@/components/ClassFiles.vue";
import ClassAssignments from "@/components/ClassAssignments.vue";
import ClassSettings from "@/components/ClassSettings.vue";
import ClassMeeting from "@/components/ClassMeeting.vue";
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
      ...FilesFragment
      ...AssignmentsFragment
      ...MeetingFragment
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
