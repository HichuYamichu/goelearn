<template>
  <h5 class="text-h5 text-center pa-3">Channels</h5>
  <v-skeleton-loader v-if="!hasChannels" type="list-item"> </v-skeleton-loader>
  <v-list v-else class="pa-0">
    <v-list-item
      v-for="channel in channels!"
      :key="channel.id"
      :title="channel.name"
      link
      :active="selectedChannelId == channel.id"
      @click="emit('changeSelectedChannelId', channel.id)"
    >
    </v-list-item>
  </v-list>
  <v-divider></v-divider>
</template>

<script lang="ts" setup>
import { FragmentType, graphql, useFragment } from "@/gql";
import { computed, ref, watch } from "vue";
import { useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { cache } from "@/client";

const ChannelsFragment = graphql(/* GraphQL */ `
  fragment ChannelsFragment on Channel {
    id
    name
  }
`);

const props = defineProps<{
  channels?: FragmentType<typeof ChannelsFragment>[] | null;
  selectedChannelId?: string | null;
}>();

const emit = defineEmits(["changeSelectedChannelId"]);

const channels = computed(() => useFragment(ChannelsFragment, props.channels));
const hasChannels = computed(() => channels.value?.length ?? 0 > 0);
</script>
