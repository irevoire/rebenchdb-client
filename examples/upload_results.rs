use rebenchdb_client::*;
use time::OffsetDateTime;

fn main() {
    // Setup everything
    let env = Environment::generate_from_current_config();
    dbg!(&env);

    let source = Source::from_repo(".").unwrap();
    dbg!(&source);

    let mut benchmark = BenchmarkData::new(env, source, "Test 1", OffsetDateTime::now_utc());
    benchmark.with_project("Test generated");

    let bench = Benchmark {
        name: String::from("Test"),
        suite: Suite {
            name: String::from("Test"),
            desc: None,
            executor: Executor {
                name: String::from("My machine"),
                desc: None,
            },
        },
        run_details: RunDetails {
            max_invocation_time: 12,
            min_iteration_time: 2,
            warmup: None,
        },
        desc: None,
    };

    let run_id = RunId {
        benchmark: bench,
        cmdline: String::from("cargo bench"),
        location: std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
        var_value: None,
        cores: Some(4),
        input_size: None,
        extra_args: None,
    };

    let mut run = Run::new(run_id);

    for i in 1..2 {
        let mut point = DataPoint::new(1, 10);

        for j in 0..10 {
            point.add_point(Measure {
                criterion_id: 0,
                value: (10000 + j * i) as f64,
            })
        }

        run.add_data(point);
    }

    benchmark.register_run(
        run,
        Criterion {
            id: 0,
            name: String::from("total"),
            unit: String::from("ms"),
        },
    );

    println!("{}", serde_json::to_string_pretty(&benchmark).unwrap());

    // Prepare to send the run to rebenchDB
    let client = Client::new("http://localhost:33333");
    client.upload_results(benchmark).unwrap();
}
