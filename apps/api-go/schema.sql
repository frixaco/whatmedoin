-- create table activities
CREATE TABLE activities (
    id SERIAL PRIMARY KEY,
    platform VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    url VARCHAR(255) NOT NULL,
    date TIMESTAMP NOT NULL);
