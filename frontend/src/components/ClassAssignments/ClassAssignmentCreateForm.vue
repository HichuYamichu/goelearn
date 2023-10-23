<template>
  <v-card width="100%">
    <v-toolbar dark color="primary">
      <v-btn icon dark @click="emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
      <v-toolbar-title>Create assignment</v-toolbar-title>
      <v-spacer></v-spacer>
    </v-toolbar>
    <form @submit.prevent="submit" class="w-100">
      <v-container>
        <v-row>
          <v-col cols="12">
            <v-text-field
              v-model="assignmentInput.name"
              variant="outlined"
              label="Name"
              :rules="assignmentNameRules"
            ></v-text-field>
          </v-col>
          <v-col cols="12">
            <v-textarea
              v-model="assignmentInput.content"
              variant="outlined"
              label="Content"
              :rules="assignmentContentRules"
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
            <v-btn class="bg-primary" @click="submit">Create Assignment</v-btn>
          </v-col>
        </v-row>
      </v-container>
    </form>
  </v-card>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";

interface AssignmentInput {
  name: string;
  content: string;
  dueDate: string;
  newFiles: File[];
  oldFiles: string[];
}

const props = defineProps<{
  assigmnent?: { id: string } & AssignmentInput;
}>();

const assignmentInput = reactive({
  name: props.assigmnent?.name ?? "",
  content: props.assigmnent?.content ?? "",
  dueDate: props.assigmnent?.dueDate ?? "",
  newFiles: props.assigmnent?.newFiles ?? [],
  oldFiles: props.assigmnent?.oldFiles ?? [],
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

  if (props.assigmnent) {
    updateMutation({
      input: {
        id: props.assigmnent.id,
        name: assignmentInput.name,
        content: assignmentInput.content,
        dueAt: dateOrNull(assignmentInput.dueDate),
        newFiles: assignmentInput.newFiles,
        deleteFiles: [],
      },
    });
  } else {
    createMutation({
      input: {
        name: assignmentInput.name,
        content: assignmentInput.content,
        dueAt: dateOrNull(assignmentInput.dueDate),
        files: assignmentInput.newFiles,
        classId: classId as string,
      },
    });
  }

  emit("close");
};

const assignmentNameRules = [
  (v: string) => !!v || "Name is required",
  (v: string) => v.length <= 35 || "Name must be less than 35 characters",
];

const assignmentContentRules = [
  (v: string) => !!v || "Content is required",
  (v: string) =>
    v.length <= 2000 || "Content must be less than 2000 characters",
];
</script>
