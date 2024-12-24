use inquire::{Confirm, Text};

#[allow(dead_code)]
pub struct Console;

impl Presenter for Console {
    #[allow(dead_code)]
    fn text_prompt(message: &str, help_prompt: &str, default_value: &str) -> String {
        Text::new(message)
            .with_help_message(help_prompt)
            .with_default(default_value)
            .prompt()
            .unwrap()
    }

    #[allow(dead_code)]
    fn confirmation(message: &str) -> bool {
        Confirm::new(message)
            .with_default(false)
            .prompt()
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    fn parse_numeric_value(input: String) -> u32 {
        match input.chars().find(|character| character.is_numeric()) {
            Some(_) => input.as_str().trim().parse::<u32>().unwrap(),
            None => panic!("Not a numeric value"),
        }
    }
}

#[allow(dead_code)]
pub trait Presenter {
    fn text_prompt(message: &str, help_prompt: &str, default_value: &str) -> String;
    fn confirmation(message: &str) -> bool;
    fn parse_numeric_value(input: String) -> u32;
}
