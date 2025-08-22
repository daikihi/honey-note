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

# Execution

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

## web-server
In this section, we show how to execute our web-server.

```bash
cd $honey_note_top_directory
cargo run -p server
```

$honey_note_top_directory is not for server directory. this is to top directory of this repository.

### javascripts

```bash
$ cd front/
$ wasm-pack build --target web  --out-dir ../server/src/assets/javascript/
$ ls ../server/src/assets/javascript
front_bg.wasm      front_bg.wasm.d.ts front.d.ts         front.js           package.json
```

Then you will see above files in `server/src/assets/javascript` directory.


# other information

- This project includes data from "ISO 3166 Countries with Regional Codes", licensed under the MIT License. <https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes>

