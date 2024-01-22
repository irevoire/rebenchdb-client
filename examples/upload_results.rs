use rebenchdb_client::*;

fn main() {
    let client = Client::new("http://localhost:33333");

    let env = Environment::generate_from_current_config();
    dbg!(env);

    let source = Source::from_repo(".");
    dbg!(source);

    // let benchmark_results = BenchmarkData::new(env, source, , , );
}
