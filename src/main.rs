pub mod initialization_functions;
pub mod database_structures;
pub mod command_logic;

use initialization_functions::initialization_functions::*;
use command_logic::command_logic::*;

use sqlx::{self, SqliteConnection, Connection};

#[tokio::main]
async fn main() -> () {
    // Establish connection to Sqlite database, initialize vector of valid subcommands, set up CLI
    // command structure and database structure if database is not initialized.
    let mut connection: SqliteConnection = SqliteConnection::connect(&std::env::var("TODO_DATABASE_LOCATION").unwrap()).await.unwrap();
    let subcommands: Vec<&str> = vec!["add","add-list", "show", "remove", "status-complete", "status-incomplete"];
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

        "add-list" => {
            add_list(&mut connection, argument_matches).await;
        }

        "show" => {
            show(&mut connection, argument_matches).await;
        }

        "remove" => {
            remove(&mut connection, argument_matches).await;
        }

        "status-complete" => {
            mark_complete(&mut connection, argument_matches).await;
        }

        "status-incomplete" => {
            mark_incomplete(&mut connection, argument_matches).await;
        }

        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use database_structures::database_structures::{ToDoElement, Status};
    use initialization_functions::initialization_functions::database_schema;
    use command_logic;
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

    #[tokio::test]
    async fn test_remove_command() {
        //todo!();
    }


    async fn create_temp_db() -> SqliteConnection {
        let mut temp_db = SqliteConnection::connect("sqlite::memory:").await.unwrap();

        database_schema(&mut temp_db).await;

        temp_db
    }
}
