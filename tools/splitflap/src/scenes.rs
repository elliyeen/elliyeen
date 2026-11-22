//! Scene definitions. `board_at(t)` is a pure function of time: the target
//! tile grid for second `t` of the 60 s loop. The renderer animates flips.

pub type Rgb = [u8; 3];

pub const ROWS: usize = 8;
pub const COLS: usize = 48;
pub const TW: usize = 16; // tile width in px
pub const TH: usize = 20; // tile height in px
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
const INK: Rgb = [0x11, 0x13, 0x12];
const GREEN: Rgb = [0x33, 0xFF, 0x66];

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
    set_tile(b, r, c, Tile::solid(bg));
}

fn set_tile(b: &mut Board, r: i64, c: i64, t: Tile) {
    if r >= 0 && c >= 0 && (r as usize) < ROWS && (c as usize) < COLS {
        b[r as usize * COLS + c as usize] = t;
    }
}

fn put_text(b: &mut Board, r: usize, c: usize, s: &str, fg: Rgb, bg: Rgb) {
    for (i, ch) in s.chars().enumerate() {
        set_tile(b, r as i64, (c + i) as i64, Tile { bg, ch, fg });
    }
}

/// Stamp an ASCII sprite. `.` is transparent; `colour` maps other chars to tiles.
fn stamp(b: &mut Board, art: &[String], top: i64, left: i64, colour: &dyn Fn(char) -> Option<Tile>) {
    for (dr, line) in art.iter().enumerate() {
        for (dc, ch) in line.chars().enumerate() {
            if ch != '.' {
                if let Some(t) = colour(ch) {
                    set_tile(b, top + dr as i64, left + dc as i64, t);
                }
            }
        }
    }
}

fn lines(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

fn mirror(a: &[String]) -> Vec<String> {
    a.iter().map(|s| s.chars().rev().collect()).collect()
}

fn mix(a: Rgb, b: Rgb, k: f64) -> Rgb {
    let f = |i: usize| (a[i] as f64 * (1.0 - k) + b[i] as f64 * k).round() as u8;
    [f(0), f(1), f(2)]
}

pub fn hash(r: usize, c: usize) -> u32 {
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
        1 => beach(tl),
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
    let dark = mix(ORANGE, [0, 0, 0], 0.22);
    // chevron rails on both sides
    for r in 0..ROWS {
        for c in 0..COLS {
            if c < 3 || c >= COLS - 3 {
                set(&mut b, r as i64, c as i64, dark);
            }
        }
    }
    for r in [1usize, 6] {
        for c in 4..COLS - 4 {
            if (c + r) % 2 == 0 {
                set(&mut b, r as i64, c as i64, mix(ORANGE, WHITE, 0.12));
            }
        }
    }
    put_text(&mut b, 3, 18, "PEACE WORLD!", WHITE, ORANGE);
    put_text(&mut b, 4, 18, "I'M ABDULLAH", WHITE, ORANGE);
    b
}

// ---------- 2. Tropical beach at sunset ----------
fn beach(tl: f64) -> Board {
    let sky: [Rgb; 4] = [[0x3B, 0x1F, 0x6B], [0x8C, 0x3A, 0x8F], [0xE0, 0x59, 0x5A], [0xF7, 0xA4, 0x41]];
    let sea: [Rgb; 4] = [[0x2F, 0x86, 0xC4], [0x26, 0x72, 0xAE], [0x1D, 0x5E, 0x94], [0x15, 0x4A, 0x7B]];
    let sun: Rgb = [0xFF, 0xE2, 0x7A];
    let sun_edge: Rgb = [0xFF, 0xC8, 0x57];
    let glint: Rgb = [0xF7, 0xC0, 0x5A];
    let sand: Rgb = [0xEA, 0xD3, 0x9C];
    let wet: Rgb = [0xC9, 0xAE, 0x78];
    let ray: Rgb = [0xFF, 0xC3, 0x6B];
    let mut b = new_board(INK);
    let (cx, cy) = (COLS as f64 * TW as f64 / 2.0, 4.0 * TH as f64);
    for r in 0..4 {
        for c in 0..COLS {
            let (x, y) = ((c as f64 + 0.5) * TW as f64, (r as f64 + 0.5) * TH as f64);
            let (dx, dy) = (x - cx, cy - y);
            let d = (dx * dx + dy * dy).sqrt();
            let a = dy.atan2(dx);
            let col = if d < 62.0 {
                if d > 46.0 { sun_edge } else { sun }
            } else if d < 260.0 && (((a + tl * 0.08) * 5.0).floor() as i64).rem_euclid(2) == 0 {
                mix(sky[r], ray, 0.22)
            } else {
                sky[r]
            };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    // sea with rolling waves
    for r in 4..8 {
        for c in 0..COLS {
            let w = ((c as f64) * 0.5 + (r as f64) * 0.9 + tl * 3.4).sin();
            let col = if w > 0.93 {
                WHITE
            } else if w > 0.74 {
                [0xA9, 0xD6, 0xF0]
            } else if r <= 6 && (22..27).contains(&c) && ((c as f64 * 1.3 + tl * 4.0).sin() > -0.2) {
                glint
            } else {
                sea[r - 4]
            };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    // birds
    for i in 0..3usize {
        let c = 33.0 + i as f64 * 4.0 + tl * 1.4;
        let (r, c) = (1 + (i % 2) as i64, c as i64);
        let up = (((tl * 2.0) as i64) + i as i64) % 2 == 0;
        let bird: Rgb = [0x1A, 0x10, 0x30];
        if up {
            set(&mut b, r - 1, c - 1, bird);
            set(&mut b, r, c, bird);
            set(&mut b, r - 1, c + 1, bird);
        } else {
            set(&mut b, r, c - 1, bird);
            set(&mut b, r - 1, c, bird);
            set(&mut b, r, c + 1, bird);
        }
    }
    // sand with a breathing shoreline, both sides
    let shore = 8 + (1.2 * (tl * 1.6).sin()).round() as i64;
    for r in 6..8i64 {
        let edge = shore + (r - 6);
        for c in 0..edge {
            set(&mut b, r, c, if c >= edge - 2 { wet } else { sand });
            set(&mut b, r, COLS as i64 - 1 - c, if c >= edge - 2 { wet } else { sand });
        }
        set(&mut b, r, edge, WHITE);
        set(&mut b, r, COLS as i64 - 1 - edge, WHITE);
    }
    // palms
    let palm = lines(&[
        "...gGGGGg....",
        "..gGGgGGGg...",
        ".gG.GgTgG.Gg.",
        "gG....T....Gg",
        "G.....T......",
        "......TT.....",
        ".....TT......",
    ]);
    let col = |ch: char| match ch {
        'G' => Some(Tile::solid([0x34, 0x9B, 0x3F])),
        'g' => Some(Tile::solid([0x1F, 0x6B, 0x2E])),
        'T' => Some(Tile::solid([0x7A, 0x4A, 0x21])),
        _ => None,
    };
    stamp(&mut b, &palm, 0, 0, &col);
    stamp(&mut b, &mirror(&palm), 0, COLS as i64 - 13, &col);
    b
}

// ---------- 3. Earth ----------
/// Coarse land map: 10-degree cells. (row for latitude band index, inclusive column spans; col = (lon+180)/10)
const LAND: [(usize, &[(usize, usize)]); 14] = [
    (1, &[(7, 16), (27, 35)]),
    (2, &[(1, 16), (19, 35)]),
    (3, &[(1, 13), (17, 35)]),
    (4, &[(5, 12), (18, 35)]),
    (5, &[(6, 12), (17, 35)]),
    (6, &[(6, 11), (17, 22), (23, 34)]),
    (7, &[(8, 12), (17, 23), (25, 33)]),
    (8, &[(10, 14), (17, 24), (29, 33)]),
    (9, &[(11, 16), (19, 24), (29, 33)]),
    (10, &[(10, 15), (19, 24)]),
    (11, &[(11, 15), (19, 22), (31, 34)]),
    (12, &[(11, 13), (20, 22), (31, 34)]),
    (13, &[(11, 12)]),
    (14, &[(12, 12)]),
];

fn is_land(lon_deg: f64, lat_deg: f64) -> bool {
    let col = (((lon_deg + 180.0) / 10.0).floor() as i64).rem_euclid(36) as usize;
    let row = ((90.0 - lat_deg) / 10.0).floor() as i64;
    if row < 0 {
        return false;
    }
    LAND.iter().any(|(r, spans)| *r as i64 == row && spans.iter().any(|&(a, b)| col >= a && col <= b))
}

fn earth(tl: f64) -> Board {
    let space: Rgb = [0x05, 0x07, 0x14];
    let mut b = new_board(space);
    for r in 0..ROWS {
        for c in 0..COLS {
            if hash(r, c) % 29 == 0 {
                let s = if hash(c, r) % 2 == 0 { WHITE } else { [0x9F, 0xB4, 0xFF] };
                set(&mut b, r as i64, c as i64, s);
            }
        }
    }
    // plus-shaped stars
    for &(r, c) in &[(2i64, 7i64), (5, 13), (1, 35), (5, 40), (3, 44), (6, 4)] {
        let s = [0xDD, 0xE6, 0xFF];
        for &(dr, dc) in &[(0i64, 0i64), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            set(&mut b, r + dr, c + dc, s);
        }
    }
    let (tw, th) = (TW as f64, TH as f64);
    let (cx, cy) = (COLS as f64 * tw / 2.0, ROWS as f64 * th / 2.0);
    let rad = 76.0;
    let view = (-85.0 + tl * 4.0_f64).to_radians(); // drifts east, Americas centred
    for r in 0..ROWS {
        for c in 0..COLS {
            let x = (c as f64 + 0.5) * tw - cx;
            let y = (r as f64 + 0.5) * th - cy;
            let d = (x * x + y * y).sqrt() / rad;
            if d > 1.0 {
                if d < 1.12 {
                    set(&mut b, r as i64, c as i64, [0x16, 0x2B, 0x5C]);
                }
                continue;
            }
            let z = (1.0 - d * d).sqrt();
            let lon = (x / rad).atan2(z) + view;
            let lat = (-y / rad).asin();
            let (lond, latd) = (lon.to_degrees(), lat.to_degrees());
            let edge = d > 0.82;
            let cloud = (lon * 4.0 + lat * 6.0 + 2.0).sin() * (lon * 3.0 - lat * 5.0).sin() > 0.62;
            let col = if latd.abs() > 72.0 || cloud {
                if edge { [0xC9, 0xD6, 0xEE] } else { [0xF2, 0xF6, 0xFF] }
            } else if is_land(lond, latd) {
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
    let night: Rgb = [0x0E, 0x16, 0x30];
    let asphalt: Rgb = [0x23, 0x25, 0x2B];
    let mut b = new_board(night);
    let ph = (tl * 4.0) as usize;
    for c in 0..COLS {
        for r in 5..7 {
            set(&mut b, r, c as i64, asphalt);
        }
        // scrolling red/white kerb
        let kerb = if (c + ph) % 2 == 0 { [0xD9, 0x1E, 0x2B] } else { WHITE };
        set(&mut b, 7, c as i64, kerb);
    }
    // waving chequered flag block, right side
    for r in 1..5usize {
        for c in 41..47usize {
            let on = (r + c + ((tl * 2.0) as usize)) % 2 == 0;
            set(&mut b, r as i64, c as i64, if on { WHITE } else { INK });
        }
    }
    // chevrons that light up in sequence
    for i in 0..3usize {
        let lit = ((tl * 3.0) as usize) % 4 > i;
        let col = if lit { [0xFF, 0x4A, 0x2B] } else { [0x5A, 0x1E, 0x1A] };
        set_tile(&mut b, 6, (36 + i * 2) as i64, Tile { bg: asphalt, ch: '>', fg: col });
    }
    let x = -28.0 + 76.0 * (tl / 5.0);
    let xi = x.floor() as i64;
    // speed streaks trailing the car
    for (k, r) in [2i64, 3, 4].iter().enumerate() {
        let len = 12 + k as i64 * 3;
        for c in (xi - len)..xi + 4 {
            let col = [[0x3B, 0xA3, 0xFF], [0xFF, 0x4A, 0x2B], [0x7F, 0xC8, 0xFF]][k];
            if (c + *r) % 3 != 0 {
                set(&mut b, *r, c, mix(night, col, 0.75));
            }
        }
    }
    let car = lines(&[
        "RRRRR...........YY.........",
        ".KRRRRRfnRRRRRRRRR.........",
        "..RRRRRRRRRRRRRRRRRRRRRRWW",
        "...KKKRRRRRRRRRRRRRKKKRRR.",
        "..KKGKK...........KKGKK...",
        "..KKKKK...........KKKKK.KK",
    ]);
    stamp(&mut b, &car, 1, xi, &|ch| match ch {
        'R' => Some(Tile::solid([0xE1, 0x06, 0x00])),
        'K' => Some(Tile::solid([0x10, 0x10, 0x10])),
        'G' => Some(Tile::solid([0x9A, 0x9A, 0x9A])),
        'W' => Some(Tile::solid(WHITE)),
        'Y' => Some(Tile::solid([0xFF, 0xD2, 0x1F])),
        'f' => Some(Tile { bg: [0xE1, 0x06, 0x00], ch: 'F', fg: WHITE }),
        'n' => Some(Tile { bg: [0xE1, 0x06, 0x00], ch: '1', fg: WHITE }),
        _ => None,
    });
    b
}

// ---------- 5. DART light rail, right → left ----------
fn dart(tl: f64) -> Board {
    let sky: Rgb = [0x3A, 0x4F, 0x86];
    let mut b = new_board(sky);
    let bld: Rgb = [0x0A, 0x10, 0x20];
    let lit: Rgb = [0xFF, 0xD1, 0x66];
    for c in 0..COLS {
        let mut h = 1 + (hash(0, c) % 3) as usize;
        if (24..42).contains(&c) {
            h += 1;
        }
        let h = h.min(4);
        for r in (4 - h)..4 {
            let col = if hash(r, c) % 6 == 0 { lit } else { bld };
            set(&mut b, r as i64, c as i64, col);
        }
    }
    // Reunion Tower
    for r in 1..4 {
        set(&mut b, r, 38, bld);
    }
    set(&mut b, 0, 38, [0x7D, 0xD3, 0xFC]);
    set(&mut b, 0, 37, [0xDD, 0xF4, 0xFF]);
    set(&mut b, 0, 39, [0xDD, 0xF4, 0xFF]);
    // track
    for c in 0..COLS {
        set(&mut b, 7, c as i64, [0x4A, 0x4A, 0x4A]);
        if (c + (tl * 3.0) as usize) % 3 == 0 {
            set(&mut b, 7, c as i64, [0x6A, 0x6A, 0x6A]);
        }
    }
    // yellow chevrons that flow right-to-left behind the train line
    for i in 0..4usize {
        let lit_ = ((tl * 3.0) as usize) % 5 > i;
        let col = if lit_ { [0xFF, 0xC7, 0x2C] } else { [0x5A, 0x4A, 0x20] };
        set_tile(&mut b, 1, (4 + (3 - i) * 2) as i64, Tile { bg: sky, ch: '<', fg: col });
    }
    let car0 = ["BBBBBBBBBBBBBB", "WWWBWWBWWBWWBB", "YYYYYYYYYYYYYY", "BKKBBBBBBBBKKB"];
    let car1 = ["BBBBBBBBBBBBBB", "BWWBWWBWWBWWBB", "YYYYdartYYYYYY", "BKKBBBBBBBBKKB"];
    let car2 = ["BBBBBBBBBBBBBB", "BWWBWWBWWBWWBB", "YYYYYYYYYYYYYY", "BKKBBBBBBBBKKB"];
    let mut rows = vec![String::new(); 4];
    for (i, car) in [car0, car1, car2].iter().enumerate() {
        for r in 0..4 {
            rows[r].push_str(car[r]);
            if i < 2 {
                rows[r].push(if r == 0 { '.' } else { 'K' });
            }
        }
    }
    rows[0].replace_range(0..1, ".");
    rows[3].replace_range(0..1, ".");
    let x = COLS as f64 - (COLS as f64 + 44.0) * (tl / 5.0);
    let navy: Rgb = [0x0F, 0x2A, 0x55];
    let yel: Rgb = [0xFF, 0xC7, 0x2C];
    stamp(&mut b, &rows, 3, x.floor() as i64, &|ch| match ch {
        'B' => Some(Tile::solid([0x2F, 0x86, 0xF0])),
        'W' => Some(Tile::solid(WHITE)),
        'Y' => Some(Tile::solid(yel)),
        'K' => Some(Tile::solid([0x10, 0x10, 0x10])),
        'd' => Some(Tile { bg: yel, ch: 'D', fg: navy }),
        'a' => Some(Tile { bg: yel, ch: 'A', fg: navy }),
        'r' => Some(Tile { bg: yel, ch: 'R', fg: navy }),
        't' => Some(Tile { bg: yel, ch: 'T', fg: navy }),
        _ => None,
    });
    b
}

// ---------- 6. WE LIKE TO CODE IN [LANGUAGE] ----------
fn language(k: usize, tl: f64) -> Board {
    let (name, colour) = LANGS[k];
    let win: Rgb = [0x18, 0x1B, 0x1A];
    let bar: Rgb = [0x2E, 0x31, 0x30];
    let mut b = new_board(INK);
    for r in 1..7 {
        for c in 1..COLS - 1 {
            set(&mut b, r as i64, c as i64, if r == 1 { bar } else { win });
        }
    }
    for (i, col) in [[0xFF, 0x5F, 0x56], [0xFF, 0xBD, 0x2E], [0x27, 0xC9, 0x3F]].iter().enumerate() {
        set(&mut b, 1, 3 + i as i64 * 2, *col);
    }
    let prompt_bg: Rgb = [0x0F, 0x3A, 0x1F];
    set_tile(&mut b, 4, 5, Tile { bg: prompt_bg, ch: '>', fg: GREEN });
    set_tile(&mut b, 4, 6, Tile { bg: prompt_bg, ch: '_', fg: GREEN });
    let prefix = "WE LIKE TO CODE IN ";
    put_text(&mut b, 4, 9, prefix, [0xF2, 0xF2, 0xF2], win);
    let start = 9 + prefix.len();
    let len = name.len();
    let n = if tl < 0.5 { 0 } else { (((tl - 0.5) / 0.11) as usize + 1).min(len) };
    for (i, ch) in name.chars().take(n).enumerate() {
        set_tile(&mut b, 4, (start + i) as i64, Tile { bg: colour, ch, fg: INK });
    }
    let typing = n < len;
    if (typing || ((tl * 2.2) as i64) % 2 == 0) && start + n < COLS - 1 {
        set(&mut b, 4, (start + n) as i64, GREEN);
    }
    b
}
