extern crate postgres;
extern crate db; 

use postgres::{Connection, SslMode};

use db::*;

struct Blog {
    title: String,
    body:  String,
}

fn main() {
    let conn:Connection=connect(); 
     
    let blog = Blog{
        title: String::from("title"),
        body: String::from("body"),
    };  
    let title = blog.title.to_string();
    let body = blog.body.to_string();
    insert_info(&conn,&title,&body); 

   for row in query::<String>(&conn,"select * from blog"){
        println!("{:?}",row);
    }   
    let sql = "select * from person";
    query_all(&conn,&sql);
}
~      