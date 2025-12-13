/// Retrieves current weather for the given location.
/// * location - Location to get the weather for.
#[ollama_rs::function]
pub async fn get_weather(
    location: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    tracing::info!("using tool `get_weather({location})`");
    match reqwest::get(format!("https://wttr.in/{location}?lang=ru&format=%C+%t")).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                tracing::info!("current weather in `{location}`: {text}");
                Ok(text)
            }
            Err(e) => {
                tracing::error!("failed to get body: {e}");
                Ok("failed to get body".to_string())
            }
        },
        Err(e) => {
            tracing::error!("failed to get response: {e}");
            Ok("failed to get response".to_string())
        }
    }
}
