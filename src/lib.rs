pub mod chess;
pub mod fen;
pub mod graphics;

#[cfg(test)]
mod tests {
    use crate::chess;
    use crate::graphics;
    use crate::graphics::Screen;
    use std::collections::HashSet;
    use std::io;

    const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
    const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    fn piece_char(piece: &chess::Piece) -> char {
        match (piece.piece_type(), piece.piece_color()) {
            (chess::PieceType::Pawn, chess::Color::White) => '♙',
            (chess::PieceType::Pawn, chess::Color::Black) => '♟',
            (chess::PieceType::Bishop, chess::Color::White) => '♗',
            (chess::PieceType::Bishop, chess::Color::Black) => '♝',
            (chess::PieceType::Knight, chess::Color::White) => '♘',
            (chess::PieceType::Knight, chess::Color::Black) => '♞',
            (chess::PieceType::Rook, chess::Color::White) => '♖',
            (chess::PieceType::Rook, chess::Color::Black) => '♜',
            (chess::PieceType::Queen, chess::Color::White) => '♕',
            (chess::PieceType::Queen, chess::Color::Black) => '♛',
            (chess::PieceType::King, chess::Color::White) => '♔',
            (chess::PieceType::King, chess::Color::Black) => '♚',
        }
    }

    fn draw_piece(piece: &chess::Piece, x: usize, y: usize, screen: &mut graphics::Screen) -> () {
        let fg = match piece.piece_color() {
            chess::Color::White => graphics::Color::new(255, 255, 255),
            chess::Color::Black => graphics::Color::new(0, 0, 0),
        };
        let c = piece_char(&piece);
        screen.set_pixel(x, y, None, Some(fg), Some(c));
    }

    fn draw_board(game: &chess::Game, x: usize, y: usize, screen: &mut graphics::Screen) -> () {
        for dy in 0..8 {
            for dx in 0..8 {
                
                let bg = if (dx + dy) % 2 == 0 {
                    graphics::Color::new(196, 196, 196)
                } else {
                    graphics::Color::new(32, 32, 32)
                };
                screen.set_pixel(x + dx, y + dy, Some(bg), None, None);
                
                if let Some(piece) = game.get_piece(dx, dy) {
                    draw_piece(&piece, x + dx, y + dy, screen);
                }
            }
        }
    }

    fn draw_moves(board_x: usize, board_y: usize, moves: &HashSet<(usize, usize)>, color: graphics::Color, screen: &mut Screen) -> () {
        for (x, y) in moves {
            screen.set_pixel(board_x + x, board_y + y, Some(color), None, None)
        }
    }

    #[test]
    fn test() -> () {
        
        let mut game = chess::Game::new(chess::Board::new(chess::BOARD_DEFAULT_SETUP));

        let mut screen: graphics::Screen = graphics::Screen::new(10, 10);

        let mut cursor_x: usize = 0;
        let mut cursor_y: usize = 0;
        
        let board_x: usize = 1;
        let board_y: usize = 1;

        let mut from: Option<(usize, usize)> = None;
        let mut to: Option<(usize, usize)> = None;

        loop {
            screen.clear(Some(graphics::Color::new(48, 48, 64)), None, Some(' '));

            draw_board(&game, board_x, board_y, &mut screen);

            for i in 0..8 {
                screen.set_pixel(board_x + i, board_y - 1, None, Some(graphics::Color::new(128, 128, 196)), Some(FILE_CHARS[i]));
                screen.set_pixel(board_x + i, board_y + 8, None, Some(graphics::Color::new(128, 128, 196)), Some(FILE_CHARS[i]));
                screen.set_pixel(board_x - 1, board_y + i, None, Some(graphics::Color::new(128, 128, 196)), Some(RANK_CHARS[7 - i]));
                screen.set_pixel(board_x + 8, board_y + i, None, Some(graphics::Color::new(128, 128, 196)), Some(RANK_CHARS[7 - i]));
            }
            
            if let Some((from_x, from_y)) = from {
                screen.set_pixel(board_x + from_x, board_y + from_y, Some(graphics::Color::new(255, 255, 196)), None, None);
                let moves = game.get_legal_moves(from_x, from_y);
                draw_moves(board_x, board_y, &moves, graphics::Color::new(128, 128, 196), &mut screen);
            }
            
            screen.set_pixel(board_x + cursor_x, board_y + cursor_y, Some(graphics::Color::new(232, 232, 196)), None, None);

            match game.player_to_move() {
                chess::Color::White => screen.set_pixel(0, 0, Some(graphics::Color::new(255, 255, 255)), Some(graphics::Color::new(16, 16, 16)), Some('W')),
                chess::Color::Black => screen.set_pixel(0, 0, Some(graphics::Color::new(16, 16, 16)), Some(graphics::Color::new(196, 196, 196)), Some('B'))
            };

            let render = screen.render(false, true);

            println!("\x1B[H");
            println!("\x1B[2J");
            println!("{}", render);

            if game.is_checkmate() {
                println!("Checkmate!");
                return;
            }

            if game.is_stalemate() {
                println!("Stalemate...");
                return;
            }
            
            match io::stdin().lines().next() {
                Some(input) => match input {
                    Ok(line) => for c in line.chars() {
                        match c.to_ascii_uppercase() {
                            'W' => if cursor_y < 8 - 1 { cursor_y += 1; },
                            'A' => if cursor_x >= 1 { cursor_x -= 1; },
                            'S' => if cursor_y >= 1 { cursor_y -= 1; },
                            'D' => if cursor_x < 8 - 1 { cursor_x += 1; },
                            ' ' => {
                                if from.is_none() {
                                    from = Some((cursor_x, cursor_y));
                                } else {
                                    to = Some((cursor_x, cursor_y));
                                }
                            },
                            '.' => {
                                from = None;
                                to = None;
                            }
                            _ => ()
                        }
                    },
                    Err(_) => ()
                }
                None => ()
            }

            if let Some((from_x, from_y)) = from {
                if let Some((to_x, to_y)) = to {
                    _ = game.try_make_move(from_x, from_y, to_x, to_y);
                    from = None;
                    to = None;
                }
            }
            
        }
    }
}
