/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query IsLoggedIn {\n    isLoggedIn @client\n  }\n": types.IsLoggedInDocument,
    "\n  fragment AssignmentsFragment on Class {\n    id\n    ownerId\n    assignments {\n      id\n      name\n      content\n      dueAt\n      createdAt\n      files {\n        id\n        name\n      }\n    }\n  }\n": types.AssignmentsFragmentFragmentDoc,
    "\n  mutation SubmitAssignment($files: [Upload!]!, $assignmentId: ID!) {\n    submitAssignment(input: { files: $files, assignmentId: $assignmentId })\n  }\n": types.SubmitAssignmentDocument,
    "\n  fragment ChannelsFragment on Channel {\n    id\n    name\n  }\n": types.ChannelsFragmentFragmentDoc,
    "\n  fragment ChatFragment on Class {\n    description\n    channels {\n      id\n      ...ChannelsFragment\n    }\n    members {\n      ...MembersFragment\n    }\n  }\n": types.ChatFragmentFragmentDoc,
    "\n  fragment MembersFragment on User {\n    id\n    username\n  }\n": types.MembersFragmentFragmentDoc,
    "\n  fragment MessageFragment on Message {\n    id\n    content\n    author {\n      id\n      username\n    }\n  }\n": types.MessageFragmentFragmentDoc,
    "\n  query MessagesQuery($classId: ID!, $channelId: ID!) {\n    messages(classId: $classId, channelId: $channelId) {\n      nodes {\n        ...MessageFragment\n      }\n    }\n  }\n": types.MessagesQueryDocument,
    "\n  subscription MessagesSubscription($channelId: ID!) {\n    messageCreated(channelId: $channelId) {\n      ...MessageFragment\n    }\n  }\n": types.MessagesSubscriptionDocument,
    "\n  mutation SendMessage($channelId: ID!, $content: String!) {\n    createMessage(input: { channelId: $channelId, content: $content }) {\n      id\n      content\n    }\n  }\n": types.SendMessageDocument,
    "\n  fragment FilesFragment on Class {\n    id\n    ownerId\n    files {\n      id\n      name\n      fileType\n      parent\n    }\n  }\n": types.FilesFragmentFragmentDoc,
    "\n  mutation CreateDirecotry($classId: ID!, $name: String!, $parentId: ID) {\n    createDirecotry(\n      input: { classId: $classId, name: $name, parentId: $parentId }\n    ) {\n      id\n    }\n  }\n": types.CreateDirecotryDocument,
    "\n  mutation UploadFiles(\n    $classId: ID!\n    $files: [Upload!]!\n    $parentId: ID\n    $public: Boolean!\n  ) {\n    uploadFiles(\n      input: {\n        classId: $classId\n        files: $files\n        parentId: $parentId\n        public: $public\n      }\n    )\n  }\n": types.UploadFilesDocument,
    "\n  query AppBarMeQuery {\n    me {\n      id\n      hasAvatar\n    }\n  }\n": types.AppBarMeQueryDocument,
    "\n    query CheckClassOwner {\n      me {\n        id\n      }\n    }\n  ": types.CheckClassOwnerDocument,
    "\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n    }\n  }\n": types.ClassClassByIdQueryDocument,
    "\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n": types.CreateClassDocument,
    "\n  query classesBySearch($query: String!) {\n    classesBySearch(query: $query) {\n      id\n      name\n      description\n      hasImage\n    }\n  }\n": types.ClassesBySearchDocument,
    "\n  mutation JoinClass($classId: ID!) {\n    joinClass(classId: $classId)\n  }\n": types.JoinClassDocument,
    "\n  mutation Login($password: String!, $username: String!) {\n    login(input: { password: $password, username: $username }) {\n      token\n    }\n  }\n": types.LoginDocument,
    "\n  mutation Signup($input: SignupInput!) {\n    signup(input: $input)\n  }\n": types.SignupDocument,
    "\n  query UserClassesMeQuery {\n    me {\n      id\n      clesses {\n        id\n        name\n        description\n        hasImage\n      }\n    }\n  }\n": types.UserClassesMeQueryDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query IsLoggedIn {\n    isLoggedIn @client\n  }\n"): (typeof documents)["\n  query IsLoggedIn {\n    isLoggedIn @client\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment AssignmentsFragment on Class {\n    id\n    ownerId\n    assignments {\n      id\n      name\n      content\n      dueAt\n      createdAt\n      files {\n        id\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  fragment AssignmentsFragment on Class {\n    id\n    ownerId\n    assignments {\n      id\n      name\n      content\n      dueAt\n      createdAt\n      files {\n        id\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SubmitAssignment($files: [Upload!]!, $assignmentId: ID!) {\n    submitAssignment(input: { files: $files, assignmentId: $assignmentId })\n  }\n"): (typeof documents)["\n  mutation SubmitAssignment($files: [Upload!]!, $assignmentId: ID!) {\n    submitAssignment(input: { files: $files, assignmentId: $assignmentId })\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment ChannelsFragment on Channel {\n    id\n    name\n  }\n"): (typeof documents)["\n  fragment ChannelsFragment on Channel {\n    id\n    name\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment ChatFragment on Class {\n    description\n    channels {\n      id\n      ...ChannelsFragment\n    }\n    members {\n      ...MembersFragment\n    }\n  }\n"): (typeof documents)["\n  fragment ChatFragment on Class {\n    description\n    channels {\n      id\n      ...ChannelsFragment\n    }\n    members {\n      ...MembersFragment\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment MembersFragment on User {\n    id\n    username\n  }\n"): (typeof documents)["\n  fragment MembersFragment on User {\n    id\n    username\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment MessageFragment on Message {\n    id\n    content\n    author {\n      id\n      username\n    }\n  }\n"): (typeof documents)["\n  fragment MessageFragment on Message {\n    id\n    content\n    author {\n      id\n      username\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query MessagesQuery($classId: ID!, $channelId: ID!) {\n    messages(classId: $classId, channelId: $channelId) {\n      nodes {\n        ...MessageFragment\n      }\n    }\n  }\n"): (typeof documents)["\n  query MessagesQuery($classId: ID!, $channelId: ID!) {\n    messages(classId: $classId, channelId: $channelId) {\n      nodes {\n        ...MessageFragment\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription MessagesSubscription($channelId: ID!) {\n    messageCreated(channelId: $channelId) {\n      ...MessageFragment\n    }\n  }\n"): (typeof documents)["\n  subscription MessagesSubscription($channelId: ID!) {\n    messageCreated(channelId: $channelId) {\n      ...MessageFragment\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SendMessage($channelId: ID!, $content: String!) {\n    createMessage(input: { channelId: $channelId, content: $content }) {\n      id\n      content\n    }\n  }\n"): (typeof documents)["\n  mutation SendMessage($channelId: ID!, $content: String!) {\n    createMessage(input: { channelId: $channelId, content: $content }) {\n      id\n      content\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment FilesFragment on Class {\n    id\n    ownerId\n    files {\n      id\n      name\n      fileType\n      parent\n    }\n  }\n"): (typeof documents)["\n  fragment FilesFragment on Class {\n    id\n    ownerId\n    files {\n      id\n      name\n      fileType\n      parent\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateDirecotry($classId: ID!, $name: String!, $parentId: ID) {\n    createDirecotry(\n      input: { classId: $classId, name: $name, parentId: $parentId }\n    ) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateDirecotry($classId: ID!, $name: String!, $parentId: ID) {\n    createDirecotry(\n      input: { classId: $classId, name: $name, parentId: $parentId }\n    ) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UploadFiles(\n    $classId: ID!\n    $files: [Upload!]!\n    $parentId: ID\n    $public: Boolean!\n  ) {\n    uploadFiles(\n      input: {\n        classId: $classId\n        files: $files\n        parentId: $parentId\n        public: $public\n      }\n    )\n  }\n"): (typeof documents)["\n  mutation UploadFiles(\n    $classId: ID!\n    $files: [Upload!]!\n    $parentId: ID\n    $public: Boolean!\n  ) {\n    uploadFiles(\n      input: {\n        classId: $classId\n        files: $files\n        parentId: $parentId\n        public: $public\n      }\n    )\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query AppBarMeQuery {\n    me {\n      id\n      hasAvatar\n    }\n  }\n"): (typeof documents)["\n  query AppBarMeQuery {\n    me {\n      id\n      hasAvatar\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n    query CheckClassOwner {\n      me {\n        id\n      }\n    }\n  "): (typeof documents)["\n    query CheckClassOwner {\n      me {\n        id\n      }\n    }\n  "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n    }\n  }\n"): (typeof documents)["\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query classesBySearch($query: String!) {\n    classesBySearch(query: $query) {\n      id\n      name\n      description\n      hasImage\n    }\n  }\n"): (typeof documents)["\n  query classesBySearch($query: String!) {\n    classesBySearch(query: $query) {\n      id\n      name\n      description\n      hasImage\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation JoinClass($classId: ID!) {\n    joinClass(classId: $classId)\n  }\n"): (typeof documents)["\n  mutation JoinClass($classId: ID!) {\n    joinClass(classId: $classId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation Login($password: String!, $username: String!) {\n    login(input: { password: $password, username: $username }) {\n      token\n    }\n  }\n"): (typeof documents)["\n  mutation Login($password: String!, $username: String!) {\n    login(input: { password: $password, username: $username }) {\n      token\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation Signup($input: SignupInput!) {\n    signup(input: $input)\n  }\n"): (typeof documents)["\n  mutation Signup($input: SignupInput!) {\n    signup(input: $input)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query UserClassesMeQuery {\n    me {\n      id\n      clesses {\n        id\n        name\n        description\n        hasImage\n      }\n    }\n  }\n"): (typeof documents)["\n  query UserClassesMeQuery {\n    me {\n      id\n      clesses {\n        id\n        name\n        description\n        hasImage\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;