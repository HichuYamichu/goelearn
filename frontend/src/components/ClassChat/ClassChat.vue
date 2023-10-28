<template>
  <v-navigation-drawer
    v-model="shouldShowChannelDrawer"
    :permanent="!mobile"
    touchless
  >
    <ChannelList
      @changeSelectedChannelId="changeSelectedChannelId"
      :selectedChannelId="selectedChannelId"
      :channels="class_?.channels"
    ></ChannelList>
  </v-navigation-drawer>
  <v-container class="ma-0 pa-0 fill-height" fluid fill-height ref="chat">
    <v-row justify="space-between" no-gutters class="fill-height">
      <v-col cols="12" class="h d-flex flex-column">
        <MessageList
          @toggleChannelDrawer="toggleChannelDrawer"
          @toggleMemberDrawer="toggleMemberDrawer"
          :selectedChannelId="selectedChannelId"
        ></MessageList>
      </v-col>
    </v-row>
  </v-container>
  <v-navigation-drawer
    v-model="shouldShowMemberDrawer"
    :permanent="!mobile"
    touchless
    location="right"
  >
    <MemberList :users="class_?.members"></MemberList>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import ChannelList from "@/components/ClassChat/ChannelList.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import MessageList from "./MessageList.vue";
import { FragmentType, graphql, useFragment } from "@/gql";
import { Ref, onMounted, reactive, watch } from "vue";
import { computed, ref, toRef } from "vue";
import { useDisplay } from "vuetify";

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
  isActive: boolean;
}>();

const class_ = computed(() => useFragment(ChatFragment, props.class_));
const selectedChannelId = ref(class_.value?.channels[0]?.id ?? null);

watch(class_, () => {
  selectedChannelId.value = class_.value?.channels[0]?.id ?? null;
});

const changeSelectedChannelId = (channelId: string) => {
  selectedChannelId.value = channelId;
};

const { mobile } = useDisplay();
const channelDrawer = ref(!mobile.value);
const toggleChannelDrawer = () => {
  channelDrawer.value = !channelDrawer.value;
};
const shouldShowChannelDrawer = computed({
  get: () => {
    return channelDrawer.value && props.isActive;
  },
  set: (value) => {
    channelDrawer.value = value;
  },
});

const memberDrawer = ref(!mobile.value);
const toggleMemberDrawer = () => {
  memberDrawer.value = !memberDrawer.value;
};
const shouldShowMemberDrawer = computed({
  get: () => {
    return memberDrawer.value && props.isActive;
  },
  set: (value) => {
    memberDrawer.value = value;
  },
});
</script>

<style scoped>
.h {
  max-height: calc(100vh- 112px) !important;
}
</style>
