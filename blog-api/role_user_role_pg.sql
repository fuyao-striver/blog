-- 角色表
DROP TABLE IF EXISTS t_role;
CREATE TABLE t_role
(
    id          varchar(20) PRIMARY KEY,
    role_name   varchar(20)      NOT NULL,
    role_desc   varchar(50),
    is_disable  smallint         NOT NULL DEFAULT 0,
    create_time timestamp        NOT NULL,
    update_time timestamp
);

-- 角色数据
INSERT INTO t_role (id, role_name, role_desc, is_disable, create_time, update_time)
VALUES ('1', 'admin', '管理员', 0, '2022-11-03 17:41:57', '2023-03-10 23:12:59');
INSERT INTO t_role (id, role_name, role_desc, is_disable, create_time, update_time)
VALUES ('2', 'user', '普通用户', 0, '2022-11-03 17:42:17', '2023-03-10 23:13:11');
INSERT INTO t_role (id, role_name, role_desc, is_disable, create_time, update_time)
VALUES ('3', 'test', '测试账号', 0, '2022-11-03 17:42:31', '2023-03-10 23:13:17');

-- 用户角色关联表
DROP TABLE IF EXISTS t_user_role;
CREATE TABLE t_user_role
(
    id       SERIAL PRIMARY KEY,
    user_id  int          NOT NULL,
    role_id  varchar(20)  NOT NULL
);

-- 用户角色关联数据
INSERT INTO t_user_role (user_id, role_id) VALUES (1, '1');
INSERT INTO t_user_role (user_id, role_id) VALUES (3, '3');
