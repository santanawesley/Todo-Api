use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::{result::Error, prelude::*};
use actix_web::{Responder, web, HttpResponse};


use crate::DbPool;
use crate::schema::todos;
use crate::schema::todos::dsl;

type Result<T> = std::result::Result<T, Error>;
const CONNECTION_ERROR: &str = "Couldn't get a connection";
#[derive(Serialize, Queryable)]
pub struct Todo {
    pub id:i32,
    pub name: String,
    pub completed: bool,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Insertable)]
#[table_name="todos"]
pub struct NewTodo {
    pub name: String
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub name: String,
    pub completed: bool
}


impl Todo {
    pub fn fetch_all_todos(conn: &PgConnection) -> Result<Vec<Todo>> {
        dsl::todos.load::<Todo>(conn)
    }

    pub fn fetch_todo(conn: &PgConnection, id: i32) -> Result<Todo> {
        dsl::todos.filter(dsl::id.eq(id))
            .first::<Todo>(conn)
    }

    pub fn new_todo(conn: &PgConnection, new_todo: NewTodo) -> Result<Todo> {
        diesel::insert_into(todos::table)
            .values(new_todo)
            .get_result(conn)
    }

    pub fn delete_todo(conn: &PgConnection, id: i32) -> Result<usize> {
        diesel::delete(dsl::todos.filter(dsl::id.eq(id)))
            .execute(conn)
    }

    pub fn update_todo(conn: &PgConnection, id: i32, update_todo: UpdateTodo) -> Result<usize> {
        diesel::update(dsl::todos.find(id))
            .set((
                dsl::name.eq(update_todo.name), 
                dsl::completed.eq(update_todo.completed)
            ))
            .execute(conn)
    }


}

async fn get_todos(pool: web::Data<DbPool>) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(CONNECTION_ERROR)
    };

    match web::block(move || Todo::fetch_all_todos(&conn)).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Some error ocurred {}", err))
    }
}

async fn get_todo(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(CONNECTION_ERROR)
    };
        

    let id = id.into_inner();
    match web::block(move || Todo::fetch_todo(&conn, id)).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(CONNECTION_ERROR)
    };

    let id = id.into_inner();
    match web::block(move || Todo::delete_todo(&conn, id)).await {
        Ok(count ) if count > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

async fn create_todo(pool: web::Data<DbPool>, new_todo: web::Json<NewTodo>) ->  impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(CONNECTION_ERROR)
    };

    let new_todo = new_todo.into_inner();
    match web::block(move || Todo::new_todo(&conn, new_todo)).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

async fn update_todo(pool: web::Data<DbPool>, id: web::Path<i32>, update_todo: web::Json<UpdateTodo>) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(CONNECTION_ERROR)
    };

    let update_todo = update_todo.into_inner();
    let id = id.into_inner();
    match web::block(move || Todo::update_todo(&conn, id, update_todo)).await {
        Ok(count) if count > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}



pub fn router(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/todos", web::get().to(get_todos))
        .route("/todos", web::post().to(create_todo))
        .route("/todos/{id}", web::get().to(get_todo))
        .route("/todos/{id}", web::delete().to(delete_todo))
        .route("/todos/{id}", web::put().to(update_todo));
}