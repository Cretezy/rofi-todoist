use serde_json::Value;

static TODOIST_API_URL: &str = "https://api.todoist.com/sync/v9/quick/add";

pub fn create_task(task: String, todoist_api_token: String) -> Result<String, reqwest::Error> {
    let payload = json!({
        "text": task
    });

    let client = reqwest::blocking::Client::new();
    let mut response = client
        .post(TODOIST_API_URL)
        .header("Authorization", format!("Bearer {todoist_api_token}"))
        .form(&payload)
        .send()?;

    response = response.error_for_status()?;

    response.json::<Value>().map(|json| {
        String::from(
            json["content"]
                .as_str()
                .expect("Response missing `content` key"),
        )
    })
}
