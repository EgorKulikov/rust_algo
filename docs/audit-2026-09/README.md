# algo_lib bug audit, 2026-09-21

Trigger: `min_cost_flow_slow` stopped early with negative costs (WA on Repovive 24F, fixed in 71dcf94).
Method: eight parallel auditors, one per area, each writing differential stress tests against brute
force. Every bug below has a failing test in `tests/` here, and every one was re-run against master
(90c662f) and seen failing. Nothing in `algo_lib/src` was changed by the audit.

To re-run a file: copy it (and its `*_support` / `*_fixed` directory if any) into `algo_lib/tests/`, then
`cargo test -p algo_lib --release --test <name>`. The failing tests are the bug reproductions.
`audit_range_ds` has tests that SIGSEGV; run those one per process with `-- --exact <test>`.
Tests marked `#[ignore]` are hangs guarded by a timeout; run them with `-- --ignored`.

"Fix checked" means the auditor applied the fix to a copy of the code inside the test file and the
stress test then passed. It does not mean the library was changed.

## Status after the first round of fixes (2026-09-21)

Fixed, each in its own commit with a regression test in the module, test written first and seen failing:
A1 interactive integer reads, A2 `Real` sign, A3 `read_line`, A6 treap node ids after `reverse` (also the
stale `NodeId::with_payload`), A7 treap `split_by`, A8 persistent treap `split_by`, A9 `MultiTreapSet`
iteration, A10 suffix array `lcp`/`find`, A11 `Circle::intersect_line`, A14 `IntegerSqrt` on `u64::MAX`,
A15 `kth_term`, A16 `all_distances`, A17 `euler_path` with an isolated vertex 0.

By design, now documented in the source instead of changed (owner's call):
- A4: `min_cost_flow_slow` is slow with negative costs; that is what the name says.
- A5: capacities of `2^62` and above (`2^30` for `i32`) are not supported by `min_cost_flow`.
- A12: NTT primes are always below `2^30`.
- A13: `ModInt` input always fits the signed base type.

The notes in section C were not touched in this round.

## Status after the second round (2026-09-21)

Section B is closed. Fixed, one commit each, test first: B1, B3 to B15, B17 to B24 (B5 and B6 share a
commit, both live in the rewritten `gen_range`/`gen_bound`). B11 needed a follow-up: skipping zero-length
edges made a single-point polygon contain everything, which this audit's own convex hull stress caught.

By design, owner's call:
- B2 `EolVec` with a trailing space: inputs are well formed.
- B16 `FixedInt::from(u128)` above `2^127`: documented on the impl.

After both rounds the full `algo_lib` suite passes (201 binaries), and the audit files here fail only on
the by-design items (A5, A12, A13, B2, B16), the ignored A4 timing test, and the `hl_decomposition`
note from section C. The notes in section C are still open.

## A. High impact: wrong answer, hang or crash on ordinary contest input

| # | Where | Problem | Suggested fix | Repro test |
|---|---|---|---|---|
| A1 | `io/input.rs:110` `ensure_slow` | **Interactive problems hang on integer reads.** The parser wants 10 buffered bytes and keeps calling `read` until it has them or EOF; an interactive judge has sent `5\n` and waits. Regression from the fast-input rewrite (b4657a5). Str/line/char/Real reads unaffected. | Stop refilling once buffered data ends in a terminator (last byte `<= b' '` and not `\r`). Fix checked. | `audit_io::interactive_int_does_not_block`, `interactive_int_pair_blocks_on_second` |
| A2 | `numbers/real.rs:28` `with_precision` | Negative reals lose the minus sign when the text contains exactly `precision + 1` zero characters: `-10` at precision 6 prints `10.000000`. About 2% of random values. | Strip the sign only if no digit `1..=9` occurs. Fix checked. | `audit_io::real_output_negative_ten`, `audit_integers::real_with_precision_keeps_sign` |
| A3 | `string/str.rs:129` `read_line` | Drops the last character of a final line with no trailing newline (`"abc"` gives `"ab"`). Also hits `read_lines`, `EolStr`. | `while let Some(c) = self.get() { if c == b'\n' { break; } res.push(c); }`. Fix checked. | `audit_io::read_line_last_line_without_newline` |
| A4 | `graph/min_cost_flow_slow.rs:38` + `negative_distances.rs` | With any negative cost the initial potentials are useless: Bellman-Ford relaxes zero-capacity reverse edges, sees false negative cycles, everything becomes `Infinite`, `adj` stays 0. Dijkstra then runs label-correcting on negative reduced costs: correct but exponential in the worst case (81 vertices, 120 edges: no answer in 10 s). | Compute initial potentials with a Bellman-Ford that skips `capacity() == 0` edges. Fix checked together with the stop-condition fix. | `audit_flows::bug_slow_mcf_exponential_hang` (ignored) |
| A5 | `graph/min_cost_flow.rs:149` | Capacity `>= 2^62` (i64) or `>= 2^30` (i32), e.g. `i64::MAX` as infinity, silently gives flow 0: the artificial return edge is doubled each scaling phase and wraps negative. | After the doubling loop reset the return edge capacity to 2. Fix checked. | `audit_flows::suspect_mcf_big_capacity_i64_b`, `_i32_b` |
| A6 | `collections/treap/treap.rs:382` `push_from_up`/`raise` | After `reverse()`, `raise`, `index_ref` and everything using `NodeId` walk to the wrong node or write through the null node (SIGSEGV). `push_from_up` pushes nothing; recorded directions are invalidated by deferred child swaps. Same cause: `NodeId::with_payload` returns stale payloads under pending deltas. | Push ancestors top-down before recording each direction, reverse the direction list in `raise`. Fix checked (patched copy in `tests/audit_range_ds_fixed/`). | `audit_range_ds::bug_min::treap_index_ref_after_reverse`, `treap_node_id_with_payload_stale` |
| A7 | `collections/treap/treap.rs:363` `split_by` | `split_by_head`/`split_by_tail` callbacks see child aggregates without pending deltas. | `self.push_down()` first inside `split_by`. Fix checked. | `bug_min::treap_split_by_sees_stale_children` |
| A8 | `collections/treap/persistent_treap.rs:358` `split_by` | Callback sees children un-reversed and un-pushed. | `self_.push_down()` right after the copy. Fix checked. | `bug_min::persistent_treap_split_by_ignores_reverse` |
| A9 | `collections/treap/multi_treap_set.rs:148` | `iter()`/`range()` repeat each key by subtree size (`total_size`) instead of `self_size`. | Return `node.self_size`. | `bug_min::multi_treap_set_iter_multiplicity` |
| A10 | `string/suffix_array.rs:64,132,142` | `lcp` and `find` compare the raw `T::zero()` sentinel although SA-IS is rank-based, so inputs containing 0 or negatives give wrong LCP and wrong `find`. Sorted order itself is right. | Exclude the last position from value comparisons. Fix checked. | `repro_suffix_array_lcp_with_zero_element`, `repro_suffix_array_find_with_nonpositive_elements` |
| A11 | `geometry/circle.rs:35` `intersect_line` | For non-normalised lines (`Point::line`, `Segment::line`, `Line::new`) returned points are scaled by `hypot(a, b)`: unit circle and x-axis give `(+-4, 0)`. | Divide `delta` by `hypot(perp.a, perp.b)`. Fix checked. | `repro_circle_intersect_line_not_normalized` |
| A12 | `numbers/mod_int/fft.rs:41` | `FFT::new` picks the Montgomery NTT for any NTT-friendly prime, but that backend asserts `p < 2^30`. Primes such as 2013265921 or 2130706433 panic on the first product over 60 terms, and in all FPS routines. | Add `p < (1 << 30)` to the backend condition (CRT path then handles it). Fix checked. | `audit_modpoly::bug_fft_new_large_ntt_prime` |
| A13 | `numbers/mod_int/mod.rs:229` `Readable` | `ModInt` is read through `i32`, so input values `>= 2^31` are silently wrong. | Read `i64` (`i128` for `ModInt64`) and use `new_wide`. | `bug_modint_read_large_value` |
| A14 | `numbers/integer_sqrt.rs:30` | `sqrt`, `lower_sqrt`, `upper_sqrt`, `lower_root` hang forever on `u64::MAX` (and `-1i64`): saturating `power` equals the input. | Make `power` report overflow (`checked_mul`/`u128`). Fix checked. | `audit_integers::integer_sqrt_u64_max` (ignored) |
| A15 | `numbers/linear_recurrence.rs:77` `kth_term` | For `k >= init.len()` it rebuilds from `init[..d]` and ignores the later terms, so a sequence with a non-recurrent prefix gets answers from a different sequence. | Use the last `d` terms and shift `k`. Fix checked. | `bug_kth_term_ignores_extra_init_terms` |
| A16 | `graph/all_distances.rs:34` | With a negative cycle some pairs that must be `Infinite` stay finite (depends on scan order). | Final pass: for every `k` with `res[(k,k)] == Infinite`, mark all `(i, j)` with `i -> k -> j`. Fix checked on 200k graphs. | `audit_flows::bug_all_distances_negative_cycle` |
| A17 | `graph/euler_path.rs:13` | All degrees even and vertex 0 isolated: returns `None` although an Euler cycle exists. | Start from any vertex of positive degree. Fix checked. | `repro_euler_path_isolated_vertex_zero` |

## B. Edge cases and lower impact

| # | Where | Problem | Suggested fix |
|---|---|---|---|
| B1 | `io/input.rs:322` | Reading an integer at EOF returns 0 (or panics on a subtraction) instead of a clean panic; an interactor reading from a dead solution now sees 0. Regression from b4657a5. | After the whitespace skip: `if at >= input.buf_read { panic!(..) }`. Fix checked. |
| B2 | `io/eol.rs:29` | `EolVec` runs into the next line when the line ends with a space (`"1 2 \n3"` gives `[1,2,3]`). | Skip blanks before `is_eol()`. |
| B3 | `collections/fast_clear_arr.rs:61` | After `clear()`, writing a lower index truncates live higher elements, and `+=` sees the pre-clear value. Type is unused in the library. | Resize only when `index >= len`; otherwise reset the slot. Fix checked. |
| B4 | `misc/expression_parser.rs:57` | A space after `)` silently drops the rest of the input: `(1) +2` parses as 1. | Skip blanks at the top of the `parse_expr` loop. Fix checked. |
| B5 | `misc/random.rs:63` | `gen_range` overflows for signed ranges spanning half the type or more (`0i32..`, `-2e9..=2e9`). Panic locally, out-of-range values without overflow checks. | Do the span arithmetic in wrapping `u64`. Fix checked. |
| B6 | `misc/random.rs:40` | `gen_bound`/`gen_range` truncate 128-bit bounds to `u64` (`gen_bound(1u128 << 64)` divides by zero). | Use `gen_u128` for wide types or restrict `T`. |
| B7 | `collections/multi_set.rs:105` | `insert_few(v, 0)` leaves a phantom key with count 0. | Return early when `qty == 0`. |
| B8 | `string/split.rs:9` | `str_split` panics when occurrences overlap (`"aaa"` by `"aa"`). | `if i >= start && ...`. Fix checked. |
| B9 | `string/aho_corasick.rs:126` | An empty pattern is not counted at depth-1 nodes. | Merge the root payload when `parent == 0`. Fix checked. |
| B10 | `string_algorithms/z_algorithm.rs:11`, `prefix_function.rs:11` | Return `[0]` for empty input. | Return an empty Vec. |
| B11 | `geometry/polygon.rs:43` | `contains` rejects interior points when a vertex is repeated consecutively. | Skip zero-length edges. |
| B12 | `graph/min_cost_flow.rs:54` | Panics when no edge has positive capacity (`highest_bit` of 0). | Return zero flow. Fix checked. |
| B13 | `graph/flow_with_demand.rs:27` | Edge with capacity 0 and a positive lower bound is skipped, infeasible instance returns `true`. | Test the demand before the capacity filter. |
| B14 | `graph/euler_path.rs:32` | Panics on a graph with 0 vertices. | Early return. |
| B15 | `collections/sparse_table.rs:14`, `sparse_table_pos.rs:16` | `new` panics on an empty array (`Rmq`, `SegTree`, `WaveletMatrix` accept it). | `levels = 0` when `n == 0`. |
| B16 | `numbers/fixed_int.rs:118` | `i256::from(u128::MAX)` is -1: conversion goes through `i128`. | Separate unsigned path with zero extension. |
| B17 | `numbers/signed_big_int.rs:197,204` | `BigInt *= i32::MIN` and `/= i32::MIN` store negative limbs (prints `--512`). | Pass `unsigned_abs()` as `u32`. |
| B18 | `numbers/number_theory.rs:129` | `stern_brocot_search(u64::MAX, ..)` overflows when the target is 0 or infinity. | `checked_add`. Fix checked. |
| B19 | `numbers/mod_int/mod.rs:50` | `new_signed` overflows for moduli above 2^30 (`ModInt64`: above 2^62); panic only because overflow checks are on. | `new_wide(n as wide)`. |
| B20 | `numbers/mod_int/mod.rs:139` | `ModInt::from(usize)` stores an out-of-range value for inputs `>= 2^63`. | Reduce in `usize`/`u128`. |
| B21 | `numbers/mod_int/mod.rs:260` | `Debug` panics on composite moduli (non-invertible denominator in the fraction search). | Skip denominators without inverse. |
| B22 | `numbers/fps.rs:23` `mul_trunc` | FPS routines panic for moduli wider than 32 bits once operands exceed 60 terms; `multiply` already falls back to quadratic. | Same guard as in `multiply`. |
| B23 | `numbers/series.rs:10` | `sum_geometric_series` with ratio 1 divides by zero. | Special-case ratio 1. |
| B24 | `numbers/num_traits/assign_to_op.rs:16` | `mult!`, `sub!`, `div!` macros do not compile (wrong trait/method names). Unused. | Rename. |

## C. Notes, not counted as bugs

- `min_cost_flow_slow` loops forever on a genuine negative cycle (the fast `min_cost_flow` handles it).
- `max_flow(s, s)` overflows; `edge_distances::diameter()` panics for n = 0.
- HLD, centroid decomposition and link-cut are recursive and overflow a 2 MB stack on a 2e5 path (fine with 16 MB).
- `hl_decomposition.rs:37` `iter` from `Excluded(root)` yields the root's part; the guard for it is dead code.
- `TreapSet::insert` returns `true` when the key was already present, the opposite of `BTreeSet`.
- Treap `Payload` has no hook for `reverse`, so direction-dependent aggregates are silently wrong after a reverse.
- `collections/id.rs`: `advance` + `by_id` can write past the buffer. `md_arr/arr5d.rs:102` `index_mut` lacks the `a4` bound assert.
- `misc/simd.rs`: safe functions call AVX2 code without runtime detection. `misc/bump_alloc.rs`: zero-sized first allocation is UB.
- `misc/bin_search.rs:13` overflows when `right - left == T::MAX`. `bit_ops::all_bits(BITS)` panics.
- `geometry/line.rs`: `Eq` is projective but `Ord`/`Hash` are field-wise. `Polygon::contains` is convex-only and undocumented.
- `suffix_automaton.rs:144` `num_substrings(0)` panics. `all_matches` with an empty pattern returns n matches, `wildcard_matches` n + 1.
- `FFT`/`PolynomialOps` reused after a dynamic modulus change give silent garbage. `ModInt::log` loops forever when no log exists.
- `tester`: `process_error` only downcasts `&str` payloads (String panics show an empty reason); interactive mode discards a solution-thread panic.
- `Output` has no `Drop` flush (by design, templates call `flush`).

## D. What passed

Stress-tested against brute force with no findings: SWAR integer input and table-driven integer output
(all types, all buffer boundaries), `Graph` storage in all four modes and `compact()`, SCC, bridges, cut
points, block-cut tree, LCA, dominators, both MST modules, DSU variants, link-cut, Euler tour tree, both
max flows, bipartite and general matching, Dijkstra, Bellman-Ford, 2-SAT, assignment, both segment trees,
Fenwick variants, RMQ, Li Chao, wavelet matrix, rectangle sum, sliding window, foldable deque, interval
heap, `IntSet`, `BitSet`, indexed heap, radix sort, `md_arr`, Mo, memoization, NTT (AVX2 and scalar paths
compared), convolution, FPS on 32-bit moduli, product tree, interpolation, Berlekamp-Massey, `BinomialMod`,
zeta/Moebius and subset convolution, FWHT, big integer arithmetic incl. Newton division around the 96-limb
threshold, Miller-Rabin, Pollard-Brent, sieves and prime counting, matrices, Gauss, delayed-reduction modular
linear algebra, suffix automaton, palindromic tree, hashing, Manacher, Lyndon, runs, wildcard matching,
segments, convex hull, closest/farthest pair, arg sort, min enclosing circle, Manhattan MST.
