// @generated automatically by Diesel CLI.

diesel::table! {
    access_logs (id) {
        id -> Uuid,
        document_id -> Uuid,
        user_id -> Nullable<Uuid>,
        action -> Varchar,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_permissions (id) {
        id -> Uuid,
        document_id -> Uuid,
        user_id -> Uuid,
        permission -> Text,
        granted_by -> Uuid,
        granted_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    document_versions (id) {
        id -> Uuid,
        document_id -> Uuid,
        version -> Int4,
        file_path -> Varchar,
        file_size -> Int8,
        comment -> Nullable<Text>,
        created_by -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        file_path -> Varchar,
        file_size -> Int8,
        mime_type -> Varchar,
        version -> Int4,
        status -> Varchar,
        owner_id -> Uuid,
        parent_folder_id -> Nullable<Uuid>,
        is_folder -> Bool,
        tags -> Nullable<Array<Nullable<Text>>>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    group_members (id) {
        id -> Uuid,
        group_id -> Uuid,
        user_id -> Uuid,
        role -> Varchar,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    group_permissions (id) {
        id -> Uuid,
        document_id -> Uuid,
        group_id -> Uuid,
        permission -> Text,
        granted_by -> Uuid,
        granted_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    share_links (id) {
        id -> Uuid,
        document_id -> Uuid,
        token -> Varchar,
        created_by -> Uuid,
        permission -> Text,
        password_hash -> Nullable<Varchar>,
        max_access_count -> Nullable<Int4>,
        access_count -> Int4,
        expires_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        role -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(access_logs -> documents (document_id));
diesel::joinable!(document_permissions -> documents (document_id));
diesel::joinable!(document_permissions -> users (user_id));
diesel::joinable!(document_versions -> documents (document_id));
diesel::joinable!(group_members -> groups (group_id));
diesel::joinable!(group_members -> users (user_id));
diesel::joinable!(group_permissions -> documents (document_id));
diesel::joinable!(group_permissions -> groups (group_id));
diesel::joinable!(share_links -> documents (document_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_logs,
    document_permissions,
    document_versions,
    documents,
    group_members,
    group_permissions,
    groups,
    share_links,
    users,
);

