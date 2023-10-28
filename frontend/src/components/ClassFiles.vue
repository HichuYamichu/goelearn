<template>
  <div class="d-flex flex-wrap mt-4 px-0 px-lg-16">
    <div v-if="isOwner" class="pa-4 full-mobile w-25">
      <v-text-field
        v-model="newDirName"
        label="Folder name"
        variant="outlined"
        hide-details="auto"
        :rules="classNameRules"
      ></v-text-field>
      <v-btn @click="createDir" class="bg-primary" size="large" block
        >Create</v-btn
      >
    </div>
    <div v-if="isOwner" class="pa-4 full-mobile w-25">
      <v-file-input
        v-model="filesToUpload"
        label="Your file"
        variant="outlined"
        hide-details="auto"
      ></v-file-input>
      <v-btn @click="uploadFiles" size="large" block>Upload here</v-btn>
    </div>
    <div class="pa-4 full-mobile w-25">
      <v-text-field
        :style="!mobile ? 'visibility: hidden' : 'display: none'"
        variant="outlined"
        hide-details="auto"
      ></v-text-field>
      <v-btn class="bg-success" size="large" block @click="downloadAll">
        Download selected
      </v-btn>
    </div>
    <div v-if="isOwner" class="pa-4 full-mobile w-25">
      <v-text-field
        :style="!mobile ? 'visibility: hidden' : 'display: none'"
        variant="outlined"
        hide-details="auto"
      ></v-text-field>

      <v-btn class="bg-error" block size="large" @click="deleteAll">
        Delete selected
      </v-btn>
    </div>
    <v-breadcrumbs class="pa-4 w-100">
      <template v-for="item in selectedDirectoryTree" :key="item.name">
        <v-breadcrumbs-item @click="open(item)" class="pointer">
          <span class="font-weight-bold text-h5">
            {{ item.name }}
          </span>
        </v-breadcrumbs-item>
        <v-breadcrumbs-divider class="pa-0">/</v-breadcrumbs-divider>
      </template>
    </v-breadcrumbs>
    <v-table density="compact" class="w-100">
      <thead>
        <tr>
          <th class="text-left font-weight-black" style="width: 10%">
            <v-checkbox-btn @click="checkAll"></v-checkbox-btn>
          </th>
          <th class="text-left font-weight-black" style="width: 90%">Name</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="file in files" :key="file.id">
          <td>
            <v-checkbox-btn
              v-model="selectedFiles"
              :value="file.id"
            ></v-checkbox-btn>
          </td>
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
        </tr>
      </tbody>
    </v-table>
  </div>
</template>

<script setup lang="ts">
import { FragmentType, graphql, useFragment } from "@/gql";
import { MyIdQuery } from "@/shared";
import { useMutation, useQuery } from "@vue/apollo-composable";
import { computed } from "vue";
import { ref, watch } from "vue";
import { useDisplay } from "vuetify";

const { mobile } = useDisplay();

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
    selectedFiles.value = [];
    return;
  }
  const stackIndex = selectedDirectoryTree.value.findIndex(
    (i) => i.id === item.id
  );
  if (stackIndex !== -1) {
    selectedDirectoryTree.value.splice(stackIndex + 1);
    selectedFiles.value = [];
    return;
  }
  if (
    item.id ===
    selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id
  ) {
    return;
  }

  selectedDirectoryTree.value.push(item);
  selectedFiles.value = [];
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

const { mutate: send } = useMutation(CreateDirectoryMutation);

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

const { mutate: upload } = useMutation(UploadFilesMutation);

const uploadFiles = () => {
  upload({
    classId: class_.value!.id,
    files: filesToUpload.value,
    parentId:
      selectedDirectoryTree.value[selectedDirectoryTree.value.length - 1].id,
    public: true,
  });
};

const selectedFiles = ref<string[]>([]);

const checkAll = () => {
  if (selectedFiles.value.length === files.value.length) {
    selectedFiles.value = [];
    return;
  }
  selectedFiles.value = files.value.map((f) => f.id);
};

const downloadAll = async () => {
  let res = await fetch(
    `http://localhost:3000/files/class-files/${class_.value?.id}/zip`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        file_ids: selectedFiles.value,
      }),
    }
  );
  let blob = await res.blob();
  let objectUrl = URL.createObjectURL(blob);
  let link = document.createElement("a");
  link.setAttribute("href", objectUrl);
  link.setAttribute("download", "files.zip");
  link.style.display = "none";
  link.click();
};

const DeleteFilesMutation = graphql(/* GraphQL */ `
  mutation DeleteFiles($fileIds: [ID!]!) {
    deleteFiles(fileIds: $fileIds)
  }
`);

const { mutate: deleteFiles } = useMutation(DeleteFilesMutation);

const deleteAll = () => {
  deleteFiles({
    fileIds: selectedFiles.value,
  });
};

const classNameRules = computed(() => {
  return [
    (v: string) => !!v || "Name is required",
    (v: string) => v.length <= 35 || "Name must be less than 20 characters",
  ];
});

const { result: myIdResult } = useQuery(MyIdQuery);
const isOwner = computed(() => {
  if (!myIdResult.value?.me?.id) return false;
  return myIdResult.value?.me?.id === class_.value?.ownerId;
});
</script>

<style scoped>
.pointer {
  cursor: pointer;
}

@media only screen and (max-width: 500px) {
  .full-mobile {
    width: 100% !important;
  }
}
</style>
