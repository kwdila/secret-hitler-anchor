-- Add migration script here
CREATE TABLE games {
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pubkey CHAR(32) NOT NULL UNIQUE,
}

CREATE TABLE players {
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pubkey CHAR(32) NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(id)
}-- Add migration script here
