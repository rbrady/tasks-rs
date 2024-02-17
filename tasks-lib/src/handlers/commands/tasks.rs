use crate::commands::CreateTaskCommand;
use crate::models::Task;
use crate::adapters::create_task_from_command;
use crate::adapters::create_task_dynamodb_adapter;
use std::error::Error;


pub async fn handle_create_task_command(command: CreateTaskCommand) -> Result<Task, Box<dyn Error>> {
    // Create a Task struct instance from the CreateTaskCommand
    let task_obj = create_task_from_command(command);

    // Attempt to add the task to DynamoDB
    match create_task_dynamodb_adapter(task_obj).await {
        Ok(_) => {
            println!("Task successfully added to DynamoDB");
            // Return the created Task instance on success
            Ok(task_obj)
        },
        Err(err) => {
            // Return the error if it occurs
            eprintln!("Error adding task to DynamoDB: {:?}", err);
            Err(err.into())
        },
    }
}