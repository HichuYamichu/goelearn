<template>
  <v-container class="mx-auto pa-0 fill-height">
    <v-row class="fill-height">
      <v-col cols="3" class="pa-5">
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
        <v-btn v-if="isOwner" block @click="createAssignmentDialog = true"
          >Add new</v-btn
        >
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="6" class="pa-5">
        <div v-if="selectedAssignment">
          <h1>{{ selectedAssignment.name }}</h1>
          <h5>
            Published at:
            {{ new Date(selectedAssignment.createdAt).toLocaleString() }}
          </h5>
          <p>
            {{ selectedAssignment.content }}
          </p>
          <v-list class="d-flex">
            <v-list-item
              class="pa-1"
              v-for="(file, i) in selectedAssignment.files"
            >
              <v-chip @click="download(file)"> {{ file.name }} </v-chip>
            </v-list-item>
          </v-list>
        </div>
        <div v-else>
          <h1>Select assignment to view</h1>
        </div>
      </v-col>
      <v-divider vertical></v-divider>
      <v-col cols="3" class="pa-5">
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
  <v-dialog v-model="createAssignmentDialog" width="100%">
    <ClassAssignmentForm></ClassAssignmentForm>
  </v-dialog>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { MyIdQuery } from "@/shared";
import ClassAssignmentForm from "@/components/ClassAssignmentForm.vue";

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

const createAssignmentDialog = ref(false);

const isOwner = ref(false);
const { onResult } = useQuery(MyIdQuery);
onResult((result) => {
  if (result.data?.me?.id === class_.value?.ownerId) {
    isOwner.value = true;
  }
});

const download = async (item: any) => {
  downloadFile(
    `http://localhost:3000/files/class-files/${class_.value?.id}/${item.id}`,
    item.name
  );
};

const downloadFile = async (url: string, filename: string) => {
  const data = await fetch(url);
  const blob = await data.blob();
  const objectUrl = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.setAttribute("href", objectUrl);
  link.setAttribute("download", filename);
  link.style.display = "none";
  link.click();
};
</script>
