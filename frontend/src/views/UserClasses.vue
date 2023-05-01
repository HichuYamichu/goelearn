<template>
  <v-main>
    <v-container>
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
  </v-main>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { computed } from "vue";

const ME_QUERY = graphql(/* GraphQL */ `
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

const { result } = useQuery(ME_QUERY);

const classes = computed(() => result.value?.me.clesses);
</script>
