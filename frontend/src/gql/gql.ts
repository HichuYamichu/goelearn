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
    "\n  fragment AssignmentsFragment on Class {\n    ownerId\n    ...OwnerAssignmentsFragment\n    ...StudentAssignmentsFragment\n  }\n": types.AssignmentsFragmentFragmentDoc,
    "\n  fragment AssignmentContentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    createdAt\n    files {\n      id\n      name\n    }\n  }\n": types.AssignmentContentFragmentFragmentDoc,
    "\n  mutation CreateAssignmentMutation($input: CreateAssignmentInput!) {\n    createAssignment(input: $input) {\n      id\n    }\n  }\n": types.CreateAssignmentMutationDocument,
    "\n  mutation UpdateAssignmentMutation($input: UpdateAssignmentInput!) {\n    updateAssignment(input: $input)\n  }\n": types.UpdateAssignmentMutationDocument,
    "\n  fragment OwnerAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      dueAt\n      content\n      files {\n        id\n        name\n      }\n      ...AssignmentContentFragment\n      submissions {\n        id\n        createdAt\n        updatedAt\n        user {\n          id\n          username\n        }\n        files {\n          id\n          name\n        }\n      }\n    }\n  }\n": types.OwnerAssignmentsFragmentFragmentDoc,
    "\n  mutation CreateAssignmentSubmissionFeedback(\n    $input: CreateAssignmanetSubmissionFeedbackInput!\n  ) {\n    createAssignmentSubmissionFeedback(input: $input)\n  }\n": types.CreateAssignmentSubmissionFeedbackDocument,
    "\n  mutation DeleteAssignment($classId: ID!, $assignmentId: ID!) {\n    deleteAssignment(classId: $classId, assignmentId: $assignmentId)\n  }\n": types.DeleteAssignmentDocument,
    "\n  query AssignmentSubmissionsQuery($id: ID!) {\n    assignmentSubmissions(assignmentId: $id) {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n": types.AssignmentSubmissionsQueryDocument,
    "\n  fragment StudentAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      ...AssignmentContentFragment\n    }\n  }\n": types.StudentAssignmentsFragmentFragmentDoc,
    "\n  mutation CreateAssignmentSubmission($assignmentId: ID!, $files: [Upload!]!) {\n    createAssignmentSubmission(\n      input: { assignmentId: $assignmentId, files: $files }\n    )\n  }\n": types.CreateAssignmentSubmissionDocument,
    "\n  fragment ChannelsFragment on Channel {\n    id\n    name\n  }\n": types.ChannelsFragmentFragmentDoc,
    "\n  fragment ChatFragment on Class {\n    description\n    channels {\n      id\n      ...ChannelsFragment\n    }\n    members {\n      ...MembersFragment\n    }\n  }\n": types.ChatFragmentFragmentDoc,
    "\n  fragment MembersFragment on User {\n    id\n    username\n  }\n": types.MembersFragmentFragmentDoc,
    "\n  query MessageListMeQuery {\n    me {\n      id\n    }\n  }\n": types.MessageListMeQueryDocument,
    "\n  fragment MessageFragment on Message {\n    id\n    content\n    createdAt\n    author {\n      id\n      username\n    }\n  }\n": types.MessageFragmentFragmentDoc,
    "\n  query MessagesQuery(\n    $classId: ID!\n    $channelId: ID!\n    $before: String\n    $last: Int\n  ) {\n    messages(\n      classId: $classId\n      channelId: $channelId\n      before: $before\n      last: $last\n    ) {\n      nodes {\n        ...MessageFragment\n      }\n      edges {\n        cursor\n        node {\n          ...MessageFragment\n        }\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n": types.MessagesQueryDocument,
    "\n  subscription MessagesSubscription($channelId: ID!) {\n    messageCreated(channelId: $channelId) {\n      ...MessageFragment\n    }\n  }\n": types.MessagesSubscriptionDocument,
    "\n  mutation SendMessage($channelId: ID!, $content: String!) {\n    createMessage(input: { channelId: $channelId, content: $content }) {\n      id\n      content\n    }\n  }\n": types.SendMessageDocument,
    "\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n": types.CreateClassDocument,
    "\n  mutation UpdateClass($classId: ID!, $input: UpdateClassInput!) {\n    updateClass(classId: $classId, classInput: $input)\n  }\n": types.UpdateClassDocument,
    "\n  fragment FilesFragment on Class {\n    id\n    ownerId\n    files {\n      id\n      name\n      fileType\n      parent\n    }\n  }\n": types.FilesFragmentFragmentDoc,
    "\n  mutation CreateDirecotry($classId: ID!, $name: String!, $parentId: ID) {\n    createDirecotry(\n      input: { classId: $classId, name: $name, parentId: $parentId }\n    ) {\n      id\n    }\n  }\n": types.CreateDirecotryDocument,
    "\n  mutation UploadFiles(\n    $classId: ID!\n    $files: [Upload!]!\n    $parentId: ID\n    $public: Boolean!\n  ) {\n    uploadFiles(\n      input: {\n        classId: $classId\n        files: $files\n        parentId: $parentId\n        public: $public\n      }\n    )\n  }\n": types.UploadFilesDocument,
    "\n  mutation DeleteFiles($fileIds: [ID!]!) {\n    deleteFiles(fileIds: $fileIds)\n  }\n": types.DeleteFilesDocument,
    "\n  query MeetingMeQuery {\n    me {\n      id\n    }\n  }\n": types.MeetingMeQueryDocument,
    "\n  fragment MeetingFragment on Class {\n    id\n    ownerId\n  }\n": types.MeetingFragmentFragmentDoc,
    "\n  fragment ClassDataFragment on Class {\n    id\n    name\n    description\n    tags\n    public\n    hasImage\n    channels {\n      id\n      name\n      description\n    }\n    members {\n      id\n      username\n    }\n  }\n": types.ClassDataFragmentFragmentDoc,
    "\n  mutation UpdateChannelMutation($input: UpdateChannelInput!) {\n    updateChannel(input: $input) {\n      id\n    }\n  }\n": types.UpdateChannelMutationDocument,
    "\n  mutation CreateChannelMutation($input: CreateChannelInput!) {\n    createChannel(input: $input) {\n      id\n    }\n  }\n": types.CreateChannelMutationDocument,
    "\n  mutation DeleteChannelMutation($classId: ID!, $channelId: ID!) {\n    deleteChannel(classId: $classId, channelId: $channelId)\n  }\n": types.DeleteChannelMutationDocument,
    "\n  query BannedMemberQuery($classId: ID!) {\n    bannedMembers(classId: $classId) {\n      id\n      username\n    }\n  }\n": types.BannedMemberQueryDocument,
    "\n  mutation BanMemberMutation($classId: ID!, $userId: ID!) {\n    banMember(classId: $classId, userId: $userId)\n  }\n": types.BanMemberMutationDocument,
    "\n  mutation UnbanMemberMutation($classId: ID!, $userId: ID!) {\n    unbanMember(classId: $classId, userId: $userId)\n  }\n": types.UnbanMemberMutationDocument,
    "\n  query AppBarMeQuery {\n    me {\n      id\n      username\n      hasAvatar\n    }\n  }\n": types.AppBarMeQueryDocument,
    "\n  query MyIdQuery {\n    me {\n      id\n    }\n  }\n": types.MyIdQueryDocument,
    "\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      id\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n      ...MeetingFragment\n      ...ClassDataFragment\n    }\n  }\n": types.ClassClassByIdQueryDocument,
    "\n  fragment FileFragment on File {\n    id\n    name\n    fileType\n    parent\n  }\n": types.FileFragmentFragmentDoc,
    "\n  fragment AssignmentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    submissions {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n": types.AssignmentFragmentFragmentDoc,
    "\n  fragment UserFragment on User {\n    id\n    username\n  }\n": types.UserFragmentFragmentDoc,
    "\n  subscription ClassResourceCreateSubscription($classId: ID!) {\n    classResourceCreated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on File {\n        ...FileFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n      ... on User {\n        ...UserFragment\n      }\n    }\n  }\n": types.ClassResourceCreateSubscriptionDocument,
    "\n  subscription ClassResourceUpdateSubscription($classId: ID!) {\n    classResourceUpdated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on Class {\n        ...ClassDataFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n    }\n  }\n": types.ClassResourceUpdateSubscriptionDocument,
    "\n  subscription ClassResourceDeletedSubscription($classId: ID!) {\n    classResourceDeleted(classId: $classId) {\n      __typename\n      ... on ChannelDeleteInfo {\n        id\n      }\n      ... on AssignmentDeleteInfo {\n        id\n      }\n      ... on FileDeleteInfo {\n        id\n      }\n      ... on MemberDeleteInfo {\n        id\n      }\n    }\n  }\n": types.ClassResourceDeletedSubscriptionDocument,
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
export function graphql(source: "\n  fragment AssignmentsFragment on Class {\n    ownerId\n    ...OwnerAssignmentsFragment\n    ...StudentAssignmentsFragment\n  }\n"): (typeof documents)["\n  fragment AssignmentsFragment on Class {\n    ownerId\n    ...OwnerAssignmentsFragment\n    ...StudentAssignmentsFragment\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment AssignmentContentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    createdAt\n    files {\n      id\n      name\n    }\n  }\n"): (typeof documents)["\n  fragment AssignmentContentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    createdAt\n    files {\n      id\n      name\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateAssignmentMutation($input: CreateAssignmentInput!) {\n    createAssignment(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateAssignmentMutation($input: CreateAssignmentInput!) {\n    createAssignment(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UpdateAssignmentMutation($input: UpdateAssignmentInput!) {\n    updateAssignment(input: $input)\n  }\n"): (typeof documents)["\n  mutation UpdateAssignmentMutation($input: UpdateAssignmentInput!) {\n    updateAssignment(input: $input)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment OwnerAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      dueAt\n      content\n      files {\n        id\n        name\n      }\n      ...AssignmentContentFragment\n      submissions {\n        id\n        createdAt\n        updatedAt\n        user {\n          id\n          username\n        }\n        files {\n          id\n          name\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  fragment OwnerAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      dueAt\n      content\n      files {\n        id\n        name\n      }\n      ...AssignmentContentFragment\n      submissions {\n        id\n        createdAt\n        updatedAt\n        user {\n          id\n          username\n        }\n        files {\n          id\n          name\n        }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateAssignmentSubmissionFeedback(\n    $input: CreateAssignmanetSubmissionFeedbackInput!\n  ) {\n    createAssignmentSubmissionFeedback(input: $input)\n  }\n"): (typeof documents)["\n  mutation CreateAssignmentSubmissionFeedback(\n    $input: CreateAssignmanetSubmissionFeedbackInput!\n  ) {\n    createAssignmentSubmissionFeedback(input: $input)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation DeleteAssignment($classId: ID!, $assignmentId: ID!) {\n    deleteAssignment(classId: $classId, assignmentId: $assignmentId)\n  }\n"): (typeof documents)["\n  mutation DeleteAssignment($classId: ID!, $assignmentId: ID!) {\n    deleteAssignment(classId: $classId, assignmentId: $assignmentId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query AssignmentSubmissionsQuery($id: ID!) {\n    assignmentSubmissions(assignmentId: $id) {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  query AssignmentSubmissionsQuery($id: ID!) {\n    assignmentSubmissions(assignmentId: $id) {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment StudentAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      ...AssignmentContentFragment\n    }\n  }\n"): (typeof documents)["\n  fragment StudentAssignmentsFragment on Class {\n    members {\n      id\n      username\n    }\n    assignments {\n      id\n      name\n      ...AssignmentContentFragment\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateAssignmentSubmission($assignmentId: ID!, $files: [Upload!]!) {\n    createAssignmentSubmission(\n      input: { assignmentId: $assignmentId, files: $files }\n    )\n  }\n"): (typeof documents)["\n  mutation CreateAssignmentSubmission($assignmentId: ID!, $files: [Upload!]!) {\n    createAssignmentSubmission(\n      input: { assignmentId: $assignmentId, files: $files }\n    )\n  }\n"];
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
export function graphql(source: "\n  query MessageListMeQuery {\n    me {\n      id\n    }\n  }\n"): (typeof documents)["\n  query MessageListMeQuery {\n    me {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment MessageFragment on Message {\n    id\n    content\n    createdAt\n    author {\n      id\n      username\n    }\n  }\n"): (typeof documents)["\n  fragment MessageFragment on Message {\n    id\n    content\n    createdAt\n    author {\n      id\n      username\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query MessagesQuery(\n    $classId: ID!\n    $channelId: ID!\n    $before: String\n    $last: Int\n  ) {\n    messages(\n      classId: $classId\n      channelId: $channelId\n      before: $before\n      last: $last\n    ) {\n      nodes {\n        ...MessageFragment\n      }\n      edges {\n        cursor\n        node {\n          ...MessageFragment\n        }\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n"): (typeof documents)["\n  query MessagesQuery(\n    $classId: ID!\n    $channelId: ID!\n    $before: String\n    $last: Int\n  ) {\n    messages(\n      classId: $classId\n      channelId: $channelId\n      before: $before\n      last: $last\n    ) {\n      nodes {\n        ...MessageFragment\n      }\n      edges {\n        cursor\n        node {\n          ...MessageFragment\n        }\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n"];
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
export function graphql(source: "\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateClass($input: CreateClassInput!) {\n    createClass(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UpdateClass($classId: ID!, $input: UpdateClassInput!) {\n    updateClass(classId: $classId, classInput: $input)\n  }\n"): (typeof documents)["\n  mutation UpdateClass($classId: ID!, $input: UpdateClassInput!) {\n    updateClass(classId: $classId, classInput: $input)\n  }\n"];
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
export function graphql(source: "\n  mutation DeleteFiles($fileIds: [ID!]!) {\n    deleteFiles(fileIds: $fileIds)\n  }\n"): (typeof documents)["\n  mutation DeleteFiles($fileIds: [ID!]!) {\n    deleteFiles(fileIds: $fileIds)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query MeetingMeQuery {\n    me {\n      id\n    }\n  }\n"): (typeof documents)["\n  query MeetingMeQuery {\n    me {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment MeetingFragment on Class {\n    id\n    ownerId\n  }\n"): (typeof documents)["\n  fragment MeetingFragment on Class {\n    id\n    ownerId\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment ClassDataFragment on Class {\n    id\n    name\n    description\n    tags\n    public\n    hasImage\n    channels {\n      id\n      name\n      description\n    }\n    members {\n      id\n      username\n    }\n  }\n"): (typeof documents)["\n  fragment ClassDataFragment on Class {\n    id\n    name\n    description\n    tags\n    public\n    hasImage\n    channels {\n      id\n      name\n      description\n    }\n    members {\n      id\n      username\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UpdateChannelMutation($input: UpdateChannelInput!) {\n    updateChannel(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation UpdateChannelMutation($input: UpdateChannelInput!) {\n    updateChannel(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateChannelMutation($input: CreateChannelInput!) {\n    createChannel(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateChannelMutation($input: CreateChannelInput!) {\n    createChannel(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation DeleteChannelMutation($classId: ID!, $channelId: ID!) {\n    deleteChannel(classId: $classId, channelId: $channelId)\n  }\n"): (typeof documents)["\n  mutation DeleteChannelMutation($classId: ID!, $channelId: ID!) {\n    deleteChannel(classId: $classId, channelId: $channelId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query BannedMemberQuery($classId: ID!) {\n    bannedMembers(classId: $classId) {\n      id\n      username\n    }\n  }\n"): (typeof documents)["\n  query BannedMemberQuery($classId: ID!) {\n    bannedMembers(classId: $classId) {\n      id\n      username\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation BanMemberMutation($classId: ID!, $userId: ID!) {\n    banMember(classId: $classId, userId: $userId)\n  }\n"): (typeof documents)["\n  mutation BanMemberMutation($classId: ID!, $userId: ID!) {\n    banMember(classId: $classId, userId: $userId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UnbanMemberMutation($classId: ID!, $userId: ID!) {\n    unbanMember(classId: $classId, userId: $userId)\n  }\n"): (typeof documents)["\n  mutation UnbanMemberMutation($classId: ID!, $userId: ID!) {\n    unbanMember(classId: $classId, userId: $userId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query AppBarMeQuery {\n    me {\n      id\n      username\n      hasAvatar\n    }\n  }\n"): (typeof documents)["\n  query AppBarMeQuery {\n    me {\n      id\n      username\n      hasAvatar\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query MyIdQuery {\n    me {\n      id\n    }\n  }\n"): (typeof documents)["\n  query MyIdQuery {\n    me {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      id\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n      ...MeetingFragment\n      ...ClassDataFragment\n    }\n  }\n"): (typeof documents)["\n  query ClassClassByIdQuery($id: ID!) {\n    classById(id: $id) {\n      id\n      name\n      ...ChatFragment\n      ...FilesFragment\n      ...AssignmentsFragment\n      ...MeetingFragment\n      ...ClassDataFragment\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment FileFragment on File {\n    id\n    name\n    fileType\n    parent\n  }\n"): (typeof documents)["\n  fragment FileFragment on File {\n    id\n    name\n    fileType\n    parent\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment AssignmentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    submissions {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  fragment AssignmentFragment on Assignment {\n    id\n    name\n    content\n    dueAt\n    submissions {\n      id\n      createdAt\n      updatedAt\n      user {\n        id\n        username\n      }\n      files {\n        id\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment UserFragment on User {\n    id\n    username\n  }\n"): (typeof documents)["\n  fragment UserFragment on User {\n    id\n    username\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription ClassResourceCreateSubscription($classId: ID!) {\n    classResourceCreated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on File {\n        ...FileFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n      ... on User {\n        ...UserFragment\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription ClassResourceCreateSubscription($classId: ID!) {\n    classResourceCreated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on File {\n        ...FileFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n      ... on User {\n        ...UserFragment\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription ClassResourceUpdateSubscription($classId: ID!) {\n    classResourceUpdated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on Class {\n        ...ClassDataFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription ClassResourceUpdateSubscription($classId: ID!) {\n    classResourceUpdated(classId: $classId) {\n      __typename\n      ... on Channel {\n        ...ChannelsFragment\n      }\n      ... on Class {\n        ...ClassDataFragment\n      }\n      ... on Assignment {\n        ...AssignmentFragment\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription ClassResourceDeletedSubscription($classId: ID!) {\n    classResourceDeleted(classId: $classId) {\n      __typename\n      ... on ChannelDeleteInfo {\n        id\n      }\n      ... on AssignmentDeleteInfo {\n        id\n      }\n      ... on FileDeleteInfo {\n        id\n      }\n      ... on MemberDeleteInfo {\n        id\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription ClassResourceDeletedSubscription($classId: ID!) {\n    classResourceDeleted(classId: $classId) {\n      __typename\n      ... on ChannelDeleteInfo {\n        id\n      }\n      ... on AssignmentDeleteInfo {\n        id\n      }\n      ... on FileDeleteInfo {\n        id\n      }\n      ... on MemberDeleteInfo {\n        id\n      }\n    }\n  }\n"];
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