extern crate benchmarking;

use benchmarking::Benchmark;

fn main() {
    const VEC_LENGTH: usize = 100;

    let mut benchmark = Benchmark::default();

    benchmark.warm_up();

    let bench_result = benchmark.bench_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure_while_loop(|loop_seq| {
            loop_seq < VEC_LENGTH
        }, |loop_seq| {
            vec.push(loop_seq);
        });

        /*
            let mut i = 0;

            while i < VEC_LENGTH {
                // Start the measurement
                vec.push(i);
                // End the measurement

                i += 1;
            }
        */
    }).unwrap();

    println!("Pushing a number into a vec takes {} nano seconds!", bench_result);
}