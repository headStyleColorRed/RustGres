## Start the Postgres DB
Let's create a postres container with docker that will save all data on the postgres_data volume
```yml
version: '3'

services:
  postgres:
    container_name: postgres_db
    image: 'postgres:latest'
    restart: always
    volumes:
      - './postgres_data:/var/lib/postgresql/data'
    environment:
      POSTGRES_USER: root
      POSTGRES_PASSWORD: 1234
      POSTGRES_DB: test_db
    ports:
      - '5432:5432'
```

Start the database 
```sh
docker compose up -d ;
```

## Set the Diesel schema
First thing, lets create an .env with your postgress data
```sh
echo DATABASE_URL=postgres://root:1234@localhost/diesel_demo > .env
```

Then lets set up diesel. This will create a diesel.toml and migrations folder on the root of the project

```sh
diesel setup
```

Once this is done we can create a table running a migration
```sh
diesel migration generate articles
```

###### SQL
This previous command will create a new migration that contains a down.sql and an up.sql that we will fill with some made up data.
```sql
# up.sql
CREATE TABLE articles (
    uuid UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
```
```sql
# down.sql
DROP TABLE articles
```
<br>
And once this is done we can run the migration!
```sh
diesel migration run
```
You'll see that now we have a new file on src called `schema.rs`. Waht his migration command did is create a table for us to interact with rust.

```rust
table! {
    articles (uuid) {
        uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

## Check DB
If you want to connect to the db you can run this command to join the docker
command line and then enter the psql cli as root with the password provided on the docker-compose

```sh
docker exec -it postgres_db bash
psql -h postgres_db -d test_db -U root ;
```
Once you are inside, you can run the `\l` command and you'll be shown a list of databases
```
test_db=# \l
                              List of databases
    Name     | Owner | Encoding |  Collate   |   Ctype    | Access privileges 
-------------+-------+----------+------------+------------+-------------------
 diesel_demo | root  | UTF8     | en_US.utf8 | en_US.utf8 | 
 postgres    | root  | UTF8     | en_US.utf8 | en_US.utf8 | 
 template0   | root  | UTF8     | en_US.utf8 | en_US.utf8 | =c/root          +
             |       |          |            |            | root=CTc/root
 template1   | root  | UTF8     | en_US.utf8 | en_US.utf8 | =c/root          +
             |       |          |            |            | root=CTc/root
 test_db     | root  | UTF8     | en_US.utf8 | en_US.utf8 | 
(5 rows)
```
The table we created is the diesel_demo, so we'll connect to that db by running `\c diesel_demo`, which will allow us to list again the contents inside that db with another `\l`