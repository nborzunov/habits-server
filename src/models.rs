﻿// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
#[derive(Queryable, Debug)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub currency: String,
    pub account_type: String,
    pub amount: f64,
    pub created_date: DateTime<Utc>,
}

#[derive(Queryable, Debug)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub a_order: i32,
    pub key: String,
    pub achievement_type: String,
    pub completed_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_date: DateTime<Utc>,
}

#[derive(Queryable, Debug)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_type: String,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub is_default: bool,
    pub created_date: DateTime<Utc>,
    pub modified_date: Option<DateTime<Utc>>,
}

#[derive(Queryable, Debug)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub periodicity: String,
    pub periodicity_value: Option<Vec<Option<String>>>,
    pub created_date: DateTime<Utc>,
    pub goal: i32,
    pub goal_type: String,
    pub allow_skip: bool,
    pub allow_partial_completion: bool,
    pub allow_over_goal_completion: bool,
    pub can_be_finished: bool,
    pub total_goal: i32,
    pub archived: bool,
    pub deleted: bool,
}

#[derive(Queryable, Debug)]
pub struct HabitsAchievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement_id: Uuid,
    pub habit_id: Uuid,
    pub progress: i32,
}

#[derive(Queryable, Debug)]
pub struct Target {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    pub target_type: String,
    pub value: i32,
    pub deleted: bool,
}

#[derive(Queryable, Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: String,
    pub note: Option<String>,
    pub amount: f64,
    pub created_date: DateTime<Utc>,
    pub archived: bool,
    pub deleted: bool,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub surname: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

