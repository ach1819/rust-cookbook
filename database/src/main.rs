use std::collections::HashMap;

use rusqlite::{Connection, Result};

fn main() {
    create_sqlite_database().expect("Error creating database or tables");
    instert_selet_data().expect("Error inserting or selecting data from database");
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

    Ok(())
}
