#![allow(dead_code)]
use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, Decode},
};
use serde_json;
use serde::{Deserialize, Serialize};

// The environment variable set in `spin.toml` that points to the
// address of the Pg server that the component will write to
const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Clone, Serialize, Deserialize)]

struct Book {
    id: i32,
    title: String,
    subtitle: Option<String>,
    summary: String,
    content: String,
    authorname: String,
    coauthor: Option<String>,
    kindle: Option<String>,
    paperback: Option<String>,
    ebook: Option<String>,
    tag: Option<String>,
}

// impl TryFrom<&pg::Row> for Article {
//     type Error = anyhow::Error;

//     fn try_from(row: &pg::Row) -> Result<Self, Self::Error> {
//         let id = i32::decode(&row[0])?;
//         let title = String::decode(&row[1])?;
//         let content = String::decode(&row[2])?;
//         let authorname = String::decode(&row[3])?;
//         let coauthor = Option::<String>::decode(&row[4])?;

//         Ok(Self {
//             id,
//             title,
//             content,
//             authorname,
//             coauthor,
//         })
//     }
// }

#[http_component]
fn process(req: Request) -> Result<Response> {
    println!("path: {}", req.headers().get("spin-full-url").unwrap().to_str().unwrap());
    
    match req.uri().path() {
        "/api" => books(req),
        // "/book" => book(req),
        _ => Ok(http::Response::builder()
            .status(404)
            .body(Some("Not found".into()))?),
    }
}

fn books(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let sql = "SELECT * FROM books";
    let rowset = pg::query(&address, sql, &[])?;

    // using the rowset, return a list of articles in json format and return using ok http response builder
    let mut books: Vec<Book> = Vec::new();

    for row in rowset.rows {
        let book = Book {
            id: i32::decode(&row[0])?,
            title: String::decode(&row[1])?,
            subtitle: Option::<String>::decode(&row[2])?,
            summary: String::decode(&row[3])?,
            content: String::decode(&row[4])?,
            authorname: String::decode(&row[5])?,
            coauthor: Option::<String>::decode(&row[6])?,
            kindle: Option::<String>::decode(&row[7])?,
            paperback: Option::<String>::decode(&row[8])?,
            ebook: Option::<String>::decode(&row[9])?,
            tag: Option::<String>::decode(&row[10])?,
        };
        books.push(book);
    }

    let json = serde_json::to_string(&books)?;
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json.into()))?)
}

// fn book(_req: Request) -> Result<Response> {
//     let address = std::env::var(DB_URL_ENV)?;

//     let id: &str = _req
//         .headers()
//         .get("spin-path-info")
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .strip_prefix('/')
//         .unwrap();

    
//     println!("id: {}", id);

//     let sql = format!("SELECT id, title, content, authorname, coauthor FROM articletest WHERE id = {}", id);
    
//     println!("sql: {}", sql);

//     let rowset = pg::query(&address, sql.as_str(), &[])?;

    
//     // using the rowset, return the article in json format and return using ok http response builder
//     if let Some(row) = rowset.rows.get(0) {
//         let article = Article::try_from(row)?;
//         let json = serde_json::to_string(&article)?;
//         Ok(http::Response::builder()
//             .status(200)
//             .body(Some(json.into()))?)
//     } else {
//         Ok(http::Response::builder()
//             .status(404)
//             .body(Some("Book not found".into()))?)
//     }
// }