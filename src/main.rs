pub mod initialization_functions;
pub mod database_structures;
pub mod command_logic;

use initialization_functions::initialization_functions::*;
use command_logic::command_logic::*;

use database_structures::database_structures::ToDoList;
use sqlx::{self, SqliteConnection, Connection};

#[tokio::main]
async fn main() -> () {
    // Establish connection to Sqlite database, initialize vector of valid subcommands, set up CLI
    // command structure and database structure if database is not initialized.
    let mut connection: SqliteConnection = SqliteConnection::connect("todo.db").await.unwrap();
    let subcommands: Vec<&str> = vec!["add"];
    let match_results = setup_command_structure();
    database_schema(&mut connection).await;

    // Check subcommands vector against user input subcommands.
    let subcommand = subcommands.iter().find(|command| {
        match match_results.subcommand_matches(command) {
            Some(_) => {
                true
            }

            None => {
                false 
            }
        }
    }).unwrap();

    // Get parsed arguments that are applicable to the parsed subcommand.
    let argument_matches = match_results.subcommand_matches(subcommand).unwrap();
    match *subcommand {
        "add" => {
            add(&mut connection, argument_matches).await;
        }

        _ => {}
    }


    let primary_list_test = ToDoList {
        id: Some(1),
        name: "test".to_owned(),
        is_primary: true
    };

    primary_list_test.display_list(&mut connection).await;
}


#[cfg(test)]
mod tests {
    use database_structures::database_structures::{ToDoElement, Status};
    use initialization_functions::initialization_functions::database_schema;
    use sqlx::{SqliteConnection, Row};
    use super::*;

    #[tokio::test]
    async fn test_todo_element() {
        let mut test_element: ToDoElement = ToDoElement::new("test_todo");
        let mut temp_db = create_temp_db().await;

        assert!(test_element.get_task() == "test_todo");
        assert!(test_element.get_id() == None);
        assert!(test_element.get_status() == &Status::Incomplete);
        assert!(test_element.get_list_id() == None);

        test_element.add_to_database(&mut temp_db, None).await;

        let test_query_result = sqlx::query("
                                            SELECT * FROM todo_elements
                                            WHERE id = 1
                                            ").fetch_one(&mut temp_db).await.unwrap();

        assert!(test_query_result.get::<i32, usize>(0) == 1);
        assert!(test_query_result.get::<i32, usize>(1) == 1);
        assert!(test_query_result.get::<String, usize>(2) == "test_todo");
        assert!(test_query_result.get::<i32, usize>(3) == 0);

        assert!(test_element.get_id().unwrap() == 1);
        assert!(test_element.get_list_id().unwrap() == 1);
    }


    async fn create_temp_db() -> SqliteConnection {
        let mut temp_db = SqliteConnection::connect("sqlite::memory:").await.unwrap();

        database_schema(&mut temp_db).await;

        temp_db
    }
}
