<template>
  <h5 class="text-h5 text-center pa-3">Students</h5>
  <v-skeleton-loader v-if="!hasUsers" type="list-item"> </v-skeleton-loader>
  <v-list v-else class="pa-0">
    <v-list-item
      v-for="user in users!"
      :key="user.id"
      :title="user.username"
      link
      :prependAvatar="`http://localhost:3000/files/user-avatar/${user.id}`"
    >
    </v-list-item>
  </v-list>
  <v-divider></v-divider>
</template>

<script lang="ts" setup>
import { FragmentType, graphql, useFragment } from "@/gql";
import { computed, ref, watch } from "vue";

const MembersFragment = graphql(/* GraphQL */ `
  fragment MembersFragment on User {
    id
    username
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
