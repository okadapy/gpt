use openai_api_rust::*;
use openai_api_rust::chat::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let auth = Auth::new("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");
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: None,
        temperature: None,
        top_p: None,
        n: None,
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: Some("OkadaPy".to_string()),
        messages: vec![
            Message {role: Role::System, content: "Ты - программист, помоги своему товарищу ответить на интересующие его вопросы".to_string()},
            Message {role: Role::User, content: args.join(" ")}
        ]
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    println!("Answer:\t{}", message.content);
}