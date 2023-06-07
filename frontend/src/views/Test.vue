<template>
  <v-container fluid>
    <v-row>
      <v-col cols="3">
        <v-card>
          <v-list>
            <template v-for="folder in folders" :key="folder.id">
              <v-list-item @click="navigate(folder)">
                <v-icon v-if="folder.open">mdi-folder-open</v-icon>
                <v-icon v-else>mdi-folder</v-icon>
                <v-list-item-content>
                  <v-list-item-title>{{ folder.name }}</v-list-item-title>
                </v-list-item-content>
              </v-list-item>
              <v-list-item-group v-if="folder.open" v-model="folder.open">
                <v-list-item
                  v-for="subfolder in folder.subfolders"
                  :key="subfolder.id"
                  @click="navigate(subfolder)"
                >
                  <v-icon>mdi-folder</v-icon>
                  <v-list-item-content>
                    <v-list-item-title>{{ subfolder.name }}</v-list-item-title>
                  </v-list-item-content>
                </v-list-item>
              </v-list-item-group>
            </template>
          </v-list>
          <v-form @submit.prevent="createFolder">
            <v-text-field
              v-model="newFolderName"
              label="New Folder"
            ></v-text-field>
            <v-btn type="submit" color="primary">Create Folder</v-btn>
          </v-form>
        </v-card>
      </v-col>
      <v-col cols="9">
        <v-card>
          <v-list>
            <v-list-item v-for="file in files" :key="file.id">
              <v-icon>mdi-file</v-icon>
              <v-list-item-content>
                <v-list-item-title>{{ file.name }}</v-list-item-title>
              </v-list-item-content>
              <v-list-item-action>
                <v-btn icon @click="deleteFile(file)">
                  <v-icon>mdi-delete</v-icon>
                </v-btn>
              </v-list-item-action>
            </v-list-item>
          </v-list>
          <v-file-input label="Upload File" @change="uploadFile"></v-file-input>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
export default {
  data() {
    return {
      folders: [
        {
          id: 1,
          name: "Folder 1",
          open: false,
          subfolders: [
            { id: 4, name: "Subfolder 1.1" },
            { id: 5, name: "Subfolder 1.2" },
          ],
        },
        {
          id: 2,
          name: "Folder 2",
          open: false,
          subfolders: [
            { id: 6, name: "Subfolder 2.1" },
            { id: 7, name: "Subfolder 2.2" },
          ],
        },
        { id: 3, name: "Folder 3", open: false, subfolders: [] },
      ],
      files: [
        { id: 1, name: "File 1.txt" },
        { id: 2, name: "File 2.jpg" },
        { id: 3, name: "File 3.pdf" },
      ],
      newFolderName: "",
    };
  },
  methods: {
    createFolder() {
      if (this.newFolderName.trim() !== "") {
        const newFolder = {
          id: this.folders.length + 1,
          name: this.newFolderName,
          open: false,
          subfolders: [],
        };
        this.folders.push(newFolder);
        this.newFolderName = "";
      }
    },
    deleteFile(file) {
      const index = this.files.findIndex((f) => f.id === file.id);
      if (index !== -1) {
        this.files.splice(index, 1);
      }
    },
    uploadFile(event) {
      const files = event.target.files;
      for (let i = 0; i < files.length; i++) {
        const newFile = { id: this.files.length + 1, name: files[i].name };
        this.files.push(newFile);
      }
    },
    navigate(folder) {
      folder.open = !folder.open;
    },
  },
};
</script>

<style scoped>
/* Add custom styling here if needed */
</style>
