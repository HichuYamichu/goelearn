<template>
  <div class="d-flex flex-wrap px-lg-16 py-8 pa-6">
    <div class="d-flex w-100 flex-wrap">
      <template v-if="isMod">
        <v-btn to="/class-create" class="bg-primary">Create new class</v-btn>
        <p class="font-weight-bold mx-5 my-3">OR</p>
      </template>
      <v-btn to="/explore">Join existing</v-btn>
    </div>
    <div class="d-flex w-100 mt-4">
      <v-text-field
        v-model="filter"
        variant="outlined"
        label="Search your classes"
        hide-details="auto"
        autofocus
      ></v-text-field>
    </div>
    <div class="d-flex w-100 mt-5 flex-wrap justify-start gap">
      <div
        class="d-flex w-15 full-mobile flex-wrap"
        v-for="c in classes"
        :key="c.id"
      >
        <v-card class="w-100" height="300" :to="`/class/${c.id}`">
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
import { useQuery } from "@vue/apollo-composable";
import { computed, ref } from "vue";
const baseURL = import.meta.env.VITE_BASE_ENDPOINT;

const filter = ref("");

const MeQuery = graphql(/* GraphQL */ `
  query UserClassesMeQuery {
    me {
      id
      userType
      clesses {
        id
        name
        description
        hasImage
      }
    }
  }
`);

const { result } = useQuery(MeQuery);
const isMod = computed(
  () =>
    result.value?.me.userType === "MOD" || result.value?.me.userType === "ADMIN"
);

const classes = computed(() => {
  if (filter.value === "") {
    return result.value?.me.clesses;
  }
  return result.value?.me.clesses.filter((c) => c.name.includes(filter.value));
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
