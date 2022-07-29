mod game;
mod belief;

use game::Game;

fn main() {
    let simulation_size = 5000.0;

    let mut agent0_count = 0;
    let mut agent1_count = 0;
    let mut agent2_count = 0;
    let mut agent3_count = 0;
    let mut agent4_count = 0;
    let mut agent5_count = 0;
    let mut agent6_count = 0;
    let mut agent7_count = 0;
    for _ in 0..simulation_size as usize {
        let game = Game::new();
        agent0_count = agent0_count + game.clone().agent_0();
        agent1_count = agent1_count + game.clone().agent_1();
        agent2_count = agent2_count + game.clone().agent_2();
        agent3_count = agent3_count + game.clone().agent_3();
        agent4_count = agent4_count + game.clone().agent_4();
        agent5_count = agent5_count + game.clone().agent_5();
        agent6_count = agent6_count + game.clone().agent_6();
        agent7_count = agent7_count + game.clone().agent_7();
    }
    println!(
        "agent0 {}\nagent1 {}\nagent2 {}\nagent3 {}\nagent4 {}\nagent5 {}\nagent6 {}\nagent7 {}",
        agent0_count as f64 / simulation_size,
        agent1_count as f64 / simulation_size,
        agent2_count as f64 / simulation_size,
        agent3_count as f64 / simulation_size,
        agent4_count as f64 / simulation_size,
        agent5_count as f64 / simulation_size,
        agent6_count as f64 / simulation_size,
        agent7_count as f64 / simulation_size
    );
}
