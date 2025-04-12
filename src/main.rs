const PA_TAPER_THRESHOLD: u64 = 100_000 * 100;
const PA_CUTOFF: u64 = 125_140 * 100;
const PA_ALLOWANCE: u64 = 12_570 * 100;

fn main() {
    // salary in pennis to avoid floating point arithmetic issues
    let salary: u64 = 5000000;
    println!("Salary: {}", format_pennies(salary));

    let personal_allowance = calculate_personal_allowance(salary);
    println!(
        "Personal Allowance is {}",
        format_pennies(personal_allowance)
    );
}

fn calculate_personal_allowance(salary: u64) -> u64 {
    if salary < PA_TAPER_THRESHOLD {
        PA_ALLOWANCE
    } else if salary >= PA_CUTOFF {
        0
    } else {
        let excess = salary - PA_TAPER_THRESHOLD;
        let reduction = (excess / 200) * 100;
        PA_ALLOWANCE.saturating_sub(reduction)
    }
}

fn format_pennies(pennies: u64) -> String {
    let pounds = pennies / 100;
    let pence = pennies % 100;
    format!("£{}.{:02}", pounds, pence)
}
