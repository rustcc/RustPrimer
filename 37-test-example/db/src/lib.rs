extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::FromSql;
use postgres::Result as PgResult;


struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


pub fn connect() -> Connection{
    let dsn = "postgresql://postgres:2015@localhost/rust_example";
    Connection::connect(dsn, SslMode::None).unwrap()
}

pub fn insert_info(conn : &Connection,title : &str, body: &str){

    let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e); 
            return;
        }   
    };  
        stmt.execute(&[&title, &body]).expect("Inserting blogposts failed");
}


pub fn query<T>(conn: &Connection,query: &str) ->PgResult<T>
        where T: FromSql {
            println!("Executing query: {}", query);
            let stmt = try!(conn.prepare(query));
            let rows = try!(stmt.query(&[]));
            &rows.iter().next().unwrap();
            let row = &rows.iter().next().unwrap();
                //rows.iter().next().unwrap()
            row.get_opt(2).unwrap()

} 

pub fn query_all(conn: &Connection,query: &str){ 
            println!("Executing query: {}", query);
            for row in &conn.query(query,&[]).unwrap(){
                let person = Person{
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2)
            };
            println!("Found person {}", person.name);
            }

} 

