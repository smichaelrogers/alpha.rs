#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct move_0 {
    pub from: libc::c_char,
    pub to: libc::c_char,
    pub piece: libc::c_char,
    pub target: libc::c_char,
}
pub type move_t = move_0;
pub type bool_0 = libc::c_uint;
pub const true_0: bool_0 = 1;
pub const false_0: bool_0 = 0;
#[no_mangle]
pub static mut moves: [move_t; 64000] = [move_t {
    from: 0,
    to: 0,
    piece: 0,
    target: 0,
}; 64000];
#[no_mangle]
pub static mut on_move: [libc::c_int; 16] = [0; 16];
#[no_mangle]
pub static mut m_from: libc::c_int = 0;
#[no_mangle]
pub static mut m_to: libc::c_int = 0;
#[no_mangle]
pub static mut ply: libc::c_int = 0;
#[no_mangle]
pub static mut nodes: libc::c_int = 0;
#[no_mangle]
pub static mut piece_rank: [libc::c_int; 2] = [7 as libc::c_int, 0 as libc::c_int];
#[no_mangle]
pub static mut pawn_rank: [libc::c_int; 2] = [6 as libc::c_int, 1 as libc::c_int];
#[no_mangle]
pub static mut promote_rank: [libc::c_int; 2] = [0 as libc::c_int, 7 as libc::c_int];
#[no_mangle]
pub static mut material: [libc::c_int; 6] = [
    10 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    52 as libc::c_int,
    91 as libc::c_int,
    3570 as libc::c_int,
];
#[no_mangle]
pub static mut nsteps: [libc::c_int; 6] = [
    0 as libc::c_int,
    8 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
];
#[no_mangle]
pub static mut up: [libc::c_int; 2] = [-(8 as libc::c_int), 8 as libc::c_int];
#[no_mangle]
pub static mut kings: [libc::c_int; 2] = [60 as libc::c_int, 4 as libc::c_int];
#[no_mangle]
pub static mut mx: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut mn: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut colors: [libc::c_int; 64] = [
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub static mut pieces: [libc::c_int; 64] = [
    3 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
];
#[no_mangle]
pub static mut piece_types: [[libc::c_char; 7]; 2] = [
    [
        'P' as i32 as libc::c_char,
        'N' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'Q' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        '.' as i32 as libc::c_char,
    ],
    [
        'p' as i32 as libc::c_char,
        'n' as i32 as libc::c_char,
        'b' as i32 as libc::c_char,
        'r' as i32 as libc::c_char,
        'q' as i32 as libc::c_char,
        'k' as i32 as libc::c_char,
        '.' as i32 as libc::c_char,
    ],
];
#[no_mangle]
pub static mut pst: [libc::c_int; 64] = [
    -(4 as libc::c_int),
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    -(4 as libc::c_int),
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    -(4 as libc::c_int),
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    -(4 as libc::c_int),
];
#[no_mangle]
pub static mut SQ64: [libc::c_int; 64] = [
    21 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    48 as libc::c_int,
    51 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    55 as libc::c_int,
    56 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    61 as libc::c_int,
    62 as libc::c_int,
    63 as libc::c_int,
    64 as libc::c_int,
    65 as libc::c_int,
    66 as libc::c_int,
    67 as libc::c_int,
    68 as libc::c_int,
    71 as libc::c_int,
    72 as libc::c_int,
    73 as libc::c_int,
    74 as libc::c_int,
    75 as libc::c_int,
    76 as libc::c_int,
    77 as libc::c_int,
    78 as libc::c_int,
    81 as libc::c_int,
    82 as libc::c_int,
    83 as libc::c_int,
    84 as libc::c_int,
    85 as libc::c_int,
    86 as libc::c_int,
    87 as libc::c_int,
    88 as libc::c_int,
    91 as libc::c_int,
    92 as libc::c_int,
    93 as libc::c_int,
    94 as libc::c_int,
    95 as libc::c_int,
    96 as libc::c_int,
    97 as libc::c_int,
    98 as libc::c_int,
];
#[no_mangle]
pub static mut SQ120: [libc::c_int; 120] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    16 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    24 as libc::c_int,
    25 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    48 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    51 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    55 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    56 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    59 as libc::c_int,
    60 as libc::c_int,
    61 as libc::c_int,
    62 as libc::c_int,
    63 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
];
#[no_mangle]
pub static mut steps: [[libc::c_int; 8]; 6] = [
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(21 as libc::c_int),
        -(19 as libc::c_int),
        -(12 as libc::c_int),
        -(8 as libc::c_int),
        8 as libc::c_int,
        12 as libc::c_int,
        19 as libc::c_int,
        21 as libc::c_int,
    ],
    [
        -(11 as libc::c_int),
        -(9 as libc::c_int),
        9 as libc::c_int,
        11 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(10 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        10 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(11 as libc::c_int),
        -(9 as libc::c_int),
        9 as libc::c_int,
        11 as libc::c_int,
        -(10 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        10 as libc::c_int,
    ],
    [
        -(11 as libc::c_int),
        -(9 as libc::c_int),
        9 as libc::c_int,
        11 as libc::c_int,
        -(10 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        10 as libc::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn search(
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if depth == 0 as libc::c_int {
        return evaluate();
    }
    nodes += 1;
    nodes;
    generate_moves();
    i = on_move[ply as usize];
    while i < on_move[(ply + 1 as libc::c_int) as usize] {
        if !(make_move(&mut *moves.as_mut_ptr().offset(i as isize)) as u64 == 0) {
            n = -search(-beta, -alpha, depth - 1 as libc::c_int);
            unmake_move(&mut *moves.as_mut_ptr().offset(i as isize));
            if n >= beta {
                return beta;
            }
            if n > alpha {
                alpha = n;
                if ply == 0 as libc::c_int {
                    m_from = moves[i as usize].from as libc::c_int;
                    m_to = moves[i as usize].to as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    return alpha;
}
#[no_mangle]
pub unsafe extern "C" fn generate_moves() {
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    on_move[(ply + 1 as libc::c_int) as usize] = on_move[ply as usize];
    f = 0 as libc::c_int;
    while f < 64 as libc::c_int {
        if !(colors[f as usize] != mx) {
            if pieces[f as usize] == 0 as libc::c_int {
                t = f + up[mx as usize];
                if colors[(t + 1 as libc::c_int) as usize] == mn
                    && SQ120[(SQ64[t as usize] + 1 as libc::c_int) as usize]
                        != -(1 as libc::c_int)
                {
                    add_move(f, t + 1 as libc::c_int);
                }
                if colors[(t - 1 as libc::c_int) as usize] == mn
                    && SQ120[(SQ64[t as usize] - 1 as libc::c_int) as usize]
                        != -(1 as libc::c_int)
                {
                    add_move(f, t - 1 as libc::c_int);
                }
                if !(colors[t as usize] != 6 as libc::c_int) {
                    add_move(f, t);
                    if colors[(t + up[mx as usize]) as usize] == 6 as libc::c_int
                        && f >> 3 as libc::c_int == pawn_rank[mx as usize]
                    {
                        add_move(f, t + up[mx as usize]);
                    }
                }
            } else {
                i = 0 as libc::c_int;
                while i < nsteps[pieces[f as usize] as usize] {
                    s = steps[pieces[f as usize] as usize][i as usize];
                    t = SQ120[(SQ64[f as usize] + s) as usize];
                    while t != -(1 as libc::c_int) {
                        if colors[t as usize] == mn
                            || colors[t as usize] == 6 as libc::c_int
                        {
                            add_move(f, t);
                        }
                        if colors[t as usize] != 6 as libc::c_int
                            || pieces[f as usize] == 1 as libc::c_int
                            || pieces[f as usize] == 5 as libc::c_int
                        {
                            break;
                        }
                        t = SQ120[(SQ64[t as usize] + s) as usize];
                    }
                    i += 1;
                    i;
                }
            }
        }
        f += 1;
        f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_move(mut from: libc::c_int, mut to: libc::c_int) {
    let mut m: *mut move_t = 0 as *mut move_t;
    let ref mut fresh0 = *on_move.as_mut_ptr().offset((ply + 1 as libc::c_int) as isize);
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    m = &mut *moves.as_mut_ptr().offset(fresh1 as isize) as *mut move_t;
    (*m).from = from as libc::c_char;
    (*m).to = to as libc::c_char;
    (*m).piece = pieces[from as usize] as libc::c_char;
    (*m).target = pieces[to as usize] as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn swap_sides() {
    mx ^= 1 as libc::c_int;
    mn ^= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn make_move(mut m: *mut move_t) -> bool_0 {
    ply += 1;
    ply;
    colors[(*m).to as usize] = mx;
    pieces[(*m).to as usize] = (*m).piece as libc::c_int;
    colors[(*m).from as usize] = 6 as libc::c_int;
    pieces[(*m).from as usize] = 6 as libc::c_int;
    if (*m).piece as libc::c_int == 5 as libc::c_int {
        kings[mx as usize] = (*m).to as libc::c_int;
    } else if (*m).piece as libc::c_int == 0 as libc::c_int
        && (*m).to as libc::c_int >> 3 as libc::c_int == promote_rank[mx as usize]
    {
        pieces[(*m).to as usize] = 4 as libc::c_int;
    }
    if in_check() as u64 != 0 {
        swap_sides();
        unmake_move(m);
        return false_0;
    }
    swap_sides();
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn unmake_move(mut m: *mut move_t) {
    ply -= 1;
    ply;
    swap_sides();
    colors[(*m).from as usize] = mx;
    pieces[(*m).from as usize] = (*m).piece as libc::c_int;
    colors[(*m).to
        as usize] = if (*m).target as libc::c_int == 6 as libc::c_int {
        6 as libc::c_int
    } else {
        mn
    };
    pieces[(*m).to as usize] = (*m).target as libc::c_int;
    if (*m).piece as libc::c_int == 5 as libc::c_int {
        kings[mx as usize] = (*m).from as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn evaluate() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if colors[i as usize] == mx {
            x += material[i as usize] + pst[i as usize];
        } else if colors[i as usize] == mn {
            x -= material[i as usize] + pst[i as usize];
        }
        i += 1;
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn in_check() -> bool_0 {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    f = kings[mx as usize];
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        t = SQ120[(SQ64[f as usize] + steps[1 as libc::c_int as usize][i as usize])
            as usize];
        if t != -(1 as libc::c_int) && pieces[t as usize] == 1 as libc::c_int
            && colors[t as usize] == mn
        {
            return true_0;
        }
        s = steps[5 as libc::c_int as usize][i as usize];
        t = SQ120[(SQ64[f as usize] + s) as usize];
        while t != -(1 as libc::c_int) && colors[t as usize] == 6 as libc::c_int {
            t = SQ120[(SQ64[t as usize] + s) as usize];
        }
        if !(t == -(1 as libc::c_int) || colors[t as usize] != mn) {
            let mut current_block_9: u64;
            match pieces[t as usize] {
                2 => {
                    if i > 3 as libc::c_int {
                        current_block_9 = 13056961889198038528;
                    } else {
                        current_block_9 = 6796534011329963016;
                    }
                }
                3 => {
                    current_block_9 = 6796534011329963016;
                }
                0 => {
                    current_block_9 = 3296498687077415761;
                }
                5 => {
                    current_block_9 = 9184986429432625616;
                }
                4 => {
                    current_block_9 = 14585597296120879182;
                }
                _ => {
                    current_block_9 = 13056961889198038528;
                }
            }
            match current_block_9 {
                6796534011329963016 => {
                    if i < 4 as libc::c_int {
                        current_block_9 = 13056961889198038528;
                    } else {
                        current_block_9 = 3296498687077415761;
                    }
                }
                _ => {}
            }
            match current_block_9 {
                3296498687077415761 => {
                    if s - up[mn as usize] != 1 as libc::c_int
                        && s - up[mn as usize] != -(1 as libc::c_int)
                    {
                        current_block_9 = 13056961889198038528;
                    } else {
                        current_block_9 = 9184986429432625616;
                    }
                }
                _ => {}
            }
            match current_block_9 {
                9184986429432625616 => {
                    if SQ120[(SQ64[f as usize] + s) as usize] != t {
                        current_block_9 = 13056961889198038528;
                    } else {
                        current_block_9 = 14585597296120879182;
                    }
                }
                _ => {}
            }
            match current_block_9 {
                13056961889198038528 => {}
                _ => return true_0,
            }
        }
        i += 1;
        i;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn print_board() {
    let mut i: libc::c_int = 0;
    printf(b"\n nodes: %i\0" as *const u8 as *const libc::c_char, nodes);
    printf(b"\n move: %i to %i\0" as *const u8 as *const libc::c_char, m_from, m_to);
    printf(b"\n\n \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        printf(
            b" %c\0" as *const u8 as *const libc::c_char,
            piece_types[(colors[i as usize] % 6 as libc::c_int)
                as usize][pieces[i as usize] as usize] as libc::c_int,
        );
        if (i + 1 as libc::c_int) % 8 as libc::c_int == 0 as libc::c_int
            && i != 63 as libc::c_int
        {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut m: move_t = move_t {
        from: 0,
        to: 0,
        piece: 0,
        target: 0,
    };
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        m_from = -(1 as libc::c_int);
        m_to = -(1 as libc::c_int);
        ply = 0 as libc::c_int;
        nodes = 0 as libc::c_int;
        search(-(5000 as libc::c_int), 5000 as libc::c_int, 7 as libc::c_int);
        if m_from < 0 as libc::c_int {
            break;
        }
        m.from = m_from as libc::c_char;
        m.to = m_to as libc::c_char;
        m.piece = pieces[m_from as usize] as libc::c_char;
        m.target = pieces[m_to as usize] as libc::c_char;
        make_move(&mut m);
        print_board();
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
