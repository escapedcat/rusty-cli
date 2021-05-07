use std::env;

fn get_tx_info(tx: &str) -> String {
    format!("{} append", &tx.to_string())
}

            fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "tx-info" {
        let tx = args[2].clone();
        println!("TX info: {}", get_tx_info(&tx.to_string()));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_tx_info() {
        assert_eq!(get_tx_info("12"), "12 append");
    }
}
