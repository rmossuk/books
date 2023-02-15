use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, Decode},
};
use serde_json;
use serde::{Deserialize, Serialize};

// set which ENV VAR to use.   &str is a reference to a string literal.
const DB_URL_ENV: &str = "DB_URL";

// The Book struct is annotated with the Serialize and Deserialize attributes, which are used to indicate that this struct can be serialized and deserialized.
// The struct is also annotated with the Debug and Clone attributes, which are used to indicate that this struct can be printed and cloned.
// the option type is used to indicate that the field is optional.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    id: i32,
    title: String,
    subtitle: Option<String>,
    summary: String,
    content: String,
    authorname: String,
    coauthor: Option<String>,
    audible: Option<String>,
    kindle: Option<String>,
    paperback: Option<String>,
    ebook: Option<String>,
    image: Option<String>,
    tag: Option<String>,
}

// The function is annotated with the http_component macro, which is used to indicate that this is a handler for an HTTP request.
#[http_component]
fn api(_req: Request) -> Result<Response> {
    // get the database address from the environment variable
    let address = std::env::var(DB_URL_ENV)?;

    // query the database and return the results as a rowset.
    // a rowset is a collection of rows, where each row is a collection of columns.
    let sql = "SELECT * FROM books";
    let rowset = pg::query(&address, sql, &[])?;

    // create an empty vector of books
    // a vector is a collection of items of the same type.
    let mut books: Vec<Book> = Vec::new();

    // iterate over the rows in the rowset
    // each row is a collection of columns
    // each column is a collection of bytes
    // the decode function is used to convert the bytes into a value of the specified type
    // the option type is used to indicate that the field is optional.
    // the ? operator is used to return an error if the decode function fails.
    for row in rowset.rows {
        let book = Book {
            id: i32::decode(&row[0])?,
            title: String::decode(&row[1])?,
            subtitle: Option::<String>::decode(&row[2])?,
            summary: String::decode(&row[3])?,
            content: String::decode(&row[4])?,
            authorname: String::decode(&row[5])?,
            coauthor: Option::<String>::decode(&row[6])?,
            audible: Option::<String>::decode(&row[7])?,
            kindle: Option::<String>::decode(&row[8])?,
            paperback: Option::<String>::decode(&row[9])?,
            ebook: Option::<String>::decode(&row[10])?,
            image: Option::<String>::decode(&row[11])?,
            tag: Option::<String>::decode(&row[12])?,
        };
        // add the book to the vector of books
        books.push(book);
    }

    // convert the vector of books to a JSON string
    // the ? operator is used to return an error if the conversion fails.
    let json = serde_json::to_string(&books)?;

    // return the JSON string as the body of the HTTP response
    // the some function is used to indicate that the body is not empty.
    // the into function is used to convert the string into a byte array.
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json.into()))?)
}
