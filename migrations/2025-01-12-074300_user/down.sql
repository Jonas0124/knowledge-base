-- This file should undo anything in `up.sql`
ALTER TABLE `knowledge`.`user`
    CHANGE COLUMN `uuid` `id` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL FIRST,
DROP PRIMARY KEY,
ADD PRIMARY KEY (`id`) USING BTREE;