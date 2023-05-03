<template>
  <v-container class="mt-5">
    <v-row>
      <v-col cols="12" class="d-flex">
        <v-btn to="/class-create" class="bg-primary">Create new class</v-btn>
        <p class="font-weight-bold mx-5 my-3">OR</p>
        <v-btn to="/explore">Join join existing</v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12" class="d-flex">
        <v-text-field
          variant="outlined"
          label="Search your classes"
          hide-details="auto"
          autofocus
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col v-for="c in classes" :key="c.id" cols="4">
        <v-card height="300" :to="`/class/${c.id}`">
          <v-img
            src="https://cdn.vuetifyjs.com/images/cards/sunshine.jpg"
            height="200px"
            cover
          ></v-img>
          <v-card-title> {{ c.name }} </v-card-title>

          <v-card-subtitle>
            {{ c.description }}
          </v-card-subtitle>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { computed } from "vue";

const MeQuery = graphql(/* GraphQL */ `
  query UserClassesMeQuery {
    me {
      clesses {
        id
        name
        description
      }
    }
  }
`);

const { result } = useQuery(MeQuery);

const classes = computed(() => result.value?.me.clesses);
</script>
