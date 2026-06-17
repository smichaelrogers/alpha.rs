use std::time::Instant;

const MAX_PLY: usize = 16;
const WHITE: usize = 0;
const BLACK: usize = 1;
const PAWN: usize = 0;
const KNIGHT: usize = 1;
const BISHOP: usize = 2;
const ROOK: usize = 3;
const QUEEN: usize = 4;
const KING: usize = 5;
const EMPTY: usize = 6;

const PAWN_RANK: [usize; 2] = [6, 1];
const PROMOTE_RANK: [usize; 2] = [0, 7];
const MATERIAL: [i32; 6] = [10, 31, 32, 52, 91, 3570];
const NSTEPS: [usize; 6] = [0, 8, 4, 4, 8, 8];
const UP: [i32; 2] = [-8, 8];

const PIECE_TYPES: [[char; 7]; 2] = [
    ['P', 'N', 'B', 'R', 'Q', 'K', '.'],
    ['p', 'n', 'b', 'r', 'q', 'k', '.'],
];

const PST: [i32; 64] = [
    -4, -3, -2, -1, -1, -2, -3, -4,
    -3, -2, -1,  0,  0, -1, -2, -3,
    -2, -1,  1,  1,  1,  1, -1, -2,
    -1,  0,  2,  3,  3,  2,  0, -1,
    -1,  0,  2,  3,  3,  2,  0, -1,
    -2, -1,  1,  1,  1,  1, -1, -2,
    -3, -2, -1,  0,  0, -1, -2, -3,
    -4, -3, -2, -1, -1, -2, -3, -4
];

const SQ64: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

const SQ120: [i32; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
    -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
    -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
    -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
    -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
    -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
    -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

const STEPS: [[i32; 8]; 6] = [
    [  0,  0,  0,  0,  0,  0,  0,  0],
    [-21,-19,-12, -8,  8, 12, 19, 21],
    [-11, -9,  9, 11,  0,  0,  0,  0],
    [-10, -1,  1, 10,  0,  0,  0,  0],
    [-11, -9,  9, 11,-10, -1,  1, 10],
    [-11, -9,  9, 11,-10, -1,  1, 10]
];

#[derive(Clone, Copy, Default)]
struct Move {
    from: u8,
    to: u8,
    piece: u8,
    target: u8,
}

struct Engine {
    moves: [Move; 64000],
    on_move: [usize; MAX_PLY + 1],
    m_from: i32,
    m_to: i32,
    ply: usize,
    nodes: i32,
    kings: [usize; 2],
    mx: usize,
    mn: usize,
    colors: [usize; 64],
    pieces: [usize; 64],
}

impl Engine {
    fn new() -> Self {
        Self {
            moves: [Move::default(); 64000],
            on_move: [0; MAX_PLY + 1],
            m_from: -1,
            m_to: -1,
            ply: 0,
            nodes: 0,
            kings: [60, 4],
            mx: WHITE,
            mn: BLACK,
            colors: [
                1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  0,  0
            ],
            pieces: [
                3,  1,  2,  4,  5,  2,  1,  3,
                0,  0,  0,  0,  0,  0,  0,  0,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                6,  6,  6,  6,  6,  6,  6,  6,
                0,  0,  0,  0,  0,  0,  0,  0,
                3,  1,  2,  4,  5,  2,  1,  3
            ],
        }
    }

    // fn col(x: usize) -> usize {
    //     x & 7
    // }

    fn row(x: usize) -> usize {
        x >> 3
    }

    fn add_move(&mut self, from: usize, to: usize) {
        let idx = self.on_move[self.ply + 1];
        self.moves[idx].from = from as u8;
        self.moves[idx].to = to as u8;
        self.moves[idx].piece = self.pieces[from] as u8;
        self.moves[idx].target = self.pieces[to] as u8;
        self.on_move[self.ply + 1] += 1;
    }

    fn swap_sides(&mut self) {
        self.mx ^= 1;
        self.mn ^= 1;
    }

    fn evaluate(&self) -> i32 {
        let mut x = 0;
        for i in 0..64 {
            if self.colors[i] == self.mx {
                x += MATERIAL[self.pieces[i] as usize] + PST[i];
            } else if self.colors[i] == self.mn {
                x -= MATERIAL[self.pieces[i] as usize] + PST[i];
            }
        }
        x
    }

    fn in_check(&self) -> bool {
        let f = self.kings[self.mx];

        for i in 0..8 {
            let t = SQ120[(SQ64[f] as i32 + STEPS[KNIGHT][i]) as usize];
            if t != -1 && self.pieces[t as usize] == KNIGHT && self.colors[t as usize] == self.mn {
                return true;
            }

            let s = STEPS[KING][i];
            let mut t_ray = SQ120[(SQ64[f] as i32 + s) as usize];

            while t_ray != -1 && self.colors[t_ray as usize] == EMPTY {
                t_ray = SQ120[(SQ64[t_ray as usize] as i32 + s) as usize];
            }

            if t_ray == -1 || self.colors[t_ray as usize] != self.mn {
                continue;
            }

            let piece = self.pieces[t_ray as usize];
            if piece == BISHOP && i <= 3 { return true; }
            if piece == ROOK && i >= 4 { return true; }
            if piece == QUEEN { return true; }

            if SQ120[(SQ64[f] as i32 + s) as usize] == t_ray {
                if piece == PAWN && (s - UP[self.mn] == 1 || s - UP[self.mn] == -1) { return true; }
                if piece == KING { return true; }
            }
        }

        false
    }

    fn make_move(&mut self, m: &Move) -> bool {
        self.ply += 1;

        self.colors[m.to as usize] = self.mx;
        self.pieces[m.to as usize] = m.piece as usize;
        self.colors[m.from as usize] = EMPTY;
        self.pieces[m.from as usize] = EMPTY;

        if m.piece as usize == KING {
            self.kings[self.mx] = m.to as usize;
        } else if m.piece as usize == PAWN && Self::row(m.to as usize) == PROMOTE_RANK[self.mx] {
            self.pieces[m.to as usize] = QUEEN;
        }

        if self.in_check() {
            self.swap_sides();
            self.unmake_move(m);
            return false;
        }
        self.swap_sides();
        true
    }

    fn unmake_move(&mut self, m: &Move) {
        self.ply -= 1;
        self.swap_sides();

        self.colors[m.from as usize] = self.mx;
        self.pieces[m.from as usize] = m.piece as usize;
        self.colors[m.to as usize] = if m.target as usize == EMPTY { EMPTY } else { self.mn };
        self.pieces[m.to as usize] = m.target as usize;

        if m.piece as usize == KING {
            self.kings[self.mx] = m.from as usize;
        }
    }

    fn generate_moves(&mut self) {
        self.on_move[self.ply + 1] = self.on_move[self.ply];

        for f in 0..64 {
            if self.colors[f] != self.mx {
                continue;
            }

            if self.pieces[f] == PAWN {
                let t = f as i32 + UP[self.mx];
                if t + 1 < 64 && self.colors[(t + 1) as usize] == self.mn && SQ120[(SQ64[t as usize] as i32 + 1) as usize] != -1 {
                    self.add_move(f, (t + 1) as usize);
                }
                if t - 1 >= 0 && self.colors[(t - 1) as usize] == self.mn && SQ120[(SQ64[t as usize] as i32 - 1) as usize] != -1 {
                    self.add_move(f, (t - 1) as usize);
                }
                if self.colors[t as usize] != EMPTY {
                    continue;
                }
                self.add_move(f, t as usize);

                if t + UP[self.mx] >= 0 && t + UP[self.mx] <= 63 {
                    if self.colors[(t + UP[self.mx]) as usize] == EMPTY && Self::row(f) == PAWN_RANK[self.mx] {
                        self.add_move(f, (t + UP[self.mx]) as usize);
                    }
                }
            } else {
                for i in 0..NSTEPS[self.pieces[f]] {
                    let s = STEPS[self.pieces[f]][i];
                    let mut t = SQ120[(SQ64[f] as i32 + s) as usize];

                    while t != -1 {
                        if self.colors[t as usize] == self.mn || self.colors[t as usize] == EMPTY {
                            self.add_move(f, t as usize);
                        }
                        if self.colors[t as usize] != EMPTY || self.pieces[f] == KNIGHT || self.pieces[f] == KING {
                            break;
                        }
                        t = SQ120[(SQ64[t as usize] as i32 + s) as usize];
                    }
                }
            }
        }
    }

    fn search(&mut self, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }

        self.nodes += 1;
        self.generate_moves();

        let start_move = self.on_move[self.ply];
        let end_move = self.on_move[self.ply + 1];

        for i in start_move..end_move {
            let m = self.moves[i].clone();
            if !self.make_move(&m) {
                continue;
            }
            let n = -self.search(-beta, -alpha, depth - 1);
            self.unmake_move(&m);

            if n >= beta {
                return beta;
            }
            if n > alpha {
                alpha = n;
                if self.ply == 0 {
                    self.m_from = m.from as i32;
                    self.m_to = m.to as i32;
                }
            }
        }

        alpha
    }

    fn print_board(&self) {
        println!(" nodes: {}", self.nodes);
        println!(" move: {} to {}\n", self.m_from, self.m_to);

        for i in 0..64 {
            print!(" {}", PIECE_TYPES[self.colors[i] % 6][self.pieces[i]]);
            if (i + 1) % 8 == 0 && i != 63 {
                println!();
            }
        }
        println!("\n\n");
    }
}



fn main() {
    let mut engine = Engine::new();
    let depth = 8 as i32;

    for _ in 0..100 {
        engine.m_from = -1;
        engine.m_to = -1;
        engine.ply = 0;
        engine.nodes = 0;

        let start = Instant::now();

        engine.search(-5000, 5000, depth);

        let duration = start.elapsed();
        let elapsed: f64 = duration.as_secs_f64();
        let nodes_per_sec_float: f64 = (engine.nodes as f64 / elapsed).round();
        let nodes_per_sec: i32 = nodes_per_sec_float as i32;

        if engine.m_from < 0 {
            break;
        }

        let mut m = Move::default();
        m.from = engine.m_from as u8;
        m.to = engine.m_to as u8;
        m.piece = engine.pieces[engine.m_from as usize] as u8;
        m.target = engine.pieces[engine.m_to as usize] as u8;
        engine.make_move(&m);

        println!(" depth: {}", nodes_per_sec);
        println!(" clock: {:?}", duration);
        println!(" nps: {}", nodes_per_sec);
        engine.print_board();
    }
}
