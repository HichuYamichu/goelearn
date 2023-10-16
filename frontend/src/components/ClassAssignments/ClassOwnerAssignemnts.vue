<template>
  <div class="d-flex flex-wrap">
    <div class="pa-4 w-20 w-xs-50">
      <h1>Assignments</h1>
      <v-text-field
        variant="outlined"
        label="Search"
        v-model="assignmentFilter"
      ></v-text-field>
      <div class="pb-4">
        <v-btn
          class="ml-2"
          @click="createAssignmentDialog = true"
          icon="mdi-plus"
        ></v-btn>
        <v-btn
          class="ml-2"
          @click="deleteSelectedAssignment"
          icon="mdi-delete"
        ></v-btn>
        <v-btn
          class="ml-2"
          @click="updateAssignmentDialog = true"
          icon="mdi-file-edit"
        ></v-btn>
      </div>
      <v-list lines="one" style="height: 80%" class="overflow-y-auto">
        <v-list-item
          v-for="item in assignments"
          :key="item.id"
          :title="item.name"
          :active="selectedAssignment?.id === item.id"
          @click="setSelectedAssignment(item)"
        ></v-list-item>
      </v-list>
    </div>
    <div class="pa-4 w-60 w-xs-100" style="min-height: 100%">
      <div v-if="selectedAssignment" class="d-flex flex-column">
        <div style="min-height: 50%">
          <AssignmentContent
            :assignment="selectedAssignment"
          ></AssignmentContent>
        </div>
        <div>
          <div v-if="selectedMember">
            <h1>Feedback</h1>
            <div v-if="selectedUserSubmission">
              <v-list class="d-flex">
                <v-list-item
                  class="pa-1"
                  v-for="(file, i) in selectedUserSubmission.files"
                >
                  <v-chip @click="download(classId, file)">
                    {{ file.name }}
                  </v-chip>
                </v-list-item>
              </v-list>
              <v-textarea
                variant="outlined"
                label="Give your feedback"
              ></v-textarea>
              <v-btn @click="saveFeedback">Save</v-btn>
              <v-btn>Delete</v-btn>
            </div>
            <div v-else>
              <h3>No submission yet</h3>
            </div>
          </div>
          <div v-else>
            <h1>Select student to give feedback</h1>
          </div>
        </div>
      </div>
      <div v-else>
        <h1>Select assignment to view</h1>
      </div>
    </div>
    <div class="pa-4 w-20 w-xs-50">
      <h1>Students</h1>
      <v-text-field
        variant="outlined"
        label="Search"
        v-model="membersFilter"
      ></v-text-field>
      <v-list lines="one" style="height: 80%" class="overflow-y-auto">
        <v-list-item
          v-for="member in members!"
          :key="member.id"
          :title="member.username"
          :active="selectedMember?.id === member.id"
          @click="selectedMember = member"
        >
        </v-list-item>
      </v-list>
    </div>
  </div>

  <v-dialog v-model="createAssignmentDialog" width="100%">
    <ClassAssignmentCreateForm @close="closeDialog"></ClassAssignmentCreateForm>
  </v-dialog>
  <v-dialog v-model="updateAssignmentDialog" width="100%">
    <ClassAssignmentUpdateForm
      @close="closeDialog"
      :assignment="(toUpdateAssignmentProp(selectedAssignment as any))"
    ></ClassAssignmentUpdateForm>
  </v-dialog>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useLazyQuery, useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { MyIdQuery } from "@/shared";
import ClassAssignmentCreateForm from "@/components/ClassAssignments/ClassAssignmentCreateForm.vue";
import ClassAssignmentUpdateForm from "@/components/ClassAssignments/ClassAssignmentUpdateForm.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import AssignmentContent from "@/components/ClassAssignments/AssignmentContent.vue";
import { useDisplay } from "vuetify";
import { useRouter } from "vue-router";
import { OwnerAssignmentsFragmentFragment } from "@/gql/graphql";
import { download } from "../../shared";
import { set } from "@vueuse/core";
import { cache } from "@/client";

const { mobile } = useDisplay();

const OwnerAssignmentsFragment = graphql(/* GraphQL */ `
  fragment OwnerAssignmentsFragment on Class {
    members {
      id
      username
    }
    assignments {
      id
      name
      dueAt
      content
      files {
        id
        name
      }
      ...AssignmentContentFragment
      submissions {
        id
        createdAt
        updatedAt
        user {
          id
          username
        }
        files {
          id
          name
        }
      }
    }
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof OwnerAssignmentsFragment> | null;
}>();

const class_ = computed(() =>
  useFragment(OwnerAssignmentsFragment, props.class_)
);
const assignmentFilter = ref("");
const assignments = computed(() => {
  if (assignmentFilter.value === "") {
    return class_.value?.assignments ?? [];
  }
  return (class_.value?.assignments ?? []).filter((c) =>
    c.name.includes(assignmentFilter.value)
  );
});

const membersFilter = ref("");
const members = computed(() => {
  if (membersFilter.value === "") {
    return class_.value?.members ?? [];
  }
  return (class_.value?.members ?? []).filter((c) =>
    c.username.includes(membersFilter.value)
  );
});

const submissions = computed(() => {
  const assignment = selectedAssignment.value;
  if (!assignment) {
    return [];
  }
  return assignment.submissions;
});

type Assignment = (typeof assignments.value)[0];
const selectedAssignment = ref<null | Assignment>(null);

const setSelectedAssignment = (assignment: Assignment) => {
  selectedAssignment.value = assignment;
  selectedMember.value = null;
};

type Member = (typeof members.value)[0];
const selectedMember = ref<null | Member>(null);

type Submission = (typeof submissions.value)[0];
const selectedUserSubmission = computed(() => {
  const user = selectedMember.value;
  if (!user) {
    return null;
  }
  return (
    submissions.value.find((s: Submission) => s.user.id === user.id) ?? null
  );
});

const createAssignmentDialog = ref(false);
const updateAssignmentDialog = ref(false);
const closeDialog = () => {
  createAssignmentDialog.value = false;
  updateAssignmentDialog.value = false;
};

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;

let SaveFeedbackMutation = graphql(/* GraphQL */ `
  mutation CreateAssignmentSubmissionFeedback(
    $input: CreateAssignmanetSubmissionFeedbackInput!
  ) {
    createAssignmentSubmissionFeedback(input: $input)
  }
`);

let { mutate: saveFeedbackMutation } = useMutation(SaveFeedbackMutation);

const saveFeedback = () => {
  saveFeedbackMutation({
    input: {
      assignmentSubmissionId: selectedUserSubmission.value!.id,
      feedback: "dupksa",
    },
  });
};

let DeleteAssignmentMutation = graphql(/* GraphQL */ `
  mutation DeleteAssignment($classId: ID!, $assignmentId: ID!) {
    deleteAssignment(classId: $classId, assignmentId: $assignmentId)
  }
`);

let { mutate: deleteAssignmentMutation } = useMutation(
  DeleteAssignmentMutation
);

const deleteSelectedAssignment = () => {
  if (!selectedAssignment.value) {
    return;
  }

  deleteAssignmentMutation({
    classId,
    assignmentId: selectedAssignment.value.id,
  });

  selectedAssignment.value = null;
};

const toUpdateAssignmentProp = (assignment: Assignment): any => {
  return {
    id: assignment.id,
    name: assignment.name,
    content: assignment.content,
    dueDate: assignment.dueAt,
    newFiles: [],
    oldFiles: assignment.files.map((f) => ({ id: f.id, name: f.name })),
  };
};
</script>
