use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub title: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    DeletePressed, // 削除ボタンが押された
}

impl Card {
    pub fn new(id: usize, title: String) -> Self {
        Self { id, title }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(&self.title).size(20),
                button("削除").on_press(Message::DeletePressed)
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .padding(20)
        .width(Length::Fixed(150.0))
        .style(container::rounded_box) // 0.14のスタイル指定
        .into()
    }
}
