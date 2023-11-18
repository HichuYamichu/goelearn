import { useQuery } from "@vue/apollo-composable";
import { graphql } from "./gql";
import { until } from "@vueuse/core";
import { useSubscription } from "@vue/apollo-composable";
import { cache } from "@/client";
import router from "@/router";

export const MyIdQuery = graphql(/* GraphQL */ `
  query MyIdQuery {
    me {
      id
      userType
    }
  }
`);

export const download = async (classId: string, item: any) => {
  downloadFile(
    `${import.meta.env.VITE_BASE_ENDPOINT}/files/class-files/${classId}/${
      item.id
    }`,
    item.name
  );
};

export const downloadFile = async (url: string, filename: string) => {
  const data = await fetch(url, {
    headers: {
      Authorization: `Bearer ${localStorage.getItem("token")} `,
    },
  });
  const blob = await data.blob();
  const objectUrl = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.setAttribute("href", objectUrl);
  link.setAttribute("download", filename);
  link.style.display = "none";
  link.click();
};

export async function validate(inputs: any[]) {
  const res = await Promise.all(
    inputs.map((input) => {
      return input.value!.validate();
    })
  );
  return res.every((r) => r.length === 0);
}

export const toLocaleString = (date?: string | null) => {
  if (!date) return "";
  return new Date(date).toLocaleString();
};

const ClassDeletedSubscription = graphql(/* GraphQL */ `
  subscription ClassDeletedSubscription($classId: ID!) {
    classDeleted(classId: $classId) {
      id
    }
  }
`);

export const useClassDeleted = (classId: string) => {
  const { onResult: onClassDeleted } = useSubscription(
    ClassDeletedSubscription,
    { classId }
  );

  onClassDeleted((result) => {
    const receivedId = result.data?.classDeleted.id;
    if (router.currentRoute.value.params.classId === receivedId) {
      router.push({ name: "My Classes" });
      cache.evict({ id: `Class:${classId}` });
    }
  });
};
