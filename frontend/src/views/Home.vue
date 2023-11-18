<template>
  <v-container class="fill-height">
    <v-responsive class="d-flex text-center fill-height pa-4 justify-center">
      <div class="text-body-2 font-weight-light mb-n1">Welcome to</div>
      <h1 class="text-h2 font-weight-bold">Go e-Learn</h1>
      <div class="text-h4 font-weight-light mb-n1">
        A simple and easy to use e-learning platform
      </div>
      <div class="d-flex justify-center">
        <div class="d-flex flex-wrap justify-space-evenly promo mt-8 pa-0">
          <v-timeline :side="side" align="start" class="mr-8 pa-0">
            <v-timeline-item
              v-for="(item, i) in items"
              :key="i"
              :dot-color="item.color"
              :icon="item.icon"
              fill-dot
            >
              <v-card>
                <v-card-title :class="['text-h6', `bg-${item.color}`]">
                  {{ item.title }}
                </v-card-title>
                <v-card-text class="pa-4 bg-white text--primary">
                  <p>
                    {{ item.content }}
                  </p>
                </v-card-text>
              </v-card>
            </v-timeline-item>
          </v-timeline>
        </div>
      </div>
    </v-responsive>
  </v-container>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import { useDisplay } from "vuetify";

const { mobile } = useDisplay();

const side = computed(() => (mobile.value ? "end" : undefined));

const items = [
  {
    color: "primary",
    icon: "mdi-account",
    title: "Communication",
    content: "Share your thoughts with your peers and teachers using our chat.",
  },
  {
    color: "primary",
    icon: "mdi-layers-triple",
    title: "Knowledge sharing",
    content: "Explore educational resources shared by your peers and teachers.",
  },
  {
    color: "primary",
    icon: "mdi-book-variant",
    title: "Student-teacher interaction",
    content: "Publish or solve assignments. Give or receive feedback.",
  },
  {
    color: "primary",
    icon: "mdi-star",
    title: "Collaboration",
    content:
      "Talk face to face with your peers and teachers using our video conferencing solution.",
  },
];
</script>

<style scoped>
.promo {
  height: 80%;
  width: 60%;
}
</style>
