# Personal Finance Tracker

## Introduction

This project is a demonstration of a full-stack Rust web application built as a simple Personal Finance Tracker. The stack is composed of Rocket for the backend, Yew for the frontend, and MySQL for the database.

Please note, this application is not intended for real-world use as a personal finance tracker. It serves as an example of how one might build a simple full-stack web app in Rust, and more specifically, how to connect a Rocket backend to a Yew frontend.

## Stack
- Backend: [Rocket](https://rocket.rs/)
- Frontend: [Yew](https://yew.rs/)
- Database: [MySQL](https://www.mysql.com/)

## API Overview
The backend of this application exposes a RESTful API, providing endpoints for managing users and their incomes. These endpoints allow for the creation, retrieval, update, and deletion (CRUD) of user and income records in the database.

#### User Endpoints
The following operations can be performed on the `/users` endpoint:

- `POST /users`: Creates a new user.
- `GET /users`: Retrieves all users.
- `GET /users/<id>`: Retrieves a specific user by their ID.
- `DELETE /users/<id>`: Deletes a specific user by their ID.

#### Income Endpoints
The following operations can be performed on the `/income` endpoint:

- `POST /income`: Creates a new income record.
- `GET /income/user/<user_id>`: Retrieves income records for a specific user by their ID.
- `PUT /income/<income_id>`: Updates a specific income record.
- `DELETE /income/<income_id>`: Deletes a specific income record.
- `OPTIONS /api/income`: An OPTIONS request typically used for Cross-Origin Resource Sharing (CORS) preflight requests.

These APIs are exposed by the frontend to display and manipulate data via the UI, enabling users to perform operations directly from the frontend UI, and have those actions persist in the MySQL database.

## Additional Tools
This project also demonstrates a simple use case for [rptree](https://github.com/night-cruise/rptree), a command-line tool for generating a directory tree, written in Rust, and the value of being able to filter out directories like 'build' and 'dist'.

## License

This project is open-sourced software licensed under the [MIT license](http://opensource.org/licenses/MIT).

