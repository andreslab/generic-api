use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

#[derive(Debug)]
pub struct User {
    ci: String,
    user: String,
    email: String,
    pass: String,
    name: String,
    subname: String,
    phone: String,
    token: String,
}


pub fn run() -> Result<()> {
    let mut conn = Connection::open("auth.db")?;

    conn.execute(
        "create table if not exists users (
             ci text primary key,
             user text not null unique,
             name text not null,
             subname text not null,
             phone text not null,
             email text not null,
             pass text not null,
             token text not null unique
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "create table if not exists last_access (
             id integer primary key,
             ci text not null unique references users(ci),
             token text not null unique
         )",
        NO_PARAMS,
    )?;

    set_test_data(&mut conn)?;

    Ok(())
}

fn set_test_data(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from users", NO_PARAMS)?;
    tx.execute("delete from last_access", NO_PARAMS)?;
    tx.execute("insert into users (ci, user, name, subname, phone, email, pass, token) values (?1, ?2, ?3, ?4, ?5, ?6, ?7,?8)", &[&"0931510507",&"andreslab", &"Jaime", &"Andrade", &"0992814433", &"andres@hotmail.com", &"1234567890", &"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"])?;
    tx.execute("insert into last_access (ci, token) values (?1, ?2)", &[&"0931510507", &"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"])?;

    tx.commit()
}

////////////////////////////////////////////////////////

pub fn get_user() -> Result<()> {
    let mut conn = Connection::open("auth.db")?;

    let mut stmt = conn.prepare("SELECT ci,user,name,subname,phone,email,pass,token FROM users")?;

    let users = stmt.query_map(NO_PARAMS, |row| {
        Ok(User {
            ci: row.get(0)?,
            user: row.get(1)?,
            name: row.get(2)?,
            subname: row.get(3)?,
            phone: row.get(4)?,
            email: row.get(5)?,
            pass: row.get(6)?,
            token: row.get(7)?,
        })
    })?;

    for user in users {
        println!("Found user {:?}", user);
    }

    Ok(())
}

pub fn set_user(user:User) -> Result<()> {
    let mut conn = Connection::open("auth.db")?;

    let tx = conn.transaction()?;
    tx.execute("insert into users (ci, name, subname, phone, email, pass) values (?1, ?2, ?3, ?4, ?5, ?6)", &[&user.ci, &user.name, &user.subname, &user.phone, &user.email, &user.pass, &user.token])?;
    tx.execute("insert into last_access (ci, token) values (?1, ?2)", &[&user.ci, &user.token])?;

    tx.commit();

    Ok(())
}

// pub fn update_info() -> Result<()> {
//     let mut conn = Connection::open("auth.db")?;

//     let tx = conn.transaction()?;
//     tx.execute("insert into users (ci, name, subname, phone, email, pass) values (?1, ?2, ?3, ?4, ?5, ?6)", &[&"0931510507", &"Jaime", &"Andrade", &"0992814433", &"andres@hotmail.com", &"1234567890"])?;
//     tx.execute("insert into tokens (ci, token) values (?1, ?2)", &[&"0931510507", &"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"])?;

//     tx.commit();

//     Ok(())
// }

// pub fn delete_info() -> Result<()> {
//     let mut conn = Connection::open("auth.db")?;

//     let tx = conn.transaction()?;
//     tx.execute("insert into users (ci, name, subname, phone, email, pass) values (?1, ?2, ?3, ?4, ?5, ?6)", &[&"0931510507", &"Jaime", &"Andrade", &"0992814433", &"andres@hotmail.com", &"1234567890"])?;
//     tx.execute("insert into tokens (ci, token) values (?1, ?2)", &[&"0931510507", &"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"])?;

//     tx.commit();

//     Ok(())
// }