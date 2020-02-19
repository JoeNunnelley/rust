use std::io;

// This will amortize a value
fn main() {
    println!("Let's Amortize!!");
    println!("Loan Amount:");
    let mut loan_amount = String::new();
    io::stdin().read_line(&mut loan_amount).expect("Failed to read input");
    let _loan_amount: f64 = loan_amount.trim().parse().expect("Not a number");

    println!("Interest Rate:");
    let mut interest = String::new();
    io::stdin().read_line(&mut interest).expect("Failed to read input");
    let _interest: f64 = interest.trim().parse().expect("Not a number");

    println!("Interest Term (Years):");
    let mut loan_term = String::new();
    io::stdin().read_line(&mut loan_term).expect("Failed to read input");
    let _loan_term: i32 = loan_term.trim().parse().expect("Not a number");

    println!("Payments Per Year:");
    let mut yearly_payments = String::new();
    io::stdin().read_line(&mut yearly_payments).expect("Failed to read input");
    let _yearly_payments: i32 = yearly_payments.trim().parse().expect("Not a number");

    amortize(_loan_amount, _interest, _loan_term, _yearly_payments);
}

fn amortize(loan_amount: f64, interest: f64, loan_term: i32, yearly_payments: i32) {
    println!("Loan Amount: {}", loan_amount);
    println!("Interest Rate: {}", interest);
    println!("Loan Term: {} (years)", loan_term);
    println!("Payments/Year: {}", yearly_payments);

    let perintrate = interest / yearly_payments as f64;
    let totpay = loan_term as f64 * yearly_payments as f64;
    let nom = perintrate * (1.0+ perintrate).powf(totpay);
    let den = (1.0 + perintrate).powf(totpay) - 1.0;
    let payment_amount = loan_amount * ( nom / den);

    println!("Payment: ${:.2}", payment_amount);
    println!("Total Paid: ${:.2}", payment_amount * totpay);
    println!("Total Interest: ${:.2}", (payment_amount * totpay) - loan_amount);
    println!("All Done!");
}

