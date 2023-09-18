import { ApolloLink, ApolloClient, InMemoryCache } from "@apollo/client/core";
import { concat, split } from "@apollo/client/link/core";
import { getOperationAST } from "graphql";
import { createUploadLink } from "apollo-upload-client";
import { createClient } from "graphql-ws";
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import gql from "graphql-tag";

const authMiddleware = new ApolloLink((operation, forward) => {
  const token = localStorage.getItem("token");
  operation.setContext({
    headers: {
      authorization: token ? `Bearer ${token}` : "",
    },
  });
  return forward(operation);
});

const wsLink = new GraphQLWsLink(
  createClient({
    url: import.meta.env.VITE_GRAPHQL_ENDPOINT_WS,
    connectionParams: () => {
      const token = localStorage.getItem("token");
      return {
        token: `Bearer ${token}`,
      };
    },
  })
);

const httpLink = createUploadLink({
  uri: import.meta.env.VITE_GRAPHQL_ENDPOINT,
});

const link = split(
  ({ query, operationName }) => {
    const definition = getOperationAST(query, operationName);

    return (
      definition?.kind === "OperationDefinition" &&
      definition.operation === "subscription"
    );
  },
  wsLink,
  httpLink
);

export const cache = new InMemoryCache();

const IS_LOGGED_IN = gql`
  query IsLoggedIn {
    isLoggedIn @client
  }
`;

cache.writeQuery({
  query: IS_LOGGED_IN,
  data: {
    isLoggedIn: !!localStorage.getItem("token"),
  },
});

export const client = new ApolloClient({
  link: concat(authMiddleware, link),
  cache,
});
