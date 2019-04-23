use serde::{Deserialize, Serialize};
use serde_json::{Value};
use clap::{App, Arg, SubCommand, ArgMatches}; 

use colored::*;

use super::CommandLine;
use super::client::{Command,FetchAndShowCommand};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: String,
    company_name: String,
    job_title: String,
    tech : Vec<String>,
    feature : bool,
    posting_date : String,
}

#[derive(Debug)]
pub struct Posts {
    pub endpoint:String,
}

impl Posts {
    pub fn new() -> Posts {
        Posts {
            endpoint:"posts".to_owned()
        }        
    }
}

impl CommandLine for Posts {
    fn register_command(&self) -> App {
        let list = SubCommand::with_name("ls")
                    .arg(
                        Arg::with_name("page")
                            .takes_value(true)                            
                            .help("page")
                    )
                    .arg(
                        Arg::with_name("tech")
                            .takes_value(true)
                            .multiple(true)
                            .last(true)
                            .help("query by tech")
                    )                    
                    .help("list postings");

        let read = SubCommand::with_name("read")
                    .arg(
                        Arg::with_name("id")
                            .required(true)                                                        
                            .help("id of job you want to read")
                    )                    
                    .help("read a specific job posting => usage: calgary_jobs_cli posts read <id>");
        
        SubCommand::with_name(&self.endpoint)
            .subcommand(list)
            .subcommand(read)
    }   

    fn handle(&self, matches: &ArgMatches) -> Vec<Box<Command>>{
        match matches.subcommand() {
            ("ls", Some(_sub_m)) => {
                let mut commands = Vec::<Box<Command>>::new();

                let handler : Box<Fn(String)> = Box::new(|result: String| {
                    let v: Value = serde_json::from_str(&result).unwrap();

                    // todo this should be refactored into a bettter generic handler 
                    for post in v["data"].as_array().unwrap() {                        
                        println!("-------{}-------", post["company_name"].to_string().bold().red());
                        println!("{}", post["id"].to_string().green());
                        println!("{}", post["job_title"].to_string().blue());
                        println!("{}", post["posting_date"]);                        

                        for tech in post["tech"].as_array().unwrap() {
                            print!("{} ", tech);                        
                        }
                        
                        println!("");                        
                    }
                });

                let endpoint : String = [&self.endpoint,"/"].concat();
                let fetch_command = FetchAndShowCommand::new(endpoint, handler);

                commands.push(Box::new(fetch_command));

                return commands
            },
            ("read", Some(sub_m)) => {
                let mut commands = Vec::<Box<Command>>::new();

                let handler : Box<Fn(String)> = Box::new(|result: String| {
                    let post: Value = serde_json::from_str(&result).unwrap();

                    // todo this should be refactored into a bettter generic handler                                         
                    println!("");                                                
                    println!("");                                                
                    println!("");                                                
                    println!("-----------------------");
                    println!("{}", post["company"]["company_name"].to_string().bold());
                    let sum_string = post["company"]["company_summary"].to_string();
                    let summary : Vec<&str> = sum_string.split("\\n").collect();
                    for req in summary {
                        println!("{}",format!("{}", req));
                    }                    
                    println!("");                                                
                    println!("{}        {}", post["job_title"].to_string().bold(), post["posting_date"]);
                    println!("");                        
                    for tech in post["tech"].as_array().unwrap() {
                        print!("{} ", tech);                        
                    }
                    println!("");   
                    let req_string = post["job_requirements"].to_string();
                    let requirements : Vec<&str> = req_string.split("\\n").collect();
                    for req in requirements {
                        println!("{}",format!("{}", req));
                    }                     
                    println!("");          
                    let nice_string = post["job_nice_to_have"].to_string();              
                    let nice : Vec<&str> = nice_string.split("\\n").collect();
                    for req in nice {
                        println!("{}", req);
                    }                    
                    println!("");                        
                    println!("Apply @{}", post["job_link"]["Path"]);

                    println!("");                        
                    
                });

                let id = sub_m.value_of("id").unwrap_or("0");
                
                let endpoint : String = [&self.endpoint,"/", id].concat();
                let fetch_command = FetchAndShowCommand::new(endpoint, handler);

                commands.push(Box::new(fetch_command));

                return commands
            },
            _ => { return Vec::new() }
        }
    }    
}


