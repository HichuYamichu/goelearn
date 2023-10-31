<template>
  <v-app-bar density="compact" border flat>
    <v-skeleton-loader v-if="loading" type="heading"> </v-skeleton-loader>
    <h5 v-else class="text-h5 mx-4 pa-3 font-weight-medium d-none d-sm-flex">
      {{ name }}
    </h5>
    <v-divider vertical></v-divider>
    <v-tabs v-model="tabValue">
      <v-tab value="Chat">Chat</v-tab>
      <v-tab value="Files">Files</v-tab>
      <v-tab value="Assignments">Assignments</v-tab>
      <v-tab value="Meeting">Meeting</v-tab>
      <v-tab value="Settings" v-if="isOwner">Settings</v-tab>
      <v-tab value="UserSettings" v-else>Settings</v-tab>
    </v-tabs>
  </v-app-bar>

  <v-window v-model="tabValue" class="fill-height">
    <v-window-item value="Chat" class="fill-height">
      <ClassChat :class_="class_" :isActive="tabValue == 'Chat'"></ClassChat>
    </v-window-item>
    <v-window-item value="Files" class="fill-height">
      <ClassFiles :class_="class_"></ClassFiles>
    </v-window-item>
    <v-window-item value="Assignments" class="fill-height">
      <ClassAssignments :class_="class_"></ClassAssignments>
    </v-window-item>
    <v-window-item value="Meeting">
      <ClassMeeting :class_="class_"></ClassMeeting>
    </v-window-item>
    <v-window-item value="Settings">
      <ClassSettings :class_="class_"></ClassSettings>
    </v-window-item>
    <v-window-item value="UserSettings">
      <UserClassSettings :class_="class_"></UserClassSettings>
    </v-window-item>
  </v-window>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import ClassChat from "@/components/ClassChat/ClassChat.vue";
import ClassFiles from "@/components/ClassFiles.vue";
import ClassAssignments from "@/components/ClassAssignments.vue";
import ClassSettings from "@/components/ClassSettings.vue";
import ClassMeeting from "@/components/ClassMeeting.vue";
import { graphql, useFragment } from "@/gql";
import { useQuery, useSubscription } from "@vue/apollo-composable";
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { ChannelsFragmentFragment, ChatFragmentFragment } from "@/gql/graphql";
import { FragmentType } from "@/gql";
import { cache } from "@/client";
import { MyIdQuery } from "@/shared";
import UserClassSettings from "@/components/UserClassSettings.vue";

const MeQuery = graphql(/* GraphQL */ `
  query MeetingMeQuery {
    me {
      id
    }
  }
`);

const { result: meResult } = useQuery(MeQuery);
const myId = computed(() => meResult.value?.me.id ?? "");

const route = useRoute();
const router = useRouter();

const tabValue = ref("Chat");

const ClassQuery = graphql(/* GraphQL */ `
  query ClassClassByIdQuery($id: ID!) {
    classById(id: $id) {
      id
      name
      ownerId
      ...ChatFragment
      ...FilesFragment
      ...AssignmentsFragment
      ...MeetingFragment
      ...ClassDataFragment
    }
  }
`);

const { result, loading, onResult, refetch, subscribeToMore } = useQuery(
  ClassQuery,
  () => ({
    id: route.params.classId as string,
  }),
  {
    fetchPolicy: "network-only",
  }
);

const class_ = computed(() => result.value?.classById ?? null);
const name = computed(() => class_.value?.name ?? "");

const FileFragment = graphql(/* GraphQL */ `
  fragment FileFragment on File {
    id
    name
    fileType
    parent
  }
`);

const AssignmentFragment = graphql(/* GraphQL */ `
  fragment AssignmentFragment on Assignment {
    id
    name
    content
    dueAt
    submissions {
      id
      createdAt
      updatedAt
      user {
        id
        username
      }
      files {
        id
        name
      }
      feedback {
        id
        content
        createdAt
        updatedAt
      }
    }
  }
`);

const UserFragment = graphql(/* GraphQL */ `
  fragment UserFragment on User {
    id
    username
  }
`);

const ClassResourceCreateSubscription = graphql(/* GraphQL */ `
  subscription ClassResourceCreateSubscription($classId: ID!) {
    classResourceCreated(classId: $classId) {
      __typename
      ... on Channel {
        ...ChannelsFragment
      }
      ... on File {
        ...FileFragment
      }
      ... on FileBatch {
        files {
          ...FileFragment
        }
      }
      ... on Assignment {
        ...AssignmentFragment
      }
      ... on User {
        ...UserFragment
      }
    }
  }
`);

subscribeToMore(() => ({
  document: ClassResourceCreateSubscription,
  variables: {
    classId: route.params.classId as string,
  },
  updateQuery: (prev, { subscriptionData }) => {
    if (!subscriptionData.data) return prev;
    const updatedClass = subscriptionData.data.classResourceCreated;

    if (updatedClass.__typename == "Channel") {
      return {
        classById: {
          ...prev.classById!,
          // @ts-ignore
          channels: [...prev.classById!.channels, updatedClass],
        },
      };
    }

    if (updatedClass.__typename == "Assignment") {
      return {
        classById: {
          ...prev.classById!,
          // @ts-ignore
          assignments: [...prev.classById!.assignments, updatedClass],
        },
      };
    }

    if (updatedClass.__typename == "File") {
      return {
        classById: {
          ...prev.classById!,
          // @ts-ignore
          files: [...prev.classById!.files, updatedClass],
        },
      };
    }

    if (updatedClass.__typename == "FileBatch") {
      console.log(updatedClass);
      return {
        classById: {
          ...prev.classById!,
          // @ts-ignore
          files: [...prev.classById!.files, ...updatedClass.files],
        },
      };
    }

    if (updatedClass.__typename == "User") {
      return {
        classById: {
          ...prev.classById!,
          // @ts-ignore
          members: [...prev.classById!.members, updatedClass],
        },
      };
    }

    return {
      ...prev,
    };
  },
}));

const ClassResourceUpdateSubscription = graphql(/* GraphQL */ `
  subscription ClassResourceUpdateSubscription($classId: ID!) {
    classResourceUpdated(classId: $classId) {
      __typename
      ... on Channel {
        ...ChannelsFragment
      }
      ... on Class {
        ...ClassDataFragment
      }
      ... on Assignment {
        ...AssignmentFragment
      }
    }
  }
`);

subscribeToMore(() => ({
  document: ClassResourceUpdateSubscription,
  variables: {
    classId: route.params.classId as string,
  },
  updateQuery: (prev, { subscriptionData }) => {
    if (!subscriptionData.data) return prev;
    const updatedClass = subscriptionData.data.classResourceUpdated;

    if (updatedClass.__typename == "Channel") {
      return {
        ...prev,
        channels: {
          // @ts-ignore
          ...prev.channels,
          ...updatedClass,
        },
      };
    }

    if (updatedClass.__typename == "Class") {
      return {
        ...prev,
        ...updatedClass,
      };
    }

    if (updatedClass.__typename == "Assignment") {
      console.log(updatedClass);

      return {
        ...prev,
        assignments: {
          // @ts-ignore
          ...prev.assignments,
          ...updatedClass,
        },
      };
    }

    return {
      ...prev,
    };
  },
}));

const ClassResourceDeletedSubscription = graphql(/* GraphQL */ `
  subscription ClassResourceDeletedSubscription($classId: ID!) {
    classResourceDeleted(classId: $classId) {
      __typename
      ... on ChannelDeleteInfo {
        id
      }
      ... on AssignmentDeleteInfo {
        id
      }
      ... on FileDeleteInfo {
        id
      }
      ... on MemberDeleteInfo {
        id
      }
    }
  }
`);

const classId = route.params.classId as string;

const { result: onDelete } = useSubscription(
  ClassResourceDeletedSubscription,
  () => ({
    classId: classId,
  })
);
watch(onDelete, () => {
  if (!onDelete.value) return;
  const data = onDelete.value.classResourceDeleted;

  if (data.__typename == "ChannelDeleteInfo") {
    cache.evict({ id: `Channel:${data.id}` });
  }
  if (data.__typename == "FileDeleteInfo") {
    cache.evict({ id: `File:${data.id}` });
  }
  if (data.__typename == "AssignmentDeleteInfo") {
    cache.evict({ id: `Assignment:${data.id}` });
  }

  if (data.__typename == "MemberDeleteInfo") {
    cache.modify({
      id: `Class:${classId}`,
      fields: {
        members(cachedMembers) {
          return cachedMembers.filter((m: any) => m.__ref != `User:${data.id}`);
        },
      },
    });

    if (data.id == myId.value) {
      router.push({ name: "Home" });
      cache.evict({ id: `Class:${classId}` });
    }
  }
});

const { result: myIdResult } = useQuery(MyIdQuery);
const isOwner = computed(() => {
  if (!myIdResult.value?.me?.id) return false;
  return myIdResult.value?.me?.id === class_.value?.ownerId;
});

const ClassDeletedSubscription = graphql(/* GraphQL */ `
  subscription ClassDeletedSubscription($classId: ID!) {
    classDeleted(classId: $classId) {
     id
    }
  }
`);

const { onResult  } = useSubscription(ClassDeletedSubscription);

onResult(result => {
  const receivedId = result.value.id;
  if (receivedId === classId) {
    router.push({ name: "Home" });
    cache.evict({ id: `Class:${classId}` });
  }
})

</script>
