use ollama_rs::{
    Ollama, coordinator::Coordinator, generation::chat::ChatMessage, models::ModelOptions,
};
use std::io::Write;
use weather_assistant::tools::get_weather;

const MODEL: &str = "qwen2.5:1.5b-instruct-q8_0";
const SYSTEM: &str = r#"
You are a helpful weather assistant.

Rules you must follow strictly:
1. You can ONLY answer questions about weather and nothing else.
2. Always respond in the same language as the user's question.
3. Respond briefly, using as few words as possible.
4. If the question is NOT about weather -> reply ONLY with this exact phrase and nothing else:
   "Sorry, I can't help with this request."
5. You have access to weather tools. 
   NEVER guess or make up weather information. 
   ALWAYS use the appropriate tool immediately when you need current or forecast weather data. 
   Do not write any answer until you have the real data from the tool.

Never break these rules.
"#;

fn input() -> String {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    s.trim().to_string()
}

#[tokio::main]
async fn main() -> ollama_rs::error::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let ollama = Ollama::default();
    let mut coordinator = Coordinator::new(
        ollama,
        MODEL.to_string(),
        vec![ChatMessage::system(SYSTEM.to_string())],
    )
    .options(
        ModelOptions::default()
            .num_ctx(1024)
            .temperature(0.2)
            .num_predict(64),
    )
    .add_tool(get_weather);
    let mut stdout = std::io::stdout();
    loop {
        print!(">> ");
        stdout.flush().unwrap();
        let response = coordinator.chat(vec![ChatMessage::user(input())]).await?;
        println!("{}", response.message.content);
    }
}
