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
            v-model="name"
            variant="outlined"
            label="Name"
          ></v-text-field>
        </v-col>
        <v-col cols="12">
          <v-textarea
            v-model="content"
            variant="outlined"
            label="Content"
          ></v-textarea>
        </v-col>
        <v-col cols="12">
          <v-text-field
            v-model="dueDate"
            variant="outlined"
            type="datetime-local"
            label="Due date"
          ></v-text-field>
        </v-col>
        <v-col cols="12">
          <v-file-input
            v-model="files"
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
  </v-card>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";

const emit = defineEmits(["close"]);

const router = useRouter();
const classId = router.currentRoute.value.params.classId;

const clear = () => {};

const CreateAssignmentMutation = graphql(/* GraphQL */ `
  mutation CreateAssignmentMutation($input: CreateAssignmentInput!) {
    createAssignment(input: $input) {
      id
    }
  }
`);

const name = ref("");
const content = ref("");
const dueDate = ref("");
const files = ref<File[]>([]);

const { mutate } = useMutation(CreateAssignmentMutation);

const submit = () => {
  mutate({
    input: {
      name: name.value,
      content: content.value,
      dueAt: new Date(dueDate.value).toISOString().slice(0, -1),
      files: files.value,
      classId: classId as string,
    },
  });
  emit("close");
};
</script>
