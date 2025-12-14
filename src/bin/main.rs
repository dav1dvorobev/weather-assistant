use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
};
use std::io::Write;
use weather_assistant::tools::GetWeather;

fn input() -> String {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    s.trim().to_string()
}

#[tokio::main]
async fn main() -> ollama_rs::error::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let model = std::env::var("OLLAMA_MODEL")
        .inspect_err(|_| tracing::error!("missing environment variable \"OLLAMA_MODEL\""))
        .unwrap();
    let ollama = Ollama::default();
    let mut stdout = std::io::stdout();
    let mut get_weather = GetWeather::new();
    loop {
        print!("👨 >> ");
        stdout.flush().unwrap();
        let response = ollama
            .send_chat_messages(
                ChatMessageRequest::new(model.clone(), vec![ChatMessage::user(input())])
                    .tools(vec![GetWeather::tool_info()]),
            )
            .await?;
        if !response.message.tool_calls.is_empty() {
            for call in response.message.tool_calls {
                match call.function.name.as_str() {
                    "get_weather" => {
                        println!(
                            "🤖 >> {}",
                            get_weather
                                .call_from_json(call.function.arguments)
                                .await
                                .unwrap()
                        );
                    }
                    _ => {}
                }
            }
        } else {
            println!("🤖 >> {}", response.message.content);
        }
    }
}
