fn get_neighbors(n: usize, board_size: usize, board_width: usize) -> [usize; 8] {
    [
        (n + board_size - board_width) % board_size,     // up
        (n + board_size - board_width + 1) % board_size, // up right
        (n + board_size + 1) % board_size,               // right
        (n + board_size + board_width + 1) % board_size, // down right
        (n + board_size + board_width) % board_size,     // down down
        (n + board_size + board_width - 1) % board_size, // down left
        (n + board_size - 1) % board_size,               // left
        (n + board_size - board_width - 1) % board_size, // up left
    ]
}

fn create_random_board(board_size: usize) -> Vec<bool> {
    let mut board = vec![false; board_size];
    for n in 0..board.len() {
        if rand::random() {
            board[n] = true;
        }
    }

    return board;
}

fn create_next_board(board: &Vec<bool>, board_width: usize, board_height: usize) -> Vec<bool> {
    let board_size = board_width * board_height;
    let mut new_board = vec![false; board_size];
    for n in 0..board_size {
        let mut neighbor_count = 0;
        let neighbors = get_neighbors(n, board_size, board_width);
        for neighbor_index in 0..8 {
            if board[neighbors[neighbor_index]] {
                neighbor_count += 1;
            }
        }

        // Any live cell with two or three live neighbours survives.
        // Any dead cell with three live neighbours becomes a live cell.
        // All other live cells die in the next generation. Similarly, all other dead cells stay dead.

        if board[n] && neighbor_count == 2 || neighbor_count == 3 {
            new_board[n] = true;
        } else if board[n] == false && neighbor_count == 3 {
            new_board[n] = true
        } else {
            new_board[n] = false
        }
    }

    return new_board;
}

fn display_board(board: &Vec<bool>, board_width: usize, board_height: usize) {
    let board_size = board_width * board_height;

    for n in 0..board_size {
        print!("{}", if board[n] { "X" } else { " " });
        if n % board_width == board_width - 1 {
            print!("\n");
        }
    }
    print!("\u{1b}[{}A", board_height);
}

fn main() {
    let board_width = 64;
    let board_height = 32;
    let board_size = board_width * board_height;

    let mut board = create_random_board(board_size);

    loop {
        display_board(&board, board_width, board_height);
        board = create_next_board(&board, board_width, board_height);
    }
}
