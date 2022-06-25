
##in order to install rust and run the server please read "README"##

This REST API server was created by Gil Nussbaum for course "data infrastructers and programing language"
In order to to create this API I used sqlite embbeded database with 4 tables:
1) person_details which holds all the information of users
2) homework_list which hold all the information of homework type tasks
3) chore_list which hold all the information of chore type tasks
4) task_to_type which hold basic information of a task.


person_details:
this table table holds the following fields: person ID, person name, person email, person favorite programming language and task count.
in order to have this API available for different uses the only primary keys are id and email and email so people with the same name can sign up and etc.
note, that while email is unique instead of creating it as a unique key we are checking if the email exisit before we insert a new user to the DB while checking for the "@", if it doesn't exisit this is an illeagel parameter(there isn't any limit to endings such as ".com" for the flexability of the API).
in addition, because the uses for this API are flexabile the "name" field isn't limited to specific charcters so nicknames can be used, and different languages names can be accepted.

homework_list:
this table hold the following field: id, type,ownerId, course, details ,dueDate, status
none of these fields can be null and you can't add a new homework without those, in addition once a task of the form of homework has been added you can't patch this task to be of type "chore".

chore_list:
this table hold the following field: id, type,ownerId,description ,size , status
none of these fields can be null and you can't add a new chore without those, in addition once a task of the form of homework has been added you can't patch this task to be of type "homework".

task_to_type:
this table hold the following fields: id, type, ownerId, status
none of these can be null, this table help us locate the right table that a task is kept.

General notes:
1) A task can only be of two types: "chore" or "homework".
2) once a task has been created you can't change it's type.
3) size has only three options: "small", "medium" or "large".
4) status can obly be one of two options: "done" or "active".

----------------------------------------------------------------------------------------------------------------------------------------
Rust is a low level data type language and such it doesn't use any OOP concepts such as inhertiance and abstract classes 
instead we work with "struct" in order to define how the insert and responses will behave.

----------------------------------------------------------------------------------------------------------------------------------------
the framework for this project is called "Rocket.rs" which as a typesafe boilerplate free framework.
the "response" struct is our handler for all the server responses and we used this struct while customizing it for each api call
so the response codes, headers and body would accumelate the succsess and errors of an a API call
for further reading please look at https://rocket.rs/


