<template>
  <v-navigation-drawer width="244">
    <v-list>
      <v-list-item v-for="n in 5" :key="n" :title="`Channel ${n}`" link>
      </v-list-item>
    </v-list>
    <v-divider></v-divider>
  </v-navigation-drawer>

  <v-main><!--  --></v-main>

  <v-navigation-drawer location="right">
    <v-list>
      <v-list-item v-for="n in 5" :key="n" :title="`User ${n}`" link>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>

  <v-footer app height="72">
    <v-text-field
      bg-color="grey-lighten-1"
      class="rounded-pill overflow-hidden"
      density="compact"
      hide-details
      variant="solo"
    ></v-text-field>
  </v-footer>
</template>
