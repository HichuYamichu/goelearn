<template>
  <v-container class="ma-0 pa-0 fill-height" fluid fill-height>
    <v-row justify="space-between" no-gutters class="fill-height">
      <v-col cols="2">
        <ChannelList
          @changeSelectedChannelId="changeSelectedChannelId"
          :selectedChannelId="selectedChannelId"
          :loading="loading"
          :channels="class_?.channels"
        ></ChannelList>
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="8" class="h d-flex flex-column">
        <MessageList :selectedChannelId="selectedChannelId"></MessageList>
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="2">
        <MemberList :users="class_?.members" :loading="loading"></MemberList>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import ChannelList from "@/components/ClassChat/ChannelList.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import MessageList from "./MessageList.vue";
import { FragmentType, graphql, useFragment } from "@/gql";
import { Ref, onMounted, reactive, watch } from "vue";
import { computed, ref, toRef } from "vue";

const ChatFragment = graphql(/* GraphQL */ `
  fragment ChatFragment on Class {
    description
    channels {
      id
      ...ChannelsFragment
    }
    members {
      ...MembersFragment
    }
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof ChatFragment> | null;
  loading: boolean;
}>();

const class_ = ref(useFragment(ChatFragment, props.class_));
const selectedChannelId = ref(class_.value?.channels[0]?.id ?? null);

watch(props, () => {
  class_.value = useFragment(ChatFragment, props.class_);
  selectedChannelId.value = class_.value?.channels[0]?.id ?? null;
});

const changeSelectedChannelId = (channelId: string) => {
  selectedChannelId.value = channelId;
};
</script>

<style scoped>
.h {
  max-height: calc(100vh- 112px) !important;
}
</style>
