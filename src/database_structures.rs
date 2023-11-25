pub mod database_structures {
    use sqlx::{SqliteConnection, Sqlite, Executor, Row};

    // This enum represents the completion status of todo elements once they have been converted to
    // rust types.
    pub enum Status {
        Complete,
        Incomplete
    }

    impl PartialEq for Status {
       fn eq(&self, other: &Self) -> bool {
           let value1 = match self {
                Status::Incomplete => 0,
                Status::Complete => 1
           };

           let value2 = match other {
                Status::Incomplete => 0,
                Status::Complete => 1
           };

           value2 == value1
       } 
    }

    // Rust type correlating to ToDo Element rows within the database
    pub struct ToDoElement {
        id: Option<i64>,
        list_id: Option<i32>,
        task: String,
        status: Status
    }

    impl ToDoElement {
        pub fn new(task: &str) -> ToDoElement {
            ToDoElement {
                id: None,
                list_id: None,
                task: task.to_string(),
                status: Status::Incomplete
            }
        }

        // Getter functions
        pub fn get_id(&self) -> Option<i64> {
            self.id
        }

        pub fn get_list_id(&self) -> Option<i32> {
            self.list_id
        }

        pub fn get_task(&self) -> &str {
            &self.task
        }

        pub fn get_status(&self) -> &Status {
            &self.status
        }

        // add self to database while ensuring that internal state reflects that of the database
        // addition (update id and list_id fields).
        pub async fn add_to_database(&mut self, database_connection: &mut SqliteConnection, todo_list_name: Option<&String>) {
            let status = match self.status {
                Status::Incomplete => 0,
                Status::Complete => 1
            };

            match todo_list_name {
                Some(list_name) => {
                    let list_query = sqlx::query::<Sqlite>("
                                                           SELECT * FROM todo_lists
                                                           WHERE list_name = $1
                                                           ")
                        .bind(list_name);


                    let list_id: i32 = database_connection.fetch_one(list_query).await.unwrap().get::<i32, usize>(0);
                    
                    // This line serves to add the element into the database and retrieve the
                    // elements associated id.
                    let row_id = sqlx::query::<Sqlite>("
                                                  INSERT INTO todo_elements (list_id, task, status)
                                                  VALUES ($1, $2, $3)
                                                  ")
                        .bind(list_id)
                        .bind(&self.task)
                        .bind(status)
                        .execute(database_connection).await.unwrap().last_insert_rowid();

                    self.id = Some(row_id);
                    self.list_id = Some(list_id);
                }

                None => {
                    let list_query = sqlx::query::<Sqlite>("
                                                           SELECT * FROM todo_lists
                                                           ");


                    let list_id = database_connection.fetch_all(list_query).await.unwrap()[0].get::<i32, usize>(0);

                    let row_id = sqlx::query::<Sqlite>("
                                                  INSERT INTO todo_elements (list_id, task, status)
                                                  VALUES ($1, $2, $3)
                                                  ")
                        .bind(list_id)
                        .bind(&self.task)
                        .bind(status)
                        .execute(database_connection).await.unwrap().last_insert_rowid();
                    
                    self.id = Some(row_id);
                    self.list_id = Some(1);

                }
            }

        }
    }


    // Rust structure which represents a row in the ToDo list database table.
    pub struct ToDoList {
        pub id: Option<i64>,
        pub name: String,
        pub is_primary: bool
    }

    impl ToDoList {
        pub fn new(name: &str) -> ToDoList {
            ToDoList {
                id: None,
                name: name.to_owned(),
                is_primary: false
            }
        }

        // Add ToDoList to the database in the case it is not already represented
        pub async fn add_to_database(&mut self, database_connection: &mut SqliteConnection) {
            match self.id {
                Some(_) => panic!("Attemeted add off ToDo List to database. Element already represents."),
                None => {
                    let row_id = sqlx::query::<Sqlite>("
                                                       INSERT INTO todo_lists (list_name, is_primary)
                                                       VALUES ($1, $2) 
                                                       ")
                        .bind(&self.name)
                        .bind(self.is_primary)
                        .execute(database_connection).await.unwrap().last_insert_rowid();

                    self.id = Some(row_id);
                }
            }
        }


        // Display the todo elements associated with this list for debug purposes.
        pub async fn display_list(&self, database_connection: &mut SqliteConnection) {
            let list_element_results = sqlx::query::<Sqlite>("
                                                             SELECT * FROM todo_elements
                                                             WHERE list_id = $1
                                                             ")
                .bind(self.id.unwrap())
                .fetch_all(database_connection).await.unwrap();

            println!("List Name: {0}\n", self.name);

            for row in list_element_results {
                println!("id: {0}, list_id: {1}, task: {2}, status: {3}\n ", row.get::<i32, usize>(0), row.get::<i32, usize>(1), row.get::<String, usize>(2), row.get::<i32, usize>(3));
            }
        }
    }
}

