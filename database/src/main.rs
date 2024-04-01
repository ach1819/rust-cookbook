use std::collections::HashMap;

use postgres::{Client, Error, GenericClient, NoTls};
use rusqlite::{Connection, Result};

fn main() {
    create_sqlite_database().expect("Error creating database or tables");
    instert_selet_data().expect("Error inserting or selecting data from database");
    using_transactions().expect("Error running transactions");

    // Postgres
    create_postgres_database().expect("Error creating database or table in postgres");
    insert_query_data().expect("Error inserting and querying data");
    aggregate_data().expect("Error agregating data");
}

fn create_sqlite_database() -> Result<()> {
    println!("\ncreate_sqlite_database - starts");
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
        (),
    )?;

    conn.execute(
        "create table if not exists cats (
            id integer primary key,
            name text not null,
            color_id integer not null references cat_colors(id)
        )",
        (),
    )?;

    println!("create_sqlite_database - OK");
    Ok(())
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}
fn instert_selet_data() -> Result<()> {
    println!("\ninstert_selet_data - starts");
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;

        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) VALUES (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }

    let mut stmt = conn.prepare(
        "
        SELECT c.name, cc.name from cats c
        INNER JOIN cat_colors cc
        ON cc.id = c.color_id;
    ",
    )?;

    let cats = stmt.query_map((), |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    println!("instert_selet_data - OK");
    Ok(())
}

fn using_transactions() -> Result<()> {
    println!("\nusing_transactions - starts");
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    println!("using_transactions - OK");
    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    println!("\nsuccessful_tx - starts");
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", ())?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"lavender"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"blue"])?;

    println!("successful_tx - OK");
    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    println!("\nrolled_back_tx - starts");
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", ())?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"lavender"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"blue"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"lavender"])?;

    println!("rolled_back_tx - OK");
    tx.commit()
}

fn create_postgres_database() -> Result<(), Error> {
    println!("\ncreate_postgres_database - starts");
    let conn_str = std::env::var("CONN_STR").expect("Not connection string provided");
    let mut client = Client::connect(&conn_str, NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id      SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            author_id   INTEGER NOT NULL REFERENCES author

        )
    ",
    )?;

    println!("create_postgres_database - OK");
    Ok(())
}

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn insert_query_data() -> Result<(), Error> {
    println!("\ninsert_query_data - starts");
    let conn_str = std::env::var("CONN_STR").expect("Not connection string provided");
    let mut client = Client::connect(&conn_str, NoTls)?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, vaule) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: vaule.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    println!("insert_query_data - OK");
    Ok(())
}

struct Nation {
    nationality: String,
    count: i64,
}

// "Not tested, just coded"
fn aggregate_data() -> Result<(), Error> {
    println!("\naggregate_data - starts");
    let conn_str = std::env::var("CONN_STR").expect("Not connection string provided");
    let mut client = Client::connect(&conn_str, NoTls)?;

    for row in client.query(
        "SELECT nationality, COUNT(nationality) as count
        FROM artists GROUP BY nationality ORDER BY count DESC",
        &[],
    )? {
        let (nationality, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));

        if nationality.is_some() && count.is_some() {
            let nation = Nation {
                nationality: nationality.unwrap(),
                count: count.unwrap(),
            };
                println!("{} {}", nation.nationality, nation.count);
        }
    }

    println!("\naggregate_data - OK");
    Ok(())
}
