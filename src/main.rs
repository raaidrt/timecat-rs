// https://github.com/glinscott/nnue-pytorch
// https://hxim.github.io/Stockfish-Evaluation-Guide/
// https://github.com/topics/nnue
// https://github.com/topics/chess-engine?l=rust
// https://github.com/dsekercioglu/blackmarlin.git
// https://github.com/zxqfl/sashimi
// https://backscattering.de/chess/uci/

mod tests;

use std::io::IsTerminal;
use tests::test;
use timecat::*;

fn main() {
    let clock = Instant::now();
    let args = env::args().collect_vec();
    let args = args.iter().map(|s| s.as_str()).collect_vec();
    if !args.contains(&"--disable-backtrace") {
        env::set_var("RUST_BACKTRACE", "1");
    }
    if !io::stdin().is_terminal() || args.contains(&"--uci") {
        set_uci_mode(true, false);
    }
    if !io::stdout().is_terminal() || args.contains(&"--no-color") {
        set_colored_output(false, false);
    }
    if args.contains(&"--test") {
        test().unwrap();
    } else if args.contains(&"-c") || args.contains(&"--command") {
        let command = args
            .iter()
            .skip_while(|&arg| !["-c", "--command"].contains(arg))
            .skip(1)
            .take_while(|&&arg| !arg.starts_with("--"))
            .join(" ");
        let mut engine = Engine::default();
        println!();
        if let Err(err) = Parser::parse_command(&mut engine, &command) {
            let err_msg = err.stringify(Some(command.as_str()));
            println!("\n{}", colorize(err_msg, ERROR_MESSAGE_STYLE));
        }
    } else {
        let info_text = format!("{} v{}", ENGINE_NAME, ENGINE_VERSION);
        println!("{}\n", colorize(info_text, SUCCESS_MESSAGE_STYLE));
        Parser::main_loop();
    }
    let elapsed_time = clock.elapsed().as_secs_f64();
    let precision = 3;
    println_info("\nRun Time", format!("{:.1$} s", elapsed_time, precision));
}
