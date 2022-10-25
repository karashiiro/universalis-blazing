use crate::db::*;
use crate::types::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;

#[allow(non_snake_case)]
#[get("/api/extra/stats/most-recently-updated?<world>&<dcName>")]
pub async fn most_recently_updated(
    mut db: Connection<Stats>,
    world: Option<u32>,
    dcName: Option<&str>,
) -> Result<Json<MostLeastRecentlyUpdated>, Status> {
    match world {
        Some(w) => db
            .zrevrange_withscores::<_, MostLeastRecentlyUpdated>(w, 0, -1)
            .await
            .map_or_else(
                |_| Err(Status::NotFound),
                |ru| {
                    Ok(Json(MostLeastRecentlyUpdated {
                        items: ru
                            .items
                            .into_iter()
                            .map(|item| WorldItemUpload {
                                world_id: w,
                                ..item
                            })
                            .collect::<Vec<WorldItemUpload>>(),
                    }))
                },
            ),
        None => Err(Status::NotFound),
    }
}

#[allow(non_snake_case)]
#[get("/api/v2/extra/stats/most-recently-updated?<world>&<dcName>")]
pub async fn most_recently_updated_v2(
    db: Connection<Stats>,
    world: Option<u32>,
    dcName: Option<&str>,
) -> Result<Json<MostLeastRecentlyUpdated>, Status> {
    most_recently_updated(db, world, dcName).await
}