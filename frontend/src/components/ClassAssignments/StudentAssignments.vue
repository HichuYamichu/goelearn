<template>
  <div class="d-flex flex-wrap">
    <div class="pa-4 w-20 w-xs-100">
      <h1>Assignments</h1>
      <v-text-field variant="outlined" label="Search"></v-text-field>
      <v-list lines="one" style="height: 80%" class="overflow-y-auto">
        <v-list-item
          v-for="item in assignments"
          :key="item.id"
          :title="item.name"
          :active="selectedAssignment?.id === item.id"
          @click="selectedAssignment = item"
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
      <v-btn @click="submit">Submit</v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { useDisplay } from "vuetify";
import AssignmentContent from "@/components/ClassAssignments/AssignmentContent.vue";

const { mobile } = useDisplay();

const StudentAssignmentsFragment = graphql(/* GraphQL */ `
  fragment StudentAssignmentsFragment on Class {
    members {
      id
      username
    }
    assignments {
      id
      name
      ...AssignmentContentFragment
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
const selectedAssignment = ref<null | Assignment>(null);

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

// const AssignmentsFragment = graphql(/* GraphQL */ `
//   fragment AssignmentsFragment on Class {
//     id
//     ownerId
//     members {
//       id
//       username
//     }
//     assignments {
//       id
//       name
//       content
//       dueAt
//       createdAt
//       files {
//         id
//         name
//       }
//     }
//   }
// `);

// const props = defineProps<{
//   class_?: FragmentType<typeof AssignmentsFragment> | null;
// }>();

// const class_ = computed(() => useFragment(AssignmentsFragment, props.class_));
// const assignments = computed(() => class_.value?.assignments ?? []);
// const users = computed(() => class_.value?.members ?? []);

// type Assignment = (typeof assignments.value)[0];
// const selectedAssignment = ref<null | Assignment>(null);

// const SubmitAssignmentMutation = graphql(/* GraphQL */ `
//   mutation SubmitAssignment($files: [Upload!]!, $assignmentId: ID!) {
//     submitAssignment(input: { files: $files, assignmentId: $assignmentId })
//   }
// `);

// const { mutate: submitAssignment } = useMutation(SubmitAssignmentMutation);
// const filesToUpload = ref<File[]>([]);

// const submit = async () => {
//   if (!selectedAssignment.value) return;
//   await submitAssignment({
//     files: filesToUpload.value,
//     assignmentId: selectedAssignment.value.id,
//   });
// };

// const createAssignmentDialog = ref(false);
// const closeDialog = () => {
//   createAssignmentDialog.value = false;
// };

// const isOwner = ref(false);
// const { onResult } = useQuery(MyIdQuery);
// onResult((result) => {
//   if (result.data?.me?.id === class_.value?.ownerId) {
//     isOwner.value = true;
//   }
// });

// const download = async (item: any) => {
//   downloadFile(
//     `http://localhost:3000/files/class-files/${class_.value?.id}/${item.id}`,
//     item.name
//   );
// };

// const downloadFile = async (url: string, filename: string) => {
//   const data = await fetch(url);
//   const blob = await data.blob();
//   const objectUrl = URL.createObjectURL(blob);
//   const link = document.createElement("a");
//   link.setAttribute("href", objectUrl);
//   link.setAttribute("download", filename);
//   link.style.display = "none";
//   link.click();
// };
</script>
