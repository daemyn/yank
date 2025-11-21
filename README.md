# yank

A small and fast CLI tool to store and quickly copy reusable text snippets to your clipboard.

Think of it like a personal key-value clipboard manager.

---

## ✨ Features

- Store text snippets under a key
- Retrieve and copy values to clipboard
- List all stored keys
- Delete stored values
- Simple JSON-based storage
- Linux clipboard support via `arboard`

---

## 📦 Installation

Clone the repository and build it:

```
git clone https://github.com/daemyn/yank.git
cd yank
cargo build --release
```
Move the binary to your PATH:

```
sudo cp target/release/yank /usr/local/bin/
```

---

## 🚀 Usage

### Set a value

```
yank put <key> <value>
```

Example:

```
yank put email "me@example.com"
```

---

### Get & copy a value

```
yank <key>
```

Example:

```
yank email
```

This prints the value and copies it to your clipboard.

---

### List all keys

```
yank ls
```

---

### Delete a key

```
yank delete <key>
```

Example:

```
yank delete email
```

---

## 🗂 Storage

Data is stored at:

```
~/.yank/data.json
```

You can back it up or edit it manually if needed.

---

## ⚠️ Errors

Some common errors:

- No key provided — You ran `yank` with no key  
- Key not found — The key does not exist in storage  
- Could not find home directory — System home dir could not be resolved  
- Clipboard error — Clipboard access failed  

---

## 🧠 Why?

This tool was built to:

- Avoid retyping common text
- Quickly copy values (tokens, emails, paths, commands)
- Stay minimal, fast, and terminal-friendly

---

## 🛠 Development

Run in debug mode:

```
cargo run -- <args>
```

Example:

```
cargo run -- put test "hello"
```

