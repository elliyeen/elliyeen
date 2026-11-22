//! ELLIYEEN split-flap board generator.
//!
//! Renders a deterministic 60 s loop to an animated GIF (GitHub READMEs do not
//! run JavaScript) plus a static PNG for `prefers-reduced-motion`.
//!
//!   cargo run --release -- --out ../../assets [--stills /tmp/stills]

mod font;
mod scenes;

use scenes::*;
use std::{borrow::Cow, collections::HashMap, fs::File, io::BufWriter, path::PathBuf};

const HALF: usize = TH / 2;
const FRX: usize = 18; // frame width left/right
const FT: usize = 18; // frame top
const FB: usize = 36; // frame bottom (label plate)
const TRANSPARENT: u8 = 0; // palette slot 0 is reserved for the rounded corners
const W: usize = COLS * TW + 2 * FRX;
const H: usize = ROWS * TH + FT + FB;
const STEP: f64 = 0.05; // 20 fps simulation; identical frames are merged
const STEP_CS: u16 = 5;

const BOARD_BG: Rgb = [0x1B, 0x1B, 0x1B];
const SEAM: Rgb = [0x0A, 0x0A, 0x0A];

struct Pal {
    list: Vec<Rgb>,
    map: HashMap<Rgb, u8>,
}

impl Pal {
    fn idx(&mut self, c: Rgb) -> u8 {
        if let Some(&i) = self.map.get(&c) {
            return i;
        }
        assert!(self.list.len() < 256, "palette overflow (>256 colours)");
        let i = self.list.len() as u8;
        self.list.push(c);
        self.map.insert(c, i);
        i
    }
}

fn shade(c: Rgb, k: f64) -> Rgb {
    [(c[0] as f64 * k).round() as u8, (c[1] as f64 * k).round() as u8, (c[2] as f64 * k).round() as u8]
}

fn make_face(t: &Tile, pal: &mut Pal) -> Vec<u8> {
    let mut px = vec![BOARD_BG; TW * TH];
    for y in 1..TH - 1 {
        for x in 1..TW - 1 {
            px[y * TW + x] = if y < HALF { t.bg } else { shade(t.bg, 0.9) };
        }
    }
    if t.ch != ' ' {
        let g = font::glyph(t.ch);
        let (x0, y0) = (2usize, 2usize);
        for (gy, row) in g.iter().enumerate() {
            for gx in 0..5 {
                if row & (1 << (4 - gx)) != 0 {
                    // 2x2 cells grown by 1px: bold stroke that fills the flap
                    for dy in 0..3 {
                        for dx in 0..3 {
                            let (x, y) = (x0 + gx * 2 + dx, y0 + gy * 2 + dy);
                            if x < TW - 1 && y < TH - 1 {
                                px[y * TW + x] = t.fg;
                            }
                        }
                    }
                }
            }
        }
    }
    for x in 1..TW - 1 {
        px[HALF * TW + x] = SEAM; // the flap seam cuts across the glyph
    }
    px.into_iter().map(|c| pal.idx(c)).collect()
}

fn compose(from: &[u8], to: &[u8], p: f64) -> Vec<u8> {
    let mut out = vec![0u8; TW * TH];
    out[..HALF * TW].copy_from_slice(&to[..HALF * TW]);
    out[HALF * TW..].copy_from_slice(&from[HALF * TW..]);
    if p < 0.5 {
        // old top flap folds down toward the seam
        let hf = ((HALF as f64) * (1.0 - 2.0 * p)).round() as usize;
        for i in 0..hf {
            let y = HALF - hf + i;
            let sy = i * HALF / hf;
            out[y * TW..(y + 1) * TW].copy_from_slice(&from[sy * TW..(sy + 1) * TW]);
        }
    } else {
        // new bottom flap swings down from the seam
        let hf = ((HALF as f64) * (2.0 * p - 1.0)).round() as usize;
        for i in 0..hf {
            let y = HALF + i;
            let sy = HALF + i * HALF / hf;
            out[y * TW..(y + 1) * TW].copy_from_slice(&to[sy * TW..(sy + 1) * TW]);
        }
    }
    out
}

struct Anim {
    seq: Vec<Tile>, // from, scramble..., to
    start: f64,
    each: f64, // seconds per flap
}
struct Cell {
    cur: Tile,
    anim: Option<Anim>,
}

fn rect(cv: &mut [u8], x0: usize, y0: usize, w: usize, h: usize, c: u8) {
    for y in y0..(y0 + h).min(H) {
        for x in x0..(x0 + w).min(W) {
            cv[y * W + x] = c;
        }
    }
}

fn draw_text(cv: &mut [u8], s: &str, x0: usize, y0: usize, scale: usize, gap: usize, c: u8) {
    let mut x = x0;
    for ch in s.chars() {
        let g = font::glyph(ch);
        for (gy, row) in g.iter().enumerate() {
            for gx in 0..5 {
                if row & (1 << (4 - gx)) != 0 {
                    rect(cv, x + gx * scale, y0 + gy * scale, scale, scale, c);
                }
            }
        }
        x += 5 * scale + gap;
    }
}

/// Light, beveled, rounded frame with an engraved ELLIYEEN plate under the board.
fn draw_frame(pal: &mut Pal) -> Vec<u8> {
    let mid = pal.idx([0xEC, 0xE9, 0xE4]);
    let hi = pal.idx([0xFF, 0xFF, 0xFF]);
    let lo = pal.idx([0xB9, 0xB5, 0xAE]);
    let dark = pal.idx([0x2B, 0x2B, 0x2B]);
    let darker = pal.idx([0x14, 0x14, 0x14]);
    let rim = pal.idx([0x70, 0x6D, 0x68]);
    let ink = pal.idx([0x4A, 0x4A, 0x4A]);
    let screw = pal.idx([0xC9, 0xC4, 0xBB]);
    let mut cv = vec![mid; W * H];
    rect(&mut cv, 0, 0, W, 3, hi);
    rect(&mut cv, 0, 0, 3, H, hi);
    rect(&mut cv, 0, H - 3, W, 3, lo);
    rect(&mut cv, W - 3, 0, 3, H, lo);
    // recessed board well
    let (bx, by, bw, bh) = (FRX - 8, FT - 8, COLS * TW + 16, ROWS * TH + 16);
    rect(&mut cv, bx, by, bw, bh, dark);
    rect(&mut cv, bx, by, bw, 2, darker);
    rect(&mut cv, bx, by, 2, bh, darker);
    rect(&mut cv, bx, by + bh - 2, bw, 2, rim);
    rect(&mut cv, bx + bw - 2, by, 2, bh, rim);
    rect(&mut cv, FRX - 2, FT - 2, COLS * TW + 4, ROWS * TH + 4, pal.idx(BOARD_BG));
    // engraved label under the board
    let label = "ELLIYEEN";
    let (scale, gap) = (2usize, 6usize);
    let lw = label.len() * (5 * scale + gap) - gap;
    let (lx, ly) = ((W - lw) / 2, FT + ROWS * TH + 14);
    draw_text(&mut cv, label, lx + 1, ly + 1, scale, gap, hi);
    draw_text(&mut cv, label, lx, ly, scale, gap, ink);
    // small screws either side of the label
    for &sx in &[FRX + 10, W - FRX - 11] {
        let sy = ly + 7;
        for dy in 0..7usize {
            for dx in 0..7usize {
                let (ox, oy) = (dx as i32 - 3, dy as i32 - 3);
                if ox * ox + oy * oy <= 10 {
                    cv[(sy + dy - 3) * W + sx + dx - 3] = screw;
                }
            }
        }
        rect(&mut cv, sx - 2, sy, 5, 1, lo);
    }
    // rounded corners: transparent outside the arc
    let rad = 16i32;
    for y in 0..H as i32 {
        for x in 0..W as i32 {
            let cx = x.clamp(rad, W as i32 - 1 - rad);
            let cy = y.clamp(rad, H as i32 - 1 - rad);
            if (x - cx) * (x - cx) + (y - cy) * (y - cy) > rad * rad {
                cv[y as usize * W + x as usize] = TRANSPARENT;
            }
        }
    }
    cv
}

fn stagger(r: usize, c: usize) -> f64 {
    let jitter = (((r * 31 + c * 17) % 11) as f64) * 0.008;
    c as f64 * 0.012 + r as f64 * 0.02 + jitter
}

struct Out {
    frames: Vec<(u16, u16, u16, u16, Vec<u8>, u16)>, // left, top, w, h, indices, delay(cs)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).map(PathBuf::from);
    let out_dir = arg("--out").unwrap_or_else(|| PathBuf::from("."));
    let stills_dir = arg("--stills");
    std::fs::create_dir_all(&out_dir).unwrap();
    if let Some(d) = &stills_dir {
        std::fs::create_dir_all(d).unwrap();
    }

    let mut pal = Pal { list: vec![[0xFF, 0x00, 0xFF]], map: HashMap::new() }; // slot 0 = transparent corners
    let frame_bg = draw_frame(&mut pal);
    let mut faces: HashMap<Tile, Vec<u8>> = HashMap::new();

    // Start from the state at the very end of the loop so the loop is seamless.
    let (end_board, _) = board_at(LOOP_SECS - STEP);
    let mut cells: Vec<Cell> = end_board.iter().map(|&t| Cell { cur: t, anim: None }).collect();

    let mut out = Out { frames: vec![] };
    let mut prev: Vec<u8> = vec![];
    let mut stills: Vec<(String, Vec<u8>)> = vec![];
    let still_times: Vec<f64> = (0..12).map(|i| i as f64 * 5.0 + 2.5).chain([0.3, 0.6, 5.3, 12.3]).collect();
    let steps = (LOOP_SECS / STEP).round() as usize;

    for step in 0..steps {
        let t = step as f64 * STEP;
        let (target, tl) = board_at(t);
        let mut cv = frame_bg.clone();
        for r in 0..ROWS {
            for c in 0..COLS {
                let i = r * COLS + c;
                let cell = &mut cells[i];
                if let Some(a) = &cell.anim {
                    if t >= a.start + a.each * (a.seq.len() - 1) as f64 {
                        cell.cur = *a.seq.last().unwrap();
                        cell.anim = None;
                    }
                }
                if cell.anim.is_none() && cell.cur != target[i] {
                    let mut seq = vec![cell.cur];
                    let mut each = 0.07; // ongoing motion: quick flips so moving art stays legible
                    let start = if tl < 0.15 {
                        each = 0.15;
                        // scene change: tiles shuffle through a few colours before settling
                        let n = 1 + (hash(r, c) as usize % 3) + c * 2 / COLS;
                        for k in 0..n {
                            let j = hash(r + k * 7, c * 13 + step) as usize % target.len();
                            seq.push(Tile::solid(target[j].bg));
                        }
                        t + stagger(r, c)
                    } else {
                        t
                    };
                    seq.push(target[i]);
                    cell.anim = Some(Anim { seq, start, each });
                }
                let px: Vec<u8> = match &cell.anim {
                    None => faces.entry(cell.cur).or_insert_with(|| make_face(&cell.cur, &mut pal)).clone(),
                    Some(a) => {
                        let u = ((t - a.start) / a.each).max(0.0);
                        let k = (u.floor() as usize).min(a.seq.len() - 2);
                        let p = (u - k as f64).clamp(0.0, 1.0);
                        let (ft, tt) = (a.seq[k], a.seq[k + 1]);
                        let f = faces.entry(ft).or_insert_with(|| make_face(&ft, &mut pal)).clone();
                        if u <= 0.0 {
                            f
                        } else {
                            let to = faces.entry(tt).or_insert_with(|| make_face(&tt, &mut pal)).clone();
                            compose(&f, &to, p)
                        }
                    }
                };
                let (ox, oy) = (FRX + c * TW, FT + r * TH);
                for y in 0..TH {
                    let o = (oy + y) * W + ox;
                    cv[o..o + TW].copy_from_slice(&px[y * TW..(y + 1) * TW]);
                }
            }
        }
        if still_times.iter().any(|&s| (s - t).abs() < STEP / 2.0) {
            stills.push((format!("t{:05.2}.png", t), cv.clone()));
        }
        if prev == cv {
            out.frames.last_mut().unwrap().5 += STEP_CS;
            continue;
        }
        let (mut x0, mut y0, mut x1, mut y1) = (W, H, 0, 0);
        if prev.is_empty() {
            (x0, y0, x1, y1) = (0, 0, W - 1, H - 1);
        } else {
            for y in 0..H {
                for x in 0..W {
                    if prev[y * W + x] != cv[y * W + x] {
                        x0 = x0.min(x);
                        x1 = x1.max(x);
                        y0 = y0.min(y);
                        y1 = y1.max(y);
                    }
                }
            }
        }
        let (w, h) = (x1 - x0 + 1, y1 - y0 + 1);
        let mut buf = Vec::with_capacity(w * h);
        for y in y0..=y1 {
            buf.extend_from_slice(&cv[y * W + x0..y * W + x1 + 1]);
        }
        out.frames.push((x0 as u16, y0 as u16, w as u16, h as u16, buf, STEP_CS));
        prev = cv;
    }

    let total_cs: u32 = out.frames.iter().map(|f| f.5 as u32).sum();
    assert_eq!(total_cs, (LOOP_SECS * 100.0) as u32, "loop must be exactly 60 s");

    let flat: Vec<u8> = pal.list.iter().flatten().copied().collect();
    let gif_path = out_dir.join("splitflap.gif");
    {
        let mut enc = gif::Encoder::new(BufWriter::new(File::create(&gif_path).unwrap()), W as u16, H as u16, &flat).unwrap();
        enc.set_repeat(gif::Repeat::Infinite).unwrap();
        for (left, top, w, h, buf, delay) in &out.frames {
            let f = gif::Frame {
                left: *left,
                top: *top,
                width: *w,
                height: *h,
                delay: *delay,
                dispose: gif::DisposalMethod::Keep,
                transparent: Some(TRANSPARENT),
                buffer: Cow::Borrowed(buf),
                ..gif::Frame::default()
            };
            enc.write_frame(&f).unwrap();
        }
    }

    let write_png = |path: PathBuf, cv: &[u8]| {
        let mut e = png::Encoder::new(BufWriter::new(File::create(path).unwrap()), W as u32, H as u32);
        e.set_color(png::ColorType::Indexed);
        e.set_depth(png::BitDepth::Eight);
        e.set_palette(flat.clone());
        e.set_trns(vec![0u8]);
        e.write_header().unwrap().write_image_data(cv).unwrap();
    };
    // Static fallback = the first message, fully flipped in (t = 2.5 s)
    let static_cv = stills.iter().find(|(n, _)| n == "t02.50.png").expect("static still").1.clone();
    write_png(out_dir.join("splitflap-static.png"), &static_cv);
    if let Some(d) = &stills_dir {
        for (n, cv) in &stills {
            write_png(d.join(n), cv);
        }
    }

    let sz = std::fs::metadata(&gif_path).unwrap().len();
    println!(
        "{}x{} px, {} frames, {:.1} s loop, {} colours, GIF {:.2} MB",
        W,
        H,
        out.frames.len(),
        total_cs as f64 / 100.0,
        pal.list.len(),
        sz as f64 / 1_048_576.0
    );
}
