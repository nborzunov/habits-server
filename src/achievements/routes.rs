use crate::achievements::models::AchievementKey;
use crate::common::middlewares::auth::AuthenticationService;
use crate::habits::models::{HabitDetails, HabitsAchievement};
use crate::{achievements, habits};
use actix::AsyncContext;
use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Scope};
use actix_web_actors::ws;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

pub fn routes() -> Scope {
    web::scope("/achievements")
        .service(get)
        .route("/ws", web::get().to(ws))
}

#[derive(Serialize, Deserialize)]
struct Response {
    habit: HabitDetails,
    achievements: Vec<HabitsAchievement>,
}
#[get("/")]
pub async fn get(user: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
    let habits = habits::repository::get_all(client.clone(), user.0.id.unwrap())
        .await
        .unwrap();

    let result = achievements::repository::get_all(client.clone(), user.0.id.unwrap(), habits)
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}

struct MyWs {
    achievements_receiver: web::Data<Arc<Mutex<mpsc::UnboundedReceiver<Vec<AchievementKey>>>>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        let achievements_receiver = self.achievements_receiver.get_ref().clone();
        ctx.run_interval(Duration::from_secs(1), move |_act, ctx| {
            let mut achievements_receiver_guard = achievements_receiver.lock().unwrap();
            if let Ok(result) = achievements_receiver_guard.try_recv() {
                if result.len() == 0 {
                    return;
                }
                let result_json = serde_json::to_string(&result).unwrap();
                ctx.text(result_json);
            }
        });
    }
}

async fn ws(
    achievements_receiver: web::Data<Arc<Mutex<mpsc::UnboundedReceiver<Vec<AchievementKey>>>>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        MyWs {
            achievements_receiver,
        },
        &req,
        stream,
    );
    resp
}
