<template>
  <form @submit.prevent="submit" class="w-100">
    <h4 class="text-h4 mb-2" v-if="!class_">Create class</h4>
    <h4 class="text-h4 mb-2" v-else>Update class</h4>
    <v-text-field
      v-model="classInput.name"
      label="Name"
      variant="outlined"
      :rules="classNameRules"
    ></v-text-field>
    <v-textarea
      v-model="classInput.description"
      label="Description"
      variant="outlined"
      :rules="classDescRules"
    ></v-textarea>
    <div class="d-flex align-center">
      <v-text-field v-model="tag" label="Tag" variant="outlined"></v-text-field>
      <v-btn class="ml-3 mb-3" @click="addTag"> save </v-btn>
    </div>
    <v-list class="d-flex">
      <v-list-item class="pa-1" v-for="(tag, i) in classInput.tags">
        <v-chip @click="removeTag(i)"> {{ tag }} </v-chip>
      </v-list-item>
    </v-list>
    <v-checkbox
      v-model="classInput.public"
      label="Public"
      type="checkbox"
    ></v-checkbox>
    <v-file-input
      v-model="image"
      label="Class image"
      variant="outlined"
    ></v-file-input>
    <v-btn class="me-4 bg-success" type="submit"> submit </v-btn>
    <v-btn class="bg-error" @click="deleteClass" v-if="class_"> Delete </v-btn>
  </form>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import router from "@/router";
import { useMutation } from "@vue/apollo-composable";
import { reactive, toRefs } from "vue";
import { Ref, ref } from "vue";

interface ClassInput {
  name: string;
  description: string;
  tags: string[];
  public: boolean;
  image: File | null;
}

const props = defineProps<{
  class_?: {
    id: string;
  } & ClassInput;
}>();

const classInput = reactive({
  name: props.class_?.name ?? "",
  description: props.class_?.description ?? "",
  tags: props.class_?.tags ?? ([] as string[]),
  public: props.class_?.public ?? true,
  image: null,
} as ClassInput);

const tag = ref("");
const image: Ref<File[] | undefined> = ref(undefined);

const addTag = () => {
  if (tag.value === "") {
    return;
  }
  classInput.tags.push(tag.value);
  tag.value = "";
};

const removeTag = (index: number) => {
  console.log(classInput.tags);
  classInput.tags.splice(index, 1);
};

const CreateClassMutation = graphql(/* GraphQL */ `
  mutation CreateClass($input: CreateClassInput!) {
    createClass(input: $input) {
      id
    }
  }
`);

const { mutate: createMutation } = useMutation(CreateClassMutation, {
  refetchQueries: ["UserClassesMeQuery"],
});

const UpdateClassMutation = graphql(/* GraphQL */ `
  mutation UpdateClass($classId: ID!, $input: UpdateClassInput!) {
    updateClass(classId: $classId, classInput: $input)
  }
`);

const { mutate: updateMutation } = useMutation(UpdateClassMutation);

const submit = () => {
  const img = image.value?.[0] ?? null;

  if (props.class_) {
    updateMutation({
      classId: props.class_.id,
      input: {
        name: classInput.name,
        description: classInput.description,
        tags: classInput.tags.join(" "),
        public: classInput.public,
        image: img,
      },
    });
  } else {
    createMutation({
      input: {
        name: classInput.name,
        description: classInput.description,
        tags: classInput.tags.join(" "),
        public: classInput.public,
        image: img,
      },
    });
    router.push("/classes");
  }
};

const classNameRules = [
  (v: string) => !!v || "Name is required",
  (v: string) => v.length <= 35 || "Name must be less than 20 characters",
];

const classDescRules = [
  (v: string) => !!v || "Description is required",
  (v: string) =>
    v.length <= 200 || "Description must be less than 100 characters",
];

const DeleteClassMutation = graphql(/* GraphQL */ `
  mutation DeleteClass($classId: ID!) {
    deleteClass(classId: $classId)
  }
`);

const { mutate: deleteMutation } = useMutation(DeleteClassMutation);

const deleteClass = () => {
  deleteMutation({
    classId: props.class_?.id ?? "",
  });
  router.push("/classes");
};
</script>
