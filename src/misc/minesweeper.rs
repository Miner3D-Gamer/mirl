// /// Turn 2d coordinates into a 1d index
// #[must_use]
// pub fn xy_to_idx<T: mirl::math::NumberWithMonotoneOps>(
//     x: T,
//     y: T,
//     width: T,
// ) -> T {
//     y * width + x
// }
// /// Turn a 1d index into a 2d coordinate
// #[must_use]
// pub fn idx_to_xy<T: mirl::math::NumberWithMonotoneOps + Copy>(
//     idx: T,
//     width: T,
// ) -> (T, T) {
//     let y = idx / width;
//     let x = idx % width;
//     (x, y)
// }
// /// Generate a minefield with a safe area of 3x3
// #[must_use]
// #[cfg(feature = "debug-window")]
// pub fn generate_minefield(
//     width: usize,
//     height: usize,
//     safe_x: usize,
//     safe_y: usize,
//     mines: usize,
// ) -> Vec<bool> {
//     let total_tiles = width * height;
//     let mut id_board: Vec<usize> = 0.repeat_value(total_tiles);
//     let mut board = Vec::with_capacity(total_tiles);
//     let safe_range = 1;

//     for x in -safe_range..safe_range {
//         for y in -safe_range..safe_range {
//             id_board[xy_to_idx(
//                 safe_x.saturated_add_sign(x),
//                 safe_y.saturated_add_sign(y),
//                 width,
//             )] = usize::MAX;
//         }
//     }
//     let offset = id_board.iter().filter(|x| **x == usize::MAX).count();
//     #[allow(clippy::needless_range_loop)] // Clippy, no.
//     for i in 0..total_tiles {
//         if id_board[i] == 0 {
//             id_board[i] = i + offset;
//         } else if id_board[i] == usize::MAX {
//             id_board[i] = 0;
//         }
//     }
//     let mut remaining_mines = mines as f64;
//     for i in id_board {
//         let chance = remaining_mines / (total_tiles - i) as f64;
//         if chance > rand::random() {
//             remaining_mines -= 1.0;
//             board.push(true);
//         } else {
//             board.push(false);
//         }
//     }

//     board
// }

// fn count_mines_around_place(
//     position: (usize, usize),
//     board: &[bool],
//     width: usize,
//     height: usize,
// ) -> usize {
//     let mut mines = 0;
//     for x in -1..1 {
//         for y in -1..1 {
//             let new_pos = position.try_tuple_into().add((x, y));
//             if new_pos.0 > 0
//                 && new_pos.1 > 0
//                 && new_pos.0 < width as isize
//                 && new_pos.1 < height as isize
//                 && board
//                     [xy_to_idx(new_pos.0 as usize, new_pos.1 as usize, width)]
//             {
//                 mines += 1;
//             }
//         }
//     }
//     mines
// }

// /// Puts all numbers into place
// #[must_use]
// pub fn get_numbers_from_minefield(
//     board: &[bool],
//     width: usize,
//     height: usize,
// ) -> Vec<usize> {
//     let mut new_board = Vec::new();
//     for i in 0..board.len() {
//         new_board.push(count_mines_around_place(
//             idx_to_xy(i, width),
//             board,
//             width,
//             height,
//         ));
//     }
//     new_board
// }
// /// Convert state boards to a final visual representation
// #[must_use]
// pub fn get_visual_board(
//     minefield: &[bool],
//     numbers: &[usize],
//     revealed: &[bool],
//     flagged: &[bool],
// ) -> Vec<MinesweeperTile> {
//     let mut result = Vec::new();
//     for i in 0..minefield.len() {
//         if flagged[i] {
//             result.push(MinesweeperTile::Flag);
//         } else if !revealed[i] {
//             result.push(MinesweeperTile::UnRevealed);
//         } else if minefield[i] {
//             result.push(MinesweeperTile::Mine);
//         } else {
//             result.push(match numbers[i] {
//                 0 => MinesweeperTile::Empty,
//                 1 => MinesweeperTile::One,
//                 2 => MinesweeperTile::Two,
//                 3 => MinesweeperTile::Three,
//                 4 => MinesweeperTile::Four,
//                 5 => MinesweeperTile::Five,
//                 6 => MinesweeperTile::Six,
//                 7 => MinesweeperTile::Seven,
//                 8 => MinesweeperTile::Eight,
//                 9 => MinesweeperTile::Nine,
//                 _ => MinesweeperTile::Flag,
//             });
//         }
//     }

//     result
// }
// /// LOGIC
// pub fn handle_minesweeper_logic(
//     flag_mode: bool,
//     click_pos: (usize, usize),
//     mask: &mut [bool],
//     flagged: &mut [bool],
//     minefield: &[bool],
//     numbers: &[usize],
//     width: usize,
// ) -> (Vec<MinesweeperTile>, bool) {
//     let idx = xy_to_idx(click_pos.0, click_pos.1, width);
//     let mut lost = false;
//     if flag_mode {
//         if !mask[idx] {
//             flagged[idx] = !flagged[idx];
//         }
//     } else if !flagged[idx] {
//         mask[idx] = true;
//         if minefield[idx] {
//             lost = true;
//         }
//     }

//     (get_visual_board(minefield, numbers, mask, flagged), lost)
// }

// #[allow(missing_docs)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum MinesweeperTile {
//     UnRevealed,
//     Empty,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Flag,
//     Mine,
// }
