<template>
  <div class="d-flex flex-wrap px-lg-16 py-8 pa-6">
    <div class="d-flex w-100 flex-wrap">
      <h1>Learn what interests you!</h1>
    </div>
    <div class="d-flex w-100 mt-4">
      <v-text-field
        @keyup.enter.native="forceRefetch"
        v-model="query"
        variant="outlined"
        label="Search for classes by their name, description or tags"
        hide-details="auto"
        autofocus
      ></v-text-field>
    </div>
    <div class="d-flex w-100 gap mt-5 flex-wrap v">
      <div
        class="d-flex w-15 full-mobile flex-wrap"
        v-for="c in classes"
        :key="c.id"
      >
        <v-card class="w-100" height="300" @click="join(c.id)">
          <v-img
            v-if="c.hasImage"
            :src="`${baseURL}/files/class-image/${c.id}`"
            alt="avatar"
            height="200px"
            cover
          ></v-img>
          <v-img
            v-else
            :src="`https://ui-avatars.com/api/?size=200&name=${c.name}}`"
            height="200px"
          ></v-img>
          <v-card-title> {{ c.name }} </v-card-title>

          <v-card-subtitle>
            {{ c.description }}
          </v-card-subtitle>
        </v-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed, onMounted, ref } from "vue";
const baseURL = import.meta.env.VITE_BASE_ENDPOINT;

const dialog = ref(false);
const query = ref("");

const ClassesSearchQuery = graphql(/* GraphQL */ `
  query classesBySearch($query: String!) {
    classesBySearch(query: $query) {
      id
      name
      description
      hasImage
    }
  }
`);

const { result, refetch } = useQuery(
  ClassesSearchQuery,
  () => ({
    query: query.value,
  }),
  {
    debounce: 350,
  }
);

const forceRefetch = () => {
  refetch();
};

const classes = computed(() => result.value?.classesBySearch ?? []);

const JoinClassMutation = graphql(/* GraphQL */ `
  mutation JoinClass($classId: ID!) {
    joinClass(classId: $classId)
  }
`);

const { mutate: joinClass, onDone } = useMutation(JoinClassMutation, {
  refetchQueries: ["UserClassesMeQuery"],
});

const join = (id: string) => {
  joinClass({ classId: id });
  dialog.value = false;
};

onDone((e) => {
  if (e.data?.joinClass) {
    router.push(`/class/${e.data.joinClass}`);
  }
});

onMounted(() => {
  forceRefetch();
});
</script>

<style scoped>
.w-45 {
  width: 49%;
}

@media only screen and (max-width: 500px) {
  .full-mobile {
    width: 100% !important;
  }
}

.gap {
  gap: 1rem;
}
</style>
