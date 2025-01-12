-- This file should undo anything in `up.sql`
CREATE TABLE `user` (
    `uuid` varchar(64) NOT NULL,
    `username` varchar(64) NOT NULL,
    `password` varchar(255) NOT NULL,
    `email` varchar(64) NOT NULL,
    `is_delete` varchar(64) NOT NULL DEFAULT '0',
    `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `create_by` varchar(64) NOT NULL DEFAULT '0',
    `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `update_by` varchar(64) NOT NULL DEFAULT '0',
    `reversion` int NOT NULL DEFAULT '0',
    PRIMARY KEY (`uuid`),
    UNIQUE KEY `uk_username` (`username`,`is_delete`),
    UNIQUE KEY `uk_email` (`email`,`is_delete`)
) COMMENT='用户表';