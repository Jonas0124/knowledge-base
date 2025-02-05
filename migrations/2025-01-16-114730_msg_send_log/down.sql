-- This file should undo anything in `up.sql`
CREATE TABLE `send_msg_log` (
    `id` varchar(64) NOT NULL,
    `user_id` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '收件人主键:如果没有则为默认值1',
    `msg_type` int NOT NULL COMMENT '消息类型：0-创建账户，1-修改密码，2-注销账户',
    `email` varchar(64) NOT NULL COMMENT '收件人邮箱',
    `success` tinyint(1) NOT NULL COMMENT '推送是否成(0-失败，1成功)',
    `verification_code` varchar(10) NOT NULL DEFAULT '0' COMMENT '验证码',
    `verification_code_expire` bigint NOT NULL DEFAULT '0' COMMENT '验证码过期时间戳',
    `content` varchar(3000) NOT NULL COMMENT '推送内容',
    `result` text COMMENT '返回类容',
    `is_delete` varchar(64) NOT NULL DEFAULT '0',
    `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `create_by` varchar(64) NOT NULL DEFAULT '0',
    `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `update_by` varchar(64) NOT NULL DEFAULT '0',
    `reversion` int NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_email` (`email`)
) COMMENT='消息推送记录表';