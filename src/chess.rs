use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: Color,
}

impl Piece {
    const fn new(piece_type: PieceType, piece_color: Color) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }
}

impl Piece {
    pub fn piece_color(&self) -> Color {
        self.piece_color
    }
    
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    } 

}

pub struct Board {
    tiles: [[Option<Piece>; 8]; 8]
}

pub struct Game {
    board: Board,
    player_to_move: Color
}

const PAWN_WHITE: Piece = Piece::new(PieceType::Pawn, Color::White);
const PAWN_BLACK: Piece = Piece::new(PieceType::Pawn, Color::Black);
const ROOK_WHITE: Piece = Piece::new(PieceType::Rook, Color::White);
const ROOK_BLACK: Piece = Piece::new(PieceType::Rook, Color::Black);
const KNIGHT_WHITE: Piece = Piece::new(PieceType::Knight, Color::White);
const KNIGHT_BLACK: Piece = Piece::new(PieceType::Knight, Color::Black);
const BISHOP_WHITE: Piece = Piece::new(PieceType::Bishop, Color::White);
const BISHOP_BLACK: Piece = Piece::new(PieceType::Bishop, Color::Black);
const QUEEN_WHITE: Piece = Piece::new(PieceType::Queen, Color::White);
const QUEEN_BLACK: Piece = Piece::new(PieceType::Queen, Color::Black);
const KING_WHITE: Piece = Piece::new(PieceType::King, Color::White);
const KING_BLACK: Piece = Piece::new(PieceType::King, Color::Black);

pub const BOARD_DEFAULT_SETUP: [[Option<Piece>; 8]; 8] = [
    [Some(ROOK_WHITE), Some(KNIGHT_WHITE), Some(BISHOP_WHITE), Some(QUEEN_WHITE), Some(KING_WHITE), Some(BISHOP_WHITE), Some(KNIGHT_WHITE), Some(ROOK_WHITE)],
    [Some(PAWN_WHITE); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(PAWN_BLACK); 8],
    [Some(ROOK_BLACK), Some(KNIGHT_BLACK), Some(BISHOP_BLACK), Some(QUEEN_BLACK), Some(KING_BLACK), Some(BISHOP_BLACK), Some(KNIGHT_BLACK), Some(ROOK_BLACK)],
];

struct RaycastInfo {
    is_hit: bool,
    point: Option<(usize, usize)>,
    path: HashSet<(usize, usize)>
}

struct MoveInfo {
    moved: Piece,
    captured: Option<Piece>,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize
}

impl Game {

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.board.tiles[y][x].is_none()
    }

    fn is_bounded(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8
    }

    fn cast_ray(&self, x: usize, y: usize, dx: isize, dy: isize, steps: Option<usize>) -> RaycastInfo {
        let mut path = HashSet::new();

        let mut rx: isize = x as isize;
        let mut ry: isize = y as isize;
        
        let mut i: usize = 0;
        loop {
            if steps.is_some_and(|r| i >= r) {
                break;
            }

            rx += dx;
            ry += dy;

            if !self.is_bounded(rx, ry) {
                rx -= dx;
                ry -= dy;
                break;
            }

            if self.is_empty(rx as usize, ry as usize) {
                path.insert((rx as usize, ry as usize));
                i += 1;
            } else {
                return RaycastInfo {
                    is_hit: true,
                    path,
                    point: Some((rx as usize, ry as usize)),
                }
            }
        }

        if i == 0 {
            return RaycastInfo {
                is_hit: false,
                path,
                point: None,
            };
        }

        RaycastInfo {
            is_hit: false,
            path,
            point: Some((rx as usize, ry as usize)),
        }
    }

    fn can_be_here(&self, x: usize, y: usize) -> bool {
        if let Some(piece) = self.get_piece(x, y) {
            if piece.piece_type == PieceType::King {
                return false;
            }
            if piece.piece_color == self.player_to_move {
                return false;
            }
        }
        true
    }

    fn get_pseudo_captures_pawn(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        let raycast = self.cast_ray(x, y, 1, 1, Some(1));
        if raycast.is_hit {
            if let Some(point) = raycast.point {
                captures.insert(point);
            }
        }
        let raycast = self.cast_ray(x, y, -1, 1, Some(1));
        if raycast.is_hit {
            if let Some(point) = raycast.point {
                captures.insert(point);
            }
        }
    }

    fn get_pseudo_moves_pawn(&self, x: usize, y: usize, moves: &mut HashSet<(usize, usize)>) -> () {
        let raycast = self.cast_ray(x, y, 0, 1, Some(1));
        if !raycast.is_hit {
            if let Some(point) = raycast.point {
                moves.insert(point);
            }
        }
        if y == 1 {
            let raycast = self.cast_ray(x, y, 0, 1, Some(2));
            if !raycast.is_hit {
                if let Some(point) = raycast.point {
                    moves.insert(point);
                }
            }
        }
    }

    fn get_pseudo_moves_bishop(&self, x: usize, y: usize, moves: &mut HashSet<(usize, usize)>) -> () {
        for dx in [-1, 1] {
            for dy in [-1, 1] {
                let raycast = self.cast_ray(x, y, dx, dy, None);
                moves.extend(raycast.path);
            }
        }
    }
    
    fn get_pseudo_captures_bishop(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        for dx in [-1, 1] {
            for dy in [-1, 1] {
                let raycast = self.cast_ray(x, y, dx, dy, None);
                if raycast.is_hit {
                    if let Some(point) = raycast.point {
                        captures.insert(point);
                    }
                }
            }
        }
    }

    fn get_pseudo_moves_knight(&self, x: usize, y: usize, moves: &mut HashSet<(usize, usize)>) -> () {
        for (dx, dy) in [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)] {
            let raycast = self.cast_ray(x, y, dx, dy, Some(1));
            if !raycast.is_hit {
                if let Some(point) = raycast.point {
                    moves.insert(point);
                }
            }
        }
    }

    fn get_pseudo_captures_knight(&self, x: usize, y: usize, moves: &mut HashSet<(usize, usize)>) -> () {
        for (dx, dy) in [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)] {
            let raycast = self.cast_ray(x, y, dx, dy, Some(1));
            if raycast.is_hit {
                if let Some(point) = raycast.point {
                    moves.insert(point);
                }
            }
        }
    }
    
    fn get_pseudo_moves_rook(&self, x: usize, y: usize, moves: &mut HashSet<(usize, usize)>) -> () {
        for d in [-1, 1] {
            let raycast = self.cast_ray(x, y, 0, d, None);
            moves.extend(raycast.path);
            let raycast = self.cast_ray(x, y, d, 0, None);
            moves.extend(raycast.path);
        }
    }

    fn get_pseudo_captures_rook(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        for d in [-1, 1] {
            let raycast = self.cast_ray(x, y, 0, d, None);
            if raycast.is_hit {
                if let Some(point) = raycast.point {
                    captures.insert(point);
                }
            }
            let raycast = self.cast_ray(x, y, d, 0, None);
            if raycast.is_hit {
                if let Some(point) = raycast.point {
                    captures.insert(point);
                }
            }
        }
    }

    // :D
    fn get_pseudo_moves_queen(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        self.get_pseudo_moves_bishop(x, y, captures);
        self.get_pseudo_moves_rook(x, y, captures);
    }

    fn get_pseudo_captures_queen(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        self.get_pseudo_captures_bishop(x, y, captures);
        self.get_pseudo_captures_rook(x, y, captures);
    }

    fn get_pseudo_moves_king(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let raycast = self.cast_ray(x, y, dx, dy, Some(1));
                if !raycast.is_hit {
                    if let Some(point) = raycast.point {
                        captures.insert(point);
                    }
                }
            }
        }
    }

    fn get_pseudo_captures_king(&self, x: usize, y: usize, captures: &mut HashSet<(usize, usize)>) -> () {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let raycast = self.cast_ray(x, y, dx, dy, Some(1));
                if raycast.is_hit {
                    if let Some(point) = raycast.point {
                        captures.insert(point);
                    }
                }
            }
        }
    }

    fn get_pseudo_moves(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut moves: HashSet<(usize, usize)> = HashSet::new();
        if let Some(piece) = self.get_piece(x, y) {
            match piece.piece_type {
                PieceType::Pawn => { self.get_pseudo_moves_pawn(x, y, &mut moves); self.get_pseudo_captures_pawn(x, y, &mut moves); },
                PieceType::Bishop => { self.get_pseudo_moves_bishop(x, y, &mut moves); self.get_pseudo_captures_bishop(x, y, &mut moves) },
                PieceType::Knight => { self.get_pseudo_moves_knight(x, y, &mut moves); self.get_pseudo_captures_knight(x, y, &mut moves) },
                PieceType::Rook => { self.get_pseudo_moves_rook(x, y, &mut moves); self.get_pseudo_captures_rook(x, y, &mut moves) },
                PieceType::Queen => { self.get_pseudo_moves_queen(x, y, &mut moves); self.get_pseudo_captures_queen(x, y, &mut moves) },
                PieceType::King => { self.get_pseudo_moves_king(x, y, &mut moves); self.get_pseudo_captures_king(x, y, &mut moves) },
            }
        }
        moves
    }
    
    fn is_pseudo_legal(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if let Some(_) = self.get_piece(from_x, from_y) {
            let pseudo_moves = self.get_pseudo_moves(from_x, from_y);
            if pseudo_moves.contains(&(to_x, to_y)) {
                return true;
            }
        }
        false
    }

    fn get_pseudo_captures(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut captures: HashSet<(usize, usize)> = HashSet::new();
        if let Some(piece) = self.get_piece(x, y) {
            match piece.piece_type {
                PieceType::Pawn => self.get_pseudo_captures_pawn(x, y, &mut captures),
                PieceType::Bishop => self.get_pseudo_captures_bishop(x, y, &mut captures),
                PieceType::Knight => self.get_pseudo_captures_knight(x, y, &mut captures),
                PieceType::Rook => self.get_pseudo_captures_rook(x, y, &mut captures),
                PieceType::Queen => self.get_pseudo_captures_queen(x, y, &mut captures),
                PieceType::King => self.get_pseudo_captures_king(x, y, &mut captures),
            }
        }
        captures
    }

    fn find_king(&self) -> Option<(usize, usize)> {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.get_piece(x, y) {
                    if piece.piece_type == PieceType::King {
                        if piece.piece_color == self.player_to_move {
                            return Some((x, y));
                        }
                    }
                }
            }
        }
        None
    }

    fn flip_board(&mut self) -> () {
        self.board.tiles.reverse();
    }

    fn in_check(&mut self) -> bool {
        self.flip_board();
        if let Some(king) = self.find_king() {
            for y in 0..8 {
                for x in 0..8 {
                    if let Some(piece) = self.get_piece(x, y) {
                        if piece.piece_color != self.player_to_move {
                            let captures = self.get_pseudo_captures(x, y);
                            if captures.contains(&king) {
                                self.flip_board();
                                return true;
                            }
                        }
                    }
                }
            }
        }
        self.flip_board();
        false
    }

    fn swap_turn(&mut self) -> () {
        self.player_to_move = match self.player_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White
        };
    }

    fn make_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> MoveInfo {
        let moved = self.board.tiles[from_y][from_x].expect("shouldn't be moving empty");
        let captured = self.board.tiles[to_y][to_x];
        let info = MoveInfo {
            moved,
            captured,
            from_x,
            from_y,
            to_x,
            to_y
        };
        self.board.tiles[to_y][to_x] = self.board.tiles[from_y][from_x];
        self.board.tiles[from_y][from_x] = None;
        info
    }

    fn unmake_move(&mut self, move_info: MoveInfo) -> () {
        self.board.tiles[move_info.to_y][move_info.to_x] = move_info.captured;
        self.board.tiles[move_info.from_y][move_info.from_x] = Some(move_info.moved);
    }

    fn is_legal_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if let Some(piece) = self.get_piece(from_x, from_y) {
            if piece.piece_color == self.player_to_move {
                if self.is_pseudo_legal(from_x, from_y, to_x, to_y) {
                    if let Some(_) = self.get_piece(to_x, to_y) {
                        if !self.can_be_here(to_x, to_y) {
                            return false;
                        }
                    }
                    let move_info = self.make_move(from_x, from_y, to_x, to_y);
                    let checked: bool = self.in_check();
                    self.unmake_move(move_info);
                    if checked {
                        return false;
                    }
                    return true;
                }
            }
        }
        false
    }
}

impl Game {
    pub fn new(board: Board) -> Self {
        Game {
            board,
            player_to_move: Color::White
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
    
    pub fn player_to_move(&self) -> Color {
        self.player_to_move
    }
    
    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        self.board.tiles[y][x]
    }

    pub fn get_legal_moves(&mut self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        self.get_pseudo_moves(x, y).into_iter().filter(|&(mx, my)| self.is_legal_move(x, y, mx, my)).collect()
    }

    fn can_make_any_move(&mut self) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(_) = self.get_piece(x, y) {
                    let moves = self.get_legal_moves(x, y);
                    if !moves.is_empty() {
                        return true;
                    }
                }
            }
        }
        return true;
    }

    pub fn is_checkmate(&mut self) -> bool {
        !self.can_make_any_move() && self.in_check()
    }

    pub fn is_stalemate(&mut self) -> bool {
        !self.can_make_any_move() && !self.in_check()
    }

    pub fn try_make_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        match self.player_to_move() {
            Color::White => {
                if self.is_legal_move(from_x, from_y, to_x, to_y) {
                    self.make_move(from_x, from_y, to_x, to_y);
                    self.swap_turn();
                    self.flip_board();
                    return true
                }
                false
            }
            Color::Black => {
                if self.is_legal_move(from_x, from_y, to_x, to_y) {
                    self.make_move(from_x, from_y, to_x, to_y);
                    self.swap_turn();
                    self.flip_board();
                    return true
                }
                false
            }
        }
    }
}

impl Board {
    pub fn new(setup: [[Option<Piece>; 8]; 8]) -> Self {
        Board {
            tiles: setup
        }
    }
}