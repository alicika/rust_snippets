mod error;

fn main() {
    let years = 20;
    let ret_ratio = 0.03;
    let div_ratio = 0.015;

    let domestic_tax_ratio = 0.20;
    let foreign_tax_ratio = 0.1;

    let start_value = 100_000_000.0;
    let etf_after_tax_value = {
        let expense_ratio = 0.00_03;
        let mut cost = start_value;
        let mut value = start_value;
        for _ in 0..years {
            let ret = value * ret_ratio;
            let expense = value * expense_ratio;
            let div = value * div_ratio;
            let tax = div * domestic_tax_ratio;
            let reinvestment = div - tax;
            value += ret - expense + reinvestment;
            cost += reinvestment;
        }

        println!("ETF:");
        println!("value:  {:>9.0}", value);
        println!("cost:   {:>9.0}", cost);
        let tax = if value > cost {
            (value - cost) * domestic_tax_ratio
        } else {
            0.0
        };
        println!("tax:    {:>9.0}", tax);
        let after_tax_value = value - tax;
        println!("after_tax_value: {:>9.0}", after_tax_value);
        after_tax_value
    };

    println!();
    let shintaku_after_tax_value = {
        let expense_ratio = 0.00_16;
        let mut value = start_value;
        for _ in 0..years {
            let ret = value * ret_ratio;
            let expense = value * expense_ratio;
            let div = value * div_ratio * (1.0 - foreign_tax_ratio);
            value += ret - expense + div;
        }

        println!("投資信託:");
        println!("value:  {:>9.0}", value);
        let tax = (value - start_value) * domestic_tax_ratio;
        println!("tax:    {:>9.0}", tax);
        let after_tax_value = value - tax;
        println!("after_tax_value: {:>9.0}", after_tax_value);
        after_tax_value
    };
    println!(
        "差 (ETF - 投資信託): {:.0}",
        etf_after_tax_value - shintaku_after_tax_value
    );
}