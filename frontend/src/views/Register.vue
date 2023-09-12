<template>
  <v-container fluid>
    <v-row>
      <v-col xl="4" sm="12" class="mx-auto">
        <form>
          <v-text-field
            v-model="state.username"
            label="Username"
            required
            focused
          ></v-text-field>
          <v-text-field
            v-model="state.firstName"
            label="Firstname"
            required
          ></v-text-field>
          <v-text-field
            v-model="state.lastName"
            label="Lastname"
            required
          ></v-text-field>
          <v-text-field
            v-model="state.email"
            label="Email"
            required
          ></v-text-field>
          <v-text-field
            v-model="state.password"
            label="Password"
            required
          ></v-text-field>
          <v-text-field label="Confirm Password" required></v-text-field>
          <v-file-input v-model="state.avatar" label="Avatar"></v-file-input>

          <v-btn class="me-4 bg-primary" @click="submit"> submit </v-btn>
          <v-btn @click="clear"> clear </v-btn>
        </form>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { useMutation } from "@vue/apollo-composable";
import { reactive } from "vue";

const clear = () => {};

const SignupMutation = graphql(/* GraphQL */ `
  mutation Signup($input: SignupInput!) {
    signup(input: $input)
  }
`);

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

const { mutate, onDone } = useMutation(SignupMutation);

const submit = () => {
  const ava = state.avatar?.[0] ?? null;

  mutate({
    input: {
      username: state.username,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      password: state.password,
      avatar: ava,
    },
  });

  onDone((res) => {
    router.push({ name: "Login" });
  });
};
</script>
