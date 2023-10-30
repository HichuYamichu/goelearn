/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

interface ImportMetaEnv {
  readonly VITE_GRAPHQL_ENDPOINT: string;
  readonly VITE_GRAPHQL_ENPOINT_WS: string;
  readonly VITE_BASE_ENDPOINT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
