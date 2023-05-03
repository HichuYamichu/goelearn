<template>
  <v-container class="mt-3">
    <v-row>
      <v-spacer></v-spacer>
      <v-col cols="6">
        <form @submit.prevent="submit">
          <h4 class="text-h4 mb-2">Create class</h4>
          <v-text-field label="Name"></v-text-field>
          <v-textarea label="Description"></v-textarea>
          <div class="d-flex align-center">
            <v-text-field label="Tags"></v-text-field>
            <v-btn class="ml-3 mb-3"> save </v-btn>
          </div>
          <v-list class="d-flex">
            <v-list-item class="pa-1" v-for="i in 5">
              <v-chip> Chip </v-chip>
            </v-list-item>
          </v-list>
          <v-checkbox label="Public" type="checkbox"></v-checkbox>
          <v-file-input label="Class image"></v-file-input>
          <v-btn class="me-4 bg-success" type="submit"> submit </v-btn>
          <v-btn> clear </v-btn>
        </form>
      </v-col>
      <v-spacer></v-spacer>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
const submit = () => {};
</script>
