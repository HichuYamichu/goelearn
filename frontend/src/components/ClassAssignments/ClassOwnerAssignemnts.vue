<template>
  <div class="d-flex flex-wrap">
    <div class="pa-4 w-20 w-xs-50">
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
      <v-btn @click="createAssignmentDialog = true" icon="mdi-plus"></v-btn>
      <v-btn @click="createAssignmentDialog = true" icon="mdi-delete"></v-btn>
      <v-btn
        @click="createAssignmentDialog = true"
        icon="mdi-file-edit"
      ></v-btn>
    </div>
    <div class="pa-4 w-20 w-xs-50">
      <h1>Students</h1>
      <v-text-field variant="outlined" label="Search"></v-text-field>
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
            </div>
            <div v-else>
              <h3>No submission yet</h3>
            </div>
            <v-textarea
              variant="outlined"
              label="Give your feedback"
            ></v-textarea>
            <v-btn @click="saveFeedback">Save</v-btn>
            <v-btn>Delete</v-btn>
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
  </div>
  <v-dialog v-model="createAssignmentDialog" width="100%">
    <ClassAssignmentForm @close="closeDialog"></ClassAssignmentForm>
  </v-dialog>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { MyIdQuery } from "@/shared";
import ClassAssignmentForm from "@/components/ClassAssignments/ClassAssignmentForm.vue";
import MemberList from "@/components/ClassChat/MemberList.vue";
import AssignmentContent from "@/components/ClassAssignments/AssignmentContent.vue";
import { useDisplay } from "vuetify";
import { useRouter } from "vue-router";
import { OwnerAssignmentsFragmentFragment } from "@/gql/graphql";
import { download } from "../../shared";

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
const assignments = computed(() => class_.value?.assignments ?? []);
const members = computed(() => class_.value?.members ?? []);
const submissions = computed(() => {
  const assignment = selectedAssignment.value;
  if (!assignment) {
    return [];
  }
  return assignment.submissions;
});

type Assignment = (typeof assignments.value)[0];
const selectedAssignment = ref<null | Assignment>(null);

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
const closeDialog = () => {
  createAssignmentDialog.value = false;
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
</script>
