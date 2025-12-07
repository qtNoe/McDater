CREATE TABLE `user` (
    `uuid` VARCHAR(255) PRIMARY KEY,
    `gender` VARCHAR(255),
    `bio` TEXT,
    `birthday` DATE NULL,
    `country` VARCHAR(255) NULL DEFAULT NULL,
    `city` VARCHAR(255) DEFAULT NULL,
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `user_dislike` (
    `uuid` VARCHAR(255),
    `targetUUID` VARCHAR(255),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `targetUUID`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`),
    FOREIGN KEY (`targetUUID`) REFERENCES `user`(`uuid`)
);

CREATE TABLE `user_like` (
    `uuid` VARCHAR(255),
    `targetUUID` VARCHAR(255),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `targetUUID`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`),
    FOREIGN KEY (`targetUUID`) REFERENCES `user`(`uuid`)
);

CREATE TABLE `user_blocked` (
    `uuid` VARCHAR(255),
    `targetUUID` VARCHAR(255),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `targetUUID`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`),
    FOREIGN KEY (`targetUUID`) REFERENCES `user`(`uuid`)
);

CREATE TABLE `user_banned` (
    `uuid` VARCHAR(255),
    `until` TIMESTAMP,
    `reason` VARCHAR(255),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `until`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- Spotify Link
CREATE TABLE `user_music` (
    `uuid` VARCHAR(255),
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `name`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- e.g. Valorant
CREATE TABLE `user_game` (
    `uuid` VARCHAR(255),
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `name`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- e.g. DE
CREATE TABLE `user_language` (
    `uuid` VARCHAR(255),
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `name`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- e.g. coding
CREATE TABLE `user_hobby` (
    `uuid` VARCHAR(255),
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`, `name`, `active`, `created`),
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- e.g. Hypixel.net
CREATE TABLE `user_favourite_server` (
    `id` INT PRIMARY KEY AUTO_INCREMENT,
    `uuid` VARCHAR(255),
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (`uuid`) REFERENCES `user`(`uuid`)
);

-- e.g. Murder Mystery
CREATE TABLE `user_favourite_server_playmode` (
    `user_favourite_serverId` INT,
    `name` VARCHAR(255),
    `order` TINYINT(1),
    `active` TINYINT(1) DEFAULT 1,
    `created` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_favourite_serverId`, `name`, `active`, `created`),
    FOREIGN KEY (`user_favourite_serverId`) REFERENCES `user_favourite_server`(`id`)
);