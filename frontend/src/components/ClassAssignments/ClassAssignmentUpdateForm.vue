<template>
  <v-card width="100%">
    <v-toolbar dark color="primary">
      <v-btn icon dark @click="emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
      <v-toolbar-title>Create assignment</v-toolbar-title>
      <v-spacer></v-spacer>
    </v-toolbar>
    <v-container>
      <v-row>
        <v-col cols="12">
          <v-text-field
            v-model="assignmentInput.name"
            variant="outlined"
            label="Name"
          ></v-text-field>
        </v-col>
        <v-col cols="12">
          <v-textarea
            v-model="assignmentInput.content"
            variant="outlined"
            label="Content"
          ></v-textarea>
        </v-col>
        <v-col cols="12">
          <v-text-field
            v-model="assignmentInput.dueDate"
            variant="outlined"
            type="datetime-local"
            label="Due date"
          ></v-text-field>
        </v-col>
        <v-col cols="12">
          <v-file-input
            v-model="assignmentInput.newFiles"
            variant="outlined"
            label="Files"
            multiple
          ></v-file-input>
        </v-col>
        <v-col cols="12">
          <v-list-item
            class="pa-1"
            v-for="(file, i) in assignmentInput.oldFiles"
          >
            <v-chip @click="deleteFile(file)"> {{ file.name }} </v-chip>
          </v-list-item>
        </v-col>
        <v-col cols="12">
          <v-btn class="bg-primary" @click="submit">Update Assignment</v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-card>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";

interface AssignmentInput {
  id: string;
  name: string;
  content: string;
  dueDate: string;
  newFiles: File[];
  oldFiles: { id: string; name: string }[];
  deleteFiles: string[];
}

const props = defineProps<{
  assignment: AssignmentInput;
}>();

const assignmentInput = reactive({
  name: props.assignment?.name ?? "",
  content: props.assignment?.content ?? "",
  dueDate: props.assignment?.dueDate ?? "",
  newFiles: props.assignment?.newFiles ?? [],
  oldFiles: props.assignment?.oldFiles ?? [],
  deleteFiles: [] as string[],
} as AssignmentInput);

const emit = defineEmits(["close"]);

const router = useRouter();
const classId = router.currentRoute.value.params.classId;

const CreateAssignmentMutation = graphql(/* GraphQL */ `
  mutation CreateAssignmentMutation($input: CreateAssignmentInput!) {
    createAssignment(input: $input) {
      id
    }
  }
`);

const { mutate: createMutation } = useMutation(CreateAssignmentMutation);

const UpdateAssignmentMutation = graphql(/* GraphQL */ `
  mutation UpdateAssignmentMutation($input: UpdateAssignmentInput!) {
    updateAssignment(input: $input)
  }
`);

const { mutate: updateMutation } = useMutation(UpdateAssignmentMutation);

const submit = () => {
  let dateOrNull = (dueDate: string) => {
    if (!dueDate) {
      return null;
    }
    return new Date(dueDate).toISOString().slice(0, -1);
  };

  updateMutation({
    input: {
      id: props.assignment.id,
      name: assignmentInput.name,
      content: assignmentInput.content,
      dueAt: dateOrNull(assignmentInput.dueDate),
      newFiles: assignmentInput.newFiles,
      deleteFiles: assignmentInput.deleteFiles,
    },
  });

  emit("close");
};

const deleteFile = (file: { id: string; name: string }) => {
  assignmentInput.oldFiles = assignmentInput.oldFiles.filter(
    (f) => f.id !== file.id
  );
  assignmentInput.deleteFiles.push(file.id);
};
</script>
