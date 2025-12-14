use ollama_rs::generation::tools::{Tool, ToolFunctionInfo, ToolInfo, ToolType};
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct GetWeatherParameters {
    #[schemars(description = "Location to get the weather for")]
    location: String,
}

pub struct GetWeather {
    client: reqwest::Client,
}

impl GetWeather {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn tool_info() -> ToolInfo {
        ToolInfo {
            tool_type: ToolType::Function,
            function: ToolFunctionInfo {
                name: Self::name().to_string(),
                description: Self::description().to_string(),
                parameters: schema_for!(GetWeatherParameters),
            },
        }
    }

    pub async fn call_from_json(&mut self, json: serde_json::Value) -> serde_json::Result<String> {
        let parameters = serde_json::from_value::<GetWeatherParameters>(json)?;
        Ok(self.call(parameters).await.unwrap())
    }

    pub async fn get_weather(
        &self,
        location: String,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        tracing::info!("using tool `get_weather({location})`");
        match self
            .client
            .get(format!("https://wttr.in/{location}?lang=ru&format=%C+%t"))
            .send()
            .await
        {
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
}

impl Tool for GetWeather {
    type Params = GetWeatherParameters;

    fn name() -> &'static str {
        "get_weather"
    }

    fn description() -> &'static str {
        "Retrieves current weather for the given location."
    }

    async fn call(
        &mut self,
        parameters: Self::Params,
    ) -> ollama_rs::generation::tools::Result<String> {
        self.get_weather(parameters.location).await
    }
}
