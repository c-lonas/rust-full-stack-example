pub struct ActiveCard {
    pub card: Option<CardType>,
}

#[derive(Clone, PartialEq)]
pub enum CardType {
    AddIncomeForm,
    // You can add more card types here
}

impl ActiveCard {
    pub fn new() -> Self {
        Self { card: None }
    }
    
    // pub fn set_card(&mut self, card: Option<CardType>) {
    // self.card = card;
    // }

}

