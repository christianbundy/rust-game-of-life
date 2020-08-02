use std::time::Instant;

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
    (0..board_size).map(|_| rand::random()).collect()
}

fn create_next_board(board: &Vec<bool>, board_width: usize, board_height: usize) -> Vec<bool> {
    let board_size = board_width * board_height;
    (0..board_size)
        .map(|n| {
            let neighbors = get_neighbors(n, board_size, board_width);

            let neighbor_count = neighbors
                .iter()
                .map(|neighbor| if board[*neighbor] { 1 } else { 0 })
                .sum();

            // Any live cell with two or three live neighbours survives.
            // Any dead cell with three live neighbours becomes a live cell.
            // All other live cells die in the next generation. Similarly, all other dead cells stay dead.

            match (board[n], neighbor_count) {
                (true, 2) => true,
                (_, 3) => true,
                _ => false,
            }
        })
        .collect()
}

fn display_board(board: &[bool], board_width: usize, board_height: usize) {
    let board_size = board_width * board_height;

    let buff = (0..board_size)
        .filter(|n| n % (board_width * 2) >= board_width && n % 2 == 1)
        .flat_map(|n| {
            let symbol = match (
                board[(n + board_size - board_width) % board_size], // up
                board[n],                                           // current
                board[(n + board_size - 1) % board_size],           // left
                board[(n + board_size - board_width - 1) % board_size],
            ) {
                (false, false, false, false) => " ",
                (false, false, false, true) => "▘",
                (false, false, true, false) => "▖",
                (false, false, true, true) => "▌",
                (false, true, false, false) => "▗",
                (false, true, false, true) => "▚",
                (false, true, true, false) => "▄",
                (false, true, true, true) => "▙",
                (true, false, false, false) => "▝",
                (true, false, false, true) => "▀",
                (true, false, true, false) => "▞",
                (true, false, true, true) => "▛",
                (true, true, false, false) => "▐",
                (true, true, false, true) => "▜",
                (true, true, true, false) => "▟",
                (true, true, true, true) => "█",
            };
            if n % board_width == board_width - 1 {
                vec![symbol, "\n"]
            } else {
                vec![symbol]
            }
        })
        .collect::<Vec<_>>();

    print!("\u{1b}[{}A{}", board_height / 2, buff.join(""));
}

fn create_display(board_height: usize) {
    let mut buffer = vec![];
    for _ in 0..board_height / 2 - 1 {
        buffer.push("\n");
    }
    print!("{}", buffer.join(""));
}

fn main() {
    use terminal_size::{terminal_size, Height, Width};

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        // HACK: I want to ensure that there are an even number of rows and columns because of the
        // weird unicode drawing stuff we're doing in `display_board()`, but I also want to be very
        // careful that we aren't displaying more characters than the terminal actually supports.
        //
        // Because of this, I remove one character, round to the nearest even character, and then
        // multiply by two (because each unicode character is actually displaying two board tiles).
        let board_width: usize = ((w as f32 - 1.5) / 2.0).floor() as usize * 4;
        let board_height: usize = ((h as f32 - 1.5) / 2.0).floor() as usize * 4;
        let board_size = board_width * board_height;
        let mut board = create_random_board(board_size);

        let mut last_frame = Instant::now();

        let target_fps = 240;
        let target_pause = 1.0 / target_fps as f32;

        // Since `display_board()` deletes a bunch of lines and then redraws them, we want to start
        // out by writing a bunch of empty lines to the terminal so that we don't erase the CLI
        // history.
        create_display(board_height);

        loop {
            if last_frame.elapsed().as_secs_f32() >= target_pause {
                display_board(&board, board_width, board_height);
                last_frame = Instant::now();
                board = create_next_board(&board, board_width, board_height);
            }
        }
    } else {
        println!("Unable to get terminal size :(");
    }
}
