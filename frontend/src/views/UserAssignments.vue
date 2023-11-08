<template>
  <div
    class="d-flex justify-center flex-wrap align-center main-wrapper center mt-8"
  >
    <h1 class="w-100 mb-4">My assignments</h1>
    <v-text-field
      v-model="assingmentSearch"
      variant="outlined"
      label="Search assignment"
    ></v-text-field>
    <v-expansion-panels variant="popout" multiple>
      <v-expansion-panel
        v-for="assignment in filteredAssignments"
        :key="assignment.id"
      >
        <v-expansion-panel-title>
          <h2 class="pa-4">{{ assignment.name }}</h2>
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <p class="pa-4">
            {{ assignment.content }}
          </p>
          <v-list class="d-flex chip-gap pb-0">
            <v-list-item
              class="pa-0 ma-0"
              v-for="(file, i) in assignment.files"
            >
              <v-chip @click="download(assignment.classId, file)">
                {{ file.name }}
              </v-chip>
            </v-list-item>
          </v-list>
          <hr />
          <div class="pa-4 d-flex justify-space-between align-center">
            <div v-if="assignment.submissions[0]">
              <h4>Your current submission:</h4>
              <v-list class="d-flex chip-gap pb-0">
                <v-list-item
                  class="pa-0 ma-0"
                  v-for="(file, i) in assignment.submissions[0].files"
                >
                  <v-chip> {{ file.name }} </v-chip>
                </v-list-item>
              </v-list>
            </div>
            <v-btn
              @click="moveToClass(assignment.classId)"
              class="bg-primary push-right"
            >
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
import { useRouter } from "vue-router";
import { computed, ref } from "vue";
import { download, toLocaleString } from "../shared";

const MyAssignmentsMeQuery = graphql(/* GraphQL */ `
  query MyAssignmentsMeQuery {
    me {
      id
      assignments {
        id
        classId
        name
        content
        dueAt
        createdAt
        files {
          id
          name
        }
        submissions {
          id
          createdAt
          files {
            id
            name
          }
          feedback {
            id
            content
            createdAt
          }
        }
      }
    }
  }
`);

const { result: meResult, onResult } = useQuery(MyAssignmentsMeQuery);
const assignments = computed(() => meResult.value?.me.assignments ?? []);

onResult((result) => {
  console.log(result);
});

const assingmentSearch = ref("");
const filteredAssignments = computed(() => {
  if (assingmentSearch.value === "") {
    return assignments.value;
  }
  return assignments.value.filter((assignment) => {
    return assignment.name.includes(assingmentSearch.value);
  });
});

const router = useRouter();
const moveToClass = (classId: string) => {
  router.push(`/class/${classId}`);
};
</script>

<style scoped>
.main-wrapper {
  width: 60%;
}

.center {
  margin: auto;
}
</style>
