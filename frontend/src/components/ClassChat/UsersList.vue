<template>
  <v-navigation-drawer location="right">
    <v-list>
      <v-list-item v-for="n in 5" :key="n" :title="`User ${n}`" link>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>
