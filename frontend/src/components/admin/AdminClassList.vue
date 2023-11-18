<template>
  <div class="pa-2">
    <v-text-field
      label="Search class"
      variant="outlined"
      v-model="classFilter"
    ></v-text-field>
    <v-list>
      <v-list-item v-for="(class_, idx) in classes">
        <div class="d-flex gap justify-start">
          <h2>{{ class_.name }}</h2>
          <div class="d-flex justify-start w-20">
            <v-checkbox
              density="compact"
              label="Disable class"
              class="w-10"
              v-model="mutations.get(class_.id)!.deletedAt"
            ></v-checkbox>
          </div>
          <v-btn color="success" @click="saveClassMutation(class_.id)"
            >Save</v-btn
          >
        </div>
      </v-list-item>
    </v-list>
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { computed, ref, watch } from "vue";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { reactive } from "vue";

const AdminClassQuery = graphql(/* GraphQL */ `
  query AdminClassQuery($query: String!) {
    classesBySearch(query: $query) {
      id
      name
      deletedAt
    }
  }
`);

const classFilter = ref("");

const { result: classesResult } = useQuery(
  AdminClassQuery,
  () => ({
    query: classFilter.value,
  }),
  {
    debounce: 350,
  }
);

const classes = computed(() => classesResult.value?.classesBySearch ?? []);

watch(classes, (newVal) => {
  console.log(newVal);
  mutations.clear();
  newVal.forEach((class_) => {
    mutations.set(class_.id, {
      id: class_.id,
      deletedAt: class_.deletedAt != null,
    });
  });
});

type Mutation = {
  id: string;
  deletedAt: boolean;
};
const mutations = reactive(new Map<string, Mutation>());

const AdminClassUpdate = graphql(/* GraphQL */ `
  mutation AdminClassUpdate($classId: ID!, $deletedState: Boolean!) {
    adminDeleteClass(classId: $classId, deletedState: $deletedState)
  }
`);

const { mutate: adminClassUpdate } = useMutation(AdminClassUpdate);

const saveClassMutation = async (id: string) => {
  const deletedState = mutations.get(id)!.deletedAt;

  adminClassUpdate({
    classId: mutations.get(id)!.id,
    deletedState,
  });
};
</script>

<style scoped>
.gap {
  gap: 20px;
}
</style>
