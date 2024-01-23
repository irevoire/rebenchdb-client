use rebenchdb_client::BenchmarkData;

fn main() {
    let payload = std::fs::read("small-payload.json").unwrap();

    let ret: BenchmarkData = serde_json::from_slice(&payload).unwrap();
    dbg!(ret);
}
