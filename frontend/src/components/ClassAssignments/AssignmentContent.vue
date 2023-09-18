<template>
  <div class="w-100">
    <h1>{{ assignment.name }}</h1>
    <h5>
      Published at:
      {{ new Date(assignment.createdAt).toLocaleString() }}
    </h5>
  </div>
  <div class="overflow-y-visible big-text d-flex">
    <!-- <p class="big-text"></p> -->
    {{ assignment.content }}
  </div>
  <v-list class="d-flex">
    <v-list-item class="pa-1" v-for="(file, i) in assignment.files">
      <v-chip @click="download(classId, file)"> {{ file.name }} </v-chip>
    </v-list-item>
  </v-list>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { MyIdQuery } from "@/shared";
import ClassAssignmentForm from "@/components/ClassAssignments/ClassAssignmentForm.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import { useDisplay } from "vuetify";
import { useRouter } from "vue-router";
import { download } from "../../shared";

const AssignmentContentFragment = graphql(/* GraphQL */ `
  fragment AssignmentContentFragment on Assignment {
    id
    name
    content
    dueAt
    createdAt
    files {
      id
      name
    }
  }
`);

const props = defineProps<{
  assignment: FragmentType<typeof AssignmentContentFragment>;
}>();

const assignment = computed(() =>
  useFragment(AssignmentContentFragment, props.assignment)
);

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;
</script>

<style scoped>
.big-text {
  height: 400px;
  overflow-y: scroll;
  word-wrap: break-all;
}
</style>
