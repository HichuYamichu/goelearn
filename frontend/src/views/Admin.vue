<template>
  <v-card>
    <v-tabs v-model="tab" bg-color="primary">
      <v-tab value="one">Users</v-tab>
      <v-tab value="two">Classes</v-tab>
    </v-tabs>

    <v-card-text>
      <v-window v-model="tab">
        <v-window-item value="one">
          <AdminUserList></AdminUserList>
        </v-window-item>
        <v-window-item value="two">
          <AdminClassList></AdminClassList>
        </v-window-item>
      </v-window>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import AdminUserList from "@/components/admin/AdminUserList.vue";
import AdminClassList from "@/components/admin/AdminClassList.vue";

const tab = ref("");
</script>
