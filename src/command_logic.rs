pub mod command_logic {
    use crate::database_structures::database_structures::ToDoElement;

    use clap::ArgMatches;
    use sqlx::SqliteConnection;

    // this function handles the show todo element command.
    pub async fn add(database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        let todo_element = arguments.get_one::<String>("todo-element").unwrap();
        let mut todo_element = ToDoElement::new(todo_element);
        let todo_list_name = arguments.get_one::<&str>("list-name"); 

        todo_element.add_to_database(database_connection, todo_list_name.copied()).await;

    }

    // this function handles the show ToDo List command.
    pub async fn show (database_connection: &mut SqliteConnection, arguments: &ArgMatches) {
        todo!();
    }
}
