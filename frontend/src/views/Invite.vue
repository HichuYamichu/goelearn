<template>
  <div v-if="!_class">
    <div class="d-flex justify-center align-center flex-wrap w-100 pa-16">
      <h2 class="text-h2 text-center w-100">
        Your invite is invalid or has expired
      </h2>
    </div>
  </div>
  <div v-else class="d-flex justify-center align-center flex-wrap w-100 pa-16">
    <h2 class="text-h2 text-center w-100">
      You are about to join: {{ _class.name }}
    </h2>
    <p class="text-subtitle-1 text-center w-50 mt-4">
      {{ _class.description }}
    </p>
    <div class="d-flex w-100 pa-4">
      <div class="center d-flex gap">
        <v-btn class="bg-success center" @click="joinClass">Confirm</v-btn>
        <v-btn class="bg-error" @click="cancel">Cancel</v-btn>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const route = useRoute();
const inviteId = route.params.inviteId as string;

const InviteClassQuery = graphql(/* GraphQL */ `
  query InviteClassQuery($inviteId: ID!) {
    classByInviteId(inviteId: $inviteId) {
      id
      name
      description
    }
  }
`);

const { result } = useQuery(InviteClassQuery, () => ({
  inviteId: inviteId,
}));

const _class = computed(() => result.value?.classByInviteId ?? null);

const JoinClassMutation = graphql(/* GraphQL */ `
  mutation JoinClassMutation($inviteId: ID!, $classId: ID!) {
    joinClass(inviteId: $inviteId, classId: $classId)
  }
`);

const { mutate } = useMutation(JoinClassMutation);

const joinClass = async () => {
  await mutate({
    inviteId: inviteId,
    classId: _class.value!.id,
  });
  router.push(`/class/${_class.value!.id}`);
};

const cancel = () => {
  router.push("/");
};
</script>

<style scoped>
.main-wrapper {
  width: 60%;
}

.center {
  margin: auto;
}

.gap {
  gap: 15px;
}
</style>
