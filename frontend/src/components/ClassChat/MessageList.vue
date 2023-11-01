<template>
  <v-card class="d-flex flex-column h">
    <v-virtual-scroll
      :items="messages"
      ref="messageBox"
      v-on:scroll.native="handleScroll"
    >
      <template v-slot:default="{ item }">
        <v-list-item
          class="py-2"
          :prependAvatar="`${baseURL}/files/user-avatar/${item.author.id}`"
        >
          <v-list-item-title
            class="font-weight-bold"
            v-text="item.author.username"
          ></v-list-item-title>
          {{ item.content }}
          <template v-slot:append>
            <v-list-item-subtitle
              class="font-weight-light"
              v-text="new Date(item.createdAt).toLocaleString()"
            ></v-list-item-subtitle>
          </template>
        </v-list-item>
      </template>
    </v-virtual-scroll>
  </v-card>
  <div class="d-flex">
    <v-btn
      variant="outlined"
      height="100%"
      size="small"
      class="hidden-md-and-up"
      @click="emit('toggleChannelDrawer')"
      ><v-icon icon="$vuetify"></v-icon
    ></v-btn>
    <v-textarea
      rows="1"
      no-resize
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
    ></v-textarea>
    <v-btn
      variant="outlined"
      height="100%"
      size="small"
      class="hidden-md-and-up"
      @click="emit('toggleMemberDrawer')"
      ><v-icon icon="$vuetify"></v-icon
    ></v-btn>
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { Ref, computed, nextTick, ref, toRef, watch } from "vue";
import { useRoute } from "vue-router";
const baseURL = import.meta.env.VITE_BASE_ENDPOINT;

const route = useRoute();

const classId = route.params.classId as string;

const emit = defineEmits(["toggleChannelDrawer", "toggleMemberDrawer"]);

const props = defineProps<{
  selectedChannelId?: string | null;
}>();

const MeQuery = graphql(/* GraphQL */ `
  query MessageListMeQuery {
    me {
      id
    }
  }
`);

const { result: meResult } = useQuery(MeQuery);

const id = computed(() => meResult.value?.me.id ?? "");

const handleScroll = (e: Event) => {
  const target = e.target as HTMLDivElement;
  if (target.scrollTop === 0) {
    fetchMore({
      variables: {
        channelId: selectedChannelId.value!,
        before: edges.value[0].cursor,
        last: 15,
      },
      updateQuery: (prev, { fetchMoreResult }) => {
        if (!fetchMoreResult) {
          return prev;
        }

        const newEdges = fetchMoreResult.messages.edges;
        const newNodes = fetchMoreResult.messages.nodes;
        const pageInfo = fetchMoreResult.messages.pageInfo;

        return {
          ...prev,
          messages: {
            ...prev.messages,
            nodes: [...newNodes, ...prev.messages.nodes],
            edges: [...newEdges, ...prev.messages.edges],
            pageInfo,
          },
        };
      },
    });
  }
};

const selectedChannelId = toRef(props, "selectedChannelId");

const MessageFragment = graphql(/* GraphQL */ `
  fragment MessageFragment on Message {
    id
    content
    createdAt
    author {
      id
      username
    }
  }
`);

const MessagesQuery = graphql(/* GraphQL */ `
  query MessagesQuery(
    $classId: ID!
    $channelId: ID!
    $before: String
    $last: Int
  ) {
    messages(
      classId: $classId
      channelId: $channelId
      before: $before
      last: $last
    ) {
      nodes {
        ...MessageFragment
      }
      edges {
        cursor
        node {
          ...MessageFragment
        }
      }
      pageInfo {
        hasNextPage
        hasPreviousPage
      }
    }
  }
`);

const { result, onResult, subscribeToMore, refetch, fetchMore } = useQuery(
  MessagesQuery,
  () => ({
    channelId: selectedChannelId.value!,
    classId: classId,
    last: 15,
  }),
  () => ({
    enabled: !!selectedChannelId.value,
  })
);

const messages = computed(() => result.value?.messages.nodes ?? []);
const edges = computed(() => result.value?.messages.edges ?? []);

const messageBox = ref();
let isFirstLoad = true;

watch(messages, (newMessages, oldMessages) => {
  if (newMessages.length === 0) {
    return;
  }
  const newMessageAdded = newMessages.length > oldMessages.length;
  const newestMessage = newMessages[newMessages.length - 1];
  // @ts-ignore graphql-codegen is trash
  const isMyMessage = newestMessage.author.id === id.value;
  const wasAtBottom =
    messageBox.value?.$el.scrollHeight -
      messageBox.value?.$el.scrollTop -
      messageBox.value?.$el.clientHeight <
    10;
  const wasAtTop = messageBox.value?.$el.scrollTop < 10;

  if (isFirstLoad && newMessageAdded) {
    isFirstLoad = false;
    nextTick(() => {
      messageBox.value?.$el.scrollTo(
        0,
        messageBox.value?.$el.scrollHeight + 100
      );
    });
    return;
  }

  if (!wasAtTop && newMessageAdded && (isMyMessage || wasAtBottom)) {
    nextTick(() => {
      scrollDown();
    });
  }

  if (wasAtTop && newMessageAdded) {
    nextTick(() => {
      scrollMiddle();
    });
  }
});

const scrollDown = () => {
  messageBox.value?.scrollToIndex(messages.value.length - 1);
};

const scrollMiddle = () => {
  messageBox.value?.scrollToIndex(1);
};

const MessageCreatedSubscription = graphql(/* GraphQL */ `
  subscription MessagesSubscription($channelId: ID!, $classId: ID!) {
    messageCreated(channelId: $channelId, classId: $classId) {
      ...MessageFragment
    }
  }
`);

subscribeToMore(() => ({
  document: MessageCreatedSubscription,
  variables: {
    channelId: selectedChannelId.value!,
    classId: classId,
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
  mutation SendMessage($channelId: ID!, $content: String!, $classId: ID!) {
    createMessage(
      input: { channelId: $channelId, content: $content, classId: $classId }
    ) {
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
    classId: classId,
  });
  msg.value = "";
};
</script>

<style scoped>
.h {
  height: calc(100vh - 174px) !important;
  width: 100%;
}
</style>
