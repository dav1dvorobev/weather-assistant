use clap::Parser;
use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
    models::ModelOptions,
};
use weather_assistant::tools::GetWeather;

// Simply weather assistant
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Ollama model name (must support tool calling)
    #[arg(short, long)]
    model_name: String,
}

const SYSTEM: &str = r#"
You are helpful weather assistant.

Rules you must follow strictly:
1. You can ONLY answer questions about weather and nothing else.
2. Always respond in the same language as the user's question.
3. Respond briefly, using as few words as possible.
4. If the question is NOT about weather -> reply Sorry, I can't help with this request.
5. You have access to weather tools. 
   NEVER guess or make up weather information. 
   ALWAYS use the appropriate tool immediately when you need current or forecast weather data. 
   Do not write any answer until you have the real data from the tool.

Never break these rules.
"#;

macro_rules! input {
    () => {
        input!("")
    };
    ($prompt:expr) => {{
        use std::io::{self, Write};
        if !$prompt.is_empty() {
            print!("{}", $prompt);
            let _ = io::stdout().flush();
        }
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    }};
}

#[tokio::main]
async fn main() -> ollama_rs::error::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let ollama = Ollama::default();
    let mut get_weather = GetWeather::new();
    loop {
        let response = ollama
            .send_chat_messages(
                ChatMessageRequest::new(
                    args.model_name.clone(),
                    vec![
                        ChatMessage::system(SYSTEM.to_string()),
                        ChatMessage::user(input!(">> ")),
                    ],
                )
                .options(
                    ModelOptions::default()
                        .num_ctx(2048)
                        .temperature(0.2)
                        .num_predict(256),
                )
                .tools(vec![GetWeather::tool_info()]),
            )
            .await?;
        if !response.message.tool_calls.is_empty() {
            for call in response.message.tool_calls {
                match call.function.name.as_str() {
                    "get_weather" => {
                        println!(
                            "🤖 \x1b[1;94m{}\x1b[0m",
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
            println!("🤖 \x1b[1;94m{}\x1b[0m", response.message.content);
        }
    }
}
