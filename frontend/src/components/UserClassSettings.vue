<template>
  <v-responsive class="d-flex text-center fill-height pa-4">
    <v-btn class="bg-error" @click="leave">Leave class</v-btn>
  </v-responsive>
</template>

<script setup lang="ts">
import { graphql } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";

const LeaveClassMutation = graphql(/* GraphQL */ `
  mutation LeaveClassMutation($classId: ID!) {
    leaveClass(classId: $classId)
  }
`);

const { mutate: leaveClass } = useMutation(LeaveClassMutation);
const route = useRoute();

const classId = route.params.classId as string;

const router = useRouter();
const leave = async () => {
  await leaveClass({ classId });
  router.push("/");
};
</script>
