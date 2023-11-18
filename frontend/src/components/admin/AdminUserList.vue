<template>
  <div class="pa-2">
    <v-text-field
      label="Search user"
      variant="outlined"
      v-model="userFileter"
    ></v-text-field>
    <v-list>
      <v-list-item v-for="(user, idx) in filteredUsers">
        <div class="d-flex gap justify-start">
          <h2>{{ user.username }}</h2>
          <div class="d-flex justify-start w-20">
            <v-checkbox
              density="compact"
              label="Allow creating classes"
              class="w-10"
              v-model="mutations.get(user.id)!.userType"
            ></v-checkbox>
            <v-checkbox
              density="compact"
              label="Disable account"
              class="w-10"
              v-model="mutations.get(user.id)!.deletedAt"
            ></v-checkbox>
          </div>
          <v-btn color="success" @click="saveUserMutation(user.id)">Save</v-btn>
        </div>
      </v-list-item>
    </v-list>
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { computed, ref, watch } from "vue";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { reactive } from "vue";
import { UserType } from "@/gql/graphql";

const AdminUsersQuery = graphql(/* GraphQL */ `
  query AdminUsersQuery {
    users {
      id
      username
      userType
      deletedAt
    }
  }
`);

const { result: usersResult } = useQuery(AdminUsersQuery);

const users = computed(() => usersResult.value?.users ?? []);
const userFileter = ref("");
const filteredUsers = computed(() =>
  users.value.filter((user) =>
    user.username.toLowerCase().includes(userFileter.value.toLowerCase())
  )
);

watch(users, (newVal) => {
  mutations.clear();
  newVal.forEach((user) => {
    mutations.set(user.id, {
      id: user.id,
      userType: user.userType == "MOD",
      deletedAt: user.deletedAt != null,
    });
  });
});

type Mutation = {
  id: string;
  userType: boolean;
  deletedAt: boolean;
};
const mutations = reactive(new Map<string, Mutation>());

const AdminUserUpdate = graphql(/* GraphQL */ `
  mutation AdminUserUpdate(
    $userId: ID!
    $userType: UserType!
    $deletedAt: NaiveDateTime
  ) {
    adminUserUpdate(userId: $userId, userType: $userType, deletedAt: $deletedAt)
  }
`);

const { mutate: adminUserUpdate } = useMutation(AdminUserUpdate);

const saveUserMutation = async (id: string) => {
  const userType = mutations.get(id)!.userType
    ? UserType.Mod
    : UserType.Regular;
  const deletedAt = mutations.get(id)!.deletedAt
    ? new Date().toISOString().slice(0, -1)
    : null;

  adminUserUpdate({
    userId: mutations.get(id)!.id,
    userType,
    deletedAt,
  });
};
</script>

<style scoped>
.gap {
  gap: 20px;
}
</style>
