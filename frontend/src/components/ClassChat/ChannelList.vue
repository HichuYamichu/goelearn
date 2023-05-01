<template>
  <v-navigation-drawer width="244">
    <v-skeleton-loader v-if="loading" type="list-item"> </v-skeleton-loader>
    <v-list v-else class="pa-0">
      <v-list-item
        v-for="channel in channels!"
        :key="channel.id"
        :title="channel.name"
        link
        :active="selectedChannelId == channel.id"
        @click="$emit('changeSelectedChannelId', channel.id)"
      >
      </v-list-item>
    </v-list>
    <v-divider></v-divider>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import { FragmentType, graphql, useFragment } from "@/gql";
import { computed, ref, watch } from "vue";

const ChannelsFragment = graphql(/* GraphQL */ `
  fragment ChannelsFragment on ChannelObject {
    id
    name
  }
`);

const props = defineProps<{
  channels?: FragmentType<typeof ChannelsFragment>[] | null;
  selectedChannelId?: string | null;
  loading: boolean;
}>();

const channels = ref(useFragment(ChannelsFragment, props.channels));
watch(props, () => {
  channels.value = useFragment(ChannelsFragment, props.channels);
});
</script>
