/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: { input: any; output: any; }
  Upload: { input: any; output: any; }
};

export type Assignment = {
  content: Scalars['String']['output'];
  createdAt: Scalars['NaiveDateTime']['output'];
  dueAt?: Maybe<Scalars['NaiveDateTime']['output']>;
  files: Array<File>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  submissions: Array<AssignmentSubmission>;
};

export type AssignmentDeleteInfo = {
  id: Scalars['ID']['output'];
};

export type AssignmentSubmission = {
  createdAt: Scalars['NaiveDateTime']['output'];
  feedback?: Maybe<AssignmentSubmissionFeedback>;
  files: Array<File>;
  id: Scalars['ID']['output'];
  updatedAt?: Maybe<Scalars['NaiveDateTime']['output']>;
  user: User;
  userId: Scalars['ID']['output'];
};

export type AssignmentSubmissionFeedback = {
  content: Scalars['String']['output'];
  createdAt: Scalars['NaiveDateTime']['output'];
  id: Scalars['ID']['output'];
  updatedAt?: Maybe<Scalars['NaiveDateTime']['output']>;
};

export type Channel = {
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['ID']['output'];
  messages: MessageConnection;
  name: Scalars['String']['output'];
};


export type ChannelMessagesArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type ChannelDeleteInfo = {
  id: Scalars['ID']['output'];
};

export type Class = {
  assignments: Array<Assignment>;
  channels: Array<Channel>;
  description: Scalars['String']['output'];
  files: Array<File>;
  hasImage: Scalars['Boolean']['output'];
  id: Scalars['ID']['output'];
  members: Array<User>;
  name: Scalars['String']['output'];
  owner: User;
  ownerId: Scalars['ID']['output'];
  public: Scalars['Boolean']['output'];
  tags: Array<Scalars['String']['output']>;
};

export type ClassResourceCreate = Assignment | Channel | File | User;

export type ClassResourceDelete = AssignmentDeleteInfo | ChannelDeleteInfo | FileDeleteInfo | MemberDeleteInfo;

export type ClassResourceUpdate = Assignment | Channel | Class;

export type CreateAssignmanetSubmissionFeedbackInput = {
  assignmentSubmissionId: Scalars['ID']['input'];
  feedback: Scalars['String']['input'];
  id?: InputMaybe<Scalars['ID']['input']>;
};

export type CreateAssignmentInput = {
  classId: Scalars['ID']['input'];
  content: Scalars['String']['input'];
  dueAt?: InputMaybe<Scalars['NaiveDateTime']['input']>;
  files: Array<Scalars['Upload']['input']>;
  name: Scalars['String']['input'];
};

export type CreateChannelInput = {
  allowMembersToPost: Scalars['Boolean']['input'];
  classId: Scalars['ID']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
};

export type CreateClassInput = {
  description: Scalars['String']['input'];
  image?: InputMaybe<Scalars['Upload']['input']>;
  name: Scalars['String']['input'];
  public: Scalars['Boolean']['input'];
  tags: Scalars['String']['input'];
};

export type CreateDirectoryInput = {
  classId: Scalars['ID']['input'];
  name: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['ID']['input']>;
};

export type CreateMessageInput = {
  channelId: Scalars['ID']['input'];
  content: Scalars['String']['input'];
};

export type File = {
  fileType: FileType;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  parent?: Maybe<Scalars['ID']['output']>;
  public: Scalars['Boolean']['output'];
};

export type FileDeleteInfo = {
  id: Scalars['ID']['output'];
};

export enum FileType {
  Directory = 'DIRECTORY',
  File = 'FILE'
}

export type LoginInput = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type LoginResult = {
  token: Scalars['String']['output'];
};

export type MemberDeleteInfo = {
  id: Scalars['ID']['output'];
};

export type Message = {
  author: User;
  authorId: Scalars['ID']['output'];
  content: Scalars['String']['output'];
  createdAt: Scalars['NaiveDateTime']['output'];
  id: Scalars['ID']['output'];
};

export type MessageConnection = {
  /** A list of edges. */
  edges: Array<MessageEdge>;
  /** A list of nodes. */
  nodes: Array<Message>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type MessageEdge = {
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: Message;
};

export type Mutation = {
  banMember: Scalars['Boolean']['output'];
  createAssignment: Assignment;
  createAssignmentSubmission: Scalars['Boolean']['output'];
  createAssignmentSubmissionFeedback: Scalars['Boolean']['output'];
  createChannel: Channel;
  createClass: Class;
  createDirecotry: File;
  createMessage: Message;
  deleteAssignment: Scalars['Boolean']['output'];
  deleteAssignmentSubmissionFeedback: Scalars['Boolean']['output'];
  deleteChannel: Scalars['Boolean']['output'];
  deleteClass: Scalars['Boolean']['output'];
  deleteFiles: Scalars['Boolean']['output'];
  joinClass: Scalars['ID']['output'];
  leaveClass: Scalars['Boolean']['output'];
  login: LoginResult;
  signup: Scalars['String']['output'];
  unbanMember: Scalars['Boolean']['output'];
  updateAssignment: Scalars['Boolean']['output'];
  updateAssignmentSubmission: Scalars['Boolean']['output'];
  updateChannel: Channel;
  updateClass: Scalars['Boolean']['output'];
  updateFile: File;
  uploadFiles: Scalars['Boolean']['output'];
};


export type MutationBanMemberArgs = {
  classId: Scalars['ID']['input'];
  userId: Scalars['ID']['input'];
};


export type MutationCreateAssignmentArgs = {
  input: CreateAssignmentInput;
};


export type MutationCreateAssignmentSubmissionArgs = {
  input: SubmitAssignmentInput;
};


export type MutationCreateAssignmentSubmissionFeedbackArgs = {
  input: CreateAssignmanetSubmissionFeedbackInput;
};


export type MutationCreateChannelArgs = {
  input: CreateChannelInput;
};


export type MutationCreateClassArgs = {
  input: CreateClassInput;
};


export type MutationCreateDirecotryArgs = {
  input: CreateDirectoryInput;
};


export type MutationCreateMessageArgs = {
  input: CreateMessageInput;
};


export type MutationDeleteAssignmentArgs = {
  assignmentId: Scalars['ID']['input'];
  classId: Scalars['ID']['input'];
};


export type MutationDeleteAssignmentSubmissionFeedbackArgs = {
  assignmentSubmissionFeedbackId: Scalars['ID']['input'];
};


export type MutationDeleteChannelArgs = {
  channelId: Scalars['ID']['input'];
  classId: Scalars['ID']['input'];
};


export type MutationDeleteClassArgs = {
  classId: Scalars['ID']['input'];
};


export type MutationDeleteFilesArgs = {
  fileIds: Array<Scalars['ID']['input']>;
};


export type MutationJoinClassArgs = {
  classId: Scalars['ID']['input'];
};


export type MutationLeaveClassArgs = {
  classId: Scalars['ID']['input'];
};


export type MutationLoginArgs = {
  input: LoginInput;
};


export type MutationSignupArgs = {
  input: SignupInput;
};


export type MutationUnbanMemberArgs = {
  classId: Scalars['ID']['input'];
  userId: Scalars['ID']['input'];
};


export type MutationUpdateAssignmentArgs = {
  input: UpdateAssignmentInput;
};


export type MutationUpdateAssignmentSubmissionArgs = {
  input: UpdateAssignmentSubmissionInput;
};


export type MutationUpdateChannelArgs = {
  input: UpdateChannelInput;
};


export type MutationUpdateClassArgs = {
  classId: Scalars['ID']['input'];
  classInput: UpdateClassInput;
};


export type MutationUpdateFileArgs = {
  input: UpdateFileInput;
};


export type MutationUploadFilesArgs = {
  input: UploadFileInput;
};

/** Information about pagination in a connection */
export type PageInfo = {
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']['output']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean']['output'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean']['output'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type Query = {
  bannedMembers: Array<User>;
  classById?: Maybe<Class>;
  classesBySearch: Array<Class>;
  errorTesting: Scalars['Boolean']['output'];
  isLoggedIn: Scalars['Boolean']['output'];
  me: User;
  messages: MessageConnection;
  randomClasses: Array<Class>;
};


export type QueryBannedMembersArgs = {
  classId: Scalars['ID']['input'];
};


export type QueryClassByIdArgs = {
  id: Scalars['ID']['input'];
};


export type QueryClassesBySearchArgs = {
  query: Scalars['String']['input'];
};


export type QueryMessagesArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  channelId: Scalars['ID']['input'];
  classId: Scalars['ID']['input'];
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type SignupInput = {
  avatar?: InputMaybe<Scalars['Upload']['input']>;
  email: Scalars['String']['input'];
  firstName: Scalars['String']['input'];
  lastName: Scalars['String']['input'];
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type SubmitAssignmentInput = {
  assignmentId: Scalars['ID']['input'];
  files: Array<Scalars['Upload']['input']>;
};

export type Subscription = {
  classResourceCreated: ClassResourceCreate;
  classResourceDeleted: ClassResourceDelete;
  classResourceUpdated: ClassResourceUpdate;
  messageCreated: Message;
};


export type SubscriptionClassResourceCreatedArgs = {
  classId: Scalars['ID']['input'];
};


export type SubscriptionClassResourceDeletedArgs = {
  classId: Scalars['ID']['input'];
};


export type SubscriptionClassResourceUpdatedArgs = {
  classId: Scalars['ID']['input'];
};


export type SubscriptionMessageCreatedArgs = {
  channelId: Scalars['ID']['input'];
};

export type UpdateAssignmentInput = {
  content?: InputMaybe<Scalars['String']['input']>;
  deleteFiles: Array<Scalars['ID']['input']>;
  dueAt?: InputMaybe<Scalars['NaiveDateTime']['input']>;
  id: Scalars['ID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  newFiles: Array<Scalars['Upload']['input']>;
};

export type UpdateAssignmentSubmissionInput = {
  deleteFiles: Array<Scalars['ID']['input']>;
  files: Array<Scalars['Upload']['input']>;
  id: Scalars['ID']['input'];
};

export type UpdateChannelInput = {
  allowMembersToPost?: InputMaybe<Scalars['Boolean']['input']>;
  classId: Scalars['ID']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['ID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateClassInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  image?: InputMaybe<Scalars['Upload']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
  public?: InputMaybe<Scalars['Boolean']['input']>;
  tags?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateFileInput = {
  id: Scalars['ID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  public?: InputMaybe<Scalars['Boolean']['input']>;
};

export type UploadFileInput = {
  classId: Scalars['ID']['input'];
  files: Array<Scalars['Upload']['input']>;
  parentId?: InputMaybe<Scalars['ID']['input']>;
  public: Scalars['Boolean']['input'];
};

export type User = {
  clesses: Array<Class>;
  email: Scalars['String']['output'];
  firstName: Scalars['String']['output'];
  hasAvatar: Scalars['Boolean']['output'];
  id: Scalars['ID']['output'];
  lastName: Scalars['String']['output'];
  ownedClasses: Array<Class>;
  username: Scalars['String']['output'];
};

export type IsLoggedInQueryVariables = Exact<{ [key: string]: never; }>;


export type IsLoggedInQuery = { isLoggedIn: boolean };

export type AssignmentsFragmentFragment = (
  { ownerId: string }
  & { ' $fragmentRefs'?: { 'OwnerAssignmentsFragmentFragment': OwnerAssignmentsFragmentFragment;'StudentAssignmentsFragmentFragment': StudentAssignmentsFragmentFragment } }
) & { ' $fragmentName'?: 'AssignmentsFragmentFragment' };

export type AssignmentContentFragmentFragment = { id: string, name: string, content: string, dueAt?: any | null, createdAt: any, files: Array<{ id: string, name: string }> } & { ' $fragmentName'?: 'AssignmentContentFragmentFragment' };

export type CreateAssignmentMutationMutationVariables = Exact<{
  input: CreateAssignmentInput;
}>;


export type CreateAssignmentMutationMutation = { createAssignment: { id: string } };

export type OwnerAssignmentsFragmentFragment = { members: Array<{ id: string, username: string }>, assignments: Array<(
    { id: string, name: string, submissions: Array<{ id: string, createdAt: any, updatedAt?: any | null, user: { id: string, username: string }, files: Array<{ id: string, name: string }> }> }
    & { ' $fragmentRefs'?: { 'AssignmentContentFragmentFragment': AssignmentContentFragmentFragment } }
  )> } & { ' $fragmentName'?: 'OwnerAssignmentsFragmentFragment' };

export type CreateAssignmentSubmissionFeedbackMutationVariables = Exact<{
  input: CreateAssignmanetSubmissionFeedbackInput;
}>;


export type CreateAssignmentSubmissionFeedbackMutation = { createAssignmentSubmissionFeedback: boolean };

export type StudentAssignmentsFragmentFragment = { members: Array<{ id: string, username: string }>, assignments: Array<(
    { id: string, name: string }
    & { ' $fragmentRefs'?: { 'AssignmentContentFragmentFragment': AssignmentContentFragmentFragment } }
  )> } & { ' $fragmentName'?: 'StudentAssignmentsFragmentFragment' };

export type CreateAssignmentSubmissionMutationVariables = Exact<{
  assignmentId: Scalars['ID']['input'];
  files: Array<Scalars['Upload']['input']> | Scalars['Upload']['input'];
}>;


export type CreateAssignmentSubmissionMutation = { createAssignmentSubmission: boolean };

export type ChannelsFragmentFragment = { id: string, name: string } & { ' $fragmentName'?: 'ChannelsFragmentFragment' };

export type ChatFragmentFragment = { description: string, channels: Array<(
    { id: string }
    & { ' $fragmentRefs'?: { 'ChannelsFragmentFragment': ChannelsFragmentFragment } }
  )>, members: Array<{ ' $fragmentRefs'?: { 'MembersFragmentFragment': MembersFragmentFragment } }> } & { ' $fragmentName'?: 'ChatFragmentFragment' };

export type MembersFragmentFragment = { id: string, username: string } & { ' $fragmentName'?: 'MembersFragmentFragment' };

export type MessageListMeQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type MessageListMeQueryQuery = { me: { id: string } };

export type MessageFragmentFragment = { id: string, content: string, createdAt: any, author: { id: string, username: string } } & { ' $fragmentName'?: 'MessageFragmentFragment' };

export type MessagesQueryQueryVariables = Exact<{
  classId: Scalars['ID']['input'];
  channelId: Scalars['ID']['input'];
  before?: InputMaybe<Scalars['String']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
}>;


export type MessagesQueryQuery = { messages: { nodes: Array<{ ' $fragmentRefs'?: { 'MessageFragmentFragment': MessageFragmentFragment } }>, edges: Array<{ cursor: string, node: { ' $fragmentRefs'?: { 'MessageFragmentFragment': MessageFragmentFragment } } }>, pageInfo: { hasNextPage: boolean, hasPreviousPage: boolean } } };

export type MessagesSubscriptionSubscriptionVariables = Exact<{
  channelId: Scalars['ID']['input'];
}>;


export type MessagesSubscriptionSubscription = { messageCreated: { ' $fragmentRefs'?: { 'MessageFragmentFragment': MessageFragmentFragment } } };

export type SendMessageMutationVariables = Exact<{
  channelId: Scalars['ID']['input'];
  content: Scalars['String']['input'];
}>;


export type SendMessageMutation = { createMessage: { id: string, content: string } };

export type CreateClassMutationVariables = Exact<{
  input: CreateClassInput;
}>;


export type CreateClassMutation = { createClass: { id: string } };

export type UpdateClassMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  input: UpdateClassInput;
}>;


export type UpdateClassMutation = { updateClass: boolean };

export type FilesFragmentFragment = { id: string, ownerId: string, files: Array<{ id: string, name: string, fileType: FileType, parent?: string | null }> } & { ' $fragmentName'?: 'FilesFragmentFragment' };

export type CreateDirecotryMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  name: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['ID']['input']>;
}>;


export type CreateDirecotryMutation = { createDirecotry: { id: string } };

export type UploadFilesMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  files: Array<Scalars['Upload']['input']> | Scalars['Upload']['input'];
  parentId?: InputMaybe<Scalars['ID']['input']>;
  public: Scalars['Boolean']['input'];
}>;


export type UploadFilesMutation = { uploadFiles: boolean };

export type DeleteFilesMutationVariables = Exact<{
  fileIds: Array<Scalars['ID']['input']> | Scalars['ID']['input'];
}>;


export type DeleteFilesMutation = { deleteFiles: boolean };

export type MeetingMeQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type MeetingMeQueryQuery = { me: { id: string } };

export type MeetingFragmentFragment = { id: string, ownerId: string } & { ' $fragmentName'?: 'MeetingFragmentFragment' };

export type ClassDataFragmentFragment = { id: string, name: string, description: string, tags: Array<string>, public: boolean, hasImage: boolean, channels: Array<{ id: string, name: string, description?: string | null }>, members: Array<{ id: string, username: string }> } & { ' $fragmentName'?: 'ClassDataFragmentFragment' };

export type UpdateChannelMutationMutationVariables = Exact<{
  input: UpdateChannelInput;
}>;


export type UpdateChannelMutationMutation = { updateChannel: { id: string } };

export type CreateChannelMutationMutationVariables = Exact<{
  input: CreateChannelInput;
}>;


export type CreateChannelMutationMutation = { createChannel: { id: string } };

export type DeleteChannelMutationMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  channelId: Scalars['ID']['input'];
}>;


export type DeleteChannelMutationMutation = { deleteChannel: boolean };

export type BannedMemberQueryQueryVariables = Exact<{
  classId: Scalars['ID']['input'];
}>;


export type BannedMemberQueryQuery = { bannedMembers: Array<{ id: string, username: string }> };

export type BanMemberMutationMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  userId: Scalars['ID']['input'];
}>;


export type BanMemberMutationMutation = { banMember: boolean };

export type UnbanMemberMutationMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
  userId: Scalars['ID']['input'];
}>;


export type UnbanMemberMutationMutation = { unbanMember: boolean };

export type AppBarMeQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type AppBarMeQueryQuery = { me: { id: string, username: string, hasAvatar: boolean } };

export type MyIdQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type MyIdQueryQuery = { me: { id: string } };

export type ClassClassByIdQueryQueryVariables = Exact<{
  id: Scalars['ID']['input'];
}>;


export type ClassClassByIdQueryQuery = { classById?: (
    { id: string, name: string }
    & { ' $fragmentRefs'?: { 'ChatFragmentFragment': ChatFragmentFragment;'FilesFragmentFragment': FilesFragmentFragment;'AssignmentsFragmentFragment': AssignmentsFragmentFragment;'MeetingFragmentFragment': MeetingFragmentFragment;'ClassDataFragmentFragment': ClassDataFragmentFragment } }
  ) | null };

export type FileFragmentFragment = { id: string, name: string, fileType: FileType, parent?: string | null } & { ' $fragmentName'?: 'FileFragmentFragment' };

export type AssignmentFragmentFragment = { submissions: Array<{ id: string, createdAt: any, updatedAt?: any | null, user: { id: string, username: string }, files: Array<{ id: string, name: string }> }> } & { ' $fragmentName'?: 'AssignmentFragmentFragment' };

export type UserFragmentFragment = { id: string, username: string } & { ' $fragmentName'?: 'UserFragmentFragment' };

export type ClassResourceCreateSubscriptionSubscriptionVariables = Exact<{
  classId: Scalars['ID']['input'];
}>;


export type ClassResourceCreateSubscriptionSubscription = { classResourceCreated: (
    { __typename: 'Assignment' }
    & { ' $fragmentRefs'?: { 'AssignmentFragmentFragment': AssignmentFragmentFragment } }
  ) | (
    { __typename: 'Channel' }
    & { ' $fragmentRefs'?: { 'ChannelsFragmentFragment': ChannelsFragmentFragment } }
  ) | (
    { __typename: 'File' }
    & { ' $fragmentRefs'?: { 'FileFragmentFragment': FileFragmentFragment } }
  ) | (
    { __typename: 'User' }
    & { ' $fragmentRefs'?: { 'UserFragmentFragment': UserFragmentFragment } }
  ) };

export type ClassResourceUpdateSubscriptionSubscriptionVariables = Exact<{
  classId: Scalars['ID']['input'];
}>;


export type ClassResourceUpdateSubscriptionSubscription = { classResourceUpdated: { __typename: 'Assignment' } | (
    { __typename: 'Channel' }
    & { ' $fragmentRefs'?: { 'ChannelsFragmentFragment': ChannelsFragmentFragment } }
  ) | (
    { __typename: 'Class' }
    & { ' $fragmentRefs'?: { 'ClassDataFragmentFragment': ClassDataFragmentFragment } }
  ) };

export type ClassResourceDeletedSubscriptionSubscriptionVariables = Exact<{
  classId: Scalars['ID']['input'];
}>;


export type ClassResourceDeletedSubscriptionSubscription = { classResourceDeleted: { __typename: 'AssignmentDeleteInfo', id: string } | { __typename: 'ChannelDeleteInfo', id: string } | { __typename: 'FileDeleteInfo', id: string } | { __typename: 'MemberDeleteInfo', id: string } };

export type ClassesBySearchQueryVariables = Exact<{
  query: Scalars['String']['input'];
}>;


export type ClassesBySearchQuery = { classesBySearch: Array<{ id: string, name: string, description: string, hasImage: boolean }> };

export type JoinClassMutationVariables = Exact<{
  classId: Scalars['ID']['input'];
}>;


export type JoinClassMutation = { joinClass: string };

export type LoginMutationVariables = Exact<{
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
}>;


export type LoginMutation = { login: { token: string } };

export type SignupMutationVariables = Exact<{
  input: SignupInput;
}>;


export type SignupMutation = { signup: string };

export type UserClassesMeQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type UserClassesMeQueryQuery = { me: { id: string, clesses: Array<{ id: string, name: string, description: string, hasImage: boolean }> } };

export const AssignmentContentFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentContentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"dueAt"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<AssignmentContentFragmentFragment, unknown>;
export const OwnerAssignmentsFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"OwnerAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}},{"kind":"Field","name":{"kind":"Name","value":"submissions"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentContentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"dueAt"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<OwnerAssignmentsFragmentFragment, unknown>;
export const StudentAssignmentsFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"StudentAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentContentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"dueAt"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<StudentAssignmentsFragmentFragment, unknown>;
export const AssignmentsFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"ownerId"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"OwnerAssignmentsFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"StudentAssignmentsFragment"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentContentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"dueAt"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"OwnerAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}},{"kind":"Field","name":{"kind":"Name","value":"submissions"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"StudentAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}}]}}]}}]} as unknown as DocumentNode<AssignmentsFragmentFragment, unknown>;
export const ChannelsFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChannelsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]} as unknown as DocumentNode<ChannelsFragmentFragment, unknown>;
export const MembersFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MembersFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]} as unknown as DocumentNode<MembersFragmentFragment, unknown>;
export const ChatFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChatFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"channels"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"ChannelsFragment"}}]}},{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"MembersFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChannelsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MembersFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]} as unknown as DocumentNode<ChatFragmentFragment, unknown>;
export const MessageFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MessageFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Message"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"author"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<MessageFragmentFragment, unknown>;
export const FilesFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"FilesFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"ownerId"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"fileType"}},{"kind":"Field","name":{"kind":"Name","value":"parent"}}]}}]}}]} as unknown as DocumentNode<FilesFragmentFragment, unknown>;
export const MeetingFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MeetingFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"ownerId"}}]}}]} as unknown as DocumentNode<MeetingFragmentFragment, unknown>;
export const ClassDataFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ClassDataFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"tags"}},{"kind":"Field","name":{"kind":"Name","value":"public"}},{"kind":"Field","name":{"kind":"Name","value":"hasImage"}},{"kind":"Field","name":{"kind":"Name","value":"channels"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}},{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<ClassDataFragmentFragment, unknown>;
export const FileFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"FileFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"File"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"fileType"}},{"kind":"Field","name":{"kind":"Name","value":"parent"}}]}}]} as unknown as DocumentNode<FileFragmentFragment, unknown>;
export const AssignmentFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"submissions"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<AssignmentFragmentFragment, unknown>;
export const UserFragmentFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"UserFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]} as unknown as DocumentNode<UserFragmentFragment, unknown>;
export const IsLoggedInDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"IsLoggedIn"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"isLoggedIn"},"directives":[{"kind":"Directive","name":{"kind":"Name","value":"client"}}]}]}}]} as unknown as DocumentNode<IsLoggedInQuery, IsLoggedInQueryVariables>;
export const CreateAssignmentMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateAssignmentMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"CreateAssignmentInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createAssignment"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<CreateAssignmentMutationMutation, CreateAssignmentMutationMutationVariables>;
export const CreateAssignmentSubmissionFeedbackDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateAssignmentSubmissionFeedback"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"CreateAssignmanetSubmissionFeedbackInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createAssignmentSubmissionFeedback"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}]}]}}]} as unknown as DocumentNode<CreateAssignmentSubmissionFeedbackMutation, CreateAssignmentSubmissionFeedbackMutationVariables>;
export const CreateAssignmentSubmissionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateAssignmentSubmission"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"assignmentId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"files"}},"type":{"kind":"NonNullType","type":{"kind":"ListType","type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Upload"}}}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createAssignmentSubmission"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"assignmentId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"assignmentId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"files"},"value":{"kind":"Variable","name":{"kind":"Name","value":"files"}}}]}}]}]}}]} as unknown as DocumentNode<CreateAssignmentSubmissionMutation, CreateAssignmentSubmissionMutationVariables>;
export const MessageListMeQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"MessageListMeQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<MessageListMeQueryQuery, MessageListMeQueryQueryVariables>;
export const MessagesQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"MessagesQuery"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"before"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"last"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"messages"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"Argument","name":{"kind":"Name","value":"channelId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}}},{"kind":"Argument","name":{"kind":"Name","value":"before"},"value":{"kind":"Variable","name":{"kind":"Name","value":"before"}}},{"kind":"Argument","name":{"kind":"Name","value":"last"},"value":{"kind":"Variable","name":{"kind":"Name","value":"last"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nodes"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"MessageFragment"}}]}},{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"cursor"}},{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"MessageFragment"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"pageInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"hasNextPage"}},{"kind":"Field","name":{"kind":"Name","value":"hasPreviousPage"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MessageFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Message"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"author"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<MessagesQueryQuery, MessagesQueryQueryVariables>;
export const MessagesSubscriptionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"MessagesSubscription"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"messageCreated"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"channelId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"MessageFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MessageFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Message"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"author"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<MessagesSubscriptionSubscription, MessagesSubscriptionSubscriptionVariables>;
export const SendMessageDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SendMessage"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"content"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createMessage"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"channelId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"content"},"value":{"kind":"Variable","name":{"kind":"Name","value":"content"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"content"}}]}}]}}]} as unknown as DocumentNode<SendMessageMutation, SendMessageMutationVariables>;
export const CreateClassDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateClass"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"CreateClassInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createClass"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<CreateClassMutation, CreateClassMutationVariables>;
export const UpdateClassDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateClass"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UpdateClassInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateClass"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"Argument","name":{"kind":"Name","value":"classInput"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}]}]}}]} as unknown as DocumentNode<UpdateClassMutation, UpdateClassMutationVariables>;
export const CreateDirecotryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateDirecotry"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createDirecotry"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"parentId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<CreateDirecotryMutation, CreateDirecotryMutationVariables>;
export const UploadFilesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UploadFiles"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"files"}},"type":{"kind":"NonNullType","type":{"kind":"ListType","type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Upload"}}}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"public"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Boolean"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"uploadFiles"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"files"},"value":{"kind":"Variable","name":{"kind":"Name","value":"files"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"parentId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"public"},"value":{"kind":"Variable","name":{"kind":"Name","value":"public"}}}]}}]}]}}]} as unknown as DocumentNode<UploadFilesMutation, UploadFilesMutationVariables>;
export const DeleteFilesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DeleteFiles"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"fileIds"}},"type":{"kind":"NonNullType","type":{"kind":"ListType","type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteFiles"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"fileIds"},"value":{"kind":"Variable","name":{"kind":"Name","value":"fileIds"}}}]}]}}]} as unknown as DocumentNode<DeleteFilesMutation, DeleteFilesMutationVariables>;
export const MeetingMeQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"MeetingMeQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<MeetingMeQueryQuery, MeetingMeQueryQueryVariables>;
export const UpdateChannelMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateChannelMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UpdateChannelInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateChannel"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<UpdateChannelMutationMutation, UpdateChannelMutationMutationVariables>;
export const CreateChannelMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateChannelMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"CreateChannelInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createChannel"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<CreateChannelMutationMutation, CreateChannelMutationMutationVariables>;
export const DeleteChannelMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DeleteChannelMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteChannel"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"Argument","name":{"kind":"Name","value":"channelId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"channelId"}}}]}]}}]} as unknown as DocumentNode<DeleteChannelMutationMutation, DeleteChannelMutationMutationVariables>;
export const BannedMemberQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"BannedMemberQuery"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"bannedMembers"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<BannedMemberQueryQuery, BannedMemberQueryQueryVariables>;
export const BanMemberMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"BanMemberMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"banMember"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"Argument","name":{"kind":"Name","value":"userId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userId"}}}]}]}}]} as unknown as DocumentNode<BanMemberMutationMutation, BanMemberMutationMutationVariables>;
export const UnbanMemberMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UnbanMemberMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"unbanMember"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}},{"kind":"Argument","name":{"kind":"Name","value":"userId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userId"}}}]}]}}]} as unknown as DocumentNode<UnbanMemberMutationMutation, UnbanMemberMutationMutationVariables>;
export const AppBarMeQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"AppBarMeQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}},{"kind":"Field","name":{"kind":"Name","value":"hasAvatar"}}]}}]}}]} as unknown as DocumentNode<AppBarMeQueryQuery, AppBarMeQueryQueryVariables>;
export const MyIdQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"MyIdQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]} as unknown as DocumentNode<MyIdQueryQuery, MyIdQueryQueryVariables>;
export const ClassClassByIdQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ClassClassByIdQuery"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classById"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"ChatFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"FilesFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentsFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"MeetingFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"ClassDataFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChannelsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MembersFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentContentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"dueAt"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"OwnerAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}},{"kind":"Field","name":{"kind":"Name","value":"submissions"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"StudentAssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"assignments"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentContentFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChatFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"channels"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"ChannelsFragment"}}]}},{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"MembersFragment"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"FilesFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"ownerId"}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"fileType"}},{"kind":"Field","name":{"kind":"Name","value":"parent"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"ownerId"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"OwnerAssignmentsFragment"}},{"kind":"FragmentSpread","name":{"kind":"Name","value":"StudentAssignmentsFragment"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"MeetingFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"ownerId"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ClassDataFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"tags"}},{"kind":"Field","name":{"kind":"Name","value":"public"}},{"kind":"Field","name":{"kind":"Name","value":"hasImage"}},{"kind":"Field","name":{"kind":"Name","value":"channels"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}},{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<ClassClassByIdQueryQuery, ClassClassByIdQueryQueryVariables>;
export const ClassResourceCreateSubscriptionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"ClassResourceCreateSubscription"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classResourceCreated"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"ChannelsFragment"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"File"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"FileFragment"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"AssignmentFragment"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"UserFragment"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChannelsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"FileFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"File"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"fileType"}},{"kind":"Field","name":{"kind":"Name","value":"parent"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"AssignmentFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Assignment"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"submissions"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}},{"kind":"Field","name":{"kind":"Name","value":"files"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"UserFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"User"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]} as unknown as DocumentNode<ClassResourceCreateSubscriptionSubscription, ClassResourceCreateSubscriptionSubscriptionVariables>;
export const ClassResourceUpdateSubscriptionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"ClassResourceUpdateSubscription"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classResourceUpdated"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"ChannelsFragment"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"ClassDataFragment"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ChannelsFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Channel"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"ClassDataFragment"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"Class"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"tags"}},{"kind":"Field","name":{"kind":"Name","value":"public"}},{"kind":"Field","name":{"kind":"Name","value":"hasImage"}},{"kind":"Field","name":{"kind":"Name","value":"channels"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}},{"kind":"Field","name":{"kind":"Name","value":"members"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}}]} as unknown as DocumentNode<ClassResourceUpdateSubscriptionSubscription, ClassResourceUpdateSubscriptionSubscriptionVariables>;
export const ClassResourceDeletedSubscriptionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"ClassResourceDeletedSubscription"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classResourceDeleted"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"ChannelDeleteInfo"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"AssignmentDeleteInfo"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"FileDeleteInfo"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}},{"kind":"InlineFragment","typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"MemberDeleteInfo"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}}]}}]}}]}}]} as unknown as DocumentNode<ClassResourceDeletedSubscriptionSubscription, ClassResourceDeletedSubscriptionSubscriptionVariables>;
export const ClassesBySearchDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"classesBySearch"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"query"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classesBySearch"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"query"},"value":{"kind":"Variable","name":{"kind":"Name","value":"query"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"hasImage"}}]}}]}}]} as unknown as DocumentNode<ClassesBySearchQuery, ClassesBySearchQueryVariables>;
export const JoinClassDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"JoinClass"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"classId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"joinClass"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"classId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"classId"}}}]}]}}]} as unknown as DocumentNode<JoinClassMutation, JoinClassMutationVariables>;
export const LoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"Login"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"password"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"username"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"password"},"value":{"kind":"Variable","name":{"kind":"Name","value":"password"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"username"},"value":{"kind":"Variable","name":{"kind":"Name","value":"username"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}}]}}]}}]} as unknown as DocumentNode<LoginMutation, LoginMutationVariables>;
export const SignupDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"Signup"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"SignupInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"signup"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}]}]}}]} as unknown as DocumentNode<SignupMutation, SignupMutationVariables>;
export const UserClassesMeQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"UserClassesMeQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"clesses"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"hasImage"}}]}}]}}]}}]} as unknown as DocumentNode<UserClassesMeQueryQuery, UserClassesMeQueryQueryVariables>;