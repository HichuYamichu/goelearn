<template>
  <div class="px-0 px-lg-16" v-if="isOwner">
    <ClassOwnerAssignments :class_="class_" />
  </div>
  <div class="px-0 px-lg-16" v-else>
    <StudentAssignments :class_="class_" />
  </div>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { MyIdQuery } from "@/shared";
import { useDisplay } from "vuetify";
import ClassOwnerAssignments from "@/components/ClassAssignments/ClassOwnerAssignemnts.vue";
import StudentAssignments from "@/components/ClassAssignments/StudentAssignments.vue";

const AssignmentsFragment = graphql(/* GraphQL */ `
  fragment AssignmentsFragment on Class {
    ownerId
    ...OwnerAssignmentsFragment
    ...StudentAssignmentsFragment
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof AssignmentsFragment> | null;
}>();

const class_ = computed(() => useFragment(AssignmentsFragment, props.class_));

const { result: myIdResult } = useQuery(MyIdQuery);
const isOwner = computed(() => {
  if (!myIdResult.value?.me?.id) return false;
  return myIdResult.value?.me?.id === class_.value?.ownerId;
});
</script>
