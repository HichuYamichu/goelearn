<template>
  <ChannelList
    @changeSelectedChannelId="changeSelectedChannelId"
    :selectedChannelId="selectedChannelId"
    :loading="loading"
    :channels="class_?.channels"
  ></ChannelList>

  <MessageList :selectedChannelId="selectedChannelId"></MessageList>

  <MemberList :users="class_?.members" :loading="loading"></MemberList>

  <v-footer app height="72">
    <v-text-field
      bg-color="grey-lighten-1"
      class="rounded-pill overflow-hidden"
      density="compact"
      hide-details
      variant="solo"
      v-model="msg"
      @keyup.enter.native="sendMsg"
    ></v-text-field>
    <!-- <v-text-field
      v-model="message"
      :append-icon="message ? 'mdi-send' : 'mdi-microphone'"
      :append-inner-icon="marker ? 'mdi-map-marker' : 'mdi-map-marker-off'"
      :prepend-icon="icon"
      variant="filled"
      clear-icon="mdi-close-circle"
      clearable
      label="Message"
      type="text"
      @click:append-inner="toggleMarker"
      @click:append="sendMessage"
      @click:prepend="changeIcon"
      @click:clear="clearMessage"
    ></v-text-field> -->
  </v-footer>
</template>

<script lang="ts" setup>
import ChannelList from "@/components/ClassChat/ChannelList.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import MessageList from "./MessageList.vue";
import { FragmentType, graphql, useFragment } from "@/gql";
import { ChatFragmentFragment } from "@/gql/graphql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import gql from "graphql-tag";
import { nextTick } from "vue";
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

const SendMessageMutation = graphql(/* GraphQL */ `
  mutation SendMessage($channelId: ID!, $content: String!) {
    createMessage(input: { channelId: $channelId, content: $content }) {
      id
      content
    }
  }
`);

const { mutate: send } = useMutation(SendMessageMutation);

const msg = ref("");
const sendMsg = () => {
  send({
    channelId: selectedChannelId.value!,
    content: msg.value,
  });
  msg.value = "";
};
</script>
