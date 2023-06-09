use iced::widget::{Button, Column, Text};
use iced::{window, Application, Command, Element, Settings, Theme};
use rand::Rng;

// Keygen
fn oem_first_segment() -> String {
    let years = ["95", "96", "97", "98", "99", "00", "01", "02", "03"];
    let three_digits = rand::thread_rng().gen_range(0..=366);
    let two_digits = rand::thread_rng().gen_range(0..years.len());
    format!("{}{}", three_digits, years[two_digits])
}

fn oem_second_segment() -> (String, u32) {
    let mut middle_digits = rand::thread_rng().gen_range(0..=99999);
    let mut last_digit = rand::thread_rng().gen_range(1..=7);
    while last_digit == 0 || last_digit >= 8 {
        last_digit = rand::thread_rng().gen_range(1..=7);
    }

    let second_segment = format!("{:07}", middle_digits * 10 + last_digit);

    let sum: u32 = second_segment
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    (second_segment, sum)
}

fn check_second_digit() -> String {
    let (mut seven_digits, mut sum) = oem_second_segment();
    while sum % 7 != 0 {
        let result = oem_second_segment();
        seven_digits = result.0;
        sum = result.1;
    }
    seven_digits
}

fn oem_third_segment() -> String {
    let third_segment = rand::thread_rng().gen_range(0..=99999);
    format!("{:05}", third_segment)
}

// GUI
pub fn main() -> iced::Result {
    KeyGen::run(Settings {
        window: window::Settings {
            size: (400, 100),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct KeyGen {
    generated_key: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GenerateButtonPressed,
}

impl Application for KeyGen {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn title(&self) -> String {
        String::from("Windows 95 Keygen")
    }

    fn new(_flags: ()) -> (KeyGen, Command<Message>) {
        (KeyGen::default(), Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GenerateButtonPressed => {
                let first_segment = oem_first_segment();
                let second_segment = check_second_digit();
                let third_segment = oem_third_segment();

                self.generated_key =
                    format!("{}-OEM-{}-{}", first_segment, second_segment, third_segment);

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let generate_button = Button::new(Text::new("Generate").size(30))
            .on_press(Message::GenerateButtonPressed)
            .style(Default::default())
            .padding(8);

        let generated_key_text = Text::new(&self.generated_key).size(30);

        let content = Column::new()
            .spacing(15)
            .push(generated_key_text)
            .push(generate_button);

        content.into()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oem_first_segment() {
        let first_segment = oem_first_segment();
        assert_eq!(first_segment.len(), 5);
    }

    #[test]
    fn test_oem_second_segment() {
        let (second_segment, sum) = oem_second_segment();
        assert_eq!(second_segment.len(), 7);
        assert_eq!(second_segment.parse::<u32>().unwrap() % 7, 0);
        assert_eq!(
            sum,
            second_segment
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum()
        );
    }

    #[test]
    fn test_check_second_digit() {
        let seven_digits = check_second_digit();
        let sum: u32 = seven_digits.chars().map(|c| c.to_digit(10).unwrap()).sum();
        assert_eq!(sum % 7, 0);
    }

    #[test]
    fn test_oem_third_segment() {
        let third_segment = oem_third_segment();
        assert_eq!(third_segment.len(), 5);
    }
}
