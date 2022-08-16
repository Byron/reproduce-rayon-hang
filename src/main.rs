use jwalk::WalkDir;
use rayon::prelude::*;

fn main() {
    let should_break = std::env::args().nth(1).is_some();

    let rounds = vec![1, 2];
    let pool_size = if should_break {
        eprintln!("It breaks if there is not enough threads for all inputs and jwalk (with custom bridge)");
        rounds.len()
    } else {
        eprintln!("Having one more thread than tasks in `input` works, add one more argument to invocation to break it");
        rounds.len() + 1
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(pool_size)
        .build_global()
        .expect("Failed to initialize worker thread pool");

    dbg!(rounds.len(), pool_size);
    rounds.par_iter().for_each(|round| {
        eprintln!("Round {round}…");
        for _entry in WalkDir::new(".") {}
        eprintln!("Round {round} completed");
    });
}
