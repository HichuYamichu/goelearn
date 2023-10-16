<template>
  <v-container fluid>
    <v-row>
      <v-col xl="4" sm="12" class="mx-auto">
        <form @keyup.enter.native="login" class="pa-5">
          <v-text-field
            label="Username or Email"
            v-model="username"
            required
            focused
            variant="outlined"
          ></v-text-field>
          <v-text-field
            label="Password"
            v-model="password"
            required
            variant="outlined"
          ></v-text-field>
          <v-btn @click="login" class="me-4 bg-primary"> submit </v-btn>
          <v-btn @click="clear"> clear </v-btn>
        </form>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { client } from "@/client";
import { graphql } from "@/gql";
import router from "@/router";
import { useMutation } from "@vue/apollo-composable";
import gql from "graphql-tag";
import { ref } from "vue";

const username = ref("");
const password = ref("");

const Login = graphql(/* GraphQL */ `
  mutation Login($password: String!, $username: String!) {
    login(input: { password: $password, username: $username }) {
      token
    }
  }
`);

const { mutate: sendCredentials } = useMutation(Login);

const login = async () => {
  try {
    const res = await sendCredentials({
      username: username.value,
      password: password.value,
    });
    localStorage.setItem("token", res?.data?.login.token!);
    client.writeQuery({
      query: gql(`
        query {
          isLoggedIn
        }
      `),
      data: {
        isLoggedIn: true,
      },
    });

    router.push({ name: "Home" });
  } catch (e) {
    console.log(e);
  }
};

const clear = () => {
  username.value = "";
  password.value = "";
};
</script>
