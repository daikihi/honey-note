# Web Page Specification

This document describes the current branch's web page endpoints under `/honey_note`.
It focuses on the HTML files that exist today and the WASM entry points they load.

## Common Rules

- The base web path is `/honey_note`
- HTML pages are served statically by the server
- Each page loads `/honey_note/javascript/front.js`
- Each page then calls the WASM entry point associated with that page

## Top and Authentication

### `/index.html`

- Top page
- Loads `top_page_main()`

### `/login.html`

- Login page
- Loads `login_main()`

### `/signup.html`

- Sign-up page
- Loads `signup_main()`

## Honeys

### `/honeys/lists.html`

- Honey list page
- Loads `honey_list_main()`

### `/honeys/show.html?id={id}`

- Honey detail page
- Loads `honey_show_main()`

### `/honeys/new.html`

- Honey create page
- Loads `honey_edit_and_new_main()`

### `/honeys/edit.html?id={id}`

- Honey edit page
- Loads `honey_edit_and_new_main()`

## Flowers

### `/flowers/lists.html`

- Flower list page
- Loads `flower_list_main()`

### `/flowers/show.html?id={id}`

- Flower detail page
- Currently present as a static detail page

### `/flowers/new.html`

- Flower create page
- Loads `flower_edit_and_new_main()`

### `/flowers/edit.html?id={id}`

- Flower edit page
- Loads `flower_edit_and_new_main()`

## Beekeepers

### `/beekeepers/lists.html`

- Beekeeper list page
- Loads `beekeepers_list_main()`

### `/beekeepers/show.html?id={id}`

- Beekeeper detail page
- Loads `beekeeper_show_main()`

### `/beekeepers/new.html`

- Beekeeper create page
- Loads `beekeeper_edit_and_new_main()`

### `/beekeepers/edit.html?id={id}`

- Beekeeper edit page
- Loads `beekeeper_edit_and_new_main()`

## Static Assets

- `/javascript/`
  - WebAssembly and related JavaScript files
- `/css/`
  - Stylesheets
- `/icons/`
  - Icon assets
