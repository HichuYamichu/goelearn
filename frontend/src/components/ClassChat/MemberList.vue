<template>
  <h5 class="text-h5 text-center pa-3">Students</h5>
  <v-skeleton-loader v-if="!hasUsers" type="list-item"> </v-skeleton-loader>
  <v-list v-else class="pa-0">
    <v-list-item
      v-for="user in users!"
      :key="user.id"
      link
      :prependAvatar="`${baseURL}/files/user-avatar/${user.id}`"
    >
      <v-tooltip :text="user.username" location="end">
        <template v-slot:activator="{ props }">
          <v-list-item-title
            v-bind="props"
            v-text="`${user.firstName} ${user.lastName}`"
          >
          </v-list-item-title>
        </template>
      </v-tooltip>
    </v-list-item>
  </v-list>
  <v-divider></v-divider>
</template>

<script lang="ts" setup>
import { FragmentType, graphql, useFragment } from "@/gql";
import { computed, ref, watch } from "vue";

const baseURL = import.meta.env.VITE_BASE_ENDPOINT;

const MembersFragment = graphql(/* GraphQL */ `
  fragment MembersFragment on User {
    id
    username
    firstName
    lastName
  }
`);

export interface Props {
  users?: FragmentType<typeof MembersFragment>[];
}

const props = withDefaults(defineProps<Props>(), {
  users: [] as any,
});

const users = computed(() => useFragment(MembersFragment, props.users));
const hasUsers = computed(() => users.value?.length ?? 0 > 0);
</script>
