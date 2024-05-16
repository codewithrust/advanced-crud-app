# Features
- Create User
- Get User
- Get All Users
- Update User
- Delete User

# Pre-requisite:
- install rust: https://www.rust-lang.org/tools/install
- install postgres: https://www.postgresql.org/download/

# how to run:
- clone the repository
- create a database in postgres
- copy .env.example, name the copied file as `.env` and update the environment variables
- run command `cargo run` from your terminal

# Application Flow:
  - current flow:  
     Main -> User Service -> User Repository -> Database
  - TODO - next flow:  
     api -> handlers -> User Service -> User Repository -> Database

# Library, framework, tool:
- Database Server: Postgres
- Database ORM: SQLx
- API/web server: Rocket (TODO)

# implemented RUST concepts:
- trait
- error handling
- async/await
- Result
- lifetimes
- ownership & borrowing
- generic data types   

# TODO:
- implement ARC (atomic Reference Counting) for database connection
- implement multiple threads for database connection
- implement channel for syncronization between threads
- implement unit testing for every layer
- implement integration testing
- implement hash, encrypt, and encryption for password
- implement JWT for authentication
- implement middleware for authentication