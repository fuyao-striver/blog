DROP TABLE IF EXISTS t_user;

CREATE TABLE t_user
(
    id          SERIAL PRIMARY KEY,
    nickname    VARCHAR(50) NOT NULL,
    username    VARCHAR(50) NOT NULL,
    password    VARCHAR(100) NOT NULL,
    avatar      VARCHAR(255) NOT NULL,
    web_site    VARCHAR(255) DEFAULT '',
    intro       VARCHAR(100) DEFAULT '',
    email       VARCHAR(50) DEFAULT '',
    ip_address  VARCHAR(50) DEFAULT '',
    ip_source   VARCHAR(50) DEFAULT '',
    login_type  SMALLINT NOT NULL DEFAULT 0,
    is_disable  SMALLINT NOT NULL DEFAULT 0,
    login_time  TIMESTAMP,
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP
);

COMMENT ON TABLE t_user IS '用户表';
COMMENT ON COLUMN t_user.id IS '用户id';
COMMENT ON COLUMN t_user.nickname IS '用户昵称';
COMMENT ON COLUMN t_user.username IS '用户名';
COMMENT ON COLUMN t_user.password IS '用户密码';
COMMENT ON COLUMN t_user.avatar IS '头像';
COMMENT ON COLUMN t_user.web_site IS '个人网站';
COMMENT ON COLUMN t_user.intro IS '个人简介';
COMMENT ON COLUMN t_user.email IS '邮箱';
COMMENT ON COLUMN t_user.ip_address IS '登录ip';
COMMENT ON COLUMN t_user.ip_source IS '登录地址';
COMMENT ON COLUMN t_user.login_type IS '登录方式 (1邮箱 2QQ 3Gitee 4Github)';
COMMENT ON COLUMN t_user.is_disable IS '是否禁用 (0否 1是)';
COMMENT ON COLUMN t_user.login_time IS '登录时间';
COMMENT ON COLUMN t_user.create_time IS '创建时间';
COMMENT ON COLUMN t_user.update_time IS '更新时间';

INSERT INTO t_user (nickname, username, password, avatar, web_site, intro, email, ip_address, ip_source, login_type, is_disable, login_time, create_time, update_time)
VALUES ('阿冬', 'admin@qq.com', '8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92',
        'https://static.ttkwsd.top/config/9c65807710f54d9d5ad398a78216ebfb.jpg', NULL, NULL, '1632167813@qq.com',
        '192.168.23.1', '内网IP|内网IP', 1, 0, '2023-03-10 22:26:23', '2022-11-29 21:45:48', '2023-03-10 22:26:23');

INSERT INTO t_user (nickname, username, password, avatar, web_site, intro, email, ip_address, ip_source, login_type, is_disable, login_time, create_time, update_time)
VALUES ('测试账号', 'test@qq.com', '8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92',
        'https://static.ttkwsd.top/config/0bca52afdb2b9998132355d716390c9f.png', 'https://www.ttkwsd.top', '个人简介',
        'test@qq.com', '192.168.23.1', '内网IP|内网IP', 1, 0, '2023-02-24 10:45:59', '2022-11-30 21:34:26',
        '2023-02-24 10:45:59');
