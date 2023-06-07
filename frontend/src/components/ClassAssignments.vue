<template>
  <v-container class="ma-0 pa-0 fill-height" fluid>
    <v-row justify="space-evenly" class="fill-height" no-gutters>
      <v-col cols="2" class="py-4">
        <h1>Assignments</h1>
        <v-list lines="one">
          <v-list-item
            v-for="item in assignments"
            :key="item.id"
            :title="item.name"
            :active="selectedAssignment?.id === item.id"
            @click="selectedAssignment = item"
          ></v-list-item>
        </v-list>
        <v-btn block @click="createAssignmentDialog = true">Add new</v-btn>
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="6" class="py-4">
        <div v-if="selectedAssignment">
          <h1>{{ selectedAssignment.name }}</h1>
          <h5>
            Published at:
            {{ new Date(selectedAssignment.createdAt).toLocaleString() }}
          </h5>
          <p>
            {{ selectedAssignment.content }}
          </p>
        </div>
        <div v-else>
          <h1>Select assignment to view</h1>
        </div>
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="3" class="py-4">
        <div v-if="selectedAssignment">
          <h3 class="mb-2">Your solutuion:</h3>
          <h5>
            Due at: {{ new Date(selectedAssignment.dueAt).toLocaleString() }}
          </h5>
          <h5>
            Submited at:
            {{ new Date(selectedAssignment.dueAt).toLocaleString() }}
          </h5>
          <v-file-input
            label="File input"
            variant="outlined"
            class="mt-3"
          ></v-file-input>
          <v-btn block class="success" @click="submit">submit</v-btn>
          <h3 class="mt-3">Feedback:</h3>
          <p>
            Lorem ipsum dolor, sit amet consectetur adipisicing elit. Adipisci
            sequi odio quidem asperiores animi alias excepturi in dolorum nemo?
            Deleniti repellendus distinctio reprehenderit aliquid laudantium
            suscipit velit! Aperiam, sunt eligendi.
          </p>
        </div>
        <div v-else-if="isOwner">Grading</div>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { isClassOwner } from "@/shared";

const AssignmentsFragment = graphql(/* GraphQL */ `
  fragment AssignmentsFragment on Class {
    id
    ownerId
    assignments {
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
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof AssignmentsFragment> | null;
  loading: boolean;
}>();

const class_ = ref(useFragment(AssignmentsFragment, props.class_));

watch(props, () => {
  class_.value = useFragment(AssignmentsFragment, props.class_);
});

const assignments = computed(() => class_.value?.assignments ?? []);

type Assignment = (typeof assignments.value)[0];
const selectedAssignment = ref<null | Assignment>(null);

const SubmitAssignmentMutation = graphql(/* GraphQL */ `
  mutation SubmitAssignment($files: [Upload!]!, $assignmentId: ID!) {
    submitAssignment(input: { files: $files, assignmentId: $assignmentId })
  }
`);

const { mutate: submitAssignment } = useMutation(SubmitAssignmentMutation);
const filesToUpload = ref<File[]>([]);

const submit = async () => {
  if (!selectedAssignment.value) return;
  await submitAssignment({
    files: filesToUpload.value,
    assignmentId: selectedAssignment.value.id,
  });
};

const isOwner = computed(() => {
  if (!class_.value) return false;
  // return isClassOwner(class_.value.ownerId);
});

const createAssignmentDialog = ref(false);
</script>
