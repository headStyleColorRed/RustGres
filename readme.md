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
      POSTGRES_DB: demo
    ports:
      - '5432:5432'
```

Start the database 
```sh
docker compose up -d
```

If you want to check if the database is up and running do a `docker ps` and then display the container's logs by a `docker logs 3290f9616b1b`
```sh
$ docker ps

CONTAINER ID   IMAGE             COMMAND                  CREATED         STATUS         PORTS                    NAMES
3290f9616b1b   postgres:latest   "docker-entrypoint.s…"   8 seconds ago   Up 6 seconds   0.0.0.0:5432->5432/tcp   postgres_db


$ docker logs 3290f9616b1b

2021-12-23 10:00:52.062 UTC [1] LOG:  starting PostgreSQL 14.1 (Debian 14.1-1.pgdg110+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 10.2.1-6) 10.2.1 20210110, 64-bit
2021-12-23 10:00:52.062 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
2021-12-23 10:00:52.062 UTC [1] LOG:  listening on IPv6 address "::", port 5432
2021-12-23 10:00:52.069 UTC [1] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
2021-12-23 10:00:52.118 UTC [1] LOG:  database system is ready to accept connections
```

## Set the Diesel schema
First thing, lets create an .env with your postgress data
```sh
echo DATABASE_URL=postgres://root:1234@localhost/demo > .env
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
psql -h postgres_db -d demo -U root ;
```
Once you are inside, you can run the `\l` command and you'll be shown a list of databases
```
demo=# \l
                             List of databases
   Name    | Owner | Encoding |  Collate   |   Ctype    | Access privileges 
-----------+-------+----------+------------+------------+-------------------
 demo      | root  | UTF8     | en_US.utf8 | en_US.utf8 | 
 postgres  | root  | UTF8     | en_US.utf8 | en_US.utf8 | 
 template0 | root  | UTF8     | en_US.utf8 | en_US.utf8 | =c/root          +
           |       |          |            |            | root=CTc/root
 template1 | root  | UTF8     | en_US.utf8 | en_US.utf8 | =c/root          +
           |       |          |            |            | root=CTc/root
(4 rows)
```
The table we created is the demo, so we'll connect to that db by running `\c demo`, which will allow us to list the relations inside that db with another `\d`.
```
demo=# \d
                  List of relations
 Schema |            Name            | Type  | Owner 
--------+----------------------------+-------+-------
 public | __diesel_schema_migrations | table | root
 public | articles                   | table | root
(2 rows)
```
