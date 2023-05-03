<template>
  <v-navigation-drawer location="right" width="244">
    <h5 class="text-h5 text-center pa-3">Members</h5>
    <v-skeleton-loader v-if="loading" type="list-item"> </v-skeleton-loader>
    <v-list v-else class="pa-0">
      <v-list-item
        v-for="user in users!"
        :key="user.id"
        :title="user.username"
        link
      >
      </v-list-item>
    </v-list>
    <v-divider></v-divider>
  </v-navigation-drawer>
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

const props = defineProps<{
  users?: FragmentType<typeof MembersFragment>[] | null;
  loading: boolean;
}>();

const users = ref(useFragment(MembersFragment, props.users));
watch(props, () => {
  users.value = useFragment(MembersFragment, props.users);
});
</script>
