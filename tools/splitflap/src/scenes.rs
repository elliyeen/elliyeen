//! Scene definitions. `board_at(t)` is a pure function of time: the target
//! tile grid for second `t` of the 60 s loop. The renderer animates flips.

pub type Rgb = [u8; 3];

pub const ROWS: usize = 8;
pub const COLS: usize = 36;
pub const SCENE_SECS: f64 = 5.0;
pub const LOOP_SECS: f64 = 60.0;

/// Ordered language loop (spec: RUST → TYPESCRIPT → JAVASCRIPT → HTML → PYTHON → GO → CSS).
pub const LANGS: [(&str, Rgb); 7] = [
    ("RUST", [0xFF, 0x8C, 0x1A]),
    ("TYPESCRIPT", [0x3B, 0x82, 0xF6]),
    ("JAVASCRIPT", [0xF7, 0xDF, 0x1E]),
    ("HTML", [0xFF, 0x4D, 0x4D]),
    ("PYTHON", [0x46, 0xC2, 0xA0]),
    ("GO", [0x5C, 0xD6, 0xFF]),
    ("CSS", [0xC0, 0x84, 0xFC]),
];

pub const ORANGE: Rgb = [0xBF, 0x57, 0x00]; // Texas burnt orange
pub const WHITE: Rgb = [0xFF, 0xFF, 0xFF];
const BLACK: Rgb = [0x11, 0x13, 0x12];
const PROMPT_GREEN: Rgb = [0x33, 0xFF, 0x66];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Tile {
    pub bg: Rgb,
    pub ch: char,
    pub fg: Rgb,
}

impl Tile {
    pub fn solid(bg: Rgb) -> Tile {
        Tile { bg, ch: ' ', fg: WHITE }
    }
}

pub type Board = Vec<Tile>;

fn new_board(bg: Rgb) -> Board {
    vec![Tile::solid(bg); ROWS * COLS]
}

fn set(b: &mut Board, r: i64, c: i64, bg: Rgb) {
    if r >= 0 && c >= 0 && (r as usize) < ROWS && (c as usize) < COLS {
        b[r as usize * COLS + c as usize] = Tile::solid(bg);
    }
}

fn put_text(b: &mut Board, r: usize, c: usize, s: &str, fg: Rgb, bg: Rgb) {
    for (i, ch) in s.chars().enumerate() {
        if c + i < COLS && r < ROWS {
            b[r * COLS + c + i] = Tile { bg, ch, fg };
        }
    }
}

/// Stamp an ASCII sprite. `.` is transparent; `colour` maps other chars.
fn stamp(b: &mut Board, art: &[&str], top: i64, left: i64, colour: &dyn Fn(char) -> Option<Rgb>) {
    for (dr, line) in art.iter().enumerate() {
        for (dc, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            if let Some(c) = colour(ch) {
                set(b, top + dr as i64, left + dc as i64, c);
            }
        }
    }
}

fn hash(r: usize, c: usize) -> u32 {
    let mut x = (r as u32).wrapping_mul(73856093) ^ (c as u32).wrapping_mul(19349663) ^ 0x9E3779B9;
    x ^= x >> 13;
    x = x.wrapping_mul(0x85EBCA6B);
    x ^ (x >> 16)
}

/// Returns (target board, seconds since the current scene/slot began).
pub fn board_at(t: f64) -> (Board, f64) {
    let t = t.rem_euclid(LOOP_SECS);
    let idx = ((t / SCENE_SECS) as usize).min(11);
    let tl = t - idx as f64 * SCENE_SECS;
    let b = match idx {
        0 => peace(),
        1 => beach(t),
        2 => earth(tl),
        3 => f1(tl),
        4 => dart(tl),
        i => language(i - 5, tl),
    };
    (b, tl)
}

// ---------- 1. PEACE WORLD! I'M ABDULLAH ----------
pub fn peace() -> Board {
    let mut b = new_board(ORANGE);
    put_text(&mut b, 3, 12, "PEACE WORLD!", WHITE, ORANGE);
    put_text(&mut b, 4, 12, "I'M ABDULLAH", WHITE, ORANGE);
    b
}

// ---------- 2. Tropical beach ----------
fn beach(t: f64) -> Board {
    let sky: [Rgb; 4] = [[0x2A, 0x1B, 0x52], [0x7A, 0x2E, 0x86], [0xD9, 0x54, 0x4F], [0xF4, 0xA3, 0x40]];
    let sea: [Rgb; 4] = [[0x2F, 0x7D, 0xB8], [0x26, 0x6C, 0xA3], [0x1D, 0x5A, 0x8C], [0x15, 0x48, 0x76]];
    let sun: Rgb = [0xFF, 0xE0, 0x66];
    let glint: Rgb = [0xF2, 0xB8, 0x4B];
    let sand: Rgb = [0xE9, 0xD2, 0x9A];
    let wet: Rgb = [0xC9, 0xAE, 0x78];
    let mut b = new_board(BLACK);
    for r in 0..4 {
        for c in 0..COLS {
            set(&mut b, r as i64, c as i64, sky[r]);
        }
    }
    for c in 23..29 {
        set(&mut b, 3, c, sun);
    }
    for c in 24..28 {
        set(&mut b, 2, c, sun);
    }
    for r in 4..8 {
        for c in 0..COLS {
            // waves roll in toward the shore (leftwards)
            let w = ((c as f64) * 0.55 + (r as f64) * 0.9 + t * 3.2).sin();
            let col = if w > 0.93 {
                WHITE
            } else if w > 0.72 {
                [0xA9, 0xD6, 0xF0]
            } else if r == 4 && (24..28).contains(&c) {
                glint
            } else if r == 5 && (25..27).contains(&c) {
                glint
            } else {
                sea[r - 4]
            };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    // sand with a breathing shoreline
    let shore = 13 + (1.5 * (t * 1.6).sin()).round() as i64;
    for r in 6..8i64 {
        let edge = shore + (r - 6);
        for c in 0..edge {
            set(&mut b, r, c, if c >= edge - 2 { wet } else { sand });
        }
        set(&mut b, r, edge, WHITE);
    }
    // palm tree
    let palm = [
        "..gGGGGg....",
        ".gGGgGGGGg..",
        "gG.GgTGg.Gg.",
        "G....T....G.",
        ".....T......",
        "....TT......",
        "....T.......",
    ];
    stamp(&mut b, &palm, 0, 3, &|ch| match ch {
        'G' => Some([0x34, 0x9B, 0x3F]),
        'g' => Some([0x1F, 0x6B, 0x2E]),
        'T' => Some([0x7A, 0x4A, 0x21]),
        _ => None,
    });
    b
}

// ---------- 3. Earth ----------
fn earth(tl: f64) -> Board {
    let space: Rgb = [0x05, 0x07, 0x14];
    let mut b = new_board(space);
    // stars (static, deterministic)
    for r in 0..ROWS {
        for c in 0..COLS {
            let h = hash(r, c);
            if h % 31 == 0 {
                let s = if h % 2 == 0 { [0xFF, 0xFF, 0xFF] } else { [0x9F, 0xB4, 0xFF] };
                set(&mut b, r as i64, c as i64, s);
            }
        }
    }
    let (tw, th) = (20.0, 24.0);
    let cx = COLS as f64 * tw / 2.0;
    let cy = ROWS as f64 * th / 2.0;
    let rad = 90.0;
    let rot = tl * 0.35; // ~20 deg/s subtle spin
    for r in 0..ROWS {
        for c in 0..COLS {
            let x = (c as f64 + 0.5) * tw - cx;
            let y = (r as f64 + 0.5) * th - cy;
            let d = (x * x + y * y).sqrt() / rad;
            if d > 1.0 {
                if d < 1.1 {
                    set(&mut b, r as i64, c as i64, [0x16, 0x2B, 0x5C]); // atmosphere rim
                }
                continue;
            }
            let z = (1.0 - d * d).sqrt();
            let lon = (x / rad).atan2(z) + rot;
            let lat = (-y / rad).asin();
            let n = (lon * 2.0 + 0.5).sin() * (lat * 1.5).cos()
                + 0.6 * (lon * 3.0 - lat * 2.0 + 1.0).sin()
                + 0.4 * (lon * 5.0 + lat * 4.0).sin();
            let edge = d > 0.82;
            let cloud = (lon * 4.0 + lat * 6.0 + rot * 1.5 + 2.0).sin() * (lon * 3.0 - lat * 5.0).sin() > 0.6;
            let col = if lat.abs() > 1.25 || cloud {
                if edge { [0xC9, 0xD6, 0xEE] } else { [0xF2, 0xF6, 0xFF] }
            } else if n > 0.5 {
                if edge { [0x2A, 0x7A, 0x3E] } else { [0x3D, 0xAA, 0x55] }
            } else if edge {
                [0x1B, 0x4D, 0xA8]
            } else {
                [0x2A, 0x6F, 0xDB]
            };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    b
}

// ---------- 4. Formula 1 car, left → right ----------
fn f1(tl: f64) -> Board {
    let mut b = new_board(BLACK);
    let sky: Rgb = [0x6F, 0xA8, 0xDC];
    let asphalt: Rgb = [0x34, 0x37, 0x3F];
    for r in 0..3 {
        for c in 0..COLS {
            set(&mut b, r, c as i64, sky);
        }
    }
    for c in 0..COLS {
        // grandstand crowd
        let h = hash(3, c);
        let crowd = match h % 4 {
            0 => [0xD9, 0x3A, 0x3A],
            1 => [0xF2, 0xF2, 0xF2],
            2 => [0x3D, 0x5A, 0x80],
            _ => [0x59, 0x5F, 0x6B],
        };
        set(&mut b, 3, c as i64, crowd);
        // scrolling red/white kerb
        let kerb = if (c + (tl * 4.0) as usize) % 2 == 0 { [0xD9, 0x1E, 0x2B] } else { WHITE };
        set(&mut b, 4, c as i64, kerb);
        for r in 5..8 {
            set(&mut b, r, c as i64, asphalt);
        }
        if (c + (tl * 4.0) as usize) % 4 < 2 {
            set(&mut b, 7, c as i64, [0xCF, 0xCF, 0xCF]);
        }
    }
    let car = [
        "KK.....RRWY.....",
        "KRRRRRRRRRRRRR..",
        "RRRRRRRRRRRRRRRW",
        ".KKK.RRRRR.KKK.K",
        ".KKK.......KKK..",
    ];
    let x = -16.0 + 52.0 * (tl / 5.0);
    stamp(&mut b, &car, 3, x.floor() as i64, &|ch| match ch {
        'R' => Some([0xE1, 0x06, 0x00]),
        'K' => Some([0x10, 0x10, 0x10]),
        'W' => Some(WHITE),
        'Y' => Some([0xFF, 0xD2, 0x1F]),
        _ => None,
    });
    b
}

// ---------- 5. DART light-rail, right → left ----------
fn dart(tl: f64) -> Board {
    let night: Rgb = [0x3A, 0x4F, 0x86];
    let mut b = new_board(night);
    let bld: Rgb = [0x0A, 0x10, 0x20];
    let lit: Rgb = [0xFF, 0xD1, 0x66];
    // Dallas skyline: building heights per column (rows tall, bottom at row 3)
    let heights: [usize; COLS] = [
        2, 2, 3, 3, 2, 4, 4, 3, 2, 1, 2, 3, 4, 4, 3, 2, 3, 4, 4, 4, 3, 2, 2, 3, 4, 3, 3, 2, 3, 4, 4, 3, 2, 2, 3, 2,
    ];
    for c in 0..COLS {
        let h = heights[c];
        for r in (4 - h)..4 {
            let col = if hash(r, c) % 7 == 0 { lit } else { bld };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    // Reunion Tower: thin stalk + ball
    for r in 1..4 {
        set(&mut b, r, 9, bld);
    }
    set(&mut b, 0, 9, [0x7D, 0xD3, 0xFC]);
    // track
    for c in 0..COLS {
        set(&mut b, 7, c as i64, [0x4A, 0x4A, 0x4A]);
    }
    let train = [
        ".BBBBBBBBBBBBBBBBBBBBBBB",
        "WBWWBWWBWWBWWBWWBWWBWWBB",
        "YYYYYYYYYYYYYYYYYYYYYYYY",
        ".KK.BBBBBBBBBBBBBBBB.KK.",
    ];
    let x = 36.0 - 60.0 * (tl / 5.0);
    stamp(&mut b, &train, 3, x.floor() as i64, &|ch| match ch {
        'B' => Some([0x2F, 0x86, 0xF0]),
        'W' => Some(WHITE),
        'Y' => Some([0xFF, 0xC7, 0x2C]),
        'K' => Some([0x10, 0x10, 0x10]),
        _ => None,
    });
    b
}

// ---------- 6. WE LIKE TO CODE IN [LANGUAGE] ----------
fn language(k: usize, tl: f64) -> Board {
    let (name, colour) = LANGS[k];
    let mut b = new_board(BLACK);
    let prefix = "WE LIKE TO CODE IN ";
    put_text(&mut b, 3, 2, "$", PROMPT_GREEN, BLACK);
    put_text(&mut b, 3, 4, prefix, PROMPT_GREEN, BLACK);
    let start = 4 + prefix.len();
    let len = name.len();
    let n = if tl < 0.5 { 0 } else { (((tl - 0.5) / 0.11) as usize + 1).min(len) };
    put_text(&mut b, 3, start, &name[..n], colour, BLACK);
    let typing = n < len;
    let blink_on = typing || ((tl * 2.2) as i64) % 2 == 0;
    if blink_on && start + n < COLS {
        set(&mut b, 3, (start + n) as i64, PROMPT_GREEN);
    }
    b
}
