<template>
  <h5 class="text-h5 text-center pa-3">Update Channels</h5>
  <v-list class="pa-0">
    <v-list-item v-for="(channel, idx) in channels!" :key="channel.id">
      <div class="d-flex">
        <v-text-field
          variant="outlined"
          :placeholder="channel.name"
          v-model="channel.name"
        ></v-text-field>
        <v-text-field
          variant="outlined"
          :placeholder="channel.description"
          v-model="channel.description"
        ></v-text-field>
        <div class="d-flex align-center">
          <v-btn @click="save(channel)">Save</v-btn>
          <v-btn @click="delete_(channel)">Delete</v-btn>
        </div>
      </div>
    </v-list-item>
  </v-list>
  <div class="mt-4">
    <h5 class="text-h5 text-center pa-3">Create Channel</h5>
    <div class="d-flex">
      <v-text-field
        variant="outlined"
        label="Channel Name"
        v-model="newChannelName"
      ></v-text-field>
      <v-text-field
        variant="outlined"
        label="Channel Description"
        v-model="newChannelDescription"
      ></v-text-field>
      <div class="d-flex align-center">
        <v-btn @click="create">Create</v-btn>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, toRef, watch } from "vue";
import { useMutation, useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { reactive } from "vue";
import { graphql } from "@/gql";

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

const create = () => {
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
</script>
