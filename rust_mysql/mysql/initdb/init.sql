DROP DATABASE if EXISTS BeautyGoose;
DROP USER IF EXISTS btyGooseUser;
CREATE DATABASE BeautyGoose DEFAULT CHARACTER SET utf8mb4;

use BeautyGoose;

create table account(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `uuid` varchar(32) unique,
    `name` varchar(16) not null
)ENGINE=InnoDB;