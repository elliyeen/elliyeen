//! Renders GitHub's own contribution counts as a split-flap style board.
//! Input is a CSV exported from the GraphQL `contributionCalendar`:
//!   week,weekday,date,count
//! Nothing is invented: every tile is one real day, and the label total is the
//! sum of the rows. GitHub's native graph on the profile page stays authoritative.
//!
//!   cargo run --release -- --data ../../data/contributions.csv --out ../../assets/contributions.png

#[path = "../../splitflap/src/font.rs"]
mod font;

use std::{collections::HashMap, fs::File, io::BufWriter, path::PathBuf};

type Rgb = [u8; 3];
const TW: usize = 16;
const TH: usize = 20;
const HALF: usize = TH / 2;
const FRX: usize = 18;
const FT: usize = 18;
const FB: usize = 40;
const BOARD_BG: Rgb = [0x1B, 0x1B, 0x1B];
const SEAM: Rgb = [0x0A, 0x0A, 0x0A];
const EMPTY: Rgb = [0x2B, 0x2B, 0x2B];
const LEVELS: [Rgb; 4] = [[0x7A, 0x3A, 0x0C], [0xBF, 0x57, 0x00], [0xE8, 0x74, 0x1A], [0xFF, 0xA9, 0x4D]];

struct Pal {
    list: Vec<Rgb>,
    map: HashMap<Rgb, u8>,
}
impl Pal {
    fn idx(&mut self, c: Rgb) -> u8 {
        if let Some(&i) = self.map.get(&c) {
            return i;
        }
        assert!(self.list.len() < 256);
        let i = self.list.len() as u8;
        self.list.push(c);
        self.map.insert(c, i);
        i
    }
}

fn shade(c: Rgb, k: f64) -> Rgb {
    [(c[0] as f64 * k).round() as u8, (c[1] as f64 * k).round() as u8, (c[2] as f64 * k).round() as u8]
}

fn level(count: u32, max: u32) -> usize {
    if count == 0 {
        return 0;
    }
    ((4.0 * count as f64 / max as f64).ceil() as usize).clamp(1, 4)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).map(PathBuf::from);
    let data = arg("--data").expect("--data <csv>");
    let out = arg("--out").expect("--out <png>");

    let text = std::fs::read_to_string(&data).unwrap();
    let mut rows: Vec<(usize, usize, String, u32)> = vec![];
    for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = line.split(',').collect();
        rows.push((f[0].parse().unwrap(), f[1].parse().unwrap(), f[2].to_string(), f[3].parse().unwrap()));
    }
    let weeks = rows.iter().map(|r| r.0).max().unwrap() + 1;
    let total: u32 = rows.iter().map(|r| r.3).sum();
    let max = rows.iter().map(|r| r.3).max().unwrap().max(1);
    let first = rows.iter().map(|r| r.2.clone()).min().unwrap();
    let last = rows.iter().map(|r| r.2.clone()).max().unwrap();
    let active = rows.iter().filter(|r| r.3 > 0).count();

    let (w, h) = (weeks * TW + 2 * FRX, 7 * TH + FT + FB);
    let mut pal = Pal { list: vec![[0xFF, 0, 0xFF]], map: HashMap::new() };
    let c = |p: &mut Pal, v: Rgb| p.idx(v);
    let mid = c(&mut pal, [0xEC, 0xE9, 0xE4]);
    let hi = c(&mut pal, [0xFF, 0xFF, 0xFF]);
    let lo = c(&mut pal, [0xB9, 0xB5, 0xAE]);
    let dark = c(&mut pal, [0x2B, 0x2B, 0x2B]);
    let darker = c(&mut pal, [0x14, 0x14, 0x14]);
    let rim = c(&mut pal, [0x70, 0x6D, 0x68]);
    let ink = c(&mut pal, [0x4A, 0x4A, 0x4A]);
    let well = c(&mut pal, BOARD_BG);
    let mut cv = vec![mid; w * h];
    let mut rect = |cv: &mut Vec<u8>, x: usize, y: usize, rw: usize, rh: usize, v: u8| {
        for yy in y..(y + rh).min(h) {
            for xx in x..(x + rw).min(w) {
                cv[yy * w + xx] = v;
            }
        }
    };
    rect(&mut cv, 0, 0, w, 3, hi);
    rect(&mut cv, 0, 0, 3, h, hi);
    rect(&mut cv, 0, h - 3, w, 3, lo);
    rect(&mut cv, w - 3, 0, 3, h, lo);
    let (bx, by, bw, bh) = (FRX - 8, FT - 8, weeks * TW + 16, 7 * TH + 16);
    rect(&mut cv, bx, by, bw, bh, dark);
    rect(&mut cv, bx, by, bw, 2, darker);
    rect(&mut cv, bx, by, 2, bh, darker);
    rect(&mut cv, bx, by + bh - 2, bw, 2, rim);
    rect(&mut cv, bx + bw - 2, by, 2, bh, rim);
    rect(&mut cv, FRX - 2, FT - 2, weeks * TW + 4, 7 * TH + 4, well);

    // one real day per tile
    let mut grid: Vec<Vec<Option<u32>>> = vec![vec![None; 7]; weeks];
    for (wk, wd, _, n) in &rows {
        grid[*wk][*wd] = Some(*n);
    }
    for (wk, col) in grid.iter().enumerate() {
        for (wd, cell) in col.iter().enumerate() {
            let (x0, y0) = (FRX + wk * TW, FT + wd * TH);
            for y in 0..TH {
                for x in 0..TW {
                    let edge = x == 0 || x == TW - 1 || y == 0 || y == TH - 1;
                    let v = match cell {
                        None => BOARD_BG,
                        _ if edge => BOARD_BG,
                        Some(n) => {
                            let base = if *n == 0 { EMPTY } else { LEVELS[level(*n, max) - 1] };
                            if y == HALF { SEAM } else if y < HALF { base } else { shade(base, 0.9) }
                        }
                    };
                    let i = c(&mut pal, v);
                    cv[(y0 + y) * w + x0 + x] = i;
                }
            }
        }
    }

    // label plate
    let label = format!("{} CONTRIBUTIONS, {} TO {}", total, first, last);
    let (scale, gap) = (2usize, 5usize);
    let lw = label.len() * (5 * scale + gap) - gap;
    let (lx, ly) = ((w.saturating_sub(lw)) / 2, FT + 7 * TH + 16);
    let mut text = |cv: &mut Vec<u8>, s: &str, x0: usize, y0: usize, v: u8| {
        let mut x = x0;
        for ch in s.chars() {
            let g = font::glyph(ch);
            for (gy, row) in g.iter().enumerate() {
                for gx in 0..5 {
                    if row & (1 << (4 - gx)) != 0 {
                        for dy in 0..scale {
                            for dx in 0..scale {
                                let (px, py) = (x + gx * scale + dx, y0 + gy * scale + dy);
                                if px < w && py < h {
                                    cv[py * w + px] = v;
                                }
                            }
                        }
                    }
                }
            }
            x += 5 * scale + gap;
        }
    };
    text(&mut cv, &label, lx + 1, ly + 1, hi);
    text(&mut cv, &label, lx, ly, ink);

    // rounded transparent corners
    let rad = 16i32;
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let cx = x.clamp(rad, w as i32 - 1 - rad);
            let cy = y.clamp(rad, h as i32 - 1 - rad);
            if (x - cx).pow(2) + (y - cy).pow(2) > rad * rad {
                cv[y as usize * w + x as usize] = 0;
            }
        }
    }

    let flat: Vec<u8> = pal.list.iter().flatten().copied().collect();
    let mut e = png::Encoder::new(BufWriter::new(File::create(&out).unwrap()), w as u32, h as u32);
    e.set_color(png::ColorType::Indexed);
    e.set_depth(png::BitDepth::Eight);
    e.set_palette(flat);
    e.set_trns(vec![0u8]);
    e.write_header().unwrap().write_image_data(&cv).unwrap();
    println!("{}x{} px, {} days, {} active, total {}, max/day {}, {} to {}", w, h, rows.len(), active, total, max, first, last);
}
