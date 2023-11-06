<template>
  <v-list class="pa-0">
    <v-list-item v-for="assignment in assignments" :key="assignment.id" link>
      <v-list-item-title v-text="assignment.name"> </v-list-item-title>
    </v-list-item>
  </v-list>
</template>

<script setup lang="ts">
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { computed } from "vue";

const MyAssignmentsMeQuery = graphql(/* GraphQL */ `
  query MyAssignmentsMeQuery {
    me {
      id
      assignments {
        id
        name
        content
        dueAt
        createdAt
      }
    }
  }
`);

const { result: meResult, onResult } = useQuery(MyAssignmentsMeQuery);
const assignments = computed(() => meResult.value?.me.assignments ?? []);
onResult(() => {
  console.log(meResult.value?.me.assignments);
});
</script>
