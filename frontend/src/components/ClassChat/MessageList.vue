<template>
  <v-card class="d-flex flex-column flex-grow-1 h">
    <v-virtual-scroll :items="messages" ref="messageBox">
      <template v-slot:default="{ item }">
        <v-list-item
          class="py-2"
          :prependAvatar="`http://localhost:3000/files/user-avatar/${item.author.id}`"
        >
          <v-list-item-title
            class="font-weight-bold"
            v-text="item.author.username"
          ></v-list-item-title>
          {{ item.content }}
        </v-list-item>
      </template>
    </v-virtual-scroll>
  </v-card>
  <v-text-field
    v-model="msg"
    append-innerdasda-icon="mdi-send"
    variant="outlined"
    clear-icon="mdi-close-circle"
    clearable
    label="Message"
    type="text"
    hide-details="auto"
    @click:append="sendMsg"
    @keyup.enter.native="sendMsg"
  ></v-text-field>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { Ref, computed, nextTick, ref, toRef, watch } from "vue";

const props = defineProps<{
  selectedChannelId?: string | null;
}>();

const selectedChannelId = toRef(props, "selectedChannelId");

const MessageFragment = graphql(/* GraphQL */ `
  fragment MessageFragment on Message {
    id
    content
    author {
      id
      username
    }
  }
`);

const MessagesQuery = graphql(/* GraphQL */ `
  query MessagesQuery($classId: ID!, $channelId: ID!) {
    messages(classId: $classId, channelId: $channelId) {
      nodes {
        ...MessageFragment
      }
    }
  }
`);

const { result, onResult, subscribeToMore, refetch } = useQuery(
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

const messageBox = ref();
onResult(() => {
  console.log(result.value);
  nextTick(() => {
    scroll();
  });
});

const scroll = () => {
  messageBox.value?.scrollToIndex(messages.value.length - 1);
};

const MessageCreatedSubscription = graphql(/* GraphQL */ `
  subscription MessagesSubscription($channelId: ID!) {
    messageCreated(channelId: $channelId) {
      ...MessageFragment
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

<style scoped>
.h {
  height: calc(100vh - 170px);
}
</style>
