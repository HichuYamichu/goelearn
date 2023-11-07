<template>
  <v-container fluid class="mt-4">
    <v-row>
        <v-col sm="12" class="d-flex fustify-center">
          <h2 class="w-100 text-center text-h2">Settings</h2>
        </v-col>
      </v-row>
    <v-row>
      <v-col xl="4" md="6" sm="12" class="mx-auto">
        <form @submit.prevent="submit">
          <v-text-field v-model="state.username" disabled label="Username cannot be changed" required focused variant="outlined"
            :rules="usernameRules" ref="usernameRulesState"></v-text-field>
          <v-text-field v-model="state.firstName" label="Firstname" required variant="outlined" :rules="firstnameRules"
            ref="firstnameRulesState"></v-text-field>
          <v-text-field v-model="state.lastName" label="Lastname" required variant="outlined" :rules="lastnameRules"
            ref="lastnameRulesState"></v-text-field>
          <v-text-field v-model="state.email" label="Email" required variant="outlined" :rules="emailRules"
            ref="emailRulesState"></v-text-field>
          <v-file-input v-model="state.avatar" label="Avatar" variant="outlined"></v-file-input>

          <v-card variant="outlined" class="mt-12" title="Save" >
            <div class="d-flex align-center pa-4">
              <v-btn class="me-4 bg-primary" type="submit"> Save </v-btn>
              <v-text-field v-model="state.password" label="Confirm your password" required variant="outlined" :rules="passwordRules"
              type="password" ref="passwordRulesState"></v-text-field>
            </div>
          </v-card>

          <v-card variant="outlined" title="Change your password" color="error" class="mt-12">
            <div class="d-flex align-center pa-4 flex-wrap gap">
              <v-btn class=" bg-primary" type="submit"> Save </v-btn>
              <v-text-field class="w-100" v-model="state.password" label="Confirm your password" required variant="outlined" :rules="passwordRules"
              type="password" ref="passwordRulesState"></v-text-field>
              <v-text-field class="w-100" v-model="state.password" label="Confirm your password" required variant="outlined" :rules="passwordRules"
              type="password" ref="passwordRulesState"></v-text-field>
            </div>
          </v-card>

        </form>
        <p class="error">{{ errorMessage }}</p>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { validate } from "@/shared";
import { useMutation } from "@vue/apollo-composable";
import { reactive, ref } from "vue";

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

const errorMessage = ref("");

const state = reactive({
  ...initialState,
});

const { mutate, onDone } = useMutation(SignupMutation);
onDone((res) => {
  router.push({ name: "Login" });
});

const submit = async () => {
  errorMessage.value = "";

  const isValid = await validate([
    usernameRulesState,
    firstnameRulesState,
    lastnameRulesState,
    emailRulesState,
    passwordRulesState,
  ]);
  if (!isValid) {
    return;
  }

  const passwordMatch = state.password === state.confirmPassword;
  if (!passwordMatch) {
    errorMessage.value = "Passwords do not match";
    return;
  }

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
};

const usernameRules = [
  (v: string) => !!v || "Username is required",
  (v: string) => v.length >= 5 || "Username must be at least 5 characters long",
  (v: string) => v.length <= 20 || "Username must be less than 20 characters",
];
const usernameRulesState = ref(null);

const firstnameRules = [
  (v: string) => !!v || "Firstname is required",
  (v: string) =>
    v.length >= 2 || "Firstname must be at least 2 characters long",
  (v: string) => v.length <= 40 || "Firstname must be less than 40 characters",
];
const firstnameRulesState = ref(null);

const lastnameRules = [
  (v: string) => !!v || "Lastname is required",
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