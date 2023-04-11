-- Up
CREATE TABLE income (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    amount INT NOT NULL,
    user_id INT NOT NULL
);

