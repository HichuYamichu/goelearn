<template>
  <v-card flat class="pa-0 ma-xl-10">
    <div class="d-flex flex-lg-row flex-column">
      <v-tabs v-model="tab" direction="vertical" color="primary">
        <v-tab value="option-1">
          <v-icon start> mdi-account </v-icon>
          General
        </v-tab>
        <v-tab value="option-2">
          <v-icon start> mdi-access-point </v-icon>
          Channels
        </v-tab>
        <v-tab value="option-3">
          <v-icon start> mdi-account </v-icon>
          Members
        </v-tab>
        <v-tab value="option-4">
          <v-icon start> mdi-account </v-icon>
          Invites
        </v-tab>
      </v-tabs>
      <v-window v-model="tab" class="w-100">
        <v-window-item value="option-1">
          <v-card flat class="w-xs-100 w-50 px-8 pl-lg-8">
            <ClassCreate :class_="(class_ as any)"></ClassCreate>
          </v-card>
        </v-window-item>
        <v-window-item value="option-2">
          <v-card flat class="w-100 px-8 pl-lg-8">
            <ClassChannelsSettings
              :channels="(class_?.channels as any)"
            ></ClassChannelsSettings>
          </v-card>
        </v-window-item>
        <v-window-item value="option-3">
          <v-card flat class="w-xs-100 w-50 px-8 pl-lg-8">
            <ClassMembersSettings
              :members="(class_?.members as any)"
            ></ClassMembersSettings>
          </v-card>
        </v-window-item>
        <v-window-item value="option-4">
          <v-card flat class="w-xs-100 w-50 px-8 pl-lg-8">
            <ClassInvitesSettings></ClassInvitesSettings>
          </v-card>
        </v-window-item>
      </v-window>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import ClassCreate from "./ClassCreate.vue";
import ClassChannelsSettings from "./ClassSettings/ClassChannelsSettings.vue";
import ClassMembersSettings from "./ClassSettings/ClassMembersSettings.vue";
import ClassInvitesSettings from "./ClassSettings/ClassInvitesSettings.vue";
import { FragmentType, graphql, useFragment } from "@/gql";

const ClassDataFragment = graphql(/* GraphQL */ `
  fragment ClassDataFragment on Class {
    id
    name
    description
    tags
    public
    hasImage
    channels {
      id
      name
      description
    }
    members {
      id
      username
      firstName
      lastName
    }
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof ClassDataFragment> | null;
}>();

const class_ = computed(() => useFragment(ClassDataFragment, props.class_));

const tab = ref("option-1");
</script>
