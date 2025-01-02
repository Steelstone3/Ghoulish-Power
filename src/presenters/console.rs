use inquire::{Confirm, Text};
use mockall::automock;

pub struct Console;

impl Presenter for Console {
    fn print(&self, message: &str) {
        println!("{message}");
    }

    fn text_prompt(&self, message: &str, help_prompt: &str, default_value: &str) -> String {
        match Text::new(message)
            .with_help_message(help_prompt)
            .with_default(default_value)
            .prompt()
        {
            Ok(text) => text,
            Err(_) => "".to_string(),
        }
    }

    fn confirmation(&self, message: &str) -> bool {
        Confirm::new(message)
            .with_default(false)
            .prompt()
            .unwrap_or_default()
    }

    fn parse_numeric_value(&self, input: String) -> u32 {
        match input.chars().find(|character| character.is_numeric()) {
            Some(_) => input.as_str().trim().parse::<u32>().unwrap_or_default(),
            None => 0,
        }
    }
}

#[allow(dead_code)]
#[automock]
pub trait Presenter {
    fn print(&self, message: &str);
    fn text_prompt(&self, message: &str, help_prompt: &str, default_value: &str) -> String;
    fn confirmation(&self, message: &str) -> bool;
    fn parse_numeric_value(&self, input: String) -> u32;
}
