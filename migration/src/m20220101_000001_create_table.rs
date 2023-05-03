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
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::HasAvatar).boolean().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
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
                    .table(Class::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Class::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(Class::Name).string().not_null())
                    .col(ColumnDef::new(Class::Description).string().not_null())
                    .col(ColumnDef::new(Class::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Class::Public).boolean().not_null())
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
                    .col(ColumnDef::new(Channel::ClassId).uuid().not_null())
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
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
    Public,
    OwnerId,
}

#[derive(Iden)]
pub enum Channel {
    Table,
    Id,
    Name,
    Description,
    ClassId,
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
