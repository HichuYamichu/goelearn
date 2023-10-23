<template>
  <h5 class="text-h5 text-center pa-3">Ban members</h5>
  <v-text-field
    variant="outlined"
    label="Search"
    v-model="memberSearch"
  ></v-text-field>
  <v-list class="pa-0">
    <v-list-item v-for="(member, idx) in filteredMembers!" :key="member.id">
      <div class="d-flex">
        <h3 class="mr-8">
          {{ member.username }}
        </h3>
        <v-btn class="bg-error" @click="ban(member)">Ban</v-btn>
      </div>
    </v-list-item>
  </v-list>
  <h5 class="text-h5 text-center pa-3">Unban members</h5>
  <v-text-field
    variant="outlined"
    label="Search"
    v-model="bannedMemberSearch"
  ></v-text-field>
  <v-list class="pa-0">
    <v-list-item v-for="(member, idx) in bannedMembers!" :key="member.id">
      <div class="d-flex">
        <h3 class="mr-8">
          {{ member.username }}
        </h3>
        <v-btn class="bg-success" @click="unban(member)">Unban</v-btn>
      </div>
    </v-list-item>
  </v-list>
</template>

<script lang="ts" setup>
import { computed, ref, toRef, watch } from "vue";
import { useMutation, useQuery, useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { reactive } from "vue";
import { graphql } from "@/gql";

const MeQuery = graphql(/* GraphQL */ `
  query MeetingMeQuery {
    me {
      id
    }
  }
`);

const { result: meResult } = useQuery(MeQuery);
const myId = computed(() => meResult.value?.me.id ?? "");

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;

interface Member {
  id: string;
  username: string;
}

const props = defineProps<{
  members?: Member[] | null;
}>();

const memberSearch = ref("");
const bannedMemberSearch = ref("");
const members = toRef(props, "members");
const filteredMembers = computed(() => {
  if (!members.value) return [];
  return members.value
    .filter((member) => member.username.includes(memberSearch.value))
    .filter((member) => member.id != myId.value);
});

const BannedMemberQuery = graphql(/* GraphQL */ `
  query BannedMemberQuery($classId: ID!) {
    bannedMembers(classId: $classId) {
      id
      username
    }
  }
`);

const { result: bannedMembersResult } = useQuery(BannedMemberQuery, {
  classId,
});

const bannedMembers = computed(
  () => bannedMembersResult.value?.bannedMembers ?? []
);

const BanMemberMutation = graphql(/* GraphQL */ `
  mutation BanMemberMutation($classId: ID!, $userId: ID!) {
    banMember(classId: $classId, userId: $userId)
  }
`);

const { mutate: banMember } = useMutation(BanMemberMutation, {
  refetchQueries: ["BannedMemberQuery"],
});

const ban = (member: Member) => {
  banMember({ classId, userId: member.id });
};

const UnbanMemberMutation = graphql(/* GraphQL */ `
  mutation UnbanMemberMutation($classId: ID!, $userId: ID!) {
    unbanMember(classId: $classId, userId: $userId)
  }
`);

const { mutate: unbanMember } = useMutation(UnbanMemberMutation, {
  refetchQueries: ["BannedMemberQuery"],
});

const unban = (member: Member) => {
  unbanMember({ classId, userId: member.id });
};
</script>
