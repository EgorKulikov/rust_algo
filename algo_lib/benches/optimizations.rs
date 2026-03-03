/// Regression benchmarks for hot-path code in algo_lib.
///
/// Run with:  cargo bench -p algo_lib --bench optimizations
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::{Output, Writable};
use std::hint::black_box;
use std::time::Instant;

fn bench<F: FnMut()>(label: &str, iters: u64, mut f: F) {
    for _ in 0..iters / 10 {
        f();
    }
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    let per_iter = elapsed / iters as u32;
    println!("  {label:44} {elapsed:>10.3?} total, {per_iter:>8.1?}/iter");
}

// ═══════════════════════════════════════════════════════════════════════════════
// 1. Integer Output via Writable trait
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_int_output() {
    println!("=== 1. Integer Output (i64, 1M writes) ===");
    let nums: Vec<i64> = (0..1_000_000)
        .map(|i| {
            let v = (i as i64)
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407)
                % 1_000_000_000_000;
            if i % 3 == 0 { -v } else { v }
        })
        .collect();

    bench("Writable::write for i64", 20, || {
        let mut buf = Vec::with_capacity(20_000_000);
        let mut output = Output::buf(&mut buf);
        for &n in &nums {
            black_box(n).write(&mut output);
            output.put(b' ');
        }
        output.flush();
        black_box(&buf);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// 2. Integer Input via Readable trait
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_int_input() {
    println!("=== 2. Integer Input (1M parses) ===");
    let data: Vec<u8> = (0..1_000_000)
        .flat_map(|i| {
            let n = (i as u64).wrapping_mul(6364136223846793005) % 1_000_000_000;
            let mut s = n.to_string().into_bytes();
            s.push(b' ');
            s
        })
        .collect();

    bench("Readable::read for u64", 20, || {
        let mut input = Input::slice(black_box(&data));
        let mut sum = 0u64;
        for _ in 0..1_000_000 {
            let v: u64 = input.read();
            sum = sum.wrapping_add(v);
        }
        black_box(sum);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// 3. Output Buffer throughput
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_output_buffer() {
    println!("=== 3. Output Buffer (10M single-byte puts) ===");

    bench("Output::put throughput", 20, || {
        let mut buf = Vec::with_capacity(11_000_000);
        let mut output = Output::buf(&mut buf);
        for i in 0..10_000_000u32 {
            output.put(black_box((i % 256) as u8));
        }
        output.flush();
        black_box(&buf);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// 4. ModInt power()
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_mod_power() {
    println!("=== 4. ModInt power() (1M calls) ===");
    use algo_lib::numbers::mod_int::ModIntF;
    use algo_lib::numbers::num_traits::algebra::One;
    use algo_lib::numbers::number_ext::Power;

    bench("ModIntF::power()", 50, || {
        let mut sum = ModIntF::one();
        let base = ModIntF::new(3);
        for i in 0..1_000_000u64 {
            sum += base.power(black_box(i % 60));
        }
        black_box(sum.val());
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// 5. DSU find + union
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_dsu() {
    println!("=== 5. DSU find() (1M ops on 500K nodes) ===");
    let n = 500_000usize;
    let ops: Vec<(usize, usize)> = (0..1_000_000)
        .map(|i| {
            let a = (i as usize).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) % n;
            let b = (i as usize).wrapping_add(1).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) % n;
            (a, b)
        })
        .collect();

    bench("DSU::union + DSU::find", 20, || {
        let mut dsu = DSU::new(n);
        for &(a, b) in black_box(&ops) {
            dsu.union(a, b);
        }
        let mut sum = 0usize;
        for &(a, _) in black_box(&ops) {
            sum = sum.wrapping_add(dsu.find(a));
        }
        black_box(sum);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// 6. Fenwick add + prefix query
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_fenwick() {
    println!("=== 6. Fenwick Tree (1M add + 1M query) ===");
    let n = 1_000_000usize;
    let ops_add: Vec<(usize, i64)> = (0..1_000_000)
        .map(|i| {
            let idx = (i as usize).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) % n;
            let val = (i as i64).wrapping_mul(2862933555777941757).wrapping_add(3037000493) % 1_000_000;
            (idx, val)
        })
        .collect();
    let ops_query: Vec<usize> = (0..1_000_000)
        .map(|i| (i as usize).wrapping_add(42).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) % (n + 1))
        .collect();

    bench("FenwickTree::add + get_to", 20, || {
        let mut fen = FenwickTree::<i64>::new(n);
        for &(idx, val) in black_box(&ops_add) {
            fen.add(idx, val);
        }
        let mut sum = 0i64;
        for &to in black_box(&ops_query) {
            sum = sum.wrapping_add(fen.get_to(to));
        }
        black_box(sum);
    });
}

// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    println!("algo_lib Regression Benchmarks");
    println!("==============================\n");

    bench_int_output();
    bench_int_input();
    bench_output_buffer();
    bench_mod_power();
    bench_dsu();
    bench_fenwick();

    println!("\nDone.");
}
