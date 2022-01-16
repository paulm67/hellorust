use mysql::*;
use mysql::prelude::*;

fn main() {
    let database_name = "termtimeusers";
    let opts = Opts::from_url(&format!("mysql://root:s2@localhost:3306/{}", database_name)).expect("Failed initialising database connection properties");
    let pool = Pool::new(opts)
        .expect(&format!("Failed connecting to local MySQL server database '{}'", database_name));
    let mut conn = pool.get_conn().unwrap();
    let user_name: Option<String> = conn.query_first("SELECT user FROM users")
        .expect(&format!("Failed querying from table 'users' in database '{}'", database_name));
    println!("The first row of the termtimeusers.users table contains name {}", user_name.unwrap());

    let row: Row = conn.query_first("SELECT * FROM users").unwrap().unwrap();
    for column in row.columns_ref() {
        let column_value = &row[column.name_str().as_ref()];
        println!(
            "Column {} of type {:?} with value {:?}",
            column.name_str(),
            column.column_type(),
            column_value,
        );
    }
    println!("{:?}", row);
}
