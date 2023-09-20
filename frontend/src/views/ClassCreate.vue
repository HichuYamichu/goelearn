<template>
  <v-container class="mt-3">
    <v-row>
      <v-spacer></v-spacer>
      <v-col cols="6">
        <ClassCreate></ClassCreate>
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
import ClassCreate from "@/components/ClassCreate.vue";
</script>
