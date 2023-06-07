<template>
  <v-container class="ma-0 full-height pad" fluid>
    <v-row class="grow pa-0" no-gutters>
      <v-col cols="12" class="d-flex px-5">
        <v-row>
          <v-col cols="2">
            <v-text-field
              v-model="newDirName"
              label="Folder name"
              variant="outlined"
              hide-details="auto"
            ></v-text-field>
            <v-btn @click="createDir" class="bg-primary" block>Create</v-btn>
          </v-col>
          <v-col cols="2">
            <v-file-input
              v-model="filesToUpload"
              label="Your file"
              variant="outlined"
              hide-details="auto"
            ></v-file-input>
            <v-btn @click="uploadFiles" block>Upload here</v-btn>
          </v-col>
          <v-col cols="2">
            <v-text-field
              style="visibility: hidden"
              variant="outlined"
              hide-details="auto"
            ></v-text-field>
            <v-btn class="bg-success" block> Download selected </v-btn>
          </v-col>
          <v-col cols="2">
            <v-text-field
              style="visibility: hidden"
              variant="outlined"
              hide-details="auto"
            ></v-text-field>
            <v-btn class="bg-error" block> Delete selected </v-btn>
          </v-col>
        </v-row>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12" class="px-5">
        <v-breadcrumbs class="pa-0">
          <template v-for="item in selectedDirectoryTree" :key="item.name">
            <v-breadcrumbs-item @click="open(item)" class="pointer">
              <span class="font-weight-bold text-h5">
                {{ item.name }}
              </span>
            </v-breadcrumbs-item>
            <v-breadcrumbs-divider class="pa-0">/</v-breadcrumbs-divider>
          </template>
        </v-breadcrumbs>
        <v-table>
          <thead>
            <tr>
              <th class="text-left font-weight-black">Name</th>
              <th class="text-left font-weight-black">Size</th>
              <th class="text-left font-weight-black">Type</th>
              <th class="text-left font-weight-black">Date modified</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="file in files" :key="file.id">
              <td>
                <p
                  @click="
                    open({
                      id: file.id,
                      name: file.name,
                      fileType: file.fileType,
                    })
                  "
                  class="pointer"
                >
                  <v-icon
                    v-if="file.fileType == 'DIRECTORY'"
                    icon="mdi-folder"
                  ></v-icon>
                  <v-icon v-else icon="mdi-file-download-outline"></v-icon>
                  {{ file.name }}
                </p>
              </td>
              <td></td>
              <td>{{ file.fileType }}</td>
              <td></td>
            </tr>
          </tbody>
        </v-table>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { useMutation } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";

const FilesFragment = graphql(/* GraphQL */ `
  fragment FilesFragment on Class {
    id
    ownerId
    files {
      id
      name
      fileType
      parent
    }
  }
`);

const props = defineProps<{
  class_?: FragmentType<typeof FilesFragment> | null;
  loading: boolean;
}>();

const class_ = ref(useFragment(FilesFragment, props.class_));

watch(props, () => {
  class_.value = useFragment(FilesFragment, props.class_);
});

const selectedDirectoryTree = ref([
  { id: null, name: "root", fileType: "DIRECTORY" },
]);
const files = computed(() => {
  if (!class_.value) return [];
  const files = class_.value.files;
  if (selectedDirectoryTree.value.length === 0) {
    return files.filter((f) => !f.parent);
  }
  return files.filter(
    (f) =>
      f.parent ===
      selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id
  );
});

const open = (item: any) => {
  if (item.fileType === "FILE") {
    downloadFile(
      `http://localhost:3000/files/class-files/${class_.value?.id}/${item.id}`,
      item.name
    );
    return;
  }

  if (item.id === null) {
    selectedDirectoryTree.value = [
      { id: null, name: "root", fileType: "DIRECTORY" },
    ];
    return;
  }
  if (
    item.id ===
    selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id
  ) {
    return;
  }

  selectedDirectoryTree.value.push(item);
};

const downloadFile = async (url: string, filename: string) => {
  const data = await fetch(url);
  const blob = await data.blob();
  const objectUrl = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.setAttribute("href", objectUrl);
  link.setAttribute("download", filename);
  link.style.display = "none";
  link.click();
};

const newDirName = ref("");

const CreateDirectoryMutation = graphql(/* GraphQL */ `
  mutation CreateDirecotry($classId: ID!, $name: String!, $parentId: ID) {
    createDirecotry(
      input: { classId: $classId, name: $name, parentId: $parentId }
    ) {
      id
    }
  }
`);

const { mutate: send } = useMutation(CreateDirectoryMutation, {
  refetchQueries: ["ClassClassByIdQuery"],
});

const createDir = () => {
  send({
    classId: class_.value!.id,
    name: newDirName.value,
    parentId:
      selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id,
  });
  newDirName.value = "";
};

const filesToUpload = ref<File[]>([]);

const UploadFilesMutation = graphql(/* GraphQL */ `
  mutation UploadFiles(
    $classId: ID!
    $files: [Upload!]!
    $parentId: ID
    $public: Boolean!
  ) {
    uploadFiles(
      input: {
        classId: $classId
        files: $files
        parentId: $parentId
        public: $public
      }
    )
  }
`);

const { mutate: upload } = useMutation(UploadFilesMutation, {
  refetchQueries: ["ClassClassByIdQuery"],
});

const uploadFiles = () => {
  upload({
    classId: class_.value!.id,
    files: filesToUpload.value,
    parentId:
      selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id,
    public: true,
  });
};
</script>

<style scoped>
.pointer {
  cursor: pointer;
}
.pad {
  padding: 2em 4em;
}
</style>
