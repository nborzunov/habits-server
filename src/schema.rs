// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        currency -> Varchar,
        account_type -> Varchar,
        amount -> Float8,
        created_date -> Timestamptz,
        a_order -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    achievements (id) {
        id -> Uuid,
        user_id -> Uuid,
        a_order -> Int4,
        key -> Varchar,
        achievement_type -> Varchar,
        completed_date -> Nullable<Timestamptz>,
        completed -> Bool,
        created_date -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    categories (id) {
        id -> Uuid,
        user_id -> Uuid,
        category_type -> Varchar,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        is_default -> Bool,
        created_date -> Timestamptz,
        modified_date -> Nullable<Timestamptz>,
        c_order -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    habits (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        periodicity -> Varchar,
        periodicity_value -> Nullable<Array<Nullable<Text>>>,
        created_date -> Timestamptz,
        goal -> Int4,
        goal_type -> Varchar,
        allow_skip -> Bool,
        allow_partial_completion -> Bool,
        allow_over_goal_completion -> Bool,
        can_be_finished -> Bool,
        total_goal -> Int4,
        archived -> Bool,
        deleted -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    habits_achievements (id) {
        id -> Uuid,
        user_id -> Uuid,
        achievement_id -> Uuid,
        habit_id -> Uuid,
        progress -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    targets (id) {
        id -> Uuid,
        habit_id -> Uuid,
        user_id -> Uuid,
        date -> Timestamptz,
        created_date -> Timestamptz,
        target_type -> Varchar,
        value -> Int4,
        deleted -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    transactions (id) {
        id -> Uuid,
        user_id -> Uuid,
        account_id -> Uuid,
        category_id -> Uuid,
        transaction_type -> Varchar,
        note -> Nullable<Text>,
        amount -> Float8,
        created_date -> Timestamptz,
        archived -> Bool,
        deleted -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        username -> Nullable<Varchar>,
        email -> Varchar,
        password_hash -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        bio -> Nullable<Text>,
        image -> Nullable<Varchar>,
        email_verified -> Bool,
        active -> Bool,
        created_date -> Timestamptz,
        updated_date -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(achievements -> users (user_id));
diesel::joinable!(categories -> users (user_id));
diesel::joinable!(habits -> users (user_id));
diesel::joinable!(habits_achievements -> achievements (achievement_id));
diesel::joinable!(habits_achievements -> habits (habit_id));
diesel::joinable!(habits_achievements -> users (user_id));
diesel::joinable!(targets -> habits (habit_id));
diesel::joinable!(targets -> users (user_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> categories (category_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    achievements,
    categories,
    habits,
    habits_achievements,
    targets,
    transactions,
    users,
);
