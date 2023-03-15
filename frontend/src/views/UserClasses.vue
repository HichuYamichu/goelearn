<template>
  <v-main>
    <v-container>
      <v-row>
        <v-col v-for="n in 24" :key="n" cols="4">
          <v-card height="300" to="/class">
            <v-img
              src="https://cdn.vuetifyjs.com/images/cards/sunshine.jpg"
              height="200px"
              cover
            ></v-img>
            <v-card-title> Class Name </v-card-title>

            <v-card-subtitle>
              Lorem ipsum dolor sit amet consectetur adipisicing elit.
              Repellendus dolor in non, laborum aliquam ea totam fugiat qui fuga
              eveniet adipisci accusantium aliquid reiciendis harum sunt saepe
              sit necessitatibus similique!
            </v-card-subtitle>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>
