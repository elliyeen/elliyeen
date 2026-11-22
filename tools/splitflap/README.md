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
- Loop is exactly 80 s, sixteen 5 s scenes in this order: greeting, `DO NO HARM.`, `ALWAYS DO WHAT IS RIGHT.`,
  `Culture eats strategy for breakfast.` with `— Mark Fields` on its own line, `The joint doesn't know that it will be visable.`
  (verbatim, including "visable", until the owner authorises a correction), beach, Earth, F1 left to right,
  DART right to left, then seven language slots (Rust, TypeScript, JavaScript, HTML, Python, Go, CSS).
- Sentence-case scenes use a 9-row lowercase glyph cell (`glyph9` in `src/font.rs`) so letters and descenders render.
- Attribution: "— Mark Fields" is as requested by the owner. The quotation is also often credited to Peter Drucker and has not been verified against a primary source, so treat the attribution as unconfirmed.
- The generator asserts the loop length and the 256-colour palette limit.
- This board is decoration. It does not display or imitate GitHub activity.
