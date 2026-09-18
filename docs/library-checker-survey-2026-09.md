# Library Checker survey: what to integrate into algo_lib

Generated 2026-09-17 from the 10 fastest accepted submissions overall and the 5 fastest Rust submissions for all 253 problems on https://judge.yosupo.jp (dedup by user). Submission ids are cited so sources can be re-fetched with `curl https://v3.api.judge.yosupo.jp/submissions/<id>`.

## Progress log

Work done against the priority list below (2026-09-18/19), one commit per change:

| # | Item | Done | Measured | Left open |
|---|---|---|---|---|
| 1 | Fast I/O | SWAR integer parsing, 4-digit-table output | read 1.6-1.9x, write 1.5-2.0x | mmap input (not portable to Windows judges / interactive tasks) |
| 2 | NTT kernel | Montgomery DIF/DIT without bit reversal, shared twiddle table, AVX2 butterflies with runtime dispatch, 8-element SIMD leaf | 2^19 x 2^19 multiply 105 ms -> 15 ms | Montgomery as the ModInt representation; radix-4; split-FFT for arbitrary moduli |
| 3 | Segment trees | `collections/seg_tree.rs`: bottom-up `SegTree` (with max_right/min_left) and `LazySegTree` with push-free queries | 2.0x / 1.7x vs recursive tree | tag-only dual tree, pool-based persistent tree |
| 4 | Graph DFS toolkit | single-pass iterative Tarjan SCC (2-SAT inherits it) | colors 3x, 2-SAT 1.5x | iterative bridges / BCC / cycle detection |
| 5 | FPS layer | `numbers/fps.rs`: inverse, log, exp, pow, sqrt, sparse variants, `mod_sqrt`; NTT-domain Newton inverse | n=5e5: inv 31 ms, log 52, exp 103, pow 160, sqrt 77 | Kinoshita-Li composition and compositional inverse, relaxed exp/log |
| 6 | Number theory | Montgomery64 deterministic Miller-Rabin, Pollard-Brent (M=512, binary gcd), segmented odd sieve, Lucy `PrimeSums`/`prime_pi` | is_prime 10x, factorize 7x, primes(5e8) 4.6x, pi(1e10) 109 ms | wheel-30 sieve, min_25 multiplicative sums |
| 7 | Linear algebra | `numbers/mod_linear.rs`: delayed-reduction det/rank/invert/solve, AVX2 mat_mul | det 3.8x, invert 3.2x, mat_mul 1024^3 1.6 s -> 0.22 s | GF(2) BitMatrix with M4RI, Strassen, Frobenius-form matrix power |
| 8 | Data structures | `IntSet` (64-ary bitmap, 37x BTreeSet), `Rmq` (O(1) block RMQ), `LiChao`, `WeightedDsu`, `WaveletMatrix`, push-relabel `BipartiteMatching` | see commits | kinetic segment tree, segment tree beats hook, wide xor trie, interval heap |
| 9 | Trees | LCA on preorder positions + parent-position RMQ | build 82 ms -> 18 ms at n=5e5 | static top tree, centroid bisect, xor-linked tree builders |
| 10 | Big integers | multiply rides the new NTT (2e6 x 2e6 digits 43 ms); Newton reciprocal block division | 200k/100k digits 1078 ms -> 15 ms | SWAR parse/print, hex big integers |

## Cross-cutting priorities (merged)

1. **Fast I/O**: mmap stdin (fstat, fallback to buffered read), SWAR 8-digit parse, 4-digit LUT writer with one final write. Named by 11 of 13 category reports as the first reason Rust entries trail C++ by 1.3-3x, and it explains most of Egor's own gaps (static_range_sum 0.087 vs 0.017; associative_array 0.206 vs 0.043). Effort M.
2. **Montgomery u32 ModInt + rewritten NTT kernel** (DIF/DIT without bit reversal, cached rate2/rate3 twiddles, radix-4, `#[target_feature(enable = "avx2")]` lanes, in-place spectrum API with transform/pointwise/inverse/doubling). Underlies convolution, FPS, polynomial algorithms, combinatorics tables and linear algebra; 1.5-3x everywhere, and lifts the 2^21 length cap. Effort L (scalar Montgomery + tables alone M).
3. **Bottom-up non-recursive segment tree family**: plain, lazy (tags in a separate array, no push on query), tag-only dual, pool-based persistent. Explains point_set_range_composite 1.28 vs 0.049 and range_affine_range_sum 0.75 vs 0.08. Effort M.
4. **Iterative DFS on CSR adjacency**: single-pass Tarjan SCC (also 2-SAT), lowlink by reverse sweep over dfs order (bridges, 2ECC, BCC), cycle detection. Current scc/two_sat/bridges are recursive Kosaraju with a transposed Graph. Effort M.
5. **Public FPS layer**: inv (5 transforms per doubling), exp maintaining the inverse, log, pow, sqrt, plus O(nk) sparse variants; then Kinoshita-Li composition and compositional inverse (49x algorithmic gap today). Effort M + M + S.
6. **Number theory core**: Montgomery64 + deterministic Miller-Rabin (2 hashed bases) + Pollard-Brent M=512 with binary gcd; Lucy_Hedgehog prime-sum sieve; wheel-30 segmented sieve. Effort M each.
7. **Linear algebra kernels**: delayed-reduction u64 rows (reduce every 16 ops, triangular only) for det/rank/solve/inverse; 4x8 tiled matmul with u64 accumulators; GF(2) BitMatrix with M4RI. Effort S / M / M.
8. **Missing data structures with big measured gaps**: 64-ary bitmap integer set (predecessor 1.40 vs 0.016), blocked sparse table B=16, wavelet matrix with batched queries, kinetic segment tree, segment tree beats hook, Li Chao on compressed xs, wide xor trie, interval heap, weighted DSU. Effort S-M each.
9. **Trees**: parent-array/xor-linked tree builders, LCA by preorder-parent RMQ with 64-block sparse table (0.124 -> ~0.03), static top tree, centroid bisect with one NTT per component. Effort S / S / L / M.
10. **Big integers**: base 10^8 limbs with real/imag-packed double FFT multiply (5-10x), Newton reciprocal + Burnikel-Ziegler division, SWAR parse and table print. Effort M / M-L / S.

Caveat found during the survey: the "Rust" leaders for static_range_lis_query (75750), point_set_range_sort_range_composite (205042) and vertex_get_range_contour_add_on_tree (217443) are embedded ELF binaries, so those three Rust gaps are not real measurements.

## Category report: ds1

```
TOP INTEGRATION CANDIDATES (data structure basics) — explains Egor's own 5-90x gaps
1. Bottom-up (ACL-style) non-recursive segment tree + lazy variant: 2n Vec<Node>, set/prod/apply_range loops, tags in separate array with identity flag, query never pushes (applies ancestor tags on the way up). Evidence: 278231 0.071 vs Egor 108007 1.28 (heap-allocated Matrix<Mod> per node, 8 mulmods + alloc per join, recursive tree with closures, old I/O); 400999 0.198 vs Egor 0.75; pure-Rust urectanc 363483/383717 get 0.095/0.31. segment_tree.rs is recursive Euler-layout with push_down on every visited node. Effort M.
2. 64-ary bitmap integer set (collections/fast_set.rs): 4 levels of Vec<u64>, insert/remove/contains/next/prev via ctz/clz; plus compressed-bitset+Fenwick ordered set (rank/kth with pdep). Evidence 269275 0.016-0.02 vs Egor BTreeSet 1.40 / TreapSet 3.36; ordered_set 382974 0.075. bit_set.rs has no next/prev hierarchy. Effort S/M.
3. I/O: SWAR 8-digit parse (x ^ 0x3030.., three multiply-shift steps) + 4-digit LUT writer into fixed buffer with single write, optional mmap stdin. Evidence static_range_sum 273878 0.017 / Rust 382975 0.022 vs Egor 0.087 (old 4KB per-byte parser); current io/input.rs has 2MB buffer tight loop but no SWAR; output.rs does n%10 loop + write_all per number. Affects every problem. Effort M.
4. Blocked sparse table B=16 (per-block prefix/suffix min + sparse table over block minima; query 2-3 loads). Evidence 362370 0.020, Rust 376011 0.049 vs Egor 0.225 (recursive SegmentTree); sparse_table.rs is plain Vec<Vec<T>> with Fn closure. Effort S.
5. Pool-based persistent lazy segment tree (u32 child indices in Vec<Node>, (sum,size)+(a,b) lazy, reserve 1e7 nodes). Evidence 307048 0.189/150MB, Rust 267749 0.229 vs Egor persistent treap 389119 0.864/767MB. Effort M.
6. Weighted DSU with Group potentials (AoS parent+pot, union by size; 368008 0.009, Rust 363679 0.013; MISSING), interval heap for double-ended PQ (357629 0.082 Rust 4KB; MISSING), flat open-addressing int->int map with fibonacci hashing 2^20 slots (362188 0.043 vs FxHashMap 0.088). Effort S each.
Other: unionfind HAVE (path-halving micro-tweak), point_add_range_sum: fenwick HAVE (leaders use 16-ary wide prefix-sum tree with AVX2; Rust 400442 0.028), range_affine_point_get needs dual tag-only tree (MISSING), large_array variants = offline compression helper.
```

## Category report: ds2

```
TOP INTEGRATION CANDIDATES (advanced sequence queries)
NOTE: "Rust" entries for static_range_lis_query (75750) and point_set_range_sort_range_composite (205042) are base64-embedded ELF binaries, so those 12x/3.6x gaps are fake. Also vertex_get_range_contour_add "Rust" 217443 is an ELF.
1. mmap-backed input + SWAR 8-digit parsing in io/input.rs (fstat -> mmap when stdin is a regular file, *const u8 cursor, no per-byte \r check; fallback to current path). Evidence: urectanc 383716/388026/382978 within 1.3x of C++ with exactly this; buffered readers 2-3x off on I/O-bound problems (queue_operate_all_composite 0.013 vs 0.032). Effort M.
2. Wavelet matrix with batched level-by-level kth/count_lt (Rust 401815 quantile_batch 0.044; 403008 0.020; {bits,cnt} interleaved 16-byte blocks, radix-sort compression); also unlocks static_range_lis via Tiskin seaweed (382253) and point_add_rectangle_sum. MISSING. Effort M.
3. Kinetic segment tree for range linear-add / range min (382268 ~130 lines, 0.12; 393894 0.102): node stores min, slope, melt time both directions. 4x over Rust hull approach 192332 0.415. MISSING. Effort S.
4. Segment tree beats: add a can_apply_whole(&delta)->bool hook to SegmentTreeNode lazy path (393927 iterative beats; Rust 383857 0.327). MISSING. Effort M.
5. Mo's "second offline" variant in misc/mo.rs (181308 ~100 lines): pointer moves recorded as (time, range) jobs, answered in a sweep with sqrt-decomp counter (O(1) per element move). 6-7x on inversions (Rust 0.6 -> 0.1). Effort S.
6. Static range mode (sqrt block-pair table, CSR u32 layout, 401534 0.074 vs Rust 285065 0.262 with Vec<Vec<usize>>; 403022 0.019 adaptive frequency layers) and wide-fanout xor trie (Rust 383716 16-ary with u16 presence mask 0.062; 403013 64-ary bitmask levels 0.021). MISSING. Effort S each.
Other: range_set_range_composite = assign-lazy segtree with per-level power table f^(2^k) (HAVE-BUT-SLOWER, gap I/O); range_kth_smallest wavelet matrix; range_reverse / dynamic_sequence: index-based top-down splay with lazy+rev packed in size bits (379333) or canard chunked B-tree list (402523 0.156) vs Rust arena splay 388026 0.257 (treap HAVE-BUT-SLOWER); sortable segtree via Patricia tries per run (393919, MISSING); Li Chao on compressed xs (MISSING; Rust 383324 0.081 = C++); two-stack foldable queue/deque (MISSING small; Rust 363682 = C++); static_range_frequency etc HAVE, only I/O gap.
```

## Category report: ds2d

```
TOP INTEGRATION CANDIDATES (2D / offline / persistent DS)
1. Radix sort by u32/u64 key (slice_ext) + sort-based coordinate compression — every leader uses it (393934, 403016, 403009, 401818); Rust 367861 vs 393934 on rectangle_sum shows ~2x from comparison sort + hashmap compression. Effort S.
2. Offline rectangle solvers module: rectangle_sum (sweep + Fenwick), static_rectangle_add_rectangle_sum (single Fenwick of 4 mod coefficients, 403016), rectangle_add_point_get (binary time blocks, 393933/186151), CDQ point_add_rectangle_sum (401366). Closes 5.4x gap. Effort M.
3. Covered-length segment tree + union_area helper (iterative count/covered-length tree 393895: 0.324 vs Rust 0.528 with lazy tree); 8-ary cache-line layout 403009 (0.131) later. Effort S/M.
4. Offline version-tree helpers for persistent queue / DSU (flat child lists + iterative DFS with undo, 393911/385805). persistent_queue 4.3x gap is I/O + allocation. Effort S.
5. KD-tree with lazy affine, 32-point SoA leaves (402764) for dynamic rectangle affine (no Rust AC); wavelet matrix + Fenwick per level (401818, fastest overall is Rust). Effort L each.
Library status: rectangle_sum HAVE-BUT-SLOWER (fenwick + compress exist); point_add_rectangle_sum MISSING (no wavelet matrix/CDQ); rectangle_add_point_get MISSING; static_rectangle_add_rectangle_sum MISSING; dynamic_point_set_rect MISSING (no KD-tree); area_of_union MISSING; persistent_queue MISSING; persistent_unionfind HAVE (dsu_rollback.rs); range_parallel_unionfind MISSING.
I/O: leaders use mmap stdin + fixed-width parsers; algo_lib Input has 2MB buffer tight loop, I/O secondary except persistent_queue.
```

## Category report: graph_conn

```
TOP INTEGRATION CANDIDATES (graph: connectivity / paths / matching / flow)
1. Iterative DFS toolkit (CSR + explicit frame stack) and rewrite scc as single-pass Tarjan — algo_lib scc is recursive Kosaraju with a transposed Graph (3x memory traffic) vs 403026 (0.029) / Rust 385044 (0.038); reuse for cycle_detection (MISSING; Rust 375418 0.052), bridges/2ECC (urectanc 368204 0.032 Rust: push-all-neighbours DFS recording dfs_order, low by one reverse sweep), BCC. bridges.rs is recursive and returns bridges only. Effort M.
2. Complement-graph components — remaining-list sweep (363104 0.040) or min-degree-vertex trick (363132 0.030); Rust 346669 (0.314) uses BTreeSet range removals = whole 10x gap. MISSING. Effort S.
3. Radix heap + lazy-heap Dijkstra — RadixHeap<u64> (Rust 402162 ~100 lines; 403027 0.040), lazy (dist,v) entries instead of IndexedHeap in distances.rs (373581 0.061 plain lazy binary heap competitive; Rust 383867 0.071 std BinaryHeap + CSR). HAVE-BUT-SLOWER. Effort S-M.
4. LAPJV (Jonker-Volgenant) for assignment — 384069 (0.008) vs classic O(n^3) Hungarian 71802 (0.072); misc/hungarian_algorithm.rs is the e-maxx variant over Arr2d<i64>. 6-9x. Effort M.
5. Offline incremental SCC — D&C over insertion time (Rust 227421 0.758 structure) with 403183's optimisations (0.186: no per-level allocation, interval shrinking, skip recursion when no merge, dense closure base case <=32). MISSING. Effort M.
6. Small linear-time additions: st_numbering (Even-Tarjan, nachia 262729 ~60 lines iterative), 3ECC Tsin (344406 ~45 lines), directed Euler trail (extend euler_path.rs: head-advance instead of removed BitSet, 286963; Rust gap on eulerian is I/O). Effort S each.
Other: k_shortest_walk Eppstein with persistent leftist heaps in arena (403409 0.061; MISSING); general_matching Edmonds blossom (MISSING, tiny input); general_weighted_matching dense O(n^3) blossom (324220 0.248 vs Rust 53472 1.17 Van Rantwijk port; MISSING); bipartite_edge_coloring (regularize + Euler splitting, nachia 228343; MISSING); min_cost_b_flow: leaders network simplex; min_cost_flow.rs has no lower bounds/b-flow API; bipartitematching: HAVE (push-relabel 396318 0.070; Rust gap is Vec<Vec> + VecDeque + I/O).
```

## Category report: graph_st

```
TOP INTEGRATION CANDIDATES (graph: spanning trees / structure / counting)
1. Fast determinant mod p (numbers/gauss.rs): forward-only elimination on u64 rows with lazy reduction every 16 ops (values < 16*P^2 fit u64), plus count_spanning_trees / count_eulerian_circuits wrappers. Evidence 362200 0.020 vs Rust 340350 0.067. gauss.rs::det does full Gauss-Jordan (2x work) with Field division per row and per-op reduction. Effort S.
2. Dominator tree (Lengauer-Tarjan) as in Rust 297499 (0.032 = C++ best): DFS renumbering, reverse graph CSR in DFS order, CSR buckets, path-compressing eval. MISSING. Effort M.
3. Directed MST tuning (minimal_spanning_rooted_tree.rs has the algorithm): AoS skew-heap node struct {l,r,from,to,weight,lz}, path-compressed DSU for cycle detection + rollback DSU for contraction, immediate self-loop pops, avoid Vec<&E>. Evidence 290398 0.021 vs Rust 296045 0.108. Also add union-by-size + path-halving to collections/dsu.rs (MST: 364954 0.025 vs Rust 383869 0.050; minimal_spanning_tree.rs uses stable sort_by_key). Effort S-M.
4. Chromatic number via DSATUR branch-and-bound with greedy-clique LB (356303, 0.002 vs Rust inclusion-exclusion 0.027). MISSING. Effort S.
5. Tree decomposition width 2 via degree<=2 elimination with per-degree queues (Rust 281208 0.189; canard 403032 0.061 with packed series reduction). MISSING. Effort M.
6. Small graph-counting helpers: triangles (Rust 313791 0.008 = fastest overall: orient low->high by (deg,id), CSR by counting sort, marker array, u64 accumulate one % per neighbor), C4 counting (377239), MDST absolute 1-center with witness pruning (403005, 0.010, 20x next) as L item.
Other: maximum_independent_set (bitset branch&bound, MISSING), chromatic_polynomial (SPS power projection, MISSING), enumerate_cliques MISSING, chordal recognition via MCS (MISSING), dynamic connectivity offline (HAVE pieces: DSURollback, LCT; no driver; leaders use deletion-time max spanning forest 402761 0.057), global min cut of star-augmented graph MISSING (extreme-set tree, no Rust AC).
```

## Category report: tree

```
TOP INTEGRATION CANDIDATES (tree)
1. Parent-array tree builder + xor-linked tree (S): Tree::from_parents (preorder/subtree size by two linear loops, 278239/278243), XorLinkedTree leaf-peeling (361517; Rust 383862 0.035) giving bottom-up order, parent, depth without adjacency lists. Basis of 2-4; tree_diameter 2x, subtree_sum 2x.
2. Preorder-parent RMQ LCA with 64-block sparse table (S/M): LCA(u,v) = min parent-preorder over (pos_u, pos_v]; block prefix/suffix + sparse table over block minima (278239 0.024, 269677). lca.rs is Euler tour 2n-1 + full sparse table with level[] indirection: Rust 363295 0.124 -> ~0.03 expected; also speeds vertex_add_path_sum via root-path prefix sums with Fenwick (278241 0.100).
3. Static top tree (L): compress/rake node arrays over HLD, packed child links, update walks links, prod(r) rerooting (yosupo 205356/275016; Rust 380241 0.294 = fastest overall, 380242 0.163). MISSING. Unlocks point_set_tree_path_composite_sum (both).
4. Centroid bisect decomposition on BFS-relabelled parent arrays (M): callback gets (par, verts, n0, n1), ONE NTT per component (maspy 215748 0.162 vs Rust 276198 0.523 two mods+CRT); central_decomposition.rs is recursive with BitSet and callback re-traversing adjacency (3x). Flat iterative centroid decomposition with packed per-vertex (depth, ancestor-BIT id, sibling-BIT id) u64 for contour queries (374968 0.183, 399767 0.179; MISSING).
5. AHU-by-index-order isomorphism (S): loop i=n-1..1 with p_i<i, type via hash map, child_hash[p] += splitmix64(type) (106481 0.026 vs Rust 177939 0.158); rerooting DP helper (346214/Rust 366502 0.044 2nd overall). MISSING.
6. LCT micro-optimizations (M): index-based nodes with null sentinel 0, cached child index k (is_root = k==-1), rotate-based access instead of re-splay, DFS-built initial forest (270626 0.174, 332878); ~1.3x over link_cut.rs.
Other: jump_on_tree offline level ancestor via bucketed queries + DFS ancestor stack (270103 0.071; canard ladder 403437 0.056); vertex_set_path_composite: HLD chains as implicit Fenwick-shaped balanced trees (319096 0.116 vs Rust 0.227); dynamic subtree sums: offline dynamic connectivity + "anti-monopoly tree" (345113 0.118); cartesian_tree monotonic stack (MISSING trivial); common_interval_decomposition (MISSING, no Rust AC); topological order min inversions greedy heap+DSU (MISSING).
```

## Category report: conv

```
TOP INTEGRATION CANDIDATES (convolution + set power series)
1. Montgomery ModInt + AVX2 radix-4 DIF/DIT NTT (no bit reversal, pointwise product in scrambled order), ACL-style rate2/rate3 twiddle tables, lazy reduction in [0,2p), cache-blocked recursion — replacing prime_fft.rs inner loop (scalar radix-2 with bit reversal, twiddles recomputed by w *= w_len per butterfly, % modmul). Evidence 393435 0.023 vs Rust 364617/401473 0.031 (target_feature avx2 + const fn root tables). Lifts the 2^21 limit to 2^23. Effort L (scalar Montgomery + tables alone M, likely 2-3x).
2. Ranked subset convolution with u64 lazy accumulation of Montgomery products, AoS rank layout (array<base, max_logn> per mask), ranks 0/n special-cased (352953 0.094 vs Rust 285365 0.436); then SPS exp by halving, composite by bit recursion with derivative stack, power projection = transpose (339836/339877/339878). MISSING. Effort M + S each.
3. Arbitrary-mod convolution via split complex-double FFT (15/16-bit pieces, AVX2 FMA, real/imag trick, centered residues |x|<p/2) instead of 3-prime CRT (403413 0.030 vs Rust 3-prime 366728 0.085; Rust 402147 0.051 does the FFT approach). convolution.rs does three scalar NTTs + i128 Garner. Effort M.
4. mul_mod2n via units of Z/2^k = <-1> x <5> indexing + cyclic power-of-two NTTs per 2-adic class (72433 0.081 vs Rust 1.567); mul_modp via primitive-root dlog + folded NTT (400023 0.052 vs 0.228). Needs primitive-root finder. Effort S each once NTT exists.
5. Small MISSING kernels: AND/OR zeta with branchless min-trick u32 add/sub (362198; fwht.rs has XOR only, uses /= for inverse scaling); gcd/lcm prime-wise divisor zeta (190529/172602; Rust 366377 0.043 matches C++); multivariate chi-index convolution (199651) and cyclic per-axis DFT with chirp-Z (207051, no Rust AC); monotone-minima min-plus convex-arbitrary (329698 0.040 vs Rust 0.088) and concave via Li-Chao-style candidate tree (360961); convex-convex merge of difference sequences (Rust 381611 fastest overall). Effort S each.
6. convolution_mod_large (n+m up to 2^25): 2^23-point NTT over blocks of 4/8 coefficients then length-8 chunk products mod x^8 - w (303498, 400724 0.738 vs Rust 2.72 splitting into pieces). MISSING.
7. convolution_mod_2_64: cp_algo dft64 splits u64 into 4 int16 parts with random odd factor + complex FFT (337130 0.076); Rust 187988 0.202 five-prime Montgomery NTT + Garner. MISSING.
I/O: mmap stdin + LUT unsigned output is what makes Rust 381611/366377 match C++.
```

## Category report: nt

```
TOP INTEGRATION CANDIDATES (number theory)
1. Montgomery64 + deterministic Miller-Rabin (base 2 + one hashed base from 2^18 table; {2,7,61} below 2^32) + Pollard-Brent M=512 with two interleaved rho sequences and binary gcd — rewrite primes/prime.rs. Evidence 365922 (0.034) vs Rust 139998 (0.053); factorize 317402 (0.013) vs Rust 312422 (0.017). algo_lib is_prime uses 20 random bases through ModInt64 with u128 % per mul; brent uses M=128, Euclid gcd. Fixes primality, factorize, primitive_root, tetration phi (Rust 355509 0.058 vs 0.004 because phi by trial division), two_square. Effort M.
2. Lucy_Hedgehog prime-sum sieve (roughs/smalls/larges arrays, odd-only, reciprocal-multiply division) as generic prime_sum(f) — counting_primes (222872 0.012; Rust 212150 0.019), sum_of_totient via min_25 DFS (349721 0.053 vs Rust 0.117; the 0.008 leaders use O(n^{2/3}/log) Mertens), base for sum_of_multiplicative_function_large (364196). MISSING. Effort M-L.
3. Wheel-30 byte-packed segmented sieve replacing primes/sieve.rs — Rust 399449 (0.22) is pure Rust 3x faster than algo_lib's BitSet sieve; 336754 (0.083 cp-algo) dense-wheel masking. Effort M.
4. Nim product: clmul GF(2^64) isomorphism with 8x256 byte tables (337840 0.049) or 16-bit exp/log tables with Karatsuba (238394 0.089, portable) — 7x over Rust 96973 (0.354, recursive 256x256 table). MISSING. Effort S.
5. sqrt_mod hybrid (207878: p=3 mod 4 pow, p=5 mod 8 Atkin, Tonelli-Shanks if v2(p-1)<6 else Cipolla) + kth_root_mod via Adleman-Manders-Miller with bucketed BSGS (152069). MISSING. Effort S/M.
6. tetration_mod, Gaussian gcd with div_round (173484), two_square via gcd(p, x+i) (235398) — S each once (1) exists. MISSING.
Also MISSING: primitive_root (Rust 138249 0.001 top overall), min_of_mod_of_linear (gsh Euclid-like loop 389955), rational_approximation + stern_brocot (Stern-Brocot binary search with doubling 290530; Rust 381187 top via continued fractions), bernoulli (needs poly inverse; Rust 383856 top), counting_squarefrees (O(n^{2/5}) Rust 187448 0.483 vs 0.078 negiizhao Mertens). discrete_log: algo_lib ModInt::log has FxHashMap, no non-coprime handling (361590 ex-BSGS). HAVE: floor_sum (Rust 385286 top), kth_root_integer.
```

## Category report: fps

```
TOP INTEGRATION CANDIDATES (FPS core)
1. Public FPS module (inv/log/exp/pow/sqrt) with in-domain Newton — inv as 5-transform-per-step schedule reusing g's transform (261768/146019 "10E(n)"); exp maintaining the inverse (Rust 364944 lines 120-170 compact template, 0.064); log = deriv*inv; pow = log/exp with leading-zero / a0^k (k mod p-1) handling (365001). Needs PrimeFFT to expose forward/inverse transforms and pointwise ops on buffers, not only multiply. algo_lib's private PolynomialOps::inverse does two full multiplies per step (~12 length-2t transforms vs 5 length-t), requires f[0]=1. Rust sqrt 185878 0.134 recomputes full inv per Newton step = 5.4x gap. Effort M.
2. Kinoshita-Li O(n log^2 n) composition + power projection + compositional inverse (Lagrange inversion via log/exp) — port 242962 comp_work/comp_inv (Graeffe step Q(x)Q(-x) done inside the NTT domain: z[i]=y[2i]*y[2i+1]); Rust 356396 (0.040) is a plain-NTT reference. Both Rust compositional-inverse entries do Newton with two compositions per step: 49x algorithmic gap (0.789 vs 0.016). MISSING. Effort M.
3. Sparse FPS routines O(nk): one pow_sparse_unit with u64 lazy accumulation (<=16 products per reduce) and two accumulators (400499/400459 sparse_dot); inv/exp/log/pow via p*q' = k*p'*q, sqrt = k=1/2. Rust 342620 inv-sparse is fastest overall (0.024). MISSING. Effort S.
4. NTT constant factor: precomputed root tables, radix-4 DIF/DIT without bit reversal, Montgomery u32, target_feature avx2 8-lane butterflies (364801 lines 700-950, 342609 ntt_mont). 1.5-2x on everything (Rust 0.035 vs scalar 0.054 in 188145). Effort L.
5. Block semi-relaxed exp/log/sqrt/quo (261766 exp 14E(n), 261769 quo, 285048 sqrt, 382189 exp_14e_block; fold_adjacent_blocks trick) — further ~1.5x after 1 and 4. Effort M-L.
6. Multivariate truncated convolution + 2D inverse (Rust 349087 0.197 fastest overall, ~130 lines: chi-weight trick with k stacked NTTs). Effort S-M.
Also: 403367 detects <=24 nonzero terms in pow and switches to O(nk) sparse recurrence.
```

## Category report: polyalg

```
TOP INTEGRATION CANDIDATES (polynomial algorithms + linear recurrences)
1. NTT kernel: DIF/DIT without bit-reversal, cached twiddle tables, Montgomery mul, in-place spectrum API (transform / pointwise / inverse / doubling). Prerequisite for everything below; Rust tops (401767, 375167, 384459) expose NTT-domain ops. PrimeFFT::fft does bit reversal and recomputes twiddles per butterfly (w *= w_len). Taylor shift Rust 375167 0.039 beats all C++ with only this. Effort M.
2. Transposed (Tellegen) multipoint evaluation + interpolation on a DIF-domain subproduct tree with NTT doubling (340010 0.039, 261773 0.055; Rust 401767 0.069, 384459). ProductTree::evaluate/interpolate recompute a Newton inverse per remainder call (Rust 347515 0.376 = same algorithm as algo_lib). 3-5x. Also gives Newton-basis conversion (noshi91 140735) nearly free. Effort M.
3. Half-GCD for polynomials (250611 keeps 2x2 matrices in both coefficient and DIF form; 282926; 386276). Unlocks inv_of_polynomials (9.9x gap: 0.109 vs 1.081), find_linear_recurrence via half-GCD BM at 0.021 vs quadratic BM 0.061, root finding (250795), factorization (386276: squarefree + DDF + Cantor-Zassenhaus; no Rust AC). algo_lib has no poly gcd or Berlekamp-Massey at all. Effort L.
4. Bostan-Mori kth term and inverse-range in NTT domain (400681 0.069, 391482 0.206): Q(-x) is an adjacent-pair swap, even/odd extraction a pairwise combine, doubling instead of retransform; naive cutoff d<=24. ~2x over plain (Rust 365366 0.226 -> 342628 0.130). MISSING. Effort S given 1.
5. Small O(n log n) closed forms: Taylor shift (375167), sampling-point shift as one middle product (146817 0.041 vs Rust 0.113; replaces tree-based Interpolation::calculate_many for consecutive samples), chirp-z evaluation (146800 0.034) and inverse chirp-z interpolation (400323 0.090). Effort S each.
6. product_all helper splitting at cumulative-degree median with an arena (399144 0.082 vs Rust 0.212). Effort S.
Other: division_of_polynomials HAVE (private remainder; 285073 uses blocked relaxed division); sum_of_exponential_times_polynomial: O(d) closed form with sieve-powered i^d + Lagrange on consecutive points (394377 0.308).
```

## Category report: combi

```
TOP INTEGRATION CANDIDATES (enumerative combinatorics + LIS)
1. Montgomery ModInt (u32, R=2^32) + AVX2 8-lane multiply in numbers/mod_int/ — current MulAssign does u64 %. Evidence: prime-mod binomial Rust 216140 (hand-rolled Montgomery) 0.329 vs 291761 0.371 same algorithm; small_p Stirling 250952 0.089 vs Rust 341046 0.237 (Vec<Vec<u64>> with %); many_factorials Rust 289922 brute-forces 5e8 mulmods in 0.18s only via AVX2 Montgomery. 1.5-3x on every table-heavy problem. Effort M.
2. FPS ops exp, log, pow, taylor_shift, public inverse in numbers/polynomial.rs (PolynomialOps::inverse exists but private, non-SIMD). Unlocks bell_number (Rust 364958 0.064 fastest overall: exp(e^x-1)), partition (364948 pentagonal + inverse), sharp_p_subset_sum (Rust 383838 0.108), Stirling first kind via doubling with Taylor shift (201768/385635), fixed_k via pow of log series (201769), second kind via convolution of i^n/i! with linear sieve (385637), second fixed_k via inverse of prod(1-ix) (243208). Effort M.
3. Checkpointed factorial for many queries: precompute products to p/2 in blocks (289922 B=2048 brute; 382670 runtime finite-difference build of degree-128 block polynomial), Wilson reflection for n>p/2, backward walk with batch inversion (352581). 385399 0.005 embeds a 1MB table. Effort S after 1.
4. Granville prime-power binomial with discrete-log tables (log_fact, exp, Barrett quorem) + CRT (maspy 210472 0.108 vs Rust ExtLucas 56411 0.305). MISSING. Effort M.
5. Stirling small-p-large-n via Lucas closed form both kinds (252963/253133 0.08; O(p^2) table + Lucas). MISSING. Effort S.
6. LIS with branchless lower_bound (372659 0.010 f += comp*half; Rust 374603 0.035 partition_point) in misc/lis.rs + branchless lower_bound in slice_ext/bounds.rs; q-Lucas q-binomial (374861: order of q, q-factorials up to order, ordinary Lucas on quotients) add-on to Combinations. Effort S each.
Other: factorial (single n) leaders embed stride tables (contest hack); montmort trivial (I/O gap); number_of_subsequences HAVE (I/O); number_of_increasing_sequences D&C with grid-path convolutions (Rust 265133 0.284 = top; MISSING).
```

## Category report: string_linalg

```
TOP INTEGRATION CANDIDATES (strings + two_sat)
1. Runs (runenumerate): Lyndon-stack algorithm (Bannai et al.: two passes with opposite alphabet order, stack of Lyndon roots, verify via LCE) with 64-bit rolling-hash LCE (313634 0.029, 191550 0.022; 403379 0.012 naive SIMD LCE with budget + Main-Lorentz fallback). Rust 313199 Main-Lorentz 0.147. MISSING. Effort M.
2. Eertree with compact child storage (400577 cp-algo linked edges 0.069; 394036 per-node BST pool 0.068) + two-sided deque variant with series links (213580 0.065, no Rust AC). Rust 227654 0.383 with Vec<(char,usize)> children + Options. MISSING. Effort M.
3. SA-IS rewrite: sign-coded induced sort without type array, packed-prefix certificate skipping recursion, SIMD mismatch in Kasai (canard 403148 0.021 vs Rust 366409 0.035 AtCoder-style). suffix_array.rs::sais uses Vec<bool> type array and always recurses. Also speeds number_of_substrings and LCS. Effort M.
4. Wildcard matching via random-weight FFT correlation (282602 0.005: random real x_c on text, 1/x_c on reversed pattern, one f64 FFT, integer result iff match) or mod-NTT variant with 2 convolutions (283569 0.009). Rust 313987 0.157 does three NTTs of (s-p)^2 expansion. MISSING. Effort M (S with existing NTT).
5. Prefix-substring LCS seaweed h-array (~30 lines, 261415 0.037, no Rust AC); Duval Lyndon factorization (394259; Rust gap pure I/O). Effort S each.
6. 2-SAT / SCC: iterative Tarjan on CSR (313812 0.137 -> 210975 0.099) or propagate-with-rollback heuristic backed by SCC (170649 0.083, 403031 0.038); two_sat.rs uses recursive Kosaraju with a transposed Graph per solve (Rust 385333 0.121). Effort S-M. Aho-Corasick: bitmask children (Rust 373052 0.136 3rd overall) instead of [u32;K] per node (5x memory).
Other: longest_common_substring q-gram seed-and-extend with SIMD LCE (403347 0.013) vs SA+LCP (Rust 374584 0.088); SuffixAutomaton is map-based per state. z/manacher HAVE (I/O only).

TOP INTEGRATION CANDIDATES (linear algebra)
Baseline: Matrix::do_mult naive i-j-k with % per product; gauss/det/invert generic Field full RREF with swap per element and /= per row; no delayed reduction, no Montgomery, no GF(2) bit-packed elimination, no xor basis.
1. Delayed-reduction row kernel for gauss/det/inverse/solve: rows as u64, row[x] += f*pivot[x] unreduced, % every 16 accumulations, triangular-only elimination (361510 0.015 vs Rust %-per-op 184975 0.049). 3x. Effort S.
2. Matrix multiply: 4x8 register tile + u64 accumulators + one reduce per 8 products (leaf of 401223 0.068; 346349 tiled u64 kernel alone 0.056 on pow_of_matrix), Strassen above 64 later. Rust 176188 0.309 Strassen+Montgomery AVX2. 3-4x. Effort M (S without Strassen).
3. BitMatrix over GF(2) with M4RI product (256-table, 264541) and blocked elimination (Rust 402964-402968 all #1 overall: 0.029-0.097). Covers 5 problems where algo_lib has nothing. Effort M.
4. Frobenius-form pow_of_matrix + charpoly (401296 0.011 vs Rust 0.388; 401294 0.047): Krylov reduction, powmod(x,K,charpoly) per block. Hessenberg charpoly alone S-M (285937). Effort L.
5. Adjugate via bordered random matrix + one inverse returning det (401318 0.034 vs Rust 0.395). Effort S.
6. Sparse det via Markowitz sparse LU with bitset nonzero tracking (298117/Rust 299154 ~0.07; MISSING); xor-basis Zassenhaus intersection with packed u64 (206339 0.24 vs Rust 1.9 heap BitSet per vector; MISSING). Effort S each.
Other: det arbitrary mod via Euclid-style pivoting + Barrett (361230 0.049; MISSING); hafnian via cycle DP + subset convolution (Rust 118101 0.54 #1; MISSING).
```

## Category report: geo_bigint

```
TOP INTEGRATION CANDIDATES (geometry + big integers)
1. Replace UBigInt multiply with a double-FFT on 10^4 half-limbs: switch limb base to 10^8/u32 (389886 0.025, 192927 0.038 no SIMD), pack lo/hi 10^4 halves into real/imag of one complex double ("conj" symmetric convolution), one forward FFT per operand + one inverse. Rust 155658 0.314 (10^6 limbs, single Goldilocks NTT). Current: three-prime NTT + CRT on 10^9 limbs. 5-10x. Effort M.
2. Newton reciprocal + Burnikel-Ziegler division on top of (1): 391180 0.029 (GMP-style two-step, base-case 64 limbs, cyclic convolution mod B^m-1), 385970 0.105; 192927 compact _divmod_newton to model. No Rust AC. Effort M-L.
3. Big-int I/O: SWAR 8-byte parse (192927 load8 multiply-shift digit packing) and 2-digit/4-digit table print, whole-stdin read. Fastest Rust add 296277 (0.039) is per-decimal-digit; 335896 0.028 with trivial 10^18 i64 limbs + from_chars. 3-4x on addition. Effort S.
4. count_points_in_triangle CF-13D precompute: shift origin, angle-sort, count B points in each O-Ai-Aj triangle via sweep + Fenwick O(NM log M), O(1) queries by inclusion-exclusion (200617 0.079 vs Rust brute 374648 1.58). MISSING. Effort M.
5. Geometry primitives MISSING outright: exact argument-sort comparator (Rust 385808 0.016 fastest overall, plain sort_unstable_by with half-plane then i64 cross; only Point::angle() atan2 exists), rotating-calipers diameter (262824 0.036 vs Rust 0.114), Welzl MEC with exact-integer inside test (372832), Manhattan-MST 4-rotation sweep + Filter-Kruskal (Rust 259204 0.218 fastest overall), closest pair via random-sample lane sweep (361621 0.059 vs Rust D&C 0.18), all_furthest_neighbors monotone-minima D&C (402758, no Rust AC), convex_layers Overmars-van Leeuwen (210846 0.257 vs Rust 1.68), euclidean_mst via Delaunay (220576, no Rust AC). S-M each.
6. Convex hull speed-up: sort points as packed u64 keys with sort_unstable, dedup, inline i64 cross instead of Line construction (Rust 390244 0.034 = C++); ConvexHull in polygon.rs uses partial_cmp sort and Line objects in inner loop. Effort S.
Hex bigints: u64 limbs with SSE nibble decode + addcarry (391745 0.009), multiply via 14-bit chunks in double FFT (391969 0.017 vs Rust 201572 3.17 two-prime i32 NTT with % modmul), division tiers (395390). MISSING (needs binary limbs).
```

