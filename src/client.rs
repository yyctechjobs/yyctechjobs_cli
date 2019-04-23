pub struct Client {
    api_endpoint: String
}

impl Client {
    // don't need auth woot
    pub fn new(dev: bool, version: &str) -> Client {
        let mut endpoint = format!("https://api.yyctech.dev/{}", version).to_owned();

        if dev {
            endpoint = format!("http://api.localhost/{}", version).to_owned();            
        }

        Client {
            api_endpoint:endpoint
        }
    }

    pub fn fetch(&self, url: &str) -> Result<String, Box<std::error::Error>> {
        let request_url = [&self.api_endpoint,url].concat().to_string();
        let response = reqwest::get(&request_url)?.text()?;       
        Ok(response)
    }
}

pub trait Command {
    fn do_command(&self, client: &Client);
}

pub struct FetchAndShowCommand {
    url: String,
    on_complete: Box<Fn(String)>
}

impl FetchAndShowCommand {
    pub fn new(url: String, closure: Box<Fn(String)>) -> Self {
        FetchAndShowCommand{
            url:url,
            on_complete:closure
        }
    }
}

impl Command for FetchAndShowCommand {
    fn do_command(&self, client : &Client) {
        match client.fetch(&self.url) {
            Ok(e) => {
                (self.on_complete)(e);
            }
            Err(e) => {
                println!("{:#?}",e);                
            }
        }
    }
}
