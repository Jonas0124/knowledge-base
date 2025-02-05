-- This file should undo anything in `up.sql`
CREATE TABLE `knowledge`.`user_secret`  (
`id` varchar(64) NOT NULL,
`user_id` varchar(64) NOT NULL COMMENT '用户id',
`question` varchar(255) NOT NULL COMMENT '问题',
`answer` varchar(255) NOT NULL COMMENT '答案',
`is_delete` varchar(64) NOT NULL DEFAULT '0',
`create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
`create_by` varchar(64) NOT NULL DEFAULT '0',
`update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
`update_by` varchar(64) NOT NULL DEFAULT '0',
`reversion` int NOT NULL DEFAULT '0',
PRIMARY KEY (`id`),
INDEX `idx_user_id`(`user_id`)
) COMMENT = '用户密保表';