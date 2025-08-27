# honey-note

This project managing your honeys. If you are a collector of honeys then you can manage your honies using this project.

# Overview of Design

![Abstract image of what will we make](docs/AbustructDesignLogs.md)

![data model design](docs/design.md)

![Overview Architecture](https://github.com/user-attachments/assets/330b914e-1d96-48c4-8480-9a4e344c53a8)

Event storm diagram

![Events](https://github.com/user-attachments/assets/fb8d6349-a483-4388-942c-7e41c75982bf)

# database migraion

```bash
$ cd resources/db
$ sqlx migrate run --database-url sqlite:${PATH_TO_YOUR_SQLITE_DB_FILE}

# ex

$ sqlx migrate run --database-url sqlite:./honey_note.db
```

If you remove database file (sqlite3) when a honey-note server is running then you need to restart the server.
Beucase the server has a database connection to old file.

# Execution

This project is mainly wirtten in rust. So, your environemt need to install rust environment.

The basic installation of rust uses rustup package. For install, check rustup installation [rustup.rs](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## build

```bash
$ cd $honey-note-path
$ cargo install wasm-bindgen-cli # build for front sub project
```

## batch
In this section, we show how to execute our batches.

### Japanese prececture loader
This batch loads the master data of prefectures from the filesystem.
It reads the file line by line and saves each entry into the database if it is new.

```bash
$ cargo run -p batchs --bin prefecture_loader resources/master_data/japanese_prefectures.scv $PATH_TO_DB_FILE
```

if you want to check log then run following command

```bash
$ RUST_LOG=info cargo run -p batchs --bin prefecture_loader resources/master_data/japanese_prefectures.csv $PATH_TO_DB_FILE
```

### flower master data loader 
This batch processes a master data file located in the file system under `resources/something_directory/something_file`. 
It checks if the data is new and stores it in the database. The master file should exclusively contain flower names, listed line by line.
This batch doesn't care about name extensions of name. This file doesn't care about header line and we assume each line has only one flower name.

```bash
$ RUST_LOG=info cargo run -p batchs --bin flower_loader flower_master_data_directory/file_name.csv database_file(sqlite file).db
```

### beekeeper master data loader
This process handles batch updates of master data for the beekeeper list. The input master file is in CSV format, with each line representing a beekeeper. The format for each line is as follows:

```
name_jp,name_en,prefecture_name,city_name
名前,英語表記,都道府県,都市
```

All fields, except for name_jp (名前), can be left blank.

We plan to create an input form on the front page, which will allow us to refresh the beekeeper database and restore its contents as needed. Currently, as this is a prototype, no production-ready code is assumed.

ex.
```bash
RUST_LOG=info cargo run -p batchs --bin beekeeper_loader resources/master_data/beekeeper.csv resources/db/honey_note.db
```

### honey master data loader

```
 RUST_LOG=info cargo run -p batchs --bin honey_loader resources/master_data/${honey_master_data_name}.jsonl  resources/db/${honey_db_name}.db
```

## web-server
In this section, we show how to execute our web-server.

```bash
cd $honey_note_top_directory
cargo run -p server
```

$honey_note_top_directory is not for server directory. this is to top directory of this repository.

### javascripts

This subsection describe about how to build javascript from rust code.

at the first time of execution, you need to install not only cargo but also wasm-pack to your environment.

```bash
cargo install wasm-pack
```

```bash
$ cd front/
$ wasm-pack build --target web  --out-dir ../server/src/assets/javascript/
$ ls ../server/src/assets/javascript
front_bg.wasm      front_bg.wasm.d.ts front.d.ts         front.js           package.json
```

Then you will see above files in `server/src/assets/javascript` directory.

# others

if you are using wsl (ubuntu) then, you need to install gcc for cmpile our package.

```bash
sudo apt install gcc
```


# other information

- This project includes data from "ISO 3166 Countries with Regional Codes", licensed under the MIT License. <https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes>

