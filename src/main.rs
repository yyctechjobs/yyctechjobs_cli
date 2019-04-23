use clap::{App, Arg, ArgMatches};

mod posts;
mod client;

trait CommandLine {
    fn register_command(&self) -> App;
    fn handle(&self, matches: &ArgMatches) -> Vec<Box<client::Command>>;
}

fn main() {

    let post_endpoint = posts::Posts::new();

    let mut app = App::new("YYC Tech Jobs Cli")
                .version("0.0.1")
                .author("yyc tech<contact@yyctech.dev>")
                .about("CLI for reading YYC Tech jobs while you're at work!")
                .arg(
                    Arg::with_name("dev")
                        .short("d")                        
                        .help("set dev testing mode")
                    );
    
    app = app.subcommand(post_endpoint.register_command());

    let matches = app.get_matches();    

    let client = client::Client::new(matches.is_present("dev"), "v1/");

    let mut commands_to_run = Vec::<Box<client::Command>>::new();

    match matches.subcommand() {
        (e, Some(sub_m)) if e == post_endpoint.endpoint => { 
            let mut cmds = post_endpoint.handle(&sub_m); 
            commands_to_run.append(&mut cmds);
        }
        _ => { println!("Incorrect Usage!") }
    }

    for command in commands_to_run {
        command.do_command(&client);
    }

}
