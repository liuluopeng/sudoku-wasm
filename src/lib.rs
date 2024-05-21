use core::fmt;
use std::fmt::Display;

use rand::seq::SliceRandom;
use rand::thread_rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Soduku {
    board: Vec<Vec<usize>>,     // 棋盘
    ori_board: Vec<Vec<usize>>, // 初始状态的棋盘
    ans_board: Vec<Vec<usize>>, // 答案的棋盘.
    dig_list: Vec<usize>,       // 挖去的空  的 列表. 代表难度
}

#[wasm_bindgen]
impl Soduku {
    // 生成一个随机的数独形状:
    fn gen_random_soduku() -> Vec<Vec<usize>> {
        let mut board = vec![vec![0; 9]; 9];

        board[0] = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut rng = thread_rng();
        board[0].shuffle(&mut rng);

        let mut line_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut col_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut sub_exists: Vec<Vec<bool>> = vec![vec![false; 9]; 9];

        let mut un_set_list = vec![];

        // 统计已经落位的格子
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != 0 {
                    let index_num = board[i][j] - 1;

                    line_exists[i][index_num] = true; // index+1已经在某一行存在了.
                    col_exists[j][index_num] = true; //
                    sub_exists[Soduku::get_sub_index(i, j)][index_num] = true;
                } else {
                    un_set_list.push(i * 9 + j);
                }
            }
        }

        Soduku::find_a_solution(
            &mut board,
            &mut line_exists,
            &mut col_exists,
            &mut sub_exists,
            0,
            &un_set_list,
            &mut 0,
        );

        board
    }

    /// 第一行是123456789 生成一个随机的数独
    fn find_a_solution(
        board: &mut Vec<Vec<usize>>,
        line_exists: &mut Vec<Vec<bool>>,
        col_exists: &mut Vec<Vec<bool>>,
        sub_exists: &mut Vec<Vec<bool>>,
        depth: usize,
        un_set_list: &Vec<usize>,
        seted_index: &mut usize,
    ) -> bool {
        if *seted_index == un_set_list.len() {
            return true;
        }

        let (i, j) = Soduku::get_i_j(un_set_list[*seted_index]);

        let sub_index = Soduku::get_sub_index(i, j);

        let mut space = String::from("");

        for k in 0..depth {
            space += "  ";
        }

        let mut maybe_list =
            Soduku::get_maybe_list(&line_exists, &col_exists, &sub_exists, i, j, sub_index);

        let mut rng = thread_rng();
        maybe_list.shuffle(&mut rng);

        for atmpt in maybe_list.clone() {
            if line_exists[i][atmpt - 1] == false
                && col_exists[j][atmpt - 1] == false
                && sub_exists[sub_index][atmpt - 1] == false
            {
                board[i][j] = atmpt;
                line_exists[i][atmpt - 1] = true;
                col_exists[j][atmpt - 1] = true;
                sub_exists[sub_index][atmpt - 1] = true;

                *seted_index += 1;
                if Soduku::find_a_solution(
                    board,
                    line_exists,
                    col_exists,
                    sub_exists,
                    depth + 1,
                    un_set_list,
                    seted_index,
                ) {
                    return true;
                } else {
                    // atmpt没有解, 回退
                    board[i][j] = 0;
                    line_exists[i][atmpt - 1] = false;
                    col_exists[j][atmpt - 1] = false;
                    sub_exists[sub_index][atmpt - 1] = false;
                    *seted_index -= 1;
                }
            }
        }

        return false;
    }

    fn only_one_solution(board: &mut Vec<Vec<usize>>) -> bool {
        let mut counter = 0;

        let mut res = vec![];

        let mut un_set_list = vec![];

        // 统计已经落位的格子
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != 0 {
                } else {
                    un_set_list.push(i * 9 + j);
                }
            }
        }

        println!("un set list: {:?}", &un_set_list);

        Soduku::find_ess_solution(
            board,
            &un_set_list,
            &mut 0,
            &mut res,
            &mut counter,
            &mut false,
        );

        counter == 1
    }

    /// .找出多个数独的解
    fn find_ess_solution(
        board: &mut Vec<Vec<usize>>,
        un_set_list: &Vec<usize>,
        now_index: &mut usize,
        res: &mut Vec<Vec<Vec<usize>>>,
        count: &mut usize,
        more_than_one: &mut bool,
    ) {
        if *more_than_one {
            return;
        }

        if *now_index == un_set_list.len() {
            // 结束
            *count += 1;
            println!("count {:?}", *count);
            if *count > 1 {
                *more_than_one = true;
            }
            return;
        }

        let (i, j) = Soduku::get_i_j(un_set_list[*now_index]);

        for num in 1..10 {
            // num: 1~9
            if Soduku::is_valid_(board, i, j, num) {
                board[i][j] = num;

                // 继续寻找
                *now_index += 1;
                Soduku::find_ess_solution(board, un_set_list, now_index, res, count, more_than_one);

                board[i][j] = 0;
                *now_index -= 1;
            } else {
                continue;
            }
        }
    }

    fn get_maybe_list(
        ex1: &Vec<Vec<bool>>,
        ex2: &Vec<Vec<bool>>,
        ex3: &Vec<Vec<bool>>,
        i: usize,
        j: usize,
        sub_index: usize,
    ) -> Vec<usize> {
        let mut res = vec![];

        for k in 0..9 {
            if ex1[i][k] == false && ex2[j][k] == false && ex3[sub_index][k] == false {
                res.push(k + 1);
            }
        }

        res
    }

    // 生成挖坑的index 的列表
    fn gen_random_dig() -> Vec<usize> {
        let mut res: Vec<usize> = (0..81).collect();

        let mut rng = thread_rng();
        res.shuffle(&mut rng);
        res
    }

    // 按照挖坑顺序不断挖坑,坑少的时候, 数独有唯一解,   直到数独的结果不唯一了停止.
    fn try_one_by_one(board: &mut Vec<Vec<usize>>) -> Vec<usize> {
        let dig_list = Soduku::gen_random_dig();

        println!("dig list: {:?}", &dig_list);

        let mut board_try = board.clone();

        let mut limit = 0;

        for k in 0..dig_list.len() {
            let index_be_diged = dig_list[k];
            let (i, j) = Soduku::get_i_j(index_be_diged);

            board_try[i][j] = 0;

            // 检测此时是否是唯一解
            let res = Soduku::only_one_solution(&mut board_try);

            if !res {
                // 开始出现不唯一的解了的时候 停止
                limit = k - 1; // 只挖一个洞肯定有唯一解  i必大于0  usize 这里不会越界的.
                break;
            }
        }

        println!("最多到: {:?}", limit);

        dig_list[0..limit].to_vec()
    }

    // 重开本局
    pub fn restart(&mut self) {
        self.board = self.ori_board.clone();
    }

    pub fn new() -> Self {
        let mut full_board = Soduku::gen_random_soduku();

        let dig_list = Soduku::try_one_by_one(&mut full_board);

        // 挖洞
        let mut board = full_board.clone();
        for &index in &dig_list {
            let (i, j) = Soduku::get_i_j(index);
            board[i][j] = 0;
        }

        let ori_board = board.clone();
        let ans_board = full_board.clone();

        Soduku {
            board: board,
            ori_board: ori_board,
            ans_board: ans_board,
            dig_list: dig_list,
        }
    }

    // vue on mounted:
    pub fn get_new_board(&self) -> Vec<usize> {
        self.get_current_board()
    }

    pub fn get_dig_list(&self) -> Vec<usize> {
        let res = self.dig_list.clone();

        res
    }

    pub fn get_current_board(&self) -> Vec<usize> {
        let mut res = vec![];

        for i in 0..9 {
            for j in 0..9 {
                res.push(self.board[i][j]);
            }
        }

        res
    }

    pub fn get_current_board_change_diffcu(&mut self, dig_size: usize) -> Vec<usize> {
        // 改动难度
        let mut temp_board = self.ans_board.clone();
        for k in 0..dig_size {
            // 共计改动dig_size个
            let index = self.dig_list[k];
            let (i, j) = Soduku::get_i_j(index);
            temp_board[i][j] = 0;
        }
        self.board = temp_board.clone();

        self.get_current_board()
    }

    pub fn get_answer_numbers(&self) -> Vec<usize> {
        let mut res = vec![];

        for i in 0..9 {
            for j in 0..9 {
                res.push(self.ans_board[i][j]);
            }
        }

        res
    }

    // pub fn get_answer_2d(&self) -> Vec<Vec<usize>> {
    //     self.ans_board
    // }

    pub fn get_ori_numbers(&self) -> Vec<usize> {
        let mut res = vec![];

        for i in 0..9 {
            for j in 0..9 {
                res.push(self.ori_board[i][j]);
            }
        }

        res
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    // 看看当前的数独有没有错误 刚刚新填入了
    fn is_valid(&self, i: usize, j: usize, num: usize) -> bool {
        let not_repeat = num;

        // 同一行:  board[i][0] 到  board[i][8]
        for k in 0..9 {
            if self.board[i][k] == not_repeat {
                return false;
            }
        }

        // col:
        for k in 0..9 {
            if self.board[k][j] == not_repeat {
                return false;
            }
        }

        for k in 0..9 {
            if self.board[3 * (i / 3) + k / 3][3 * (j / 3) + k % 3] == not_repeat {
                // 来自:  【【回溯37】解数独】 https://www.bilibili.com/video/BV15z4y1h7L9/?share_source=copy_web&vd_source=5a2d7ecacd3ba507f3c18fde1192c687

                return false;
            }
        }

        true
    }
    // 看看当前的数独有没有错误 刚刚新填入了
    fn is_valid_(board: &mut Vec<Vec<usize>>, i: usize, j: usize, num: usize) -> bool {
        let not_repeat = num;

        // 同一行:  board[i][0] 到  board[i][8]
        for k in 0..9 {
            if board[i][k] == not_repeat {
                return false;
            }
        }

        // col:
        for k in 0..9 {
            if board[k][j] == not_repeat {
                return false;
            }
        }

        for k in 0..9 {
            if board[3 * (i / 3) + k / 3][3 * (j / 3) + k % 3] == not_repeat {
                // 来自:  【【回溯37】解数独】 https://www.bilibili.com/video/BV15z4y1h7L9/?share_source=copy_web&vd_source=5a2d7ecacd3ba507f3c18fde1192c687

                return false;
            }
        }

        true
    }

    // 某格子是否属于原始的棋盘
    pub fn is_in_ori_board(&self, i: usize, j: usize) -> bool {
        self.ori_board[i][j] != 0
    }

    // [i,j]属于第几个九宫格:
    pub fn get_sub_index(i: usize, j: usize) -> usize {
        // 等比缩小到原来的三分之一
        let i_zoom = i / 3;
        let j_zoom = j / 3;
        i_zoom * 3 + j_zoom
    }

    // 填空
    pub fn input_a_number(&mut self, i: usize, j: usize, num: usize) -> bool {
        // self.is_valid();

        self.board[i][j] = num;

        //  更新 辅助 的成员变量
        // self.line_exists[i][num - 1] = true;
        // self.col_exists[j][num - 1] = true;
        // self.sub_block_exists[Soduku::get_sub_index(i, j)][num - 1] = true;

        true
    }

    // 回退: 把[i,j]变成0
    pub fn back(&mut self, i: usize, j: usize) {
        self.board[i][j] = 0;
    }

    // 格子当前可能的数字:
    pub fn get_candi_list(&self, i: usize, j: usize) -> Vec<usize> {
        let mut res = vec![];

        for k in 1..10 {
            if self.is_valid(i, j, k) {
                res.push(k);
            }
        }

        res
    }

    // 显示答案
    pub fn get_answer(&self) -> String {
        let mut res = String::new();

        for line in &self.ans_board {
            let mut line_st = String::new();

            for k in line {
                line_st.push_str(&k.to_string());
                line_st.push_str(" ");
            }

            line_st.push_str("\n");
            res.push_str(&line_st);
        }

        res
    }

    fn get_i_j(index: usize) -> (usize, usize) {
        let i = index / 9;
        let j = index % 9;
        (i, j)
    }

    // 被设置数字了, 不管是原来就有还是玩家填写的
    pub fn is_setted(&mut self, i: usize, j: usize) -> bool {
        self.board[i][j] != 0
    }
}

impl Display for Soduku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 打印九宫格

        for line in &self.board {
            writeln!(f, "{:?}\n", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Soduku;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        let mut board = Soduku::gen_random_soduku();

        let dig_list = Soduku::try_one_by_one(&mut board);
        println!("挖去的坐标: {:?}", dig_list)
    }
}
