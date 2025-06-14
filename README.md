# honey-note

This project managing your honeies. If you are a collector of honeies then you can manage your honies using this project.

# Overview of Design

![Overview Architecture](https://github.com/user-attachments/assets/330b914e-1d96-48c4-8480-9a4e344c53a8)

![Events](https://github.com/user-attachments/assets/fb8d6349-a483-4388-942c-7e41c75982bf)

# database migraion

```bash
$ cd resources/db
$ sqlx migrate run --database-url sqlite:${PATH_TO_YOUR_SQLITE_DB_FILE}

# ex

$ sqlx migrate run --database-url sqlite:./honey_note.db
```

# Execution

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

# other information

- This project includes data from "ISO 3166 Countries with Regional Codes", licensed under the MIT License. <https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes>
