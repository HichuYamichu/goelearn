<template>
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
        <v-btn class="bg-primary" type="submit"> Change </v-btn>
      </div>
    </v-card>
  </form>
</template>
<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const token = route.params.token as string;
const router = useRouter();

const changedPassword = ref("");
const changedPasswordConfirm = ref("");

const EmergencyChangePasswordMutation = graphql(/* GraphQL */ `
  mutation EmergencyChangePasswordMutation($token: ID!, $password: String!) {
    emergencyChangePassword(token: $token, password: $password)
  }
`);

const { mutate: changePasswordMutation } = useMutation(
  EmergencyChangePasswordMutation
);

const changePassword = async () => {
  await changePasswordMutation({
    token,
    password: changedPassword.value,
  });
  router.push("/login");
};

const passwordRules = [
  (v: string) => !!v || "Password is required",
  (v: string) => v.length >= 8 || "Password must be at least 8 characters long",
  (v: string) => v.length <= 100 || "Password must be less than 100 characters",
];
const passwordRulesState = ref(null);
</script>
