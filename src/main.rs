use ollama_rs::{
    coordinator::Coordinator,
    generation::{chat::ChatMessage, tools::Tool},
    models::ModelOptions,
    Ollama,
};
use schemars::JsonSchema;
use serde::Deserialize;

/// A counter tool
struct Counter(u32);

impl Tool for Counter {
    type Params = CounterParams;

    fn name() -> &'static str {
        "counter"
    }

    fn description() -> &'static str {
        "A counter which can be incremented. The current state of the counter is returned."
    }

    async fn call(
        &mut self,
        parameters: Self::Params,
    ) -> ollama_rs::generation::tools::Result<String> {
        let increment: u32 = parameters.increment.parse().unwrap_or_default();
        self.0 += increment;
        println!("Counter status: {}", self.0);
        Ok(self.0.to_string())
    }
}

/// Parameters for calling the counter tool
#[derive(Deserialize, JsonSchema)]
pub struct CounterParams {
    #[schemars(
        description = "The amount by which to increment the counter, if any. For example, '1'. An empty string can be given to not increment the counter"
    )]
    increment: String,
}

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let history = vec![];

    let counter = Counter(0);
    let mut coordinator = Coordinator::new(ollama, "llama3.2:1b".to_string(), history)
        .options(ModelOptions::default().num_ctx(16384))
        .add_tool(counter);

    loop {
        print!(">");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match coordinator
            .chat(vec![ChatMessage::user(input.trim().to_string())])
            .await
        {
            Ok(resp) => {
                println!("{}", resp.message.content);
            }
            Err(err) => {
                println!("Error: {err:?}");
            }
        }
    }
}
