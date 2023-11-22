pub mod command_logic {
    use std::str::FromStr;
    use crate::database_structures::database_structures::ToDoElement;

    use clap::ArgMatches;
    use sqlx::{SqliteConnection, Row, Sqlite};

    // this function handles the show todo element command.
    pub async fn add(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element = arguments.get_one::<String>("todo-element").unwrap();
        let mut todo_element = ToDoElement::new(todo_element);
        let todo_list_name = arguments.get_one::<&str>("list-name"); 

        todo_element.add_to_database(database_connection, todo_list_name.copied()).await;

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
    pub async fn show (database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let table_contents_query = sqlx::query("
                                              SELECT * FROM todo_elements
                                              WHERE list_id = 1
                                              ");

        let query_results = table_contents_query.fetch_all(database_connection).await.unwrap();

        println!("List Name: Primary Test List\n");

        let mut index = 1;
        for row in query_results {
            println!("({0}) {1}. {2}.", row.get::<i32, usize>(0), index, row.get::<String, usize>(2));
            index += 1;
        }

    }
}
