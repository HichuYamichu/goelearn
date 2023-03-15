<template>
  <!-- <v-data-table
    v-model:items-per-page="itemsPerPage"
    :headers="headers"
    :items="desserts"
    item-value="name"
    class="elevation-1"
  ></v-data-table> -->

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
      <tr v-for="item in desserts" :key="item.name">
        <td>
          <v-icon
            v-if="item.type == ItemType.Directory"
            icon="mdi-folder"
          ></v-icon>
          <v-icon v-else icon="mdi-file-download-outline"></v-icon>
          {{ item.name }}
        </td>
        <td>{{ item.size }}</td>
        <td>{{ ItemType[item.type] }}</td>
        <td>{{ item.date }}</td>
      </tr>
    </tbody>
  </v-table>
</template>

<script setup lang="ts">
enum ItemType {
  Directory,
  File,
}

const itemsPerPage = 10;
const headers = ["Name", "Size", "Type", "Date Modified"];

const desserts = [
  {
    name: "Images",
    size: 159,
    type: ItemType.Directory,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
  {
    name: "HelpfulFiles",
    size: 237,
    type: ItemType.Directory,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
  {
    name: "stuff",
    size: 262,
    type: ItemType.Directory,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
  {
    name: "File 1",
    size: 305,
    type: ItemType.File,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
  {
    name: "File 2",
    size: 356,
    type: ItemType.File,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
  {
    name: "File 3",
    size: 375,
    type: ItemType.File,
    date: new Date().toLocaleDateString("pl-Pl"),
  },
];
</script>
