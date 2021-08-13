pub fn show_help(){
    println!("Simulation of the Conway's Game of Life");
    println!("");
    println!("SEE:");
    println!("\thttps://en.wikipedia.org/wiki/Conway's_Game_of_Life");
    println!("");
    println!("USAGE:");
    println!("\tlgol [help] [ARGUMENTS]");
    println!("");
    println!("ARGUMENTS:");
    println!("\t width=[number]\t\t\tSet the width of the grid, default is 20");
    println!("\t height=[number]\t\tSet the height of the grid, default is 20");
    println!("\t delay=[number]\t\t\tSet the interval in milliseconds between each iteration, default is 500");
    println!("\t alive=[char]\t\t\tSet the character used for alive cells, default is O");
    println!("\t dead=[char]\t\t\tSet the character used for dead cells, default is \'");
}