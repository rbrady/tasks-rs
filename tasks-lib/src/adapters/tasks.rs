use crate::commands::CreateTaskCommand;
use crate::models::Task;


pub fn create_task_from_command(command: CreateTaskCommand) -> Task {
    // Construct a Task instance from the fields of CreateTaskCommand
    let task = Task {
        title: command.title,
        task_url: command.task_url,
        task_url_type: command.task_url_type,
        task_url_web: command.task_url_web,
        callback_url: command.callback_url,
        assigner: command.assigner,
        description: command.description,
        due_date: command.due_date,
        expiration_date: command.expiration_date,
        show_due_date: command.show_due_date,
        past_due_duration_days: command.past_due_duration_days,
        created_at: command.created_at,
        completed_at: command.completed_at,
        owner_app: command.owner_app,
        extra: command.extra,
        date_available: command.date_available,
    };
    task
}

pub async fn create_task_dynamodb_adapter(task: Task) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    // Create a DynamoDB client
    let client = DynamoDbClient::new(Region::default());

    // Create a PutItemInput instance
    let input = PutItemInput {
    table_name: "YourTableName".to_string(), // Replace "YourTableName" with your actual table name
    item: serde_dynamodb::to_hashmap(&task)?, // Convert Task struct to HashMap
    ..Default::default()
    };

    // Call the put_item method
    client.put_item(input).await.map(|_| ())
}