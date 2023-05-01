import App from "./App.vue";

import { createApp, h, provide } from "vue";

import { registerPlugins } from "@/plugins";
import { client } from "./client";
import { DefaultApolloClient } from "@vue/apollo-composable";

const app = createApp({
  setup() {
    provide(DefaultApolloClient, client);
  },
  render: () => h(App),
});

registerPlugins(app);

app.mount("#app");
