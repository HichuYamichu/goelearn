import { useQuery } from "@vue/apollo-composable";
import { graphql } from "./gql";
import { until } from "@vueuse/core";

export const isClassOwner = async (classOnwerId: string) => {
  const MeQuery = graphql(/* GraphQL */ `
    query CheckClassOwner {
      me {
        id
      }
    }
  `);

  const { result } = useQuery(MeQuery);
  await until(result).toBeTruthy();

  return result!.value!.me.id === classOnwerId;
};
