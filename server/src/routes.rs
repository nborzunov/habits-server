use actix_web::{web, HttpResponse};
use chrono::{DateTime, TimeZone, Utc};
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use serde_json::Result as SerdeResult;
use std::fmt;
use std::sync::Mutex;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/habits")
            .app_data(web::Data::new(AppState {
                habits: Habits::new(),
            }))
            .route(web::get().to(get_habits))
            .route(web::post().to(add_habit)),
    );
}

async fn get_habits(state: web::Data<AppState>) -> SerdeResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(state.habits.inner()))
}

async fn add_habit(
    state: web::Data<AppState>,
    request: web::Json<HabitModel>,
) -> SerdeResult<HttpResponse> {
    let habit_model = request.into_inner();

    let new_habit = Habit::new(&habit_model);

    println!("{:?}", new_habit);

    // TODO: handle if there is habit with the same name
    state.habits.inner().lock().unwrap().push(new_habit.clone());
    Ok(HttpResponse::Ok().json(new_habit))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Custom
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomPeriodicityValue(Vec<DayOfTheWeek>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DayOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ActivityType {
    Boolean,
    Counter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActivityCounterValue(i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Habit {
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    created_at: DateTime<Utc>,
}

impl Habit {
    pub fn new(data: &HabitModel) -> Self {
        Habit {
            title: data.title.clone(),
            periodicity: data.periodicity.clone(),
            periodicity_value: data.periodicity_value.clone(),
            activity_type: data.activity_type.clone(),
            activity_counter_value: data.activity_counter_value.clone(),
            created_at: Utc::now(),
        }
    }
}

type HabitsValue = Mutex<Vec<Habit>>;

#[derive(Serialize, Deserialize)]
pub struct Habits(HabitsValue);

impl Habits {
    pub fn new() -> Self {
        Habits(Mutex::new(Vec::new()))
    }
    pub fn inner(&self) -> &HabitsValue {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HabitsValue {
        &mut self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HabitModel {
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
}

struct AppState {
    habits: Habits,
}
