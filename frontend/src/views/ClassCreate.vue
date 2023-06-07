<template>
  <v-container class="mt-3">
    <v-row>
      <v-spacer></v-spacer>
      <v-col cols="6">
        <form @submit.prevent="submit">
          <h4 class="text-h4 mb-2">Create class</h4>
          <v-text-field v-model="name" label="Name"></v-text-field>
          <v-textarea v-model="description" label="Description"></v-textarea>
          <div class="d-flex align-center">
            <v-text-field v-model="tag" label="Tag"></v-text-field>
            <v-btn class="ml-3 mb-3" @click="addTag"> save </v-btn>
          </div>
          <v-list class="d-flex">
            <v-list-item class="pa-1" v-for="(tag, i) in tags">
              <v-chip @click="removeTag(i)"> {{ tag }} </v-chip>
            </v-list-item>
          </v-list>
          <v-checkbox
            v-model="isPublic"
            label="Public"
            type="checkbox"
          ></v-checkbox>
          <v-file-input v-model="image" label="Class image"></v-file-input>
          <v-btn class="me-4 bg-success" type="submit"> submit </v-btn>
          <v-btn> clear </v-btn>
        </form>
      </v-col>
      <v-spacer></v-spacer>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { useMutation } from "@vue/apollo-composable";
import { reactive } from "vue";
import { Ref, ref } from "vue";

const tags = reactive([] as string[]);
const tag = ref("");
const name = ref("");
const description = ref("");
const isPublic = ref(true);
const image: Ref<File[] | undefined> = ref(undefined);

const addTag = () => {
  if (tag.value === "") {
    return;
  }
  tags.push(tag.value);
  tag.value = "";
};

const removeTag = (index: number) => {
  console.log({ index });
  console.log({ tags });

  console.log(tags.splice(index, 1));
};

const CreateClassMutation = graphql(/* GraphQL */ `
  mutation CreateClass($input: CreateClassInput!) {
    createClass(input: $input) {
      id
    }
  }
`);

const { mutate } = useMutation(CreateClassMutation, {
  refetchQueries: ["UserClassesMeQuery"],
});

const submit = () => {
  const img = image.value?.[0] ?? null;
  mutate({
    input: {
      name: name.value,
      description: description.value,
      tags: tags.join(" "),
      public: isPublic.value,
      image: img,
    },
  });
  router.push("/classes");
};
</script>
