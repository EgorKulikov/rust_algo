# 42nd Cloudflight Coding Contest (Classic), 2026-10-23

Solo entry, AI-assisted (allowed by the rules). Game: "Nanofabricator" (hex board, six emitters
on the border ring, beams create matter where exactly two cross and destroy it where three or
more cross). Levels 1-5 accepted; level 6 (shortest programs producing each pattern `count`
times) is the scored optimization level. Final level-6 score 18129 (medium 6043, large 5817,
enormous 6269; the `1-small` file is not scored), leaders ~14.9k.

Layout:

- `ccc.py` - client for codingcontest.org (login, game token, statements, inputs, submit).
  Credentials are read from a `creds.txt` (`login=`/`pass=`) that is not archived.
- `submit_if_better.py`, `merge_best.py`, `merge_partial.py` - submit only when fewer lines
  than the server best; merge the best program per job across several solver outputs; overlay
  per-job partial outputs (`NANO_PARTIAL`) onto a full output.
- `nano_rs/` - Rust solver (`nano <level> <input> <output>`; `nano 7 <input> <program> <trace>`
  replays a program tick by tick). `level6h.rs` is the tick-level beam search that halved every
  file; `level6j.rs` the sweep+beam hybrid; `level6i.rs` the exact radius-3 solver;
  `level6b`-`level6g` the earlier phase planners (still run as a baseline). Behaviour is
  controlled by environment variables (`NANO_BEAM`, `NANO_BEAM_SETS`, `NANO_GUIDE`,
  `NANO_EXACT`, `NANO_THREADS`, `NANO_PARTIAL`, `NANO_STATS`, ...), see `level6.rs`.
- `nano/levelN/` - statements, inputs, organizer example outputs, submitted outputs
  (`my_*` for levels 1-5, `final_level-6_*` for level 6), hand-made test slices.
- `spaceship/`, `spaceship_rs/` - the training game ("Spaceship", CCC41) used to verify the
  read/submit path before the contest: Python prototypes and the Rust port.
