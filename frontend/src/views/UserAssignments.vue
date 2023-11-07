<template>
  <div class="d-flex justify-center flex-wrap align-center main-wrapper center mt-8">
    <h1 class="w-100 mb-4">My assignments</h1>
    <v-text-field variant="outlined" label="Search assignment"></v-text-field>
    <v-expansion-panels variant="popout" multiple>
      <v-expansion-panel v-for="assignment in assignments" :key="assignment.id">
        <v-expansion-panel-title>
          <h2 class="pa-4">{{ assignment.name }}</h2>
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <p class="pa-4">
            {{ assignment.content }}
          </p>
          <v-list class="d-flex chip-gap pb-0">
            <v-list-item class="pa-0 ma-0" v-for="(file, i) in assignment.files">
              <v-chip> {{ file.name }} </v-chip>
            </v-list-item>
          </v-list>
          <hr>
          <div class="pa-4 d-flex justify-space-between align-center">
            <h4>Your current submission:</h4>
            <v-btn class="bg-primary push-right">
              view in class
            </v-btn>

          </div>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>


  </div>
</template>

<script setup lang="ts">
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { computed, ref } from "vue";

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

// const { result: meResult, onResult } = useQuery(MyAssignmentsMeQuery);
// const assignments = computed(() => meResult.value?.me.assignments ?? []);
// onResult(() => {
//   console.log(meResult.value?.me.assignments);
// });

// TODO: add class_id to assignment
const assignments = ref([
  { id: 1, name: "aaa", content: "aaaaaaa", files: [{name: "aa"}, {name: "aaaa"}] },
  { id: 1, name: "bbb", content: "bbbbbbb", files: [{name: "bb"}, {name: "bbbb"}] },
  { id: 1, name: "ccc", content: "ccccccc", files: [{name: "cc"}, {name: "cccc"}] },
  { id: 1, name: "ddd", content: "ddddddd", files: [{name: "dd"}, {name: "dddd"}] },
])
</script>

<style scoped>
.main-wrapper {
  width: 60%;
}

.center {
  margin: auto;
}

.push-right {
}
</style>