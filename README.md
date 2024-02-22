
# Q&A Backend Web Server

**This project is the backend server of an Q&A website.**

## Introduction

This is a backend web server project written in the Rust programming language, using
the Warp framework. 
Below are some key features of the server:
- User registration and login.
- Creation, editing, and deletion of questions.
- Answering posted questions.
- Guests can read all questions and answers without logging in.

## Installation

#### Install Rust: 

Make sure Rust and Cargo are installed. If not, you can install them from 
the [official Rust website](https://www.rust-lang.org).
#### Install Linux and Docker: 

This project was built on Linux using Docker. For optimal performance, please 
run it on a Linux environment.
#### Clone the repository: 

Clone this repository to your machine with the following command:
```
git clone https://github.com/VanhGer/rust_web_dev.git
```
#### Configure the database:

First, you need to configure the database via `.env` file. This is some values we need to consider:
```
POSTGRES_HOST = "localhost"

POSTGRES_PORT = 5432

POSTGRES_DB = "rustwebdev"

POSTGRES_USER = "postgres"

POSTGRES_PASSWORD = "522017"
```

Here, you will define your database name in `POSTGRES_DB`, database user in `POSTGRES_USER`, password via `POSTGRES_PASSWORD`.

Then, navigate to the project directory and run: 
```
docker-compose up -d
```

Then, you need to configure the database which created by Docker. \

You can also access to that database via this command:

```
psql -U username -d database_name
```


#### Run

Finally, run:
```
cargo build
cargo run
```
to start the server.
## Usage

The server will run on the default port (usually port `3000`), database will run on port `5432`. You can access the server through the URL `http://localhost:3000`.

## API Routes

Below are some API routes supported by the server:
- `POST /register:` Register a new account.
- `POST /login`: Log in to an account.
- `POST /questions`: Create a new question.
- `GET /questions`: Get a list of questions.
- `PUT /questions/{id}`: Edit a question.
- `DELETE /api/questions/{id}`: Delete a question.
- `POST /answers`: Answer a question.
- `GET /answers`: Get a list of answers to a question.

## Documentation

First, you should read the [overall.md](./web/documents/overall.md) file in document folder to have the overall
about what I did in this project.

Then, I recommend you to run 
```
cargo doc --open
```
To fully understand every objects and functions that I implemented.
## Contributing

If you'd like to contribute to this project, you can:

- Report bugs or suggest improvements by opening issues on GitHub.
- Submit pull requests with enhancements or bug fixes.

## License

This project is distributed under the [MIT License](https://en.wikipedia.org/wiki/MIT_License).



