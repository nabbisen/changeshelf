// components/search_bar.rs
use crate::app::components::dashboard::card;
use iced::{
    Element,
    widget::{row, scrollable, text},
};

#[derive(Default)]
pub struct Dashboard {
    cards: Vec<card::Card>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CardMsg(usize, card::Message),
}

impl Dashboard {
    pub fn new() -> Self {
        let cards = (1..=5)
            .map(|i| card::Card::new(i, format!("カード #{}", i)))
            .collect();

        Self { cards }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // カードのリストをループで生成
        let card_list = row(self
            .cards
            .iter()
            .map(|card| {
                let id = card.id;
                // 子の view を map して親の Message に変換
                card.view().map(move |msg| Message::CardMsg(id, msg))
            })
            .collect::<Vec<_>>())
        .spacing(20);

        scrollable(
            row![text("Dashboard").size(30), card_list,]
                .spacing(20)
                .padding(20),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::CardMsg(id, card_msg) => {
                match card_msg {
                    card::Message::DeletePressed => {
                        // 特定のIDのカードを削除
                        self.cards.retain(|c| c.id != id);
                    }
                }
            }
        }
    }
}
