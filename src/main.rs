use std::{env, process};
use tourney::config::*;
use tourney::programs::all::*;
use tourney::game::Player;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    println!("\nTournament start\n");

    let players = vec![
        Player::with_name("Anurag-strategy1", take_back_once_prisoner),
        Player::with_name("Simple-strategy1", friendly),
        Player::with_name("Simple-strategy2", evil),
        Player::with_name("Anurag-strategy2", tit_for_tat_prisoner),
        Player::with_name("Anurag-strategy3", tit_for_two_tats_prisoner),
        Player::with_name("Tanvi-strategy1", greedy_blue_and_friendly),
        Player::with_name("Tanvi-strategy2", greedy_blue_and_evil),
        Player::with_name("Simple-strategy3", blue),
        Player::with_name("Navya-strategy1", try_to_guess),
        Player::with_name("Simple-strategy4", random),
        Player::with_name("Anjali-strategy1", chat_gpt_adaptive),
        Player::with_name("Anjali-strategy2", chat_gpt_proactive),
        Player::with_name("Anjali-strategy3", chat_gpt_versatile),
        Player::with_name("Simple-strategy5", cooperate_until_defection),
        Player::with_name("Tanvi-strategy3", greedy_if_winning_else_random),
        Player::with_name("Tanvi-strategy4", greedy_if_2x_score_else_random),
        Player::with_name("Simple-strategy6", copy),
        Player::with_name("Simple strategy7", smarter_copy),
        Player::with_name("Navya-strategy2", greed_first_15),
    ];

    println!("Pairing every program... ({0} games)\n", players.len() * (players.len() - 1) / 2);

    let scores = run(&config, &players).unwrap();

    println!("{0} rounds!\n", config.rounds());
    println!("no. program_name                     avg_score   rel_win_ratio");
    println!("--------------------------------------------------------------");

    for (i, v) in scores.iter().enumerate() {
        let placement = format!("{}.", i + 1);
        let ratio = format!("({:.2}%)", v.1 * 100.0);
        println!("{0:<3} {2:<32} {1:<11.2} {3:<8}", placement, v.0 as f32 / players.len() as f32, v.2, ratio);
    }

    println!("\nTournament end\n");
}