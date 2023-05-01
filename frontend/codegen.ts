import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  overwrite: true,
  schema: ["http://127.0.0.1:3000/api/v1/graphql", "./schema.graphql"],
  ignoreNoDocuments: true, // for better experience with the watcher
  documents: ["src/**/*.vue", "src/**/*.ts"],
  generates: {
    "./src/gql/": {
      preset: "client",
      config: {
        useTypeImports: true,
        skipTypename: true,
      },
    },
  },
};

export default config;
