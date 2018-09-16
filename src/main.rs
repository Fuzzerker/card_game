use std::io::stdin;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
fn main() {
    
    let mut g = Game::new();
    println!("enter a number");
    let num = read_as_int_with_limit(10);
    g.init(num);
    g.play();
  
}
fn read_as_int_with_limit(limit: usize) -> usize{
    let entered_number = read_as_int();
    if entered_number > limit {
        println!("Too high, must be lower than {}",limit); 
        return read_as_int_with_limit(limit);
    }
    return entered_number;
}
fn read_as_int() -> usize{
    let mut str_buff = String::new();
    let read_res =  std::io::stdin().read_line(&mut str_buff);
    match(read_res)  {
        Err(e) => {
            println!("couldn't read, try again?");
            return read_as_int();
        },
        Ok(i) => {
            let buff_len = str_buff.len();
            str_buff.truncate(buff_len -2);
        }
    }
    let str_buff_to_int = str_buff.parse::<usize>();
    let mut entered_number = 0;
    match(str_buff_to_int) {
        Err(e) => {
            println!("you didn't enter a number! {} err: {}", str_buff, e);
            return read_as_int();
        },
        Ok(parsed_int) => {
            entered_number = parsed_int;
        }
    }
    return entered_number;
}
struct Game {
    board: Board,
    cur_highest_val: usize,
    cur_card: usize,
    card_opts: Vec<usize>,
}
struct Board {
    cols_and_rows: Vec<Vec<usize>>,
    col_count: usize,
    row_count: usize,
}
impl Game {
    fn init(&mut self, size: usize) {
        self.board.setup(size, 4);
    }
    fn new() -> Game {
        let mut starting_card_opts = Vec::new();
        starting_card_opts.push(1);
        let g = Game{
            board: Board{
                cols_and_rows: Vec::new(),
                col_count: 0,
                row_count: 0,
            },
            card_opts: starting_card_opts,
            cur_card: 0,
            cur_highest_val: 0,
        };
        g        
    }
    fn play(&mut self) {
        self.board.print();
        self.draw_card();
        self.print_card();
        let chosen_col = read_as_int_with_limit(self.board.col_count);
        self.place_on_col(chosen_col);
      
        self.play();
    }
    fn place_on_col(&mut self, col: usize) {
        if col > self.board.col_count {
            self.end_game_with_message("hmmm, somehow got an invalid column.  this shouldn't be possible.")
        }       
        let col_idx = col -1;
          let col_overflow=self.board.place_on_col(col_idx, self.cur_card);
          if col_overflow {
              self.end_game();
          }
    }
    fn end_game_with_message(&self, msg: &str) {
        println!("{}", msg);
        self.end_game();
    }
    
    fn end_game(&self) {
        println!("game over!");
        std::process::exit(0);
    }
    fn print_card(&self) {
        println!("cur card: {}", self.cur_card)
    }
    fn draw_card(&mut self) {
        let len = self.card_opts.len();
        let rand_idx = poor_mans_rand(len - 1);
        self.cur_card = self.card_opts[rand_idx];
    }
}
impl Board {
    fn setup(&mut self, num_rows: usize, num_cols: usize) {
        let mut columns: Vec<Vec<usize>> = Vec::new();
        for _ in 0..num_cols {
            let mut col = Vec::new();
            for _ in 0..num_rows {
                col.push(0);
            }
            self.cols_and_rows.push(col);
        }
        self.row_count = num_rows;
        self.col_count = num_cols;
    }
    fn print(&self) {
        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                 print!("{}", self.cols_and_rows.get(col_index).unwrap().get(row_index).unwrap());
            }     
            println!("{}","");
        }
    }
    fn place_on_col(&mut self, col_idx: usize, card: usize) -> bool {
        let mut end_col = Vec::new();
        let immutable_self_col_ref =  self.cols_and_rows[col_idx].clone();
        for row_idx in 0..self.row_count {
            end_col.push(immutable_self_col_ref[row_idx]);
        }
        let mut replace_at_idx: isize = -1;
        for row_idx in 0..self.row_count {
            let val_at_idx = end_col[row_idx];
            if val_at_idx == 0 {
                println!("would place at {}", row_idx);
                replace_at_idx = row_idx as isize;
                break;
            }
        }
        if replace_at_idx == -1 {
            return true;
        }
        let usize_replace = replace_at_idx as usize;
        end_col[usize_replace] = card;
        self.cols_and_rows[col_idx] = end_col.to_vec();
        false
    }
} 
fn poor_mans_rand(limit: usize) -> usize {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();
    let mut rand = 0;
    let iters = reduce(since_the_epoch as usize, &limit);
    for _ in 0..iters {
        rand += 1;
        if rand > limit {
            rand = 0;
        }
    }
    return rand
}
fn reduce(num: usize, limit: &usize) -> usize{
    let act_limit = *limit;
    let end_num = num;
    if num > act_limit * 10 {
        return reduce((num / 2) as usize, limit)
    }
    return end_num;
}