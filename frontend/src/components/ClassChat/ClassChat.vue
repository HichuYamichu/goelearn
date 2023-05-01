<template>
  <ChannelList
    @changeSelectedChannelId="changeSelectedChannelId"
    :selectedChannelId="selectedChannelId"
    :loading="loading"
    :channels="class_?.channels"
  ></ChannelList>

  <div ref="messageBox" style="height: 100%" class="overflow-auto">
    <v-virtual-scroll :items="messages" height="100%" item-height="50">
      <template v-slot:default="{ item }">
        <v-list-item
          prependAvatar="https://cdn.vuetifyjs.com/images/lists/1.jpg"
        >
          <v-list-item-title v-text="item.id"></v-list-item-title>
          {{ item.content }}
        </v-list-item>
      </template>
    </v-virtual-scroll>
  </div>

  <UsersList></UsersList>

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
  </v-footer>
</template>

<script lang="ts" setup>
import ChannelList from "@/components/ClassChat/ChannelList.vue";
import UsersList from "@/components/ClassChat/UsersList.vue";
import { FragmentType, graphql, useFragment } from "@/gql";
import { ChatFragmentFragment } from "@/gql/graphql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import gql from "graphql-tag";
import { nextTick } from "vue";
import { Ref, onMounted, reactive, watch } from "vue";
import { computed, ref, toRef } from "vue";

const ChatFragment = graphql(/* GraphQL */ `
  fragment ChatFragment on ClassObject {
    description
    channels {
      id
      ...ChannelsFragment
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

const MessagesQuery = graphql(/* GraphQL */ `
  query MessagesQuery($classId: ID!, $channelId: ID!) {
    messages(classId: $classId, channelId: $channelId) {
      nodes {
        id
        content
      }
    }
  }
`);

const { result, onResult, subscribeToMore } = useQuery(
  MessagesQuery,
  () => ({
    channelId: selectedChannelId.value!,
    classId: "",
  }),
  () => ({
    enabled: !!selectedChannelId.value,
  })
);

const messages = computed(() => result.value?.messages.nodes ?? []);
watch(messages, () => {
  console.log({ messages: messages.value });
  nextTick(() => {
    messageBox.value!.scrollTop = messageBox.value!.scrollHeight;
  });
});

const messageBox: Ref<HTMLDivElement | null> = ref(null);
onResult(() => {
  nextTick(() => {
    messageBox.value!.scrollTop = messageBox.value!.scrollHeight;
  });
});

const MessageCreatedSubscription = graphql(/* GraphQL */ `
  subscription MessagesSubscription($channelId: ID!) {
    messageCreated(channelId: $channelId) {
      id
      content
    }
  }
`);

subscribeToMore(() => ({
  document: MessageCreatedSubscription,
  variables: {
    channelId: selectedChannelId.value!,
  },
  updateQuery: (prev, { subscriptionData }) => {
    if (!subscriptionData.data) return prev;
    const newMessage = subscriptionData.data.messageCreated;
    return {
      ...prev,
      messages: {
        ...prev.messages,
        nodes: [...prev.messages.nodes, newMessage],
      },
    };
  },
}));

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
