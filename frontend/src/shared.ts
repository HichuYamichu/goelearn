import { useQuery } from "@vue/apollo-composable";
import { graphql } from "./gql";
import { until } from "@vueuse/core";

export const MyIdQuery = graphql(/* GraphQL */ `
  query MyIdQuery {
    me {
      id
    }
  }
`);
// export const isClassOwner = async (classOnwerId: string) => {

//   const { result } = useQuery(MeQuery);
//   await until(result).toBeTruthy();

//   return result!.value!.me.id === classOnwerId;
// };
