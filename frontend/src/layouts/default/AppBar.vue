<template>
  <v-app-bar>
    <v-app-bar-nav-icon
      class="hidden-md-and-up"
      @click="drawer = !drawer"
    ></v-app-bar-nav-icon>
    <v-spacer></v-spacer>

    <v-container class="fill-height align-center d-none d-sm-flex">
      <v-btn
        v-for="link in links"
        :key="link.target"
        variant="text"
        :to="link.target"
        exact
      >
        {{ link.text }}
      </v-btn>

      <v-spacer></v-spacer>
    </v-container>

    <div class="fill-height d-flex align-center mr-3" v-if="!isLoggedIn">
      <v-btn to="/login"> Login </v-btn>
      <v-btn to="/register"> Register </v-btn>
    </div>
    <div class="fill-height d-flex align-center mr-3" v-else>
      <v-btn to="/settings">
        <template v-slot:default>
          {{ username }}
        </template>
        <template v-slot:prepend>
          <v-avatar v-if="hasAvatar">
            <v-img
              :src="`${baseURL}/files/user-avatar/${id}`"
              alt="avatar"
            ></v-img>
          </v-avatar>
          <v-icon v-else icon="mdi-account-circle"></v-icon>
        </template>
      </v-btn>
      <v-btn @click="logout">Logout</v-btn>
    </div>
  </v-app-bar>

  <v-navigation-drawer v-model="drawer" temporary>
    <v-list nav>
      <v-list-item v-for="link in links" :key="link.target" :to="link.target">{{
        link.text
      }}</v-list-item>
    </v-list>
  </v-navigation-drawer>

  <v-dialog v-model="showErrorDialog" width="100%">
    <ErrorPopup v-if="error" :error="error" @close="showErrorDialog = false" />
  </v-dialog>
</template>

<script lang="ts" setup>
import { graphql } from "@/gql";
import { useQuery } from "@vue/apollo-composable";
import { ref, watch } from "vue";
import { client, error } from "@/client";
import { computed } from "vue";
import { useRouter } from "vue-router";
import ErrorPopup from "@/components/ErrorPopup.vue";
const baseURL = import.meta.env.VITE_BASE_ENDPOINT;

const router = useRouter();

const drawer = ref(false);

const IsLoggedIn = graphql(/* GraphQL */ `
  query IsLoggedIn {
    isLoggedIn @client
  }
`);

const { result } = useQuery(IsLoggedIn);

const isLoggedIn = computed(() => result.value?.isLoggedIn ?? false);

const links = computed(() => {
  if (isLoggedIn.value) {
    return [
      { text: "Home", target: "/" },
      { text: "My Classes", target: "/classes" },
      { text: "My Assignments", target: "/assignments" },
      { text: "Explore", target: "/explore" },
    ];
  } else {
    return [{ text: "Home", target: "/" }];
  }
});

const MeQuery = graphql(/* GraphQL */ `
  query AppBarMeQuery {
    me {
      id
      username
      hasAvatar
    }
  }
`);

const { result: meResult } = useQuery(MeQuery, null, () => ({
  enabled: isLoggedIn.value,
}));

const hasAvatar = computed(() => meResult.value?.me.hasAvatar ?? false);
const id = computed(() => meResult.value?.me.id ?? "");
const username = computed(() => meResult.value?.me.username ?? "");

const logout = () => {
  localStorage.removeItem("token");
  client.writeQuery({
    query: IsLoggedIn,
    data: {
      isLoggedIn: false,
    },
  });
  router.push({ name: "Home" });
};

const showErrorDialog = ref(false);
watch(error, (err: any) => {
  showErrorDialog.value = true;
});
</script>
