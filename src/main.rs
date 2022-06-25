#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;


use std::f32::consts::E;
use std::fmt::format;
use std::result;

use rocket::*;
use std::io::Cursor;
use rocket::http::{Header, ContentType, MediaType};
use rocket::response::Body;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::path::PathBuf;
use std::str::FromStr;
use rocket::Responder;
use rocket::Response;
use rocket::response::status::Created;
use rocket::http::Status as Stat;


#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn create_error(message: &str, err_type: &str) -> Error {
        if(err_type.eq("400")){
            return Error::StandardError(Json(ErrorResponse {
                message: message.to_string(),
            }))
        }
        else {
            return Error::Standard404dError(Json(ErrorResponse {
                message: message.to_string(),
            })) ;
        }
        
    }
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    StandardError(Json<ErrorResponse>),
    #[response(status = 404, content_type = "json")]
    Standard404dError(Json<ErrorResponse>),
}


#[derive(Debug,Serialize, Deserialize)]
struct PeopleList {
    people: Vec<PersonDetails>,
}


#[derive(Debug,Serialize, Deserialize)]
struct TaskToTypeList {
    people: Vec<TaskToTypeDetails>,
}

#[derive( Debug,Serialize, Deserialize)]
struct TaskToTypeDetails {
    id: String,
    r#type: String,
    ownerId: String,
    status: String,
}

#[derive( Debug,Serialize, Deserialize)]

struct StatusList {
    status: Vec<StatusString>,
}
#[derive( Debug,Serialize, Deserialize)]

struct StatusString{
    status: String,
}
#[derive( Debug,Serialize, Deserialize)]

struct OwnerList {
    owners: Vec<OwnerIdString>,
}
#[derive( Debug,Serialize, Deserialize)]

struct  OwnerIdString{
    OwnerId: String,
}

#[derive( Debug,Serialize, Deserialize)]
struct PersonDetails {
    id: String,
    name: String,
    email: String,
    favoriteProgrammingLanguage: String,
    #[serde(default = "default_number")]
    activeTaskCount: i32,
}

#[derive(Debug,Serialize, Deserialize)]
struct PersonData {
    name: String,
    email: String,
    favoriteProgrammingLanguage: String,
    
}



 #[derive(Debug,Serialize, Deserialize)]
struct TaskListImporved {
    chore: ChoreList,
    homeWork: HomeworkList,

}

#[derive(Debug,Serialize, Deserialize)]
struct PersonPatch {
    name: Option<String>,
    email: Option<String>,
    favoriteProgrammingLanguage: Option<String>,
}

#[derive(Debug,Serialize, Deserialize)]
struct TaskPatch {
    r#type: Option<String>,
    course: Option<String>,
    details: Option<String>,
    dueDate: Option<String>,
    status: Option<Status>,
    description:Option<String>,
    size: Option<String>,
}



#[derive(Debug,Serialize, Deserialize)]
struct TaskData {
    r#type: Option<String>,
    course: Option<String>,
    details: Option<String>,
    dueDate: Option<String>,
    status: Option<Status>,
    description:Option<String>,
    size: Option<String>,
}

#[derive(Debug,Serialize, Deserialize)]
struct ChoreDetails {
    id: String,
    taskType: String,
    ownerId: String,
    status: String,
    description: String,
    size: String,
}


#[derive(Debug,Serialize, Deserialize)]
struct HomeworkList {
    homeworks: Vec<HomeworkDetails>,
}

#[derive(Debug,Serialize, Deserialize)]
struct ChoreList {
    chores: Vec<ChoreDetails>,
}

#[derive(Debug,Serialize, Deserialize)]
struct HomeworkDetails {
    id: String,
    taskType: String,
    ownerId: String,
    status: String,
    course: String,
    details: String,
    dueDate: String,
}



#[derive(Debug,Serialize, Deserialize)]
enum Status {
    Active,
    Done,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::Active => "active",
            Status::Done => "done",
        })
    }
}

#[derive(Debug,Serialize, Deserialize)]
enum TaskType {
    Chore, HomeWork
}
impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TaskType::Chore => "chore",
            TaskType::HomeWork => "homework",
        })
    }
}
impl FromStr for TaskType {

    type Err = ();

    fn from_str(input: &str) -> Result<TaskType, Self::Err> {
        match input {
            "chore"  => Ok(TaskType::Chore),
            "homework"  => Ok(TaskType::HomeWork),
            _      => Err(()),
        }
    }
}


#[derive(Debug,Serialize, Deserialize)]
enum Size {
    Small, Medium, Large
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
        })
    }
}
impl FromStr for Size {

    type Err = ();

    fn from_str(input: &str) -> Result<Size, Self::Err> {
        match input {
            "small"  => Ok(Size::Small),
            "medium"  => Ok(Size::Medium),
            "large"  => Ok(Size::Large),
            _      => Err(()),
        }
    }
}


#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

fn default_resource() -> String {
    "Doesn't have".to_string()
}

fn default_status() -> Status {
    Status::Active
}

fn default_number() -> i32 {
    0
}

#[get("/api/people")]
fn fetch_all_person_details() -> Result<Json<PeopleList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("something went horribly wrong",&"400"));
        }
    };



    let mut statement = match db_connection.prepare("select * from person_details;") {
        Ok(statement) => statement,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong",&"400")),
    };
    
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(PersonDetails {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            favoriteProgrammingLanguage: row.get(3)?,
            activeTaskCount: row.get(4)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(people) => Ok(Json(PeopleList { people })),
                Err(_) =>Err(ErrorResponse::create_error("something went horribly wrong",&"400")),
            }
        }
        Err(_) =>Err(ErrorResponse::create_error("something went horribly wrong",&"400")),
    }
}

#[get("/api/people/<id>")]
fn fetch_a_person_details(id: String) -> Result<Json<PeopleList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("A person with that id does not exist1.",&"404"));
        }
    };


    let query = format!("select id, name, email, favoriteProgrammingLanguage, activeTaskCount from person_details where id = '{}';",id);
    print!("{}",query);

    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A person with that id does not exist2.",&"404")),
    };
    
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(PersonDetails {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            favoriteProgrammingLanguage: row.get(3)?,
            activeTaskCount: row.get(4)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(people) =>
                {
                    if people.is_empty()
                    {
                        return  Err(ErrorResponse::create_error("A person with that id does not exist3.",&"404"));
                    }
                    else
                    {
                        return Ok(Json(PeopleList { people }))
                    } 
                } ,
                Err(e) => Err(ErrorResponse::create_error(&e.to_string(),&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A person with that id does not exist5.",&"404")),
    }
}




#[post("/api/people", format = "json", data = "<person>")]
fn add_person(person: Json<PersonData>) -> Response<'static> {
    let error = "this is the error".to_string();
    let mut response = Response::new();
    let mut error_response = Response::new();

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            error_response.set_status(Stat::BadRequest);
            let error_body = "couldn't connect to DB".to_string();
            let body = Body::Sized(error_body.to_string(), 22);
            error_response.set_sized_body(Cursor::new("couldn't connect to DB"));
            return  error_response;
        }
    };

    if((person.name.is_empty() || person.favoriteProgrammingLanguage.is_empty() || person.email.is_empty())){
        error_response.set_status(Stat::BadRequest);
        let error_body = "missing arguments in body".to_string();
        let body = Body::Sized(error_body.to_string(), 25);
        error_response.set_sized_body(Cursor::new("missing arguments in body"));
        return error_response;
    }


    if(!(&person.email.contains('@'))){
        error_response.set_status(Stat::BadRequest);
        let error_body = "Illegal format of email".to_string();
        let body = Body::Sized(error_body.to_string(), 23);
        error_response.set_sized_body(Cursor::new("Illegal format of email"));
        return error_response;
    }
    let id = Uuid::new_v4().to_string();
    let mut statement =
        match db_connection.prepare("insert into person_details (id, name,email, favoriteProgrammingLanguage,activeTaskCount ) values ($1, $2, $3, $4 , 0)") {
            Ok(statement) => statement,
            Err(e) => {
                error_response.set_status(Stat::BadRequest);
                let error_body = "bad insertion to database".to_string();
                let body = Body::Sized(error_body.to_string(), 23);
                error_response.set_sized_body(Cursor::new("email already exisits")); 
                return error_response;
            },
        };
    let results = statement.execute(&[&id, &person.name, &person.email, &person.favoriteProgrammingLanguage]);
    response.set_status(Stat::Created);
    response.adjoin_raw_header("x-created-id", id.to_string());
    let url = format!("/api/person/{}", &id);
    response.adjoin_raw_header("Location", url);

    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    let error_body = "bad insertion to database".to_string();
    error_response.set_sized_body(Cursor::new("email already exisits"));
    
    match results {
        Ok(rows_affected) => response,
        Err(_) => error_response,
    }
}


#[delete("/api/tasks/<taskId>")]
fn remove_task(taskId: String) -> Result<Json<StatusMessage>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect to DB.",&"400"));
        }
    };
    let task_to_type_result = get_task_to_type(&taskId);
    let task_id = &taskId.to_string();
    let task_to_type = match task_to_type_result{
        Ok(inner) => inner,
        Err(e) => return Err(ErrorResponse::create_error("Task id doesn't exisit.",&"400")),
    };
    let mut task_original_status= task_to_type.status.to_lowercase().to_string();

    if task_to_type.r#type.to_string().eq("chore"){
        let mut delete_chore_statement = match db_connection.prepare("delete from chore_list where id = $1;") {
            Ok(delete_chore_statement) => delete_chore_statement,
            Err(_) => return Err(ErrorResponse::create_error("cant create query.",&"400")),
        };
        let chore_delete_results = delete_chore_statement.execute(&[&taskId]);
    }
    else {
        let mut delete_homework_statement = match db_connection.prepare("delete from homework_list where id = $1;") {
            Ok(delete_homework_statement) => delete_homework_statement,
            Err(_) => return Err(ErrorResponse::create_error("cant create query.",&"400")),
        };
        let homework_delete_result = delete_homework_statement.execute(&[&taskId]);

    };
    let mut delte_task_to_typestatement = match db_connection.prepare("delete from task_to_type where id = $1;") {
        Ok(delte_task_to_typestatement) => delte_task_to_typestatement,
        Err(_) => return Err(ErrorResponse::create_error("cant create query.",&"400")),
    };
    let task_to_type_delete_results = delte_task_to_typestatement.execute(&[&taskId]);

    match task_to_type_delete_results {
        Ok(rows_affected) =>
        {
            if rows_affected == 0 {
                return Err(ErrorResponse::create_error("a task with id doens't exist.",&"404"));
            }
            else {
                {
                    if task_original_status.eq("active"){let new_owner_result = db_connection.execute("update person_details set activeTaskCount = activeTaskCount - 1 where person_details.id = $1 ", &[&task_to_type.ownerId.to_string()]);}
                    Ok(Json(StatusMessage {
                        message: format!("task removed successfully"),
                    }))
                }
            }
        },
         
        Err(_) => Err(ErrorResponse::create_error("a task with id doens't exist.",&"404")),
    }

    
}

#[put("/api/tasks/<taskId>/status",format = "json", data = "<newStatus>")]
fn update_task_status(taskId: String, newStatus: Json<String>) -> Response<'static> {

    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::NoContent);


    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    error_response.set_sized_body(Cursor::new("task Id doesn't exist"));


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return error_response;
        }
    };


    
    let task_to_type_result = get_task_to_type(&taskId);
    let task_id = &taskId.to_string();
    let task_to_type = match task_to_type_result{
        Ok(inner) => inner,
        Err(e) => return error_response,
    };

    
    if (!newStatus.to_string().eq("active") && !newStatus.to_string().eq("done")){
        error_response.set_sized_body(Cursor::new("status isn't done or active"));
        return error_response;
    }

    
    if task_to_type.r#type.to_string().eq("chore"){
        let mut ownerId_result = match get_chore_by_chore_id(&taskId)
        {
            Ok(inner) => inner,
            Err(_) => return error_response,
        };
        let task_original_status = &ownerId_result.chores.get(0).unwrap().status;
        let ownerId_string = &ownerId_result.chores.get(0).unwrap().ownerId;
        let mut update_chore_statement = match db_connection.prepare("update chore_list SET status = $1 where id = $2;") {
            Ok(update_chore_statement) => update_chore_statement,
            Err(_) => return error_response,
        };
        let chore_delete_results = update_chore_statement.execute(&[&newStatus.to_string(),&taskId]);
        if(newStatus.to_lowercase().to_string().eq("active") && task_original_status.to_lowercase().to_string().eq("done")){
            db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
        }
        if(newStatus.to_lowercase().to_string().eq("done") && task_original_status.to_lowercase().to_string().eq("active")){
            db_connection.execute("update person_details set activeTaskCount = activeTaskCount -1  where person_details.id = $1 ", &[ownerId_string.to_string()]); 
        }

        



    }
    else {
        let mut ownerId_result = match get_homrwork_by_homework_id(&taskId)
        {
            Ok(inner) => inner,
            Err(_) => return error_response,
        };
        let task_original_status = &ownerId_result.homeworks.get(0).unwrap().status;
        let ownerId_string = &ownerId_result.homeworks.get(0).unwrap().ownerId;
        let mut update_homework_statement = match db_connection.prepare("update homework_list SET status = $1 where id = $2;") {
            Ok(update_homework_statement) => update_homework_statement,
            Err(_) => return error_response,
        };
        let homework_delete_result = update_homework_statement.execute(&[&newStatus.to_string(),&taskId]);
        println!("{}", newStatus.to_lowercase().to_string());
        println!("{}", task_original_status.to_lowercase().to_string());
        if(newStatus.to_lowercase().to_string().eq("active") && task_original_status.to_lowercase().to_string().eq("done")){
            db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
        }
        if(newStatus.to_lowercase().to_string().eq("done") && task_original_status.to_lowercase().to_string().eq("active")){
            db_connection.execute("update person_details set activeTaskCount = activeTaskCount -1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
        }

    };

    let new_status_for_task = db_connection.execute("update task_to_type SET status = $1 where id = $2 ", &[&newStatus.to_string(),&taskId]);
    return response 

    }


#[put("/api/tasks/<taskId>/owner",format = "json", data = "<newOwner>")]
fn update_task_owner(taskId: String, newOwner: Json<String>) -> Response<'static> {

    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::NoContent);


    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    error_response.set_sized_body(Cursor::new("task Id doesn't exist"));


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return error_response;
        }
    };

    let task_to_type_result = get_task_to_type(&taskId);
    let task_id = &taskId.to_string();
    let task_to_type = match task_to_type_result{
        Ok(inner) => inner,
        Err(e) => return error_response,
    };

    let new_owner_result1 = db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[&newOwner.to_string()]);
    match new_owner_result1 {
        Ok(new_rows_affected1) =>{
            if new_rows_affected1 == 0 
            {   
                error_response.set_sized_body(Cursor::new("ownerId doesn't exisit"));
                return error_response
            }
        },
        Err(e) => {error_response.set_sized_body(Cursor::new(e.to_string()));
            return error_response;
        }
    };
        
    if task_to_type.r#type.to_string().eq("chore"){
        let mut delete_chore_statement = match db_connection.prepare("update chore_list SET ownerId = $1 where id = $2;") {
            Ok(delete_chore_statement) => delete_chore_statement,
            Err(_) => return error_response,
        };
        let chore_delete_results = delete_chore_statement.execute(&[&newOwner,&taskId]);
    }
    else {
        let mut delete_homework_statement = match db_connection.prepare("update homework_list SET ownerId = $1 where id = $2;") {
            Ok(delete_homework_statement) => delete_homework_statement,
            Err(_) => return error_response,
        };
        let homework_delete_result = delete_homework_statement.execute(&[&newOwner,&taskId]);

    };
    let mut delte_task_to_typestatement = match db_connection.prepare("update task_to_type SET ownerId = $1 where id = $2;") {
        Ok(delte_task_to_typestatement) => delte_task_to_typestatement,
        Err(_) => return error_response,
    };
    let task_to_type_delete_results = delte_task_to_typestatement.execute(&[&newOwner,&taskId]);

    let new_owner_result = db_connection.execute("update person_details set activeTaskCount = activeTaskCount - 1 where person_details.id = $1 ", &[&task_to_type.ownerId.to_string()]);
    return response

}






#[post("/api/people/<personId>/tasks", format = "json", data = "<task>")]
fn add_a_task_to_person(personId: String, task: Json<TaskData>) -> Response<'static> {

    let id = Uuid::new_v4().to_string();

    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::Created);
    response.adjoin_raw_header("x-created-id", id.to_string());
    let url = format!("/api/tasks/{}", &id);
    response.adjoin_raw_header("Location", url);

    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return error_response;
        }

    };
    let person_detail = get_a_person(&personId);
    let test_person = match person_detail{
        Ok(inner) => inner,
        Err(e) => {
            error_response.set_sized_body(Cursor::new("Person with this Id doesn't exisit".to_string()));
            return error_response
        }
    };

    let mut statement =
    match db_connection.prepare("insert into task_to_type (id, type, ownerId, status) values ($1, $2, $3, $4)") {
        Ok(statement) => statement,
        Err(e) => {
            error_response.set_sized_body(Cursor::new(e.to_string()));
            return error_response
        }
    };

    let mut taskStatus=Status::Active.to_string();
    match &task.status {
        Some(inner) =>
        {
            if (inner.to_string().eq("active") || inner.to_string().eq("done")){
                taskStatus = inner.to_string();
            }
            else {
                error_response.set_sized_body(Cursor::new("status isn't valid".to_string()));
                return error_response ;
            }
        }
        None => {taskStatus=Status::Active.to_string();}
    }
    let mut task_type ;
    match &task.r#type {
        Some(inner) =>
        {
            println!("this is the inner {}", inner);
            if (inner.to_string().to_lowercase().eq("chore") || inner.to_string().to_lowercase().eq("homework")){
                task_type = inner.to_string().to_lowercase();
            }
            else {
                error_response.set_sized_body(Cursor::new("task type isn't valid".to_string()));
                return error_response ;
            }
        }
        None => 
        {
            error_response.set_sized_body(Cursor::new("task type1 is missing".to_string()));
            return error_response ;
        }
    };
    let mut task_description = "none given".to_string();
    let mut task_size= "none given".to_string();

    if task_type.to_string().eq("chore"){
        match &task.description {
            Some(inner) => {
                println!("the desc is {}", inner.to_string());
                task_description = inner.to_string();
            }
            None => {
                error_response.set_sized_body(Cursor::new("task description is missing for chore type task".to_string()));
                return error_response ;
            }
        }
        match &task.size {
            Some(inner) => 
            {   
                println!("the size is {}", inner.to_string());

                if (inner.to_string().to_lowercase().eq("small") || inner.to_string().to_lowercase().eq("medium")|| inner.to_string().to_lowercase().eq("large")){
                task_size = inner.to_string().to_lowercase();
                }
                else {
                    error_response.set_sized_body(Cursor::new("task size is of invalid value for chore type task".to_string()));
                    return error_response ;
                }
            }
            None => {
                error_response.set_sized_body(Cursor::new("task size is missing for chore type task".to_string()));
                return error_response ;
            }

        }
    }

    let mut task_course ="none given".to_string();
    let mut task_details = "none given".to_string();
    let mut task_due_date = "none given".to_string();
    if task_type.to_string().eq("homework")
    {
        match &task.course {
            Some(inner) => {task_course = inner.to_string()}
            None => {
                error_response.set_sized_body(Cursor::new("task course is missing for homework type task".to_string()));
                return error_response ;
            }
        }

        match &task.details {
            Some(inner) => {task_details = inner.to_string()}
            None => {
                error_response.set_sized_body(Cursor::new("task details is missing for homework type task".to_string()));
                return error_response ;
            }
        }
        match &task.dueDate {
            Some(inner) => {task_due_date = inner.to_string()}
            None => {
                error_response.set_sized_body(Cursor::new("task due date is missing for homework type task".to_string()));
                return error_response ;
            }
        }
    }

    let chore = "chore".to_string();
    let homeWork = "homework".to_string();
   let results = statement.execute(&[&id, &task_type, &personId, &taskStatus]);
    println!("the id is {}", &task_type);
    if task_type.to_string().eq("chore")
    {
    
         let chore = ChoreDetails{
            id: id,
            taskType: task_type,
            ownerId: personId,
            status: taskStatus,
            description: task_description,
            size: task_size,
         };
         println!("we got here1");
        return add_a_chore_to_person( chore)
    }
    if task_type.to_string().eq("homework")
    {
        println!("am I getting here? ");
        let homework = HomeworkDetails{
            id: id,
            taskType: task_type,
            ownerId: personId,
            status: taskStatus,
            course: task_course,
            details: task_details,
            dueDate: task_due_date,
         };
        return add_a_homework_to_person(homework)
    }
    else {
        let error_new_body = format!("task type {} is illeagal", task_type.to_string());
        error_response.set_sized_body(Cursor::new(error_new_body));
        return error_response;

    };

}




//////////////////////////////////////////////////
/// 
/// 
#[get("/api/people/<personId>/<tasks>?<status>")]
fn fetch_all_person_tasks_for_good(personId: String ,tasks: String, status: Option<String> ) -> Result<Json<TaskListImporved>, Error> {
    
    let person_detail = get_a_person(&personId);
    let test_person = match person_detail{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };

    let chores = get_chore_by_id(&personId, &status);

    let homeworks = get_homrwork_by_id(&personId, &status);


    let clean_chores = match chores {
        Ok(inner) =>  inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong1",&"400")),
    };

    let clean_homeworks = match homeworks {
        Ok(inner) => inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong2",&"400")),
    };

    return Ok(Json(TaskListImporved
        {
            chore: clean_chores ,
            homeWork: clean_homeworks,
}))


}

#[get("/api/tasks/<taskId>")]
fn fetch_task_by_id(taskId: String) -> Result<Json<TaskListImporved>, Error> {
    
    let person_detail = get_a_task(&taskId);
    let test_person = match person_detail{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };

    let chores = get_chore_by_chore_id(&taskId);

    let homeworks = get_homrwork_by_homework_id(&taskId);


    let clean_chores = match chores {
        Ok(inner) =>  inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong1",&"400")),
    };

    let clean_homeworks = match homeworks {
        Ok(inner) => inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong2",&"400")),
    };

    return Ok(Json(TaskListImporved
        {
            chore: clean_chores ,
            homeWork: clean_homeworks,
}))


}



#[get("/api/tasks/<taskId>/status")]
fn fetch_task_status_by_id(taskId: String) -> Result<Json<StatusList>, Error> {

    let task_result_list = get_a_task(&taskId);
    let task_list = match task_result_list{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };
    let mut option_task = task_list.people.get(0);

    let mut task = match option_task{
        Some(inner) => inner.status.to_string(),
        None => return Err(ErrorResponse::create_error("task doesn't exist",&"400")),
    };

    let mut status = "active";
    if task.to_string().eq(&"chore".to_string())
    {
        return fetch_a_chore_status(&taskId);
    }
    else {return fetch_a_homework_status(&taskId);;}

}


#[get("/api/tasks/<taskId>/owner")]
fn fetch_a_taskOwner(taskId: String ) -> Result<Json<OwnerList>, Error> {
    let task_result_list = get_a_task(&taskId);
    let task_list = match task_result_list{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };
    let mut option_task = task_list.people.get(0);

    let mut task = match option_task{
        Some(inner) => inner.r#type.to_string(),
        None => return Err(ErrorResponse::create_error("task doesn't exist",&"400")),
    };

    let mut status = "active";
    if task.to_string().eq(&"chore".to_string())
    {
        return fetch_a_chore_owner(&taskId);
    }
    else {return fetch_a_homework_owner(&taskId);;}

}


#[delete("/api/people/<personId>")]
fn remove_person(personId: String) -> Result<Json<StatusMessage>, Error> {
    

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("something went horribly wrong2",&"400"));
        }
    };


    let person_detail = get_a_person(&personId);
    let test_person = match person_detail{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };


    let mut chore_statement = match db_connection.prepare("delete from chore_list where ownerId = $1;") {
        Ok(statement) => statement,
        Err(_) =>return  Err(ErrorResponse::create_error("a person with id doens't exist1.",&"404")),
    };
    let chore_results = chore_statement.execute(&[&personId]);

    let mut homework_statement = match db_connection.prepare("delete from homework_list where ownerId = $1;") {
        Ok(statement) => statement,
        Err(_) =>return  Err(ErrorResponse::create_error("a person with id doens't exist.",&"404")),
    };
    let homework_results = homework_statement.execute(&[&personId]);

    let mut homework_statement = match db_connection.prepare("delete from task_to_type where ownerId = $1;") {
        Ok(statement) => statement,
        Err(_) =>return  Err(ErrorResponse::create_error("a person with id doens't exist.",&"404")),
    };
    let homework_results = homework_statement.execute(&[&personId]);

    let mut person_statement = match db_connection.prepare("delete from person_details where id = $1;") {
        Ok(statement) => statement,
        Err(_) =>return  Err(ErrorResponse::create_error("a person with id doens't exist.",&"404")),
    };
    let person_results = person_statement.execute(&[&personId]);


    Ok(Json(StatusMessage {message: format!("Person removed successfully")}))
                
    
}

#[patch("/api/people/<personId>",format = "json", data = "<updated_details>")]
fn patch_person_details(personId: String, updated_details: Json<PersonPatch>) -> Result<Json<PeopleList>, Error> {

    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::NoContent);


    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    error_response.set_sized_body(Cursor::new("person Id doesn't exist"));


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect.",&"400"));
        }
    };

    
    let person_detail = get_a_person(&personId);
    let test_person = match person_detail{
        Ok(inner) => inner,
        Err(e) => {
            return Err(ErrorResponse::create_error("Person with this ID doesn't exist.",&"404"));

        }
    };

    match &updated_details.email{
        Some(inner) => {
            if !(inner.to_string().contains("@")){
                return Err(ErrorResponse::create_error("Illegal email address.",&"404"));
            }
            else {
                db_connection.execute("update person_details set email = $1 where id = $2 ", &[&inner.to_string(), &personId]);
                }
            }
        None => (),
    };

    match &updated_details.name{
        Some(inner) => {db_connection.execute("update person_details set name = $1 where id = $2 ", &[&inner.to_string(), &personId]);}
        None => (),
    };

    match &updated_details.favoriteProgrammingLanguage{
        Some(inner) => {db_connection.execute("update person_details set favoriteProgrammingLanguage = $1 where id = $2 ", &[&inner.to_string(), &personId]);}
        None => (),
    };

    let updated_person_detail = get_a_person(&personId);
    let updated_test_person = match updated_person_detail{
        Ok(inner) => inner,
        Err(e) => {
            return Err(ErrorResponse::create_error("Person with this ID doesn't exist.",&"404"));

        }
    };


    return Ok(Json(updated_test_person)); 

    }


//////////////////////////////////////////////////////



fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists person_details (
                    id varchar(128) not null ,
                    name varchar(64) not null,
                    email varchar(64) not null,
                    favoriteProgrammingLanguage varchar(64) not null,
                    activeTaskCount int not null,
                    PRIMARY KEY (id, email),
                    UNIQUE(email)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();

            db_connection
            .execute(
                "create table if not exists task_details (
                    id varchar(128) not null ,
                    type varchar(64) not null,
                    ownerId varchar(128) not null,
                    course varchar(64),
                    details varchar(64),
                    dueDate varchar(64),
                    description varchar(64),
                    size varchar(64),
                    status varchar(64),
                    PRIMARY KEY (id, ownerId)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();

            db_connection
            .execute(
                "create table if not exists chore_list (
                    id varchar(128) not null ,
                    type varchar(64) not null,
                    ownerId varchar(128) not null,
                    description varchar(64) not null,
                    size varchar(64) not null,
                    status varchar(64) not null,
                    PRIMARY KEY (id, ownerId)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();


            db_connection
            .execute(
                "create table if not exists homework_list (
                    id varchar(128) not null ,
                    type varchar(64) not null,
                    ownerId varchar(128) not null,
                    course varchar(64) not null ,
                    details varchar(64) not null ,
                    dueDate varchar(64) not null ,
                    status varchar(64) not null ,
                    PRIMARY KEY (id, ownerId)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();

            db_connection
            .execute(
                "create table if not exists task_to_type (
                    id varchar(128) not null ,
                    type varchar(64) not null,
                    ownerId varchar(128) not null ,
                    status varchar(128) not null ,
                    PRIMARY KEY(id)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();

            
    }


    rocket::ignite()
        .mount(
            "/",
            routes![fetch_a_person_details, fetch_all_person_details, add_person, remove_person, add_a_task_to_person,
            fetch_all_person_tasks_for_good, fetch_task_by_id, fetch_task_status_by_id, fetch_a_taskOwner, remove_task, update_task_status,
            update_task_owner, patch_person_details, patch_task_details],
        )
        .launch();
}




fn get_task_to_type(taskId: &String) -> Result<TaskToTypeDetails, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(e) => {
            return Err(ErrorResponse::create_error("problem connecting to DB.",&"400"));
        }
    };

    let query = format!("select * from task_to_type where id = '{}';",taskId);

    let mut old_owner_statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(e) => {
            return Err(ErrorResponse::create_error("Task id doesn't exisit.",&"404"));
        },
    };
    
    let results = old_owner_statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            TaskToTypeDetails {
            id: row.get(0)?,
            r#type:  row.get(1)?,
            ownerId: row.get(2)?,
            status: row.get(3)?,
        },)
    });
    let task_to_type = match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            match collection {
                Ok(tasks) =>{
                    if tasks.is_empty()
                    {
                        return Err(ErrorResponse::create_error("Task id doesn't exisit.",&"404"));
                    }
                    else 
                        {
                        let return_value = tasks.get(0).unwrap();
                        return Ok(TaskToTypeDetails{
                            id: return_value.id.to_string(),
                            r#type: return_value.r#type.to_string(),
                            ownerId: return_value.ownerId.to_string(),
                            status: return_value.status.to_string(),
                        })
                        }
                        }
                        Err(_) => return Err(ErrorResponse::create_error("A task with that id does not exist3.",&"404"))
                }
            }
        Err(_) => return Err(ErrorResponse::create_error("A task with that id does not exist3.",&"404"))
        }; 
    }





fn get_a_person(id: &String) -> Result<PeopleList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("A person with that id does not exist1.",&"404"));
        }
    };


    let query = format!("select id, name, email, favoriteProgrammingLanguage, activeTaskCount from person_details where id = '{}';",id);

    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A person with that id does not exist2.",&"404")),
    };
    
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(PersonDetails {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            favoriteProgrammingLanguage: row.get(3)?,
            activeTaskCount: row.get(4)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(people) =>
                {
                    if people.is_empty()
                    {
                        return  Err(ErrorResponse::create_error("A person with that id does not exist3.",&"404"));
                    }
                    else
                    {
                        return Ok(PeopleList { people })
                    } 
                } ,
                Err(e) => Err(ErrorResponse::create_error(&e.to_string(),&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A person with that id does not exist5.",&"404")),
    }
}






// #[post("/api/people/<personId>/tasks", format = "json", data = "<task>")]
fn add_a_chore_to_person( task: ChoreDetails) -> Response<'static> {
    let header_id = &task.id.to_string();
    let header_url = &task.id.to_string();
    let insertion_id = &task.id.to_string();
    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::Created);
    response.adjoin_raw_header("x-created-id", header_id.to_string());
    let url = format!("/api/tasks/{}", header_url.to_string());
    response.adjoin_raw_header("Location", url);

    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return error_response;
        }

    };

    let mut statement =
    match db_connection.prepare("insert into chore_list (id, type,ownerId, description , size, status ) values ($1, $2, $3, $4,$5,$6)") {
        Ok(statement) => statement,
        Err(e) => {
            error_response.set_sized_body(Cursor::new(e.to_string()));
            return error_response
        }
    };
   let results = statement.execute(&[&task.id.to_string(), &task.taskType.to_string(), &task.ownerId , &task.description, &task.size.to_string(), &task.status]);
    println!("the id is {}", &task.ownerId);
match results {
    Ok(rows_affected) =>{
        if(task.status.to_lowercase().to_string().eq("active")){
        db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[&task.ownerId]);
        }
        return response},
    Err(e) => {
        db_connection.execute("delete from task_details where id = $1;", &[&task.ownerId]);
        db_connection.execute("delete from task_to_type where id = $1;", &[&task.ownerId]);
        error_response.set_sized_body(Cursor::new(e.to_string()));
        return error_response
    }
}




}



fn add_a_homework_to_person( task:HomeworkDetails) -> Response<'static> {

    let header_id = &task.id.to_string();
    let header_url = &task.id.to_string();
    let insertion_id = &task.id.to_string();
    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::Created);
    response.adjoin_raw_header("x-created-id", header_id.to_string());
    let url = format!("/api/tasks/{}", &header_url);
    response.adjoin_raw_header("Location", url);

    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    let error_body = "bad insertion to database".to_string();
    error_response.set_sized_body(Cursor::new("person ID doesn't exisit"));

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return error_response;
        }

    };
    let mut taskStatus=Status::Active.to_string();
    if (!task.status.to_string().is_empty()){
        taskStatus = task.status.to_string();
    }


    let mut statement =
    match db_connection.prepare("insert into homework_list (id, type,ownerId, course, details ,dueDate, status ) values ($1, $2, $3, $4,$5,$6,$7)") {
        Ok(statement) => statement,
        Err(e) => {
            error_response.set_sized_body(Cursor::new(e.to_string()));
            return error_response
        }
    };
   let results = statement.execute(&[&task.id.to_string(), &task.taskType.to_string(), &task.ownerId ,&task.course, &task.details  ,&task.dueDate.to_string(), &taskStatus]);
    println!("the id is {}", &task.ownerId);
match results {
    Ok(rows_affected) =>{
        if(task.status.to_lowercase().to_string().eq("active")){
        db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[&task.ownerId]);
        }
        return response},
    Err(e) => {
        db_connection.execute("delete from homework_list where id = $1;", &[&task.ownerId]);
        db_connection.execute("delete from task_to_type where id = $1;", &[&task.ownerId]);
        error_response.set_sized_body(Cursor::new("missing arguments in homeork".to_string()));
        return error_response
    },
}




}




fn get_homrwork_by_id(personId: &String , status: &Option<String> ) -> Result<HomeworkList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("problem connecting to DB.",&"400"));
        }
    };
    let query;
    match status {
        Some(inner) =>     { query = format!("select * from homework_list where ownerId = '{0}' and status = '{1}';",personId, inner);},
        None => { query = format!("select * from homework_list where ownerId = '{}';",personId);}
        ,

    }
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A person with that id does not exist.",&"404")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            HomeworkDetails {
            id: row.get(0)?,
            taskType:  row.get(1)?,
            ownerId: row.get(2)?,
            course: row.get(3)?,
            details: row.get(4)?,
            status: row.get(6)?,
            dueDate: row.get(5)?,

            
        },)
    });
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            match collection {
                Ok(homeworks) =>{Ok(HomeworkList{homeworks})},
                Err(_) => Err(ErrorResponse::create_error("user has no tasks or doesn't exisit.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("user has no tasks or doesn't exisit.",&"404")),
    }
}


fn get_chore_by_id(personId: &String , status: &Option<String> ) -> Result<ChoreList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("problem connecting to DB.",&"400"));
        }
    };
    let query;
    match status {
        Some(inner) =>     { query = format!("select * from chore_list where ownerId = '{0}' and status = '{1}';",personId, inner);},
        None => { query = format!("select * from chore_list where ownerId = '{}';",personId);}
        ,

    }
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A person with that id does not exist.",&"404")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            ChoreDetails {
            id: row.get(0)?,
            taskType:  row.get(1)?,
            ownerId: row.get(2)?,
            status: row.get(5)?,
            size: row.get(4)?,
            description: row.get(3)?,

            
        },)
    });
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            match collection {
                Ok(chores) =>{Ok(ChoreList{chores})},
                Err(e) =>
                {
                    println!("this is the error {}", e);
                    return Err(ErrorResponse::create_error("user has no tasks or doesn't exisit.",&"404"));
                } 
            }
        }
        Err(e) => 
        {
            println!("this is the query {}", e);
            Err(ErrorResponse::create_error("user has no tasks or doesn't exisit.",&"404"))
        },
    }
}



fn get_homrwork_by_homework_id(taskId: &String) -> Result<HomeworkList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("problem connecting to DB.",&"400"));
        }
    };
    let query = format!("select * from homework_list where id = '{}';",taskId);


    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            HomeworkDetails {
            id: row.get(0)?,
            taskType:  row.get(1)?,
            ownerId: row.get(2)?,
            course: row.get(3)?,
            details: row.get(4)?,
            status: row.get(6)?,
            dueDate: row.get(5)?,

            
        },)
    });
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            match collection {
                Ok(homeworks) =>{Ok(HomeworkList{homeworks})},
                Err(_) => Err(ErrorResponse::create_error("task.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("task.",&"404")),
    }
}


fn get_chore_by_chore_id(taskId: &String ) -> Result<ChoreList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("problem connecting to DB.",&"400"));
        }
    };
    let query = format!("select * from chore_list where id = '{}';",taskId);

    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            ChoreDetails {
            id: row.get(0)?,
            taskType:  row.get(1)?,
            ownerId: row.get(2)?,
            status: row.get(5)?,
            size: row.get(4)?,
            description: row.get(3)?,

            
        },)
    });
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            match collection {
                Ok(chores) =>{Ok(ChoreList{chores})},
                Err(e) =>
                {
                    println!("this is the error {}", e);
                    return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"));
                } 
            }
        }
        Err(e) => 
        {
            println!("this is the query {}", e);
            Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"))
        },
    }
}


fn get_a_task(id: &String) -> Result<TaskToTypeList, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("A task with that id does not exist1.",&"404"));
        }
    };


    let query = format!("select * from task_to_type where id = '{}';",id);

    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("A task with that id does not exist2.",&"404")),
    };
    
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(TaskToTypeDetails {
            id: row.get(0)?,
            r#type: row.get(1)?,
            ownerId: row.get(2)?,
            status: row.get(3)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(people) =>
                {
                    if people.is_empty()
                    {
                        return  Err(ErrorResponse::create_error("A task with that id does not exist32.",&"404"));
                    }
                    else
                    {
                        return Ok(TaskToTypeList { people })
                    } 
                } ,
                Err(e) => Err(ErrorResponse::create_error(&e.to_string(),&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist5.",&"404")),
    }
}





fn fetch_a_chore_status(taskId: &String ) -> Result<Json<StatusList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect to DB.",&"400"));
        }
    };
    let query  = format!("select status from chore_list where id = '{}';",taskId);
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("problem with creating query.",&"400")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            StatusString {status : row.get(0)?,       
        },)
    });

    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            println!("this is the collection {:?}", collection);

            match collection {
                Ok(status) =>
                {
                    if status.is_empty()
                    {
                        return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"))
                    }
                    else {
                        {
                            return Ok(Json(StatusList{status}));
                        }
                    }
                } ,
                Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    }
}


fn fetch_a_homework_status(taskId: &String ) -> Result<Json<StatusList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect to DB.",&"400"));
        }
    };
    let query  = format!("select status from homework_list where id = '{}';",taskId);
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("problem with creating query.",&"400")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            StatusString {status : row.get(0)?,       
        },)
    });

    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            println!("this is the collection {:?}", collection);

            match collection {
                Ok(status) =>
                {
                    if status.is_empty()
                    {
                        return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"))
                    }
                    else {
                        {
                            return Ok(Json(StatusList{status}));
                        }
                    }
                } ,
                Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    }
}



fn fetch_a_chore_owner(taskId: &String ) -> Result<Json<OwnerList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect to DB.",&"400"));
        }
    };
    let query  = format!("select ownerId from chore_list where id = '{}';",taskId);
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("problem with creating query.",&"400")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            OwnerIdString {OwnerId : row.get(0)?,       
        },)
    });

    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            println!("this is the collection {:?}", collection);

            match collection {
                Ok(owners) =>
                {
                    if owners.is_empty()
                    {
                        return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"))
                    }
                    else {
                        {
                            return Ok(Json(OwnerList{owners}));
                        }
                    }
                } ,
                Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    }
}


fn fetch_a_homework_owner(taskId: &String ) -> Result<Json<OwnerList>, Error> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect to DB.",&"400"));
        }
    };
    let query  = format!("select ownerId from homework_list where id = '{}';",taskId);
    println!("this is the query {}", query);
    let mut statement = match db_connection.prepare(&query) {
        Ok(statement) => statement,
        Err(_) => return Err(ErrorResponse::create_error("problem with creating query.",&"400")),
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(
            OwnerIdString {OwnerId : row.get(0)?,       
        },)
    });

    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();
            println!("this is the collection {:?}", collection);

            match collection {
                Ok(owners) =>
                {
                    if owners.is_empty()
                    {
                        return Err(ErrorResponse::create_error("A task with that id does not exist.",&"404"))
                    }
                    else {
                        {
                            return Ok(Json(OwnerList{owners}));
                        }
                    }
                } ,
                Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
            }
        }
        Err(_) => Err(ErrorResponse::create_error("A task with that id does not exist.",&"404")),
    }
}




#[patch("/api/tasks/<taskId>",format = "json", data = "<updated_details>")]
fn patch_task_details(taskId: String, updated_details: Json<TaskPatch>) -> Result<Json<TaskListImporved>, Error> {

    let mut response = Response::new();
    let mut error_response = Response::new();
    response.set_status(Stat::NoContent);


    error_response.set_status(Stat::BadRequest);
    error_response.set_header(ContentType(MediaType::Plain));
    error_response.set_sized_body(Cursor::new("person Id doesn't exist"));


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(ErrorResponse::create_error("can't connect.",&"400"));
        }
    };

    let task_result_list = get_a_task(&taskId);
    let task_list = match task_result_list{
        Ok(inner) => inner,
        Err(e) => return Err(e),
    };
    let mut option_task = task_list.people.get(0);

    let mut task_type = "chore".to_string();
    let mut task_status= "active".to_string();
    match option_task{
        Some(inner) => {
            task_type = inner.r#type.to_string();
            task_status = inner.status.to_string();
        }
        None => return Err(ErrorResponse::create_error("task doesn't exist",&"400")),
    };

    let mut update_patched_task_type = match &updated_details.r#type{
        Some(inner) => {
            inner.to_string()
        }
        None => "".to_string(),
    };


    let tpye_of_task = "chore";
    if task_type.to_string().to_lowercase().eq(&"chore".to_string()) && updated_details.details.is_none() && updated_details.dueDate.is_none() && updated_details.course.is_none()
    {
        match &updated_details.status{
            Some(inner) => {
                let task_status ;
                if (inner.to_string().to_lowercase().eq("active") || inner.to_string().to_lowercase().eq("done"))&& (update_patched_task_type.to_lowercase().eq(&"chore".to_string())||update_patched_task_type.to_lowercase().is_empty()){
                    task_status = inner.to_string().to_lowercase();
                }
                else {
                    error_response.set_sized_body(Cursor::new("status isn't valid".to_string()));
                    return Err(ErrorResponse::create_error("status isn't valid",&"400")) ;
                }
                let mut ownerId_result = match get_chore_by_chore_id(&taskId)
                {
                    Ok(inner) => inner,
                    Err(_) => return Err(ErrorResponse::create_error("can't get chore",&"400")) ,
                };
                let task_original_status = &ownerId_result.chores.get(0).unwrap().status;
                db_connection.execute("update task_to_type set status = $1 where id = $2 ", &[&task_status.to_string(), &taskId]);
                db_connection.execute("update chore_list set status = $1 where id = $2 ", &[&task_status.to_string(), &taskId]);
                let ownerId_string = &ownerId_result.chores.get(0).unwrap().ownerId;
                println!("{}",inner.to_string().to_lowercase());
                println!("{}", task_original_status.to_lowercase().to_string());
                println!("{}", ownerId_string.to_lowercase().to_string());
                if(inner.to_string().to_lowercase().eq("active") && task_original_status.to_lowercase().to_string().eq("done")){
                    db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
                }
                if(inner.to_string().to_lowercase().eq("done") && task_original_status.to_lowercase().to_string().eq("active")){
                    db_connection.execute("update person_details set activeTaskCount = activeTaskCount -1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
                }
        
            }
            None => (),
        };

        match &updated_details.description{
            Some(inner) => {db_connection.execute("update chore_list set description = $1 where id = $2 ", &[&inner.to_string(), &taskId]);}
            None => (),
        };

        match &updated_details.size{
            Some(inner) => {
                let size ;
                if (inner.to_string().to_lowercase().eq("large") || inner.to_string().to_lowercase().eq("medium")|| inner.to_string().to_lowercase().eq("small"))
                {
                    size = inner.to_string().to_lowercase();
                }
                else {
                    return Err(ErrorResponse::create_error("Size isn't valid",&"400")) ;
                }
                db_connection.execute("update chore_list set size = $1 where id = $2 ", &[&size.to_string(), &taskId]);
            }
            None => (),
        };


    }
    else if task_type.to_string().to_lowercase().eq(&"homework".to_string()) && updated_details.description.is_none() && updated_details.size.is_none()
    {
        match &updated_details.status{
            Some(inner) => {
                let task_status ;
                if (inner.to_string().to_lowercase().eq("active") || inner.to_string().to_lowercase().eq("done")){
                    task_status = inner.to_string().to_lowercase();
                }
                else {
                    error_response.set_sized_body(Cursor::new("status isn't valid".to_string()));
                    return Err(ErrorResponse::create_error("status isn't valid",&"400")) ;
                }
                let mut ownerId_result = match get_homrwork_by_homework_id(&taskId)
                {
                    Ok(inner) => inner,
                    Err(_) => return Err(ErrorResponse::create_error("can't get chore",&"400")) ,
                };
                let task_original_status = &ownerId_result.homeworks.get(0).unwrap().status;
                db_connection.execute("update task_to_type set status = $1 where id = $2 ", &[&task_status.to_string(), &taskId]);
                db_connection.execute("update homework_list set status = $1 where id = $2 ", &[&task_status.to_string(), &taskId]);
                let ownerId_string = &ownerId_result.homeworks.get(0).unwrap().ownerId;
                println!("{}", inner.to_string().to_lowercase());
                println!("{}", task_original_status.to_lowercase().to_string());
                println!("{}", ownerId_string.to_lowercase().to_string());
                if(inner.to_string().to_lowercase().eq("active") && task_original_status.to_lowercase().to_string().eq("done")){
                    println!("should print here{}", ownerId_string.to_lowercase().to_string());
                    db_connection.execute("update person_details set activeTaskCount = activeTaskCount + 1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
                }
                if(inner.to_string().to_lowercase().eq("done") && task_original_status.to_lowercase().to_string().eq("active")){
                    db_connection.execute("update person_details set activeTaskCount = activeTaskCount -1 where person_details.id = $1 ", &[ownerId_string.to_string()]); 
                }
            }
            None => (),
        };

        match &updated_details.details{
            Some(inner) => {db_connection.execute("update homework_list set description = $1 where id = $2 ", &[&inner.to_string(), &taskId]);}
            None => (),
        };

        match &updated_details.dueDate{
            Some(inner) => {db_connection.execute("update homework_list set dueDate = $1 where id = $2 ", &[&inner.to_string(), &taskId]);}
            None => (),
        };

        
        match &updated_details.course{
            Some(inner) => {db_connection.execute("update homework_list set course = $1 where id = $2 ", &[&inner.to_string(), &taskId]);}
            None => (),
        };
    }
    else {
    let msg = format!("you tried to patch a {} with unsupported fields",task_type.to_string().to_lowercase());        
    return Err(ErrorResponse::create_error(&msg,&"404"));} ;

    let chores = get_chore_by_chore_id(&taskId);

    let homeworks = get_homrwork_by_homework_id(&taskId);


    let clean_chores = match chores {
        Ok(inner) =>  inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong1",&"400")),
    };

    let clean_homeworks = match homeworks {
        Ok(inner) => inner,
        Err(_) =>return Err(ErrorResponse::create_error("something went horribly wrong2",&"400")),
    };

    return Ok(Json(TaskListImporved
        {
            chore: clean_chores ,
            homeWork: clean_homeworks,
}))

    }



