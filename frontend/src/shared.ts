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

export const download = async (classId: string, item: any) => {
  downloadFile(
    `http://localhost:3000/files/class-files/${classId}/${item.id}`,
    item.name
  );
};

export const downloadFile = async (url: string, filename: string) => {
  const data = await fetch(url);
  const blob = await data.blob();
  const objectUrl = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.setAttribute("href", objectUrl);
  link.setAttribute("download", filename);
  link.style.display = "none";
  link.click();
};
