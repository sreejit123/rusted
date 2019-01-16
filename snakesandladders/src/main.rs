use std::{thread, time};

extern crate rand;

mod dice;
mod board;

fn main() {

    let board = &board::get_board();

    let mut pos_a :i32 = 1;
    let mut pos_b :i32 = 1;

    let mut has_a_started = false;
    let mut has_b_started = false;

    let mut is_pos_a_playing = true;
    let mut is_pos_b_playing = true;

    let ten_sec = time::Duration::from_millis(10000);

    while pos_a != 100 && pos_b != 100 {
        let dice_roll = dice::dice();



        if is_pos_a_playing {

            println!(" Pos A dice roll::: {}", dice_roll);
            is_pos_a_playing = false;
            is_pos_b_playing = true;

            // Player has to start with a 1
            if !has_a_started  && dice_roll != 1 {

                continue;
            }

            // If this is the first time, player gets a 1, they get a second chance
            if !has_a_started  && dice_roll == 1 {
                is_pos_a_playing = true;
                is_pos_b_playing = false;
                has_a_started = true;

                println!(" Pos A started with a 1, gets another chance, next move advances Pos A");
                continue;
            }


            // look for a snake or a ladder in the new position
            let new_pos_a = board::get_new_position(&board,  dice_roll as i32 + pos_a as i32);

            if new_pos_a <= 100 {
                pos_a = new_pos_a;
            }


            println!(" Pos A dice new position::: {}", pos_a);

            if pos_a == 100 {
                println!("Winner is Pos A");

            }


        } else if is_pos_b_playing {

            println!(" Pos B dice roll::: {}", dice_roll);

            is_pos_b_playing = false;
            is_pos_a_playing = true;

            if !has_b_started  && dice_roll != 1 {

                continue;
            }

            if !has_b_started  && dice_roll == 1 {
                is_pos_b_playing = true;
                is_pos_a_playing = false;
                has_b_started = true;

                println!(" Pos B started with a 1, gets another chance, next move advances Pos B");
                continue;
            }

            has_b_started = true;

            let new_pos_b = board::get_new_position(&board, dice_roll as i32 + pos_b as i32);

            if new_pos_b <= 100 {
                pos_b = new_pos_b;
            }

            println!(" Pos B dice new position::: {}", pos_b);

            if pos_b == 100 {
                println!("Winner is Pos B");

            }

        }


        thread::sleep(ten_sec);
    }
}
