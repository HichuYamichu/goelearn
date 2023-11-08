<template>
  <v-container fluid class="mt-4">
    <v-row>
      <v-col sm="12" class="d-flex fustify-center">
        <h2 class="w-100 text-center text-h2">Settings</h2>
      </v-col>
    </v-row>
    <v-row>
      <v-col xl="4" md="6" sm="12" class="mx-auto">
        <form @submit.prevent="updateUserData">
          <v-text-field
            v-model="state.username"
            disabled
            label="Username cannot be changed"
            focused
            variant="outlined"
            :placeholder="me?.username"
            ref="usernameRulesState"
          ></v-text-field>
          <v-text-field
            v-model="state.firstName"
            label="Firstname"
            variant="outlined"
            :placeholder="me?.firstName"
            :rules="firstnameRules"
            ref="firstnameRulesState"
          ></v-text-field>
          <v-text-field
            v-model="state.lastName"
            label="Lastname"
            variant="outlined"
            :placeholder="me?.lastName"
            :rules="lastnameRules"
            ref="lastnameRulesState"
          ></v-text-field>
          <v-text-field
            disabled
            label="Email cannot be changed"
            variant="outlined"
            :placeholder="me?.email"
            :rules="emailRules"
            ref="emailRulesState"
          ></v-text-field>
          <v-file-input
            v-model="state.avatar"
            label="Avatar"
            variant="outlined"
          ></v-file-input>
          <v-card variant="outlined" class="mt-12" title="Save">
            <div class="d-flex flex-rap gap align-center pa-4">
              <v-text-field
                class="w-100"
                v-model="state.password"
                label="Confirm your password"
                variant="outlined"
                :rules="passwordRules"
                type="password"
                ref="passwordRulesState"
              ></v-text-field>
              <v-btn class="me-4 bg-primary" type="submit"> Save </v-btn>
            </div>
          </v-card>
        </form>

        <form @submit.prevent="changePassword">
          <v-card
            variant="outlined"
            title="Change your password"
            color="error"
            class="mt-12 mb-8"
          >
            <div class="d-flex align-center pa-4 flex-wrap gap">
              <v-text-field
                class="w-100"
                v-model="changedPassword"
                label="Confirm your password"
                required
                variant="outlined"
                :rules="passwordRules"
                type="password"
                ref="passwordRulesState"
              ></v-text-field>
              <v-text-field
                class="w-100"
                v-model="changedPasswordConfirm"
                label="Confirm your password"
                required
                variant="outlined"
                :rules="passwordRules"
                type="password"
                ref="passwordRulesState"
              ></v-text-field>
              <v-text-field
                class="w-100"
                v-model="currentPassword"
                label="Enter your current password"
                required
                variant="outlined"
                :rules="passwordRules"
                type="password"
              ></v-text-field>
              <v-btn class="bg-primary" type="submit"> Save </v-btn>
            </div>
          </v-card>
        </form>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { validate } from "@/shared";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { reactive, ref } from "vue";

const UserSettingsMeQuery = graphql(/* GraphQL */ `
  query UserSettingsMeQuery {
    me {
      id
      username
      firstName
      lastName
      email
    }
  }
`);

const { result, onResult } = useQuery(UserSettingsMeQuery);
const me = computed(() => result.value?.me);

const UpdateUserDataMutation = graphql(/* GraphQL */ `
  mutation UpdateUserDataMutation(
    $userId: ID!
    $firstName: String
    $lastName: String
    $avatar: Upload
    $password: String!
  ) {
    updateUser(
      userId: $userId
      firstName: $firstName
      lastName: $lastName
      avatar: $avatar
      password: $password
    ) {
      id
      firstName
      lastName
    }
  }
`);

const { mutate: updateUserDataMutation } = useMutation(UpdateUserDataMutation, {
  refetchQueries: () => ["AppBarMeQuery"],
});

const updateUserData = async () => {
  const newFirstName = state.firstName || null;
  if (
    newFirstName &&
    // @ts-ignore // this is a bug in the type definition
    (await firstnameRulesState.value!.validate()).length > 0
  ) {
    return;
  }

  const newLastName = state.lastName || null;
  if (
    newFirstName &&
    // @ts-ignore // see above
    !(await lastnameRulesState.value!.validate()).length > 0
  ) {
    return;
  }

  await updateUserDataMutation({
    userId: me.value!.id,
    firstName: newFirstName,
    lastName: newLastName,
    avatar: state.avatar?.[0],
    password: state.password,
  });
};

const initialState = {
  username: "",
  firstName: "",
  lastName: "",
  email: "",
  password: "",
  confirmPassword: "",
  avatar: undefined as File[] | undefined,
};

const state = reactive({
  ...initialState,
});

const changedPassword = ref("");
const changedPasswordConfirm = ref("");
const currentPassword = ref("");

const ChangePasswordMutation = graphql(/* GraphQL */ `
  mutation ChangePasswordMutation(
    $userId: ID!
    $oldPassword: String!
    $newPassword: String!
  ) {
    changePassword(
      userId: $userId
      oldPassword: $oldPassword
      newPassword: $newPassword
    ) {
      id
    }
  }
`);

const { mutate: changePasswordMutation } = useMutation(ChangePasswordMutation);

const changePassword = async () => {
  if (changedPassword.value !== changedPasswordConfirm.value) {
    // TODO: show error
    return;
  }

  await changePasswordMutation({
    userId: me.value!.id,
    newPassword: changedPassword.value,
    oldPassword: currentPassword.value,
  });
};

const firstnameRules = [
  (v: string) =>
    v.length >= 2 || "Firstname must be at least 2 characters long",
  (v: string) => v.length <= 40 || "Firstname must be less than 40 characters",
];
const firstnameRulesState = ref(null);

const lastnameRules = [
  (v: string) => v.length >= 2 || "Lastname must be at least 2 characters long",
  (v: string) => v.length <= 40 || "Lastname must be less than 40 characters",
];
const lastnameRulesState = ref(null);

const emailRules = [
  (v: string) => !!v || "E-mail is required",
  (v: string) => /.+@.+/.test(v) || "E-mail must be valid",
];
const emailRulesState = ref(null);

const passwordRules = [
  (v: string) => !!v || "Password is required",
  (v: string) => v.length >= 8 || "Password must be at least 8 characters long",
  (v: string) => v.length <= 100 || "Password must be less than 100 characters",
];
const passwordRulesState = ref(null);
</script>

<style scoped>
.gap {
  gap: 15px;
}
</style>
