pub mod command_logic {
    use std::{str::FromStr};
    use crate::database_structures::database_structures::{ToDoElement, ToDoList};

    use clap::ArgMatches;
    use sqlx::{SqliteConnection, Row, Sqlite, Connection};

    // this function handles the show todo element command.
    pub async fn add(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element = arguments.get_one::<String>("todo-element").unwrap();
        let mut todo_element = ToDoElement::new(todo_element);
        let todo_list_name = arguments.get_one::<String>("list-name").unwrap(); 

        todo_element.add_to_database(database_connection, Some(todo_list_name)).await;

    }

    pub async fn add_list(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_list_name = arguments.get_one::<String>("list-name").unwrap();
        let mut todo_list_element = ToDoList::new(&todo_list_name);

        todo_list_element.add_to_database(database_connection).await;
    }

    pub async fn remove(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element_id: i32 = FromStr::from_str(arguments.get_one::<String>("todo-element-id").unwrap()).unwrap();
        let remove_statement = sqlx::query::<Sqlite>("
                                           DELETE FROM todo_elements
                                           WHERE id = $1
                                           ")
            .bind(todo_element_id);

        remove_statement.execute(database_connection).await.unwrap();
    }

    // this function handles the show ToDo List command.
    pub async fn show(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let specific_list_declared: Option<&String> = arguments.get_one::<String>("list-name");

        match specific_list_declared {
            Some(list_name) => {
                let list_id = sqlx::query("
                                          SELECT * FROM todo_lists
                                          WHERE list_name = $1
                                          ")
                    .bind(list_name).fetch_one(database_connection).await.unwrap()
                    .get::<i32, usize>(0);

                let mut new_db_connection = SqliteConnection::connect(&std::env::var("TODO_DATABASE_LOCATION").unwrap()).await.unwrap();

                let table_contents_query = sqlx::query("
                                                       SELECT * FROM todo_elements
                                                       WHERE list_id = $1
                                                       ")
                    .bind(list_id);

                let query_results = table_contents_query.fetch_all(&mut new_db_connection).await.unwrap();

                println!("List Name: {}\n", list_name);

                let unicode_checkmark = 0x2713;
                let complete_status_string = format!("[{}]", std::char::from_u32(unicode_checkmark).unwrap());
                let incomplete_status_string = "[ ]".to_string();
                let mut index = 1;
                for row in query_results {
                    let status_string = match row.get::<i32, usize>(3) {
                        0 => &incomplete_status_string,
                        1 => &complete_status_string,
                        _ => "error"
                    };

                    println!("({0}) {1}. {2}. {3}", row.get::<i32, usize>(0), index, row.get::<String, usize>(2), status_string);
                    index += 1;
                }
            }

            None => {
                let table_contents_query = sqlx::query("
                                                       SELECT * FROM todo_elements
                                                       WHERE list_id = 1
                                                       ");

                let query_results = table_contents_query.fetch_all(database_connection).await.unwrap();

                println!("List Name: Primary\n");

                let unicode_checkmark = 0x2713;
                let complete_status_string = format!("[{}]", std::char::from_u32(unicode_checkmark).unwrap());
                let incomplete_status_string = "[ ]".to_string();
                let mut index = 1;
                for row in query_results {
                    let status_string = match row.get::<i32, usize>(3) {
                        0 => &incomplete_status_string,
                        1 => &complete_status_string,
                        _ => "error"
                    };

                    println!("({0}) {1}. {2}. {3}", row.get::<i32, usize>(0), index, row.get::<String, usize>(2), status_string);
                    index += 1;
                }
            }
        }


        

    }

    pub async fn mark_complete(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element_id: i32 = FromStr::from_str(arguments.get_one::<String>("todo-element-id").unwrap()).unwrap();

        let update_statement = sqlx::query::<Sqlite>("
                                                     UPDATE todo_elements
                                                     SET status = 1
                                                     WHERE id = $1
                                                     ")
            .bind(todo_element_id);

        update_statement.execute(database_connection).await.unwrap();
    }

    pub async fn mark_incomplete(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element_id: i32 = FromStr::from_str(arguments.get_one::<String>("todo-element-id").unwrap()).unwrap();

        let update_statement = sqlx::query::<Sqlite>("
                                                    UPDATE todo_elements
                                                    SET status = 0
                                                    WHERE id = $1
                                                     ")
            .bind(todo_element_id);

        update_statement.execute(database_connection).await.unwrap();
    }
}
