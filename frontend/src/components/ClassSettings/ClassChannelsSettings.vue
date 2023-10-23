<template>
  <div class="w-50 w-xs-100">
    <h5 class="text-h5 text-center pa-3">Create Channel</h5>
    <div class="d-flex .justify-space-between">
      <div class="d-flex w-100">
        <form @submit.prevent="create" class="w-100">
          <v-text-field
            class="w-100"
            variant="outlined"
            label="Channel Name"
            v-model="newChannelName"
            :rules="channelNameRules"
            ref="channelName"
          ></v-text-field>
          <v-text-field
            variant="outlined"
            class="w-100"
            label="Channel Description"
            v-model="newChannelDescription"
            :rules="channelDescRules"
            ref="channelDesc"
          ></v-text-field>
          <div class="w-25 d-flex align-center">
            <v-btn class="bg-success block" type="submit">Create</v-btn>
          </div>
        </form>
      </div>
    </div>
  </div>
  <div class="w-50 w-xs-100">
    <h5 class="text-h5 text-center pa-3">Update Channels</h5>
    <div
      class="d-flex mb-4"
      v-for="(channel, idx) in channels!"
      :key="channel.id"
    >
      <form @submit.prevent="save(channel)" class="w-100">
        <v-text-field
          variant="outlined"
          :placeholder="channel.name"
          v-model="channel.name"
          :rules="channelDescRules"
          ref="channelDesc"
        ></v-text-field>
        <v-text-field
          variant="outlined"
          :placeholder="channel.description"
          v-model="channel.description"
          :rules="channelDescRules"
          ref="channelDesc"
        ></v-text-field>
        <div class="d-flex align-center">
          <v-btn class="bg-success mr-4" type="submit">Save</v-btn>
          <v-btn class="bg-error" @click="delete_(channel)">Delete</v-btn>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Ref, VNodeRef, computed, ref, toRef, watch } from "vue";
import { useMutation, useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { reactive } from "vue";
import { graphql } from "@/gql";
import { validate } from "@/shared";

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;

interface Channel {
  id: string;
  name: string;
  description: string;
}

const props = defineProps<{
  channels?: Channel[] | null;
}>();

const channels = toRef(props, "channels");
const updateChannels = reactive(
  new Array<Channel>(channels.value!.length).fill({
    name: "",
    description: "",
  } as Channel)
);

const UpdateChannelMutation = graphql(/* GraphQL */ `
  mutation UpdateChannelMutation($input: UpdateChannelInput!) {
    updateChannel(input: $input) {
      id
    }
  }
`);

const { mutate: updateChannel } = useMutation(UpdateChannelMutation);

const save = (channel: Channel) => {
  const isValid = validate([channelName, channelDesc]);
  if (!isValid) {
    return;
  }

  updateChannel({
    input: {
      id: channel.id,
      name: channel.name,
      description: channel.description,
      classId: classId,
    },
  });
};

const newChannelName = ref("");
const newChannelDescription = ref("");

const CreateChannelMutation = graphql(/* GraphQL */ `
  mutation CreateChannelMutation($input: CreateChannelInput!) {
    createChannel(input: $input) {
      id
    }
  }
`);

const { mutate: createChannel } = useMutation(CreateChannelMutation);

const channelName = ref(null);
const channelDesc = ref(null);

const create = async () => {
  const isValid = validate([channelName, channelDesc]);
  if (!isValid) {
    return;
  }

  createChannel({
    input: {
      name: newChannelName.value,
      description: newChannelDescription.value,
      classId: classId,
      allowMembersToPost: true,
    },
  });
};

const DeleteChannelMutation = graphql(/* GraphQL */ `
  mutation DeleteChannelMutation($classId: ID!, $channelId: ID!) {
    deleteChannel(classId: $classId, channelId: $channelId)
  }
`);

const { mutate: deleteChannel } = useMutation(DeleteChannelMutation);

const delete_ = (channel: Channel) => {
  deleteChannel({
    classId: classId,
    channelId: channel.id,
  });
};

const channelNameRules = computed(() => {
  return [
    (v: string) => !!v || "Channel name is required",
    (v: string) => v.length <= 15 || "Channel name must be less than 20",
  ];
});

const channelDescRules = computed(() => {
  return [
    (v: string) =>
      v.length <= 100 || "Channel description must be less than 100",
  ];
});
</script>
