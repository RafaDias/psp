const CARD_DIGITS_TO_SAVE:usize = 4;
const DEFAULT_FEE_FOR_DEBIT: f32 = 3.0;
const DEFAULT_FEE_FOR_CREDIT: f32 = 5.0;
const DEFAULT_DAYS_FOR_CREDIT_PAYABLE: u64 = 30;

struct Transaction {
    value: f32,
    description: String,
    method: PaymentMethod,
    card: Card,
}
#[derive(PartialEq, Debug)]
enum PayableStatus {
    Paid,
    WaitingFunds,
}

struct Payable {
    status:PayableStatus,
    tx: Transaction,
    date: String,
    fee: f32,
}

impl Payable {
    fn new(status: PayableStatus, tx: Transaction, date: String) -> Self {
        Payable {
            status,
            tx,
            date,
            fee: 0.0,
        }
    }

    fn calculate_fee(self) -> f32 {
        let fee = {
            match self.tx.method {
                PaymentMethod::Debit => DEFAULT_FEE_FOR_DEBIT,
                PaymentMethod::Credit => DEFAULT_FEE_FOR_CREDIT,
            }
        };
        self.tx.value * (fee / 100.0)
    }
}


impl Transaction {
    fn new(value: f32, description: String, method: PaymentMethod, card: Card) -> Self {
        Transaction {
            value,
            description,
            method,
            card
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Card {
    number: String,
    holder: String,
    expires_at: String,
    cvv: String,
}

impl Card {
    fn new(number: String, holder: String, expires_at: String, cvv: String) -> Self {
        let last_four_digits = number[number.len() - CARD_DIGITS_TO_SAVE..].to_string();
        Card {
            number: last_four_digits,
            holder,
            expires_at,
            cvv
        }
    }
}

#[derive(PartialEq, Debug)]
enum PaymentMethod {
    Debit,
    Credit
}

use chrono::{DateTime, Local, Days};

fn make_payable(tx: Transaction) -> Payable {
    let now = Local::now().date_naive();
    
    match tx.method {
        PaymentMethod::Debit => Payable::new(PayableStatus::Paid, tx, now.to_string()),
        PaymentMethod::Credit => {
            let thirty_days_later = now.checked_add_days(Days::new(DEFAULT_DAYS_FOR_CREDIT_PAYABLE));
            Payable::new(PayableStatus::WaitingFunds, tx, thirty_days_later.unwrap().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_card_but_hiding_the_last_four_digits() {
        let number = "12345678".to_owned();
        let holder = "Rafael Dias".to_owned();
        let expires_at = "12/30".to_owned();
        let cvv = "789".to_owned();
        let card = Card::new(number, holder.clone(), expires_at.clone(), cvv.clone());
        assert_eq!(card.number, "5678");
        assert_eq!(card.holder, holder);
        assert_eq!(card.expires_at, expires_at);
        assert_eq!(card.cvv, cvv);
    }

    #[test]
    fn should_create_a_txn() {
        let card = Card::new("12345678".to_owned(), "Rafael Dias".to_owned(), "12/30".to_owned(), "123".to_owned());
        let transaction = Transaction::new(20.50, "A nice description".to_owned(), PaymentMethod::Debit, card.clone());
        assert_eq!(transaction.value, 20.50);
        assert_eq!(transaction.description, "A nice description".to_owned());
        assert_eq!(transaction.method, PaymentMethod::Debit);
        assert_eq!(transaction.value, 20.50);
        assert_eq!(transaction.card, card);
    }

    #[test]
    fn test_make_payable_with_debit() {
        let card = Card::new("12345678".to_owned(), "Rafael Dias".to_owned(), "12/30".to_owned(), "123".to_owned());
        let tx = Transaction::new(100.0, "Test Transaction".to_owned(), PaymentMethod::Debit, card);
        
        let payable = make_payable(tx);
        let today = Local::now().date_naive();

        assert_eq!(payable.status, PayableStatus::Paid);
        assert_eq!(payable.fee, 3.0);
        assert_eq!(payable.date, today.to_string());
    }

    #[test]
    fn test_make_payable_with_credit() {
        let card = Card::new("12345678".to_owned(), "Rafael Dias".to_owned(), "12/30".to_owned(), "123".to_owned());
        let tx = Transaction::new(100.0, "Test Transaction".to_owned(), PaymentMethod::Credit, card);

        let payable = make_payable(tx);
        let thirty_days_later = Local::now().date_naive().checked_add_days(Days::new(DEFAULT_DAYS_FOR_CREDIT_PAYABLE));

        assert_eq!(payable.status, PayableStatus::WaitingFunds);
        assert_eq!(payable.fee, 5.0);
        assert_eq!(payable.date, thirty_days_later.unwrap().to_string());
    }

}