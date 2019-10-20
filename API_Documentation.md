# Calling a Web API

# Query the GitHub API


Queries GitHub stargazers API v3 with reqwest::get to get list of all users who have marked a GitHub project with a star. reqwest::Response is deserialized with Response::json into User objects implementing serde::Deserialize.

```
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
```

# Check if an API resource exists


Query the GitHub Users Endpoint using a HEAD request (Client::head) and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. reqwest::Client cofigured with ClientBuilder::timeout ensures a request will not last longer than a timeout.

```
extern crate reqwest;

use reqwest::Error;
use std::time::Duration;
use reqwest::ClientBuilder;


fn main() -> Result<(), Error> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
```

# Create and delete Gist with GitHub API


Creates a gist with POST request to GitHub gists API v3 using Client::post and removes it with DELETE request using Client::delete.

The reqwest::Client is responsible for details of both requests including URL, body and authentication. The POST body from serde_json::json! macro provides arbitrary JSON body. Call to RequestBuilder::json sets the request body. RequestBuilder::basic_auth handles authentication. The call to RequestBuilder::send synchronously executes the requests.

```
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

fn main() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let mut response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}",request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
    Ok(())
}

```

The example uses HTTP Basic Auth in order to authorize access to GitHub API. Typical use case would employ one of the much more complex OAuth authorization flows.

# Consume a paginated RESTful API


Wraps a paginated web API in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page.

```
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self, Error> {
        Ok(ReverseDependencies {
               crate_id: crate_id.to_owned(),
               dependencies: vec![].into_iter(),
               client: reqwest::Client::new(),
               page: 0,
               per_page: 100,
               total: 0,
           })
    }

    fn try_next(&mut self) -> Result<Option<Dependency>, Error> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                          self.crate_id,
                          self.page,
                          self.per_page);

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn main() -> Result<(), Error> {
    for dep in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dep?.crate_id);
    }
    Ok(())
}

```