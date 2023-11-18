use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserType::Type)
                    .values([UserType::Regular, UserType::Mod, UserType::Admin])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(FileType::Enum)
                    .values([FileType::File, FileType::Directory])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).not_null().uuid().primary_key())
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::HasAvatar).boolean().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp().null())
                    .col(ColumnDef::new(User::Active).boolean().not_null())
                    .col(
                        ColumnDef::new(User::UserType)
                            .custom(UserType::Type)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PasswordResetToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordResetToken::Id)
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PasswordResetToken::UserId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Class::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(Class::Name).string().not_null())
                    .col(ColumnDef::new(Class::Description).string().not_null())
                    .col(ColumnDef::new(Class::Tags).string().not_null())
                    .col(ColumnDef::new(Class::HasImage).boolean().not_null())
                    .col(ColumnDef::new(Class::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Class::Public).boolean().not_null())
                    .col(ColumnDef::new(Class::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ClassBlacklist::Table)
                    .if_not_exists()
                    .primary_key(
                        index::Index::create()
                            .col(ClassBlacklist::UserId)
                            .col(ClassBlacklist::ClassId),
                    )
                    .col(ColumnDef::new(ClassBlacklist::UserId).uuid().not_null())
                    .col(ColumnDef::new(ClassBlacklist::ClassId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Channel::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(Channel::Name).string().not_null())
                    .col(ColumnDef::new(Channel::Description).string().null())
                    .col(
                        ColumnDef::new(Channel::AllowMembersToPost)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Channel::ClassId).uuid().not_null())
                    .col(ColumnDef::new(Channel::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Message::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(ColumnDef::new(Message::ChannelId).uuid().not_null())
                    .col(ColumnDef::new(Message::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(Message::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Membership::Table)
                    .if_not_exists()
                    .primary_key(
                        index::Index::create()
                            .col(Membership::UserId)
                            .col(Membership::ClassId),
                    )
                    .col(ColumnDef::new(Membership::UserId).uuid().not_null())
                    .col(ColumnDef::new(Membership::ClassId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Invite::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invite::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Invite::ExpiresAt).timestamp().null())
                    .col(ColumnDef::new(Invite::ClassId).uuid().not_null())
                    .col(ColumnDef::new(Invite::Multiuse).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Report::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Report::Content).string().not_null())
                    .col(ColumnDef::new(Report::AuthorId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(ColumnDef::new(File::Public).boolean().not_null())
                    .col(
                        ColumnDef::new(File::FileType)
                            .enumeration(FileType::Enum, [FileType::Directory, FileType::File])
                            .not_null(),
                    )
                    .col(ColumnDef::new(File::ParentId).uuid().null())
                    .col(ColumnDef::new(File::ClassId).uuid().not_null())
                    .col(ColumnDef::new(File::MessageId).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Assignment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Assignment::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Assignment::Name).string().not_null())
                    .col(ColumnDef::new(Assignment::Content).string().not_null())
                    .col(ColumnDef::new(Assignment::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Assignment::DueAt).timestamp().null())
                    .col(ColumnDef::new(Assignment::ClassId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssignmentFile::Table)
                    .if_not_exists()
                    .primary_key(
                        index::Index::create()
                            .col(AssignmentFile::AssignmentId)
                            .col(AssignmentFile::FileId),
                    )
                    .col(
                        ColumnDef::new(AssignmentFile::AssignmentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssignmentFile::FileId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssignmentSubmission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssignmentSubmission::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmission::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmission::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmission::AssignmentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmission::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssignmentSubmissionFile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssignmentSubmissionFile::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFile::AssignmentSubmissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFile::FileId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssignmentSubmissionFeedback::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssignmentSubmissionFeedback::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFeedback::AssignmentSubmissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFeedback::Feedback)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFeedback::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssignmentSubmissionFeedback::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AssignmentSubmissionFeedback::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AssignmentSubmissionFile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AssignmentSubmission::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AssignmentFile::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Assignment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PasswordResetToken::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Membership::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Invite::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(UserType::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(FileType::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    FirstName,
    LastName,
    HasAvatar,
    Email,
    Password,
    Active,
    UserType,
    CreatedAt,
    DeletedAt,
}

pub enum UserType {
    Type,
    Regular,
    Mod,
    Admin,
}

#[derive(Iden)]
pub enum PasswordResetToken {
    Table,
    Id,
    UserId,
}

impl Iden for UserType {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "user_type",
                Self::Admin => "Admin",
                Self::Mod => "Mod",
                Self::Regular => "Regular",
            }
        )
        .unwrap();
    }
}

#[derive(Iden)]
pub enum Class {
    Table,
    Id,
    Name,
    Description,
    Tags,
    HasImage,
    Public,
    OwnerId,
    DeletedAt,
}

#[derive(Iden)]
pub enum ClassBlacklist {
    Table,
    UserId,
    ClassId,
}

#[derive(Iden)]
pub enum Channel {
    Table,
    Id,
    Name,
    Description,
    AllowMembersToPost,
    ClassId,
    DeletedAt,
}

#[derive(Iden)]
pub enum Message {
    Table,
    Id,
    Content,
    ChannelId,
    AuthorId,
    CreatedAt,
}

#[derive(Iden)]
pub enum Membership {
    Table,
    UserId,
    ClassId,
}

#[derive(Iden)]
pub enum Invite {
    Table,
    Id,
    ClassId,
    ExpiresAt,
    Multiuse,
}

#[derive(Iden)]
pub enum Report {
    Table,
    Id,
    Content,
    AuthorId,
}

#[derive(Iden)]
pub enum File {
    Table,
    Id,
    Name,
    Public,
    FileType,
    ParentId,
    ClassId,
    MessageId,
}

#[derive(Iden)]
pub enum Assignment {
    Table,
    Id,
    Name,
    Content,
    CreatedAt,
    DueAt,
    ClassId,
}

#[derive(Iden)]
pub enum AssignmentFile {
    Table,
    AssignmentId,
    FileId,
}

#[derive(Iden)]
pub enum AssignmentSubmission {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AssignmentId,
    UserId,
}

#[derive(Iden)]
pub enum AssignmentSubmissionFile {
    Table,
    Id,
    AssignmentSubmissionId,
    FileId,
}

#[derive(Iden)]
pub enum AssignmentSubmissionFeedback {
    Table,
    Id,
    AssignmentSubmissionId,
    Feedback,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum FileType {
    #[iden = "file_type"]
    Enum,
    #[iden = "Directory"]
    Directory,
    #[iden = "File"]
    File,
}
