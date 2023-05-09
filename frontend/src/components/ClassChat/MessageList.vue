<template>
  <div ref="messageBox" class="overflow-y-visible">
    <v-virtual-scroll :items="messages" height="100%" item-height="50">
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
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
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
    fetchPolicy: "cache-and-network",
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
    console.log({ newMessage });
    return {
      ...prev,
      messages: {
        ...prev.messages,
        nodes: [...prev.messages.nodes, newMessage],
      },
    };
  },
}));
</script>
