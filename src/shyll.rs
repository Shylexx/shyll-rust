mod shell;

use std::io::BufRead;

use shell::Shell;

fn main() -> Result<(), std::io::Error> {
    // Load Config
    //
    // Create Shell
    let mut shell = Shell::start();
    // Run Command Loop
    shell.shy_loop(); 
    // Shutdown
    Ok(())
}


