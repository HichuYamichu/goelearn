<template>
  <v-card width="50%" class="center">
    <v-toolbar dark color="error">
      <v-btn icon dark @click="emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
      <v-toolbar-title>Error</v-toolbar-title>
      <v-spacer></v-spacer>
    </v-toolbar>
    <div class="pa-16">
      <p class="text-h6 text-center">{{ errorMessage }}</p>
    </div>
  </v-card>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { ref, watch } from "vue";
import { client, error } from "@/client";
import { computed } from "vue";
import { useRouter } from "vue-router";

interface Error {
  message: string;
}

const props = defineProps<{
  error: Error[];
}>();

const emit = defineEmits(["close"]);

const errorMessage = computed(() => {
  const err = props.error[0];
  return err.message;
});
</script>

<style scoped>
.center {
  margin: auto;
}
</style>
