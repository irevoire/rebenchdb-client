use rebenchdb_client::Environment;

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&Environment::generate_from_current_config()).unwrap()
    );
}
