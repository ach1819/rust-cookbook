use std::collections::HashMap;

use rusqlite::{Connection, Result};

fn main() {
    create_sqlite_database().expect("Error creating database or tables");
    instert_selet_data().expect("Error inserting or selecting data from database");
    using_transactions().expect("Error running transactions");
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
