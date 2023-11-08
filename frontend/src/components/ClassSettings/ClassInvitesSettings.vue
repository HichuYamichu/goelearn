<template>
  <h2>Create invite</h2>
  <form class="my-2">
    <v-text-field
      variant="outlined"
      type="datetime-local"
      label="Due date"
      v-model="expiresAt"
    ></v-text-field>
    <v-checkbox label="Checkbox" v-model="multiuse"></v-checkbox>
    <v-btn class="bg-success" @click="create"> Create </v-btn>
  </form>

  <h2>Active invites</h2>
  <v-table density="compact" class="w-100">
    <thead>
      <tr>
        <th class="text-left font-weight-black" style="width: 50%">ID</th>
        <th class="text-left font-weight-black" style="width: 30%">
          Expires at
        </th>
        <th class="text-left font-weight-black" style="width: 10%">Multiuse</th>
        <th class="text-left font-weight-black" style="width: 10%">Action</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="invite in invites" :key="invite.id">
        <td>
          <p @click="copyInvite(invite.id)" class="pointer">
            {{ invite.id }}
          </p>
        </td>
        <td>{{ invite.expiresAt }}</td>
        <td>{{ invite.multiuse }}</td>
        <td class="d-flex">
          <v-btn class="bg-success" @click="copyInvite(invite.id)">Copy</v-btn>
          <v-btn class="bg-error" @click="delete_(invite.id)">Delete</v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>

<script lang="ts" setup>
import { computed, ref, toRef, watch } from "vue";
import { useMutation, useQuery, useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { reactive } from "vue";
import { graphql } from "@/gql";

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;

const InvitesQuery = graphql(/* GraphQL */ `
  query InvitesQuery($id: ID!) {
    invites(classId: $id) {
      id
      multiuse
      expiresAt
    }
  }
`);

const { result, loading, onResult, refetch, subscribeToMore } = useQuery(
  InvitesQuery,
  () => ({
    id: classId,
  })
);

const invites = computed(() => result.value?.invites ?? []);

const CreateInviteMutation = graphql(/* GraphQL */ `
  mutation CreateInviteMutation(
    $classId: ID!
    $multiuse: Boolean!
    $expiresAt: NaiveDateTime
  ) {
    createInvite(
      input: { classId: $classId, multiuse: $multiuse, expiresAt: $expiresAt }
    ) {
      id
      multiuse
      expiresAt
    }
  }
`);

const multiuse = ref(false);
const expiresAt = ref("");

const { mutate: createInvite } = useMutation(CreateInviteMutation, {
  refetchQueries: () => ["InvitesQuery"],
});

const create = async () => {
  const ex = expiresAt.value
    ? new Date(expiresAt.value).toISOString().slice(0, -1)
    : null;
  await createInvite({
    classId: classId,
    multiuse: multiuse.value,
    expiresAt: ex,
  });
};

const copyInvite = (inviteId: string) => {
  const url = `${window.location.origin}/invite/${inviteId}`;
  navigator.clipboard.writeText(url);
};

const DeleteInviteMutation = graphql(/* GraphQL */ `
  mutation DeleteInviteMutation($classId: ID!, $inviteId: ID!) {
    deleteInvite(classId: $classId, inviteId: $inviteId)
  }
`);

const { mutate: deleteInvite } = useMutation(DeleteInviteMutation, {
  refetchQueries: () => ["InvitesQuery"],
});

const delete_ = (inviteId: string) => {
  deleteInvite({
    classId: classId,
    inviteId: inviteId,
  });
};
</script>
