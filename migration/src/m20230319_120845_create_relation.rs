use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::{
    Assignment, AssignmentFile, AssignmentSubmission, AssignmentSubmissionFeedback,
    AssignmentSubmissionFile, Channel, Class, ClassBlacklist, File, Invite, Membership, Message,
    PasswordResetToken, Report, User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_class_owner_id")
                    .from(Class::Table, Class::OwnerId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_password_reset_user_id")
                    .from(PasswordResetToken::Table, PasswordResetToken::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_classblacklist_user_id")
                    .from(ClassBlacklist::Table, ClassBlacklist::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_classblacklist_class_id")
                    .from(ClassBlacklist::Table, ClassBlacklist::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_channel_class_id")
                    .from(Channel::Table, Channel::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_message_author_id")
                    .from(Message::Table, Message::AuthorId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_message_channel_id")
                    .from(Message::Table, Message::ChannelId)
                    .to(Channel::Table, Channel::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_membership_user_id")
                    .from(Membership::Table, Membership::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_membership_class_id")
                    .from(Membership::Table, Membership::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_invite_class_id")
                    .from(Invite::Table, Invite::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_report_author_id")
                    .from(Report::Table, Report::AuthorId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_file_class_id")
                    .from(File::Table, File::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_file_message_id")
                    .from(File::Table, File::MessageId)
                    .to(Message::Table, Message::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_file_parent_id")
                    .from(File::Table, File::ParentId)
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_class_id")
                    .from(Assignment::Table, Assignment::ClassId)
                    .to(Class::Table, Class::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_file_assignment")
                    .from(AssignmentFile::Table, AssignmentFile::AssignmentId)
                    .to(Assignment::Table, Assignment::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_file_file")
                    .from(AssignmentFile::Table, AssignmentFile::FileId)
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_submission_assignment_id")
                    .from(
                        AssignmentSubmission::Table,
                        AssignmentSubmission::AssignmentId,
                    )
                    .to(Assignment::Table, Assignment::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_channel_id")
                    .from(AssignmentSubmission::Table, AssignmentSubmission::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_submission_file_assignment_submission_id")
                    .from(
                        AssignmentSubmissionFile::Table,
                        AssignmentSubmissionFile::AssignmentSubmissionId,
                    )
                    .to(AssignmentSubmission::Table, AssignmentSubmission::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_submission_file_file_id")
                    .from(
                        AssignmentSubmissionFile::Table,
                        AssignmentSubmissionFile::FileId,
                    )
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_assignment_submission_feedback_assignment_id")
                    .from(
                        AssignmentSubmissionFeedback::Table,
                        AssignmentSubmissionFeedback::AssignmentSubmissionId,
                    )
                    .to(AssignmentSubmission::Table, AssignmentSubmission::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_submission_feedback_assignment_id")
                    .table(AssignmentSubmissionFeedback::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_file_assignment")
                    .table(AssignmentFile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_file_file")
                    .table(AssignmentFile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_submission_file_file_id")
                    .table(AssignmentSubmissionFile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_submission_file_assignment_submission_id")
                    .table(AssignmentSubmissionFile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_channel_id")
                    .table(AssignmentSubmission::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_submission_assignment_id")
                    .table(AssignmentSubmission::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_assignment_class_id")
                    .table(Assignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_class_owner_id")
                    .table(Class::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_channel_class_id")
                    .table(Channel::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_message_author_id")
                    .table(Message::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_message_channel_id")
                    .table(Message::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_membership_user_id")
                    .table(Membership::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_membership_class_id")
                    .table(Membership::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_invite_class_id")
                    .table(Invite::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_report_author_id")
                    .table(Report::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_file_class_id")
                    .table(File::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_file_message_id")
                    .table(File::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_password_reset_user_id")
                    .table(PasswordResetToken::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_file_parent_id")
                    .table(File::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
