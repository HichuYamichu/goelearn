<template>
  <v-container class="mt-5">
    <v-row>
      <v-col cols="12" class="d-flex">
        <h1>Learn what interests you!</h1>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12" class="d-flex">
        <v-text-field
          @keyup.enter.native="forceRefetch"
          v-model="query"
          variant="outlined"
          label="Search for classes by their name, description or tags"
          hide-details="auto"
          autofocus
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col v-for="c in classes" :key="c.id" cols="4">
        <v-dialog v-model="dialog" width="auto">
          <template v-slot:activator="{ props }">
            <v-card v-bind="props" height="300" @click="join(c.id)">
              <v-img
                v-if="c.hasImage"
                :src="`http://localhost:3000/files/class-image/${c.id}`"
                alt="avatar"
                height="200px"
                cover
              ></v-img>
              <v-img
                v-else
                :src="`https://ui-avatars.com/api/?size=200&name=${c.name}`"
                height="200px"
              ></v-img>
              <v-card-title> {{ c.name }} </v-card-title>
              <v-card-subtitle>
                {{ c.description }}
              </v-card-subtitle>
            </v-card>
          </template>

          <v-card>
            <v-card-text>
              You are about to join
              <span class="text-weight-bold">{{ c.name }}</span
              >. Are you sure?
            </v-card-text>
            <v-card-actions>
              <v-btn color="bg-primary" @click="join(c.id)">Join</v-btn>
              <v-btn color="bg-warning" @click="dialog = false">Close</v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
      </v-col>
    </v-row>
  </v-container>
  <v-dialog v-model="errDialog" width="auto">
    <v-card>
      <v-card-text>
        <span class="text-weight-bold">Error:</span> {{ errMessage }}
      </v-card-text>
      <v-card-actions>
        <v-btn color="primary" block @click="errDialog = false">Ok</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed, ref } from "vue";

const dialog = ref(false);
const errDialog = ref(false);
const errMessage = ref("");
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

const { mutate: joinClass, onError } = useMutation(JoinClassMutation);

const join = (id: string) => {
  joinClass({ classId: id });
  dialog.value = false;
};

onError((e) => {
  errMessage.value = e.message;
  errDialog.value = true;
});
</script>
