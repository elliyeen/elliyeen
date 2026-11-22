# Split-flap board generator

Generates the animated ELLIYEEN board at the top of the profile README.
GitHub READMEs do not run JavaScript, so the board is an offline-rendered GIF.
A static PNG is served to visitors who prefer reduced motion.

```sh
cd tools/splitflap
cargo run --release -- --out ../../assets            # splitflap.gif + splitflap-static.png
cargo run --release -- --out ../../assets --stills /tmp/stills   # also dump review stills
```

- Deterministic: same source, same bytes. Two dependencies (`gif`, `png`), no network at runtime.
- Run locally when content changes. There is no workflow, so no recurring CI cost.
- Messages, language order and colours: `src/scenes.rs`. Tile flip rendering and framing: `src/main.rs`.
- Board is 8 rows x 48 columns of 16x20 px tiles, in a rounded light frame with transparent corners.
- Scene changes shuffle every tile through several colours before it settles; ongoing motion uses short single flips so moving art stays legible.
- Loop is exactly 60 s: five 5 s scenes (message, beach, Earth, F1 left to right, DART right to left),
  then seven 5 s language slots (Rust, TypeScript, JavaScript, HTML, Python, Go, CSS).
- The generator asserts the loop length and the 256-colour palette limit.
- This board is decoration. It does not display or imitate GitHub activity.
