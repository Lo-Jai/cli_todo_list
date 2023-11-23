pub mod initialization_functions {
    use sqlx::{SqliteConnection, Sqlite, Executor};
    use clap::{Arg, ArgMatches, Command, command};
    
    // This function contains the command structure for the user facing portion of the tool.
    pub fn setup_command_structure() -> ArgMatches {
        command!()
            .about("This is a command line todo list utility")
            .subcommand(
                Command::new("add")
                .about("Add a ToDo Element to the database.")
                .arg(
                    Arg::new("todo-element")
                    .required(true)
                    )
                .arg(
                    Arg::new("list-name")
                    .long("ln")
                    )
                )
            .subcommand(
                Command::new("add-list")
                .visible_alias("addl")
                .about("Add a ToDo List to the database.")
                .arg(
                    Arg::new("list-name")
                    .required(true)
                    )
                )
            .subcommand(
                Command::new("show")
                .about("Show the elements associatied with a particular ToDo List.")
                .arg(
                    Arg::new("todo-list")
                    )
                .arg(
                    Arg::new("todo-list-id")
                    )
                .arg(
                    Arg::new("list-name")
                    .long("ln")
                    )
                )
            .subcommand(
                Command::new("remove")
                .about("Remove ToDo Elements from the database by index.")
                .arg(
                    Arg::new("todo-element-id")
                    .required(true)
                    )
                )
            .subcommand(
                Command::new("status-complete")
                .about("Toggle the completion status of a ToDo Element to complete.")
                .visible_alias("mark")
                .arg(
                    Arg::new("todo-element-id")
                    .required(true)
                    )
                )
            .subcommand(
                Command::new("status-incomplete")
                .about("Toggle the completion status of a ToDo Element to incomplete.")
                .visible_alias("unmark")
                .arg(
                    Arg::new("todo-element-id")
                    .required(true)
                    )
                )
            .get_matches()
    }

    // This function establishes the structure of the database representing todo lists and todo
    // elements in the case it does not already exist.
    pub async fn database_schema(database_connection: &mut SqliteConnection) {
        database_connection.execute("
                                    CREATE TABLE IF NOT EXISTS todo_lists (
                                        id INTEGER PRIMARY KEY,
                                        list_name TEXT,
                                        is_primary BOOL
                                        )
                                    ").await.unwrap();

        database_connection.execute("
                                    CREATE TABLE IF NOT EXISTS todo_elements (
                                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                                        list_id INTEGER,
                                        task TEXT,
                                        status INTEGER
                                        )
                                    ").await.unwrap();

        // Insert initial primary table into the database.
        let _ = sqlx::query::<Sqlite>("
                                      INSERT INTO todo_lists (list_name, is_primary)
                                      VALUES ($1, $2)
                                      ")
            .bind("Primary")
            .bind(true)
            .execute(database_connection).await.unwrap();
    }
}
