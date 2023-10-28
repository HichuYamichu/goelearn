<template>
  <div class="d-flex flex-wrap">
    <div class="pa-4 w-20 w-xs-100">
      <h1>Assignments</h1>
      <v-text-field variant="outlined" label="Search"></v-text-field>
      <v-list lines="one" style="height: 80%" class="overflow-y-auto">
        <v-list-item
          v-for="(item, idx) in assignments"
          :key="item.id"
          :title="item.name"
          :active="selectedAssignment?.id === item.id"
          @click="selectedAssignmentIdx = idx"
        ></v-list-item>
      </v-list>
    </div>
    <div class="pa-4 w-60 w-xs-100 d-flex flex-column">
      <div v-if="selectedAssignment" class="d-flex flex-column">
        <AssignmentContent :assignment="selectedAssignment"></AssignmentContent>
      </div>
      <div v-else>
        <h1>Select assignment to view</h1>
      </div>
      <div>
        <h1>Feedback</h1>
        <div v-if="mySubmission?.feedback">
          <p>
            First graded at:
            {{ toLocaleString(mySubmission?.feedback!.createdAt) }}
          </p>
          <p>
            Last updated at:
            {{ toLocaleString(mySubmission?.feedback!.updatedAt) }}
          </p>
          <p>
            {{ mySubmission!.feedback!.content }}
          </p>
        </div>
        <div v-else>
          <p>Your submission was not graded yet.</p>
        </div>
      </div>
    </div>
    <div class="pa-4 w-20 w-xs-100">
      <h1>Your submission</h1>
      <v-file-input
        v-model="filesToUpload"
        multiple
        variant="outlined"
        label="File input"
      ></v-file-input>
      <div v-if="mySubmission">
        <h3>Your submission</h3>
        <v-list class="d-flex">
          <v-list-item class="pa-1" v-for="(file, i) in mySubmission.files">
            <v-chip @click="download(classId, file)"> {{ file.name }} </v-chip>
          </v-list-item>
        </v-list>
        <p>
          First submitted at:
          {{ toLocaleString(mySubmission.createdAt) }}
        </p>
        <p>
          Last updated at:
          {{ toLocaleString(mySubmission.updatedAt) }}
        </p>
        <div class="d-flex button-gap mt-4">
          <v-btn @click="update">Update</v-btn>
          <v-btn class="bg-error" @click="delete_">Delete</v-btn>
        </div>
      </div>
      <div v-else>
        <h3>You haven't submitted yet</h3>
        <v-btn class="mt-4" @click="submit">Submit</v-btn>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import AssignmentContent from "@/components/ClassAssignments/AssignmentContent.vue";
import { download, toLocaleString } from "../../shared";
import { useRoute } from "vue-router";

const route = useRoute();

const classId = route.params.classId as string;

const StudentAssignmentsFragment = graphql(/* GraphQL */ `
  fragment StudentAssignmentsFragment on Class {
    assignments {
      id
      name
      ...AssignmentContentFragment
      submissions {
        id
        createdAt
        updatedAt
        files {
          id
          name
        }
        feedback {
          id
          content
          createdAt
          updatedAt
        }
      }
    }
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof StudentAssignmentsFragment> | null;
}>();

const class_ = computed(() =>
  useFragment(StudentAssignmentsFragment, props.class_)
);
const assignments = computed(() => class_.value?.assignments ?? []);

type Assignment = (typeof assignments.value)[0];
const selectedAssignmentIdx = ref<null | number>(null);

const selectedAssignment = computed(() => {
  if (selectedAssignmentIdx.value === null) return null;
  return assignments.value[selectedAssignmentIdx.value];
});

const CreateAssignmentSubmissionMutation = graphql(/* GraphQL */ `
  mutation CreateAssignmentSubmission($assignmentId: ID!, $files: [Upload!]!) {
    createAssignmentSubmission(
      input: { assignmentId: $assignmentId, files: $files }
    )
  }
`);

const { mutate: submitAssignment } = useMutation(
  CreateAssignmentSubmissionMutation
);

const filesToUpload = ref<File[]>([]);

const submit = async () => {
  if (!selectedAssignment.value) return;
  await submitAssignment({
    files: filesToUpload.value,
    assignmentId: selectedAssignment.value.id,
  });
};

const mySubmission = computed(() => {
  if (!selectedAssignment.value) return null;
  return selectedAssignment.value.submissions[0] ?? null;
});

const UpdateAssignmentSubmissionMutation = graphql(/* GraphQL */ `
  mutation UpdateAssignmentSubmission(
    $assignmentSubmissionId: ID!
    $assignmentId: ID!
    $files: [Upload!]!
  ) {
    updateAssignmentSubmission(
      input: {
        id: $assignmentSubmissionId
        assignmentId: $assignmentId
        files: $files
      }
    )
  }
`);

const { mutate: updateAssignment } = useMutation(
  UpdateAssignmentSubmissionMutation
);

const update = async () => {
  if (!selectedAssignment.value) return;
  await updateAssignment({
    files: filesToUpload.value,
    assignmentSubmissionId: mySubmission.value?.id ?? "",
    assignmentId: selectedAssignment.value.id,
  });

  filesToUpload.value = [];
};

const DeleteAssignmentSubmissionMutation = graphql(/* GraphQL */ `
  mutation DeleteAssignmentSubmission(
    $classId: ID!
    $assignmentId: ID!
    $assignmentSubmissionId: ID!
  ) {
    deleteAssignmentSubmission(
      classId: $classId
      assignmentId: $assignmentId
      assignmentSubmissionId: $assignmentSubmissionId
    )
  }
`);

const { mutate: deleteAssignment } = useMutation(
  DeleteAssignmentSubmissionMutation
);

const delete_ = async () => {
  if (!selectedAssignment.value) return;
  await deleteAssignment({
    classId,
    assignmentId: selectedAssignment.value.id,
    assignmentSubmissionId: mySubmission.value?.id ?? "",
  });
};
</script>

<style scoped>
.button-gap {
  gap: 1rem;
}
</style>
