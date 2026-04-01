-- 菜单表
DROP TABLE IF EXISTS t_menu;
CREATE TABLE t_menu
(
    id          SERIAL PRIMARY KEY,
    parent_id   int              NOT NULL DEFAULT 0,
    menu_type   char(1)          NOT NULL,
    menu_name   varchar(50)      NOT NULL,
    path        varchar(255),
    icon        varchar(50),
    component   varchar(50),
    perms       varchar(100)     NOT NULL DEFAULT '',
    is_hidden   smallint         NOT NULL DEFAULT 0,
    is_disable  smallint         NOT NULL DEFAULT 0,
    order_num   int              NOT NULL DEFAULT 1,
    create_time timestamp        NOT NULL,
    update_time timestamp
);

-- 菜单数据
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (1, 0, 'M', '文章管理', 'article', 'archives', '', '', 0, 0, 1, '2022-12-04 09:13:31', '2023-02-21 15:36:45');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (3, 1, 'C', '分类管理', 'category', 'category', '/blog/category/index', 'blog:category:list', 0, 0, 3, '2022-12-04 09:22:20', '2023-02-21 15:21:19');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (4, 1, 'C', '标签管理', 'tag', 'tag', '/blog/tag/index', 'blog:tag:list', 0, 0, 4, '2022-12-04 09:23:01', '2023-02-21 15:21:23');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (5, 3, 'B', '添加分类', NULL, NULL, NULL, 'blog:category:add', 0, 0, 1, '2022-12-04 09:30:55', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (6, 3, 'B', '删除分类', NULL, NULL, NULL, 'blog:category:delete', 0, 0, 2, '2022-12-04 09:32:15', '2022-12-26 15:39:20');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (7, 3, 'B', '修改分类', NULL, NULL, NULL, 'blog:category:update', 0, 0, 3, '2022-12-04 09:33:52', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (8, 4, 'B', '添加标签', NULL, NULL, '', 'blog:tag:add', 0, 0, 1, '2022-12-04 10:19:51', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (9, 4, 'B', '删除标签', NULL, NULL, NULL, 'blog:tag:delete', 0, 0, 2, '2022-12-04 10:20:41', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (10, 4, 'B', '修改标签', NULL, NULL, NULL, 'blog:tag:update', 0, 0, 3, '2022-12-04 10:21:32', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (11, 0, 'M', '系统管理', 'system', 'system', '', '', 0, 0, 3, '2022-12-06 10:58:50', '2023-01-03 18:47:19');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (12, 11, 'C', '菜单管理', 'menu', 'tree-table', '/system/menu/index', 'system:menu:list', 0, 0, 1, '2022-12-06 16:33:56', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (13, 11, 'C', '角色管理', 'role', 'peoples', '/system/role/index', 'system:role:list', 0, 0, 2, '2022-12-06 17:09:55', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (14, 11, 'C', '用户管理', 'user', 'user', '/system/user/index', 'system:user:list', 0, 0, 3, '2022-12-06 17:10:28', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (15, 12, 'B', '添加菜单', NULL, '', NULL, 'system:menu:add', 0, 0, 1, '2022-12-07 10:50:22', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (16, 12, 'B', '删除菜单', NULL, '', NULL, 'system:menu:delete', 0, 0, 2, '2022-12-07 10:50:54', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (17, 12, 'B', '修改菜单', NULL, '', NULL, 'system:menu:update', 0, 0, 3, '2022-12-07 10:55:21', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (18, 13, 'B', '添加角色', NULL, NULL, NULL, 'system:role:add', 0, 0, 1, '2022-12-07 10:56:24', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (19, 13, 'B', '删除角色', NULL, NULL, NULL, 'system:role:delete', 0, 0, 2, '2022-12-07 10:56:50', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (20, 13, 'B', '修改角色', NULL, NULL, NULL, 'system:role:update', 0, 0, 3, '2022-12-07 10:57:15', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (21, 0, 'M', '日志管理', 'log', 'log', '', '', 0, 0, 4, '2022-12-21 17:36:39', '2023-02-21 15:20:13');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (22, 21, 'C', '操作日志', 'operation', 'form', '/system/log/operation', 'log:operation:list', 0, 0, 1, '2022-12-21 20:14:01', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (23, 21, 'C', '异常日志', 'exception', 'bug', '/system/log/exception', 'log:exception:list', 0, 0, 2, '2022-12-21 20:48:25', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (24, 22, 'B', '删除操作日志', NULL, NULL, NULL, 'log:operation:delete', 0, 0, 1, '2022-12-26 16:43:00', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (25, 23, 'B', '删除异常日志', NULL, NULL, NULL, 'log:exception:delete', 0, 0, 1, '2022-12-27 13:21:50', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (26, 0, 'M', '系统监控', 'monitor', 'monitor', NULL, '', 0, 0, 5, '2022-12-27 13:23:29', '2023-01-03 18:47:27');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (27, 26, 'C', '定时任务', 'task', 'job', '/monitor/task/index', 'monitor:task:list', 0, 0, 2, '2022-12-27 13:26:29', '2023-01-01 21:08:35');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (28, 27, 'B', '添加任务', NULL, NULL, NULL, 'monitor:task:add', 0, 0, 1, '2022-12-27 13:32:42', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (29, 27, 'B', '修改任务', NULL, NULL, NULL, 'monitor:task:update', 0, 0, 2, '2022-12-27 13:33:45', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (30, 27, 'B', '删除任务', NULL, NULL, NULL, 'monitor:task:delete', 0, 0, 3, '2022-12-27 13:34:29', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (31, 27, 'B', '修改任务状态', NULL, NULL, NULL, 'monitor:task:status', 0, 0, 4, '2022-12-27 13:43:24', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (32, 27, 'B', '运行任务', NULL, NULL, NULL, 'monitor:task:run', 0, 0, 5, '2022-12-27 13:45:34', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (33, 13, 'B', '修改角色状态', NULL, NULL, NULL, 'system:role:status', 0, 0, 4, '2022-12-27 13:46:39', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (34, 0, 'M', '网站管理', 'web', 'international', NULL, '', 0, 0, 6, '2022-12-30 17:22:33', '2023-02-14 09:46:29');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (35, 34, 'C', '友链管理', 'friend', 'friend', '/web/friend/index', 'web:friend:list', 0, 0, 1, '2022-12-30 17:33:15', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (36, 0, 'M', '消息管理', 'news', 'email', NULL, '', 0, 0, 2, '2022-12-30 17:50:06', '2022-12-30 18:02:12');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (37, 36, 'C', '留言管理', 'message', 'form', '/news/message/index', 'news:message:list', 0, 0, 2, '2022-12-30 17:58:25', '2022-12-30 18:01:47');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (38, 36, 'C', '评论管理', 'comment', 'comment', '/news/comment/index', 'news:comment:list', 0, 0, 1, '2022-12-30 17:59:37', '2022-12-30 18:03:35');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (39, 35, 'B', '添加友链', NULL, NULL, NULL, 'web:friend:add', 0, 0, 1, '2022-12-30 18:56:22', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (40, 35, 'B', '删除友链', NULL, NULL, NULL, 'web:friend:delete', 0, 0, 2, '2022-12-30 18:56:42', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (41, 35, 'B', '修改友链', NULL, NULL, NULL, 'web:friend:update', 0, 0, 3, '2022-12-30 18:57:08', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (42, 37, 'B', '删除留言', NULL, NULL, NULL, 'news:message:delete', 0, 0, 1, '2022-12-30 22:05:53', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (45, 37, 'B', '审核留言', NULL, NULL, NULL, 'news:message:pass', 0, 0, 2, '2022-12-30 22:29:24', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (46, 34, 'C', '网站配置', 'site', 'example', '/web/site/index', 'web:site:list', 0, 0, 5, '2022-12-31 11:50:45', '2023-01-03 18:49:17');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (51, 34, 'C', '相册管理', 'album', 'album', '/web/album/index', 'web:album:list', 0, 0, 3, '2023-01-01 18:16:40', '2023-01-03 18:49:06');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (52, 34, 'C', '照片管理', 'photo/:albumId', 'photo', '/web/photo/index', 'web:photo:list', 1, 0, 4, '2023-01-01 18:18:11', '2023-01-01 18:39:22');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (53, 26, 'C', '在线用户', 'online', 'online', '/monitor/online/index', 'monitor:online:list', 0, 0, 1, '2023-01-01 21:07:48', '2023-01-01 21:08:29');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (54, 51, 'B', '添加相册', NULL, NULL, NULL, 'web:album:add', 0, 0, 1, '2023-01-02 19:01:33', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (55, 51, 'B', '删除相册', NULL, NULL, NULL, 'web:album:delete', 0, 0, 2, '2023-01-02 19:02:03', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (56, 51, 'B', '修改相册', NULL, NULL, NULL, 'web:album:update', 0, 0, 3, '2023-01-02 19:02:50', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (57, 51, 'B', '编辑相册', NULL, NULL, NULL, 'web:album:edit', 0, 0, 4, '2023-01-02 19:03:40', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (58, 51, 'B', '上传相册封面', NULL, NULL, NULL, 'web:album:upload', 0, 0, 5, '2023-01-02 19:04:38', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (60, 12, 'B', '编辑菜单', NULL, NULL, NULL, 'system:menu:edit', 0, 0, 4, '2023-01-03 18:29:57', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (61, 34, 'C', '说说管理', 'talk', 'talk', '/web/talk/index', 'web:talk:list', 0, 0, 2, '2023-01-03 18:48:28', '2023-01-03 18:48:41');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (62, 61, 'B', '添加说说', NULL, NULL, NULL, 'web:talk:add', 0, 0, 1, '2023-01-05 19:16:42', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (63, 61, 'B', '删除说说', NULL, NULL, NULL, 'web:talk:delete', 0, 0, 2, '2023-01-05 19:17:07', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (64, 61, 'B', '修改说说', NULL, NULL, NULL, 'web:talk:update', 0, 0, 3, '2023-01-05 19:17:36', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (65, 61, 'B', '编辑说说', NULL, NULL, NULL, 'web:talk:edit', 0, 0, 4, '2023-01-05 19:18:27', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (66, 61, 'B', '上传说说图片', NULL, NULL, NULL, 'web:talk:upload', 0, 0, 5, '2023-01-05 19:18:52', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (67, 46, 'B', '修改网站配置', NULL, NULL, NULL, 'web:site:update', 0, 0, 1, '2023-01-08 09:15:56', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (68, 46, 'B', '上传网站配置图片', NULL, NULL, NULL, 'web:site:upload', 0, 0, 2, '2023-01-08 14:53:16', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (69, 14, 'B', '修改用户', NULL, NULL, NULL, 'system:user:update', 0, 0, 1, '2023-01-09 17:03:18', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (70, 14, 'B', '修改用户状态', NULL, NULL, NULL, 'system:user:status', 0, 0, 2, '2023-01-09 17:03:51', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (71, 53, 'B', '下线用户', NULL, NULL, NULL, 'monitor:online:kick', 0, 0, 1, '2023-01-09 19:18:33', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (73, 1, 'C', '文章列表', 'list', 'chart', '/blog/article/list', 'blog:article:list', 0, 0, 2, '2023-01-10 17:37:29', '2023-02-21 15:36:09');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (76, 52, 'B', '添加照片', NULL, NULL, NULL, 'web:photo:add', 0, 0, 1, '2023-01-11 18:45:28', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (77, 52, 'B', '删除照片', NULL, NULL, NULL, 'web:photo:delete', 0, 0, 2, '2023-01-11 18:45:51', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (78, 52, 'B', '修改照片', NULL, NULL, NULL, 'web:photo:update', 0, 0, 3, '2023-01-11 18:46:12', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (79, 52, 'B', '上传照片', NULL, NULL, NULL, 'web:photo:upload', 0, 0, 3, '2023-01-11 18:46:48', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (80, 73, 'B', '添加文章', NULL, NULL, NULL, 'blog:article:add', 0, 0, 1, '2023-01-14 15:25:29', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (81, 73, 'B', '物理删除文章', NULL, NULL, NULL, 'blog:article:delete', 0, 0, 2, '2023-01-14 15:26:44', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (82, 73, 'B', '逻辑删除文章', NULL, NULL, NULL, 'blog:article:recycle', 0, 0, 3, '2023-01-14 15:28:32', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (83, 73, 'B', '更新文章', NULL, NULL, NULL, 'blog:article:update', 0, 0, 4, '2023-01-14 15:30:11', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (84, 73, 'B', '编辑文章', NULL, NULL, NULL, 'blog:article:edit', 0, 0, 5, '2023-01-14 15:32:34', '2023-01-14 15:32:54');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (85, 73, 'B', '上传文章图片', NULL, NULL, NULL, 'blog:article:upload', 0, 0, 6, '2023-01-14 15:34:05', '2023-01-15 11:19:13');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (86, 73, 'B', '置顶文章', NULL, NULL, NULL, 'blog:article:top', 0, 0, 7, '2023-01-14 15:35:33', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (87, 73, 'B', '推荐文章', NULL, NULL, NULL, 'blog:article:recommend', 0, 0, 8, '2023-01-14 15:36:08', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (88, 73, 'B', '点赞文章', NULL, NULL, NULL, 'blog:article:like', 0, 0, 9, '2023-01-14 15:36:39', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (93, 21, 'C', '访问日志', 'visit', 'logininfor', '/system/log/visit', 'log:visit:list', 0, 0, 3, '2023-01-28 19:04:09', '2023-01-28 19:06:52');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (94, 93, 'B', '删除访问日志', NULL, NULL, NULL, 'log:visit:delete', 0, 0, 1, '2023-01-28 19:05:31', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (95, 14, 'B', '修改用户邮箱', NULL, NULL, NULL, 'user:email:update', 0, 0, 3, '2023-01-31 09:26:22', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (96, 14, 'B', '修改用户头像', NULL, NULL, NULL, 'user:avatar:update', 0, 0, 4, '2023-01-31 09:27:03', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (97, 14, 'B', '修改用户信息', NULL, NULL, NULL, 'user:info:update', 0, 0, 5, '2023-01-31 09:27:37', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (98, 14, 'B', '修改用户密码', NULL, NULL, NULL, 'user:password:update', 0, 0, 6, '2023-01-31 09:28:10', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (99, 38, 'B', '添加评论', NULL, NULL, NULL, 'news:comment:add', 0, 0, 1, '2023-02-08 19:09:25', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (100, 38, 'B', '删除评论', NULL, NULL, NULL, 'news:comment:delete', 0, 0, 2, '2023-02-08 19:09:57', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (101, 38, 'B', '审核评论', NULL, NULL, NULL, 'news:comment:pass', 0, 0, 3, '2023-02-08 19:10:26', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (102, 38, 'B', '点赞评论', NULL, NULL, NULL, 'news:comment:like', 0, 0, 4, '2023-02-08 19:10:45', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (103, 61, 'B', '点赞说说', NULL, NULL, NULL, 'web:talk:like', 0, 0, 6, '2023-02-10 11:16:23', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (104, 21, 'C', '任务日志', 'task', 'job', '/system/log/task', 'log:task:list', 0, 0, 3, '2023-02-14 10:28:28', '2023-02-14 10:28:41');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (105, 104, 'B', '删除任务日志', NULL, NULL, NULL, 'log:task:delete', 0, 0, 1, '2023-02-14 11:21:06', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (106, 104, 'B', '清空任务日志', NULL, NULL, NULL, 'log:task:clear', 0, 0, 2, '2023-02-14 11:21:28', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (108, 1, 'C', '发布文章', 'write', 'edit', '/blog/article/write', 'blog:article:list', 0, 0, 1, '2023-02-21 13:32:22', '2023-02-21 15:36:04');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (111, 1, 'C', '修改文章', 'write/:articleId', 'edit', '/blog/article/write', 'blog:article:list', 1, 0, 5, '2023-02-21 15:40:11', '2023-02-21 15:41:07');
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (123, 11, 'C', '文件管理', 'file', 'file', '/system/file/index', 'system:file:list', 0, 0, 4, '2023-03-09 10:57:29', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (124, 123, 'B', '上传文件', NULL, NULL, NULL, 'system:file:upload', 0, 0, 1, '2023-03-10 23:11:33', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (125, 123, 'B', '新建文件夹', NULL, NULL, NULL, 'system:file:createFolder', 0, 0, 2, '2023-03-10 23:12:11', NULL);
INSERT INTO t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, is_hidden, is_disable, order_num, create_time, update_time)
VALUES (126, 123, 'B', '删除文件', NULL, NULL, NULL, 'system:file:delete', 0, 0, 3, '2023-03-10 23:12:45', NULL);

-- 重置序列值
SELECT setval('t_menu_id_seq', (SELECT COALESCE(MAX(id), 0) FROM t_menu));

-- 角色菜单关联表
DROP TABLE IF EXISTS t_role_menu;
CREATE TABLE t_role_menu
(
    id      SERIAL PRIMARY KEY,
    role_id varchar(20) NOT NULL,
    menu_id int         NOT NULL
);

-- 角色菜单关联数据
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5703, '1', 1);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5704, '1', 108);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5705, '1', 73);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5706, '1', 80);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5707, '1', 81);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5708, '1', 82);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5709, '1', 83);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5710, '1', 84);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5711, '1', 85);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5712, '1', 86);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5713, '1', 87);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5714, '1', 88);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5715, '1', 3);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5716, '1', 5);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5717, '1', 6);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5718, '1', 7);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5719, '1', 4);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5720, '1', 8);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5721, '1', 9);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5722, '1', 10);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5723, '1', 111);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5724, '1', 36);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5725, '1', 38);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5726, '1', 99);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5727, '1', 100);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5728, '1', 101);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5729, '1', 102);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5730, '1', 37);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5731, '1', 42);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5732, '1', 45);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5733, '1', 11);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5734, '1', 12);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5735, '1', 15);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5736, '1', 16);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5737, '1', 17);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5738, '1', 60);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5739, '1', 13);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5740, '1', 18);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5741, '1', 19);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5742, '1', 20);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5743, '1', 33);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5744, '1', 14);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5745, '1', 69);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5746, '1', 70);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5747, '1', 123);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5748, '1', 124);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5749, '1', 125);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5750, '1', 126);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5751, '1', 21);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5752, '1', 22);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5753, '1', 24);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5754, '1', 23);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5755, '1', 25);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5756, '1', 93);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5757, '1', 94);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5758, '1', 104);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5759, '1', 105);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5760, '1', 106);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5761, '1', 26);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5762, '1', 53);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5763, '1', 71);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5764, '1', 27);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5765, '1', 28);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5766, '1', 29);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5767, '1', 30);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5768, '1', 31);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5769, '1', 32);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5770, '1', 34);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5771, '1', 35);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5772, '1', 39);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5773, '1', 40);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5774, '1', 41);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5775, '1', 61);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5776, '1', 62);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5777, '1', 63);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5778, '1', 64);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5779, '1', 65);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5780, '1', 66);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5781, '1', 103);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5782, '1', 51);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5783, '1', 54);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5784, '1', 55);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5785, '1', 56);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5786, '1', 57);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5787, '1', 58);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5788, '1', 52);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5789, '1', 76);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5790, '1', 77);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5791, '1', 78);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5792, '1', 79);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5793, '1', 46);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5794, '1', 67);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5795, '1', 68);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5796, '2', 1);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5797, '2', 108);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5798, '2', 73);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5799, '2', 84);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5800, '2', 88);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5801, '2', 3);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5802, '2', 4);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5803, '2', 111);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5804, '2', 36);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5805, '2', 38);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5806, '2', 99);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5807, '2', 102);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5808, '2', 37);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5809, '2', 11);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5810, '2', 12);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5811, '2', 60);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5812, '2', 13);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5813, '2', 14);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5814, '2', 95);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5815, '2', 96);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5816, '2', 97);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5817, '2', 98);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5818, '2', 123);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5819, '2', 21);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5820, '2', 22);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5821, '2', 23);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5822, '2', 93);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5823, '2', 104);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5824, '2', 26);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5825, '2', 53);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5826, '2', 27);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5827, '2', 34);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5828, '2', 35);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5829, '2', 61);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5830, '2', 65);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5831, '2', 103);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5832, '2', 51);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5833, '2', 57);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5834, '2', 52);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5835, '2', 46);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5836, '3', 1);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5837, '3', 108);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5838, '3', 73);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5839, '3', 3);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5840, '3', 4);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5841, '3', 111);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5842, '3', 36);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5843, '3', 38);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5844, '3', 37);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5845, '3', 11);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5846, '3', 12);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5847, '3', 60);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5848, '3', 13);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5849, '3', 14);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5850, '3', 123);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5851, '3', 21);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5852, '3', 22);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5853, '3', 23);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5854, '3', 93);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5855, '3', 104);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5856, '3', 26);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5857, '3', 53);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5858, '3', 27);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5859, '3', 34);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5860, '3', 35);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5861, '3', 61);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5862, '3', 65);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5863, '3', 51);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5864, '3', 57);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5865, '3', 52);
INSERT INTO t_role_menu (id, role_id, menu_id) VALUES (5866, '3', 46);

-- 重置序列值
SELECT setval('t_role_menu_id_seq', (SELECT COALESCE(MAX(id), 0) FROM t_role_menu));
