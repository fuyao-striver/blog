--
-- PostgreSQL database dump
--

\restrict gIeEQCmge4KiE7ZBX9PfDYVdOdP2k9bWMfxDhDKYJgNwbeVRdWFNsiqj9hzwgMY

-- Dumped from database version 18.3
-- Dumped by pg_dump version 18.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: t_menu; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.t_menu (
    id integer NOT NULL,
    parent_id integer DEFAULT 0 NOT NULL,
    menu_type character(1) NOT NULL,
    menu_name character varying(50) NOT NULL,
    path character varying(255) DEFAULT NULL::character varying,
    icon character varying(50) DEFAULT NULL::character varying,
    component character varying(50) DEFAULT NULL::character varying,
    perms character varying(100) DEFAULT ''::character varying,
    order_num integer DEFAULT 1 NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone,
    is_hidden boolean DEFAULT false CONSTRAINT t_menu_is_hidden_new_not_null NOT NULL,
    is_disable boolean DEFAULT false CONSTRAINT t_menu_is_disable_new_not_null NOT NULL
);


ALTER TABLE public.t_menu OWNER TO postgres;

--
-- Name: TABLE t_menu; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.t_menu IS '菜单表';


--
-- Name: COLUMN t_menu.id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.id IS '主键';


--
-- Name: COLUMN t_menu.parent_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.parent_id IS '父菜单id (parent_id为0且type为M则是一级菜单)';


--
-- Name: COLUMN t_menu.menu_type; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.menu_type IS '权限类型 (M目录 C菜单 B按钮)';


--
-- Name: COLUMN t_menu.menu_name; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.menu_name IS '名称';


--
-- Name: COLUMN t_menu.path; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.path IS '路由地址';


--
-- Name: COLUMN t_menu.icon; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.icon IS '菜单图标';


--
-- Name: COLUMN t_menu.component; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.component IS '菜单组件';


--
-- Name: COLUMN t_menu.perms; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.perms IS '权限标识';


--
-- Name: COLUMN t_menu.order_num; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.order_num IS '排序';


--
-- Name: COLUMN t_menu.create_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.create_time IS '创建时间';


--
-- Name: COLUMN t_menu.update_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.update_time IS '更新时间';


--
-- Name: COLUMN t_menu.is_hidden; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.is_hidden IS '是否隐藏 (false否 true是)';


--
-- Name: COLUMN t_menu.is_disable; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_menu.is_disable IS '是否禁用 (false否 true是)';


--
-- Name: t_menu_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.t_menu_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.t_menu_id_seq OWNER TO postgres;

--
-- Name: t_menu_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_menu_id_seq OWNED BY public.t_menu.id;


--
-- Name: t_role; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.t_role (
    id character varying(20) NOT NULL,
    role_name character varying(20) NOT NULL,
    role_desc character varying(50) DEFAULT NULL::character varying,
    is_disable boolean DEFAULT false NOT NULL,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone
);


ALTER TABLE public.t_role OWNER TO postgres;

--
-- Name: TABLE t_role; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.t_role IS '角色表';


--
-- Name: COLUMN t_role.id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.id IS '主键id';


--
-- Name: COLUMN t_role.role_name; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.role_name IS '角色名称';


--
-- Name: COLUMN t_role.role_desc; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.role_desc IS '角色描述';


--
-- Name: COLUMN t_role.is_disable; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.is_disable IS '是否禁用 (false否 true是)';


--
-- Name: COLUMN t_role.create_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.create_time IS '创建时间';


--
-- Name: COLUMN t_role.update_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role.update_time IS '更新时间';


--
-- Name: t_role_menu; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.t_role_menu (
    id integer NOT NULL,
    role_id character varying(20) NOT NULL,
    menu_id integer NOT NULL
);


ALTER TABLE public.t_role_menu OWNER TO postgres;

--
-- Name: TABLE t_role_menu; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.t_role_menu IS '角色菜单关联表';


--
-- Name: COLUMN t_role_menu.id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role_menu.id IS '主键';


--
-- Name: COLUMN t_role_menu.role_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role_menu.role_id IS '角色id';


--
-- Name: COLUMN t_role_menu.menu_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_role_menu.menu_id IS '菜单id';


--
-- Name: t_role_menu_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.t_role_menu_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.t_role_menu_id_seq OWNER TO postgres;

--
-- Name: t_role_menu_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_role_menu_id_seq OWNED BY public.t_role_menu.id;


--
-- Name: t_user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.t_user (
    id integer NOT NULL,
    nickname character varying(50) NOT NULL,
    username character varying(50) NOT NULL,
    password character varying(100) NOT NULL,
    avatar character varying(255) NOT NULL,
    web_site character varying(255) DEFAULT ''::character varying,
    intro character varying(100) DEFAULT ''::character varying,
    email character varying(50) DEFAULT ''::character varying,
    ip_address character varying(50) DEFAULT ''::character varying,
    ip_source character varying(50) DEFAULT ''::character varying,
    login_type smallint DEFAULT 0 NOT NULL,
    is_disable smallint DEFAULT 0 NOT NULL,
    login_time timestamp without time zone,
    create_time timestamp without time zone NOT NULL,
    update_time timestamp without time zone
);


ALTER TABLE public.t_user OWNER TO postgres;

--
-- Name: TABLE t_user; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.t_user IS '用户表';


--
-- Name: COLUMN t_user.id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.id IS '用户id';


--
-- Name: COLUMN t_user.nickname; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.nickname IS '用户昵称';


--
-- Name: COLUMN t_user.username; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.username IS '用户名';


--
-- Name: COLUMN t_user.password; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.password IS '用户密码';


--
-- Name: COLUMN t_user.avatar; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.avatar IS '头像';


--
-- Name: COLUMN t_user.web_site; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.web_site IS '个人网站';


--
-- Name: COLUMN t_user.intro; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.intro IS '个人简介';


--
-- Name: COLUMN t_user.email; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.email IS '邮箱';


--
-- Name: COLUMN t_user.ip_address; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.ip_address IS '登录ip';


--
-- Name: COLUMN t_user.ip_source; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.ip_source IS '登录地址';


--
-- Name: COLUMN t_user.login_type; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.login_type IS '登录方式 (1邮箱 2QQ 3Gitee 4Github)';


--
-- Name: COLUMN t_user.is_disable; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.is_disable IS '是否禁用 (0否 1是)';


--
-- Name: COLUMN t_user.login_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.login_time IS '登录时间';


--
-- Name: COLUMN t_user.create_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.create_time IS '创建时间';


--
-- Name: COLUMN t_user.update_time; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user.update_time IS '更新时间';


--
-- Name: t_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.t_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.t_user_id_seq OWNER TO postgres;

--
-- Name: t_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_user_id_seq OWNED BY public.t_user.id;


--
-- Name: t_user_role; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.t_user_role (
    id integer NOT NULL,
    user_id integer NOT NULL,
    role_id character varying(20) NOT NULL
);


ALTER TABLE public.t_user_role OWNER TO postgres;

--
-- Name: TABLE t_user_role; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.t_user_role IS '用户角色关联表';


--
-- Name: COLUMN t_user_role.id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user_role.id IS '主键';


--
-- Name: COLUMN t_user_role.user_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user_role.user_id IS '用户id';


--
-- Name: COLUMN t_user_role.role_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.t_user_role.role_id IS '角色id';


--
-- Name: t_user_role_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.t_user_role_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.t_user_role_id_seq OWNER TO postgres;

--
-- Name: t_user_role_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_user_role_id_seq OWNED BY public.t_user_role.id;


--
-- Name: t_menu id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_menu ALTER COLUMN id SET DEFAULT nextval('public.t_menu_id_seq'::regclass);


--
-- Name: t_role_menu id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_role_menu ALTER COLUMN id SET DEFAULT nextval('public.t_role_menu_id_seq'::regclass);


--
-- Name: t_user id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user ALTER COLUMN id SET DEFAULT nextval('public.t_user_id_seq'::regclass);


--
-- Name: t_user_role id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user_role ALTER COLUMN id SET DEFAULT nextval('public.t_user_role_id_seq'::regclass);


--
-- Data for Name: t_menu; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.t_menu (id, parent_id, menu_type, menu_name, path, icon, component, perms, order_num, create_time, update_time, is_hidden, is_disable) FROM stdin;
1	0	M	文章管理	article	archives			1	2022-12-04 09:13:31	2023-02-21 15:36:45	f	f
3	1	C	分类管理	category	category	/blog/category/index	blog:category:list	3	2022-12-04 09:22:20	2023-02-21 15:21:19	f	f
4	1	C	标签管理	tag	tag	/blog/tag/index	blog:tag:list	4	2022-12-04 09:23:01	2023-02-21 15:21:23	f	f
5	3	B	添加分类	\N	\N	\N	blog:category:add	1	2022-12-04 09:30:55	\N	f	f
6	3	B	删除分类	\N	\N	\N	blog:category:delete	2	2022-12-04 09:32:15	2022-12-26 15:39:20	f	f
7	3	B	修改分类	\N	\N	\N	blog:category:update	3	2022-12-04 09:33:52	\N	f	f
8	4	B	添加标签	\N	\N		blog:tag:add	1	2022-12-04 10:19:51	\N	f	f
9	4	B	删除标签	\N	\N	\N	blog:tag:delete	2	2022-12-04 10:20:41	\N	f	f
10	4	B	修改标签	\N	\N	\N	blog:tag:update	3	2022-12-04 10:21:32	\N	f	f
11	0	M	系统管理	system	system			3	2022-12-06 10:58:50	2023-01-03 18:47:19	f	f
12	11	C	菜单管理	menu	tree-table	/system/menu/index	system:menu:list	1	2022-12-06 16:33:56	\N	f	f
13	11	C	角色管理	role	peoples	/system/role/index	system:role:list	2	2022-12-06 17:09:55	\N	f	f
14	11	C	用户管理	user	user	/system/user/index	system:user:list	3	2022-12-06 17:10:28	\N	f	f
15	12	B	添加菜单	\N		\N	system:menu:add	1	2022-12-07 10:50:22	\N	f	f
16	12	B	删除菜单	\N		\N	system:menu:delete	2	2022-12-07 10:50:54	\N	f	f
17	12	B	修改菜单	\N		\N	system:menu:update	3	2022-12-07 10:55:21	\N	f	f
18	13	B	添加角色	\N	\N	\N	system:role:add	1	2022-12-07 10:56:24	\N	f	f
19	13	B	删除角色	\N	\N	\N	system:role:delete	2	2022-12-07 10:56:50	\N	f	f
20	13	B	修改角色	\N	\N	\N	system:role:update	3	2022-12-07 10:57:15	\N	f	f
21	0	M	日志管理	log	log			4	2022-12-21 17:36:39	2023-02-21 15:20:13	f	f
22	21	C	操作日志	operation	form	/system/log/operation	log:operation:list	1	2022-12-21 20:14:01	\N	f	f
23	21	C	异常日志	exception	bug	/system/log/exception	log:exception:list	2	2022-12-21 20:48:25	\N	f	f
24	22	B	删除操作日志	\N	\N	\N	log:operation:delete	1	2022-12-26 16:43:00	\N	f	f
25	23	B	删除异常日志	\N	\N	\N	log:exception:delete	1	2022-12-27 13:21:50	\N	f	f
26	0	M	系统监控	monitor	monitor	\N		5	2022-12-27 13:23:29	2023-01-03 18:47:27	f	f
27	26	C	定时任务	task	job	/monitor/task/index	monitor:task:list	2	2022-12-27 13:26:29	2023-01-01 21:08:35	f	f
28	27	B	添加任务	\N	\N	\N	monitor:task:add	1	2022-12-27 13:32:42	\N	f	f
29	27	B	修改任务	\N	\N	\N	monitor:task:update	2	2022-12-27 13:33:45	\N	f	f
30	27	B	删除任务	\N	\N	\N	monitor:task:delete	3	2022-12-27 13:34:29	\N	f	f
31	27	B	修改任务状态	\N	\N	\N	monitor:task:status	4	2022-12-27 13:43:24	\N	f	f
32	27	B	运行任务	\N	\N	\N	monitor:task:run	5	2022-12-27 13:45:34	\N	f	f
33	13	B	修改角色状态	\N	\N	\N	system:role:status	4	2022-12-27 13:46:39	\N	f	f
34	0	M	网站管理	web	international	\N		6	2022-12-30 17:22:33	2023-02-14 09:46:29	f	f
35	34	C	友链管理	friend	friend	/web/friend/index	web:friend:list	1	2022-12-30 17:33:15	\N	f	f
36	0	M	消息管理	news	email	\N		2	2022-12-30 17:50:06	2022-12-30 18:02:12	f	f
37	36	C	留言管理	message	form	/news/message/index	news:message:list	2	2022-12-30 17:58:25	2022-12-30 18:01:47	f	f
38	36	C	评论管理	comment	comment	/news/comment/index	news:comment:list	1	2022-12-30 17:59:37	2022-12-30 18:03:35	f	f
39	35	B	添加友链	\N	\N	\N	web:friend:add	1	2022-12-30 18:56:22	\N	f	f
40	35	B	删除友链	\N	\N	\N	web:friend:delete	2	2022-12-30 18:56:42	\N	f	f
41	35	B	修改友链	\N	\N	\N	web:friend:update	3	2022-12-30 18:57:08	\N	f	f
42	37	B	删除留言	\N	\N	\N	news:message:delete	1	2022-12-30 22:05:53	\N	f	f
45	37	B	审核留言	\N	\N	\N	news:message:pass	2	2022-12-30 22:29:24	\N	f	f
46	34	C	网站配置	site	example	/web/site/index	web:site:list	5	2022-12-31 11:50:45	2023-01-03 18:49:17	f	f
51	34	C	相册管理	album	album	/web/album/index	web:album:list	3	2023-01-01 18:16:40	2023-01-03 18:49:06	f	f
52	34	C	照片管理	photo/:albumId	photo	/web/photo/index	web:photo:list	4	2023-01-01 18:18:11	2023-01-01 18:39:22	f	f
53	26	C	在线用户	online	online	/monitor/online/index	monitor:online:list	1	2023-01-01 21:07:48	2023-01-01 21:08:29	f	f
54	51	B	添加相册	\N	\N	\N	web:album:add	1	2023-01-02 19:01:33	\N	f	f
55	51	B	删除相册	\N	\N	\N	web:album:delete	2	2023-01-02 19:02:03	\N	f	f
56	51	B	修改相册	\N	\N	\N	web:album:update	3	2023-01-02 19:02:50	\N	f	f
57	51	B	编辑相册	\N	\N	\N	web:album:edit	4	2023-01-02 19:03:40	\N	f	f
58	51	B	上传相册封面	\N	\N	\N	web:album:upload	5	2023-01-02 19:04:38	\N	f	f
60	12	B	编辑菜单	\N	\N	\N	system:menu:edit	4	2023-01-03 18:29:57	\N	f	f
61	34	C	说说管理	talk	talk	/web/talk/index	web:talk:list	2	2023-01-03 18:48:28	2023-01-03 18:48:41	f	f
62	61	B	添加说说	\N	\N	\N	web:talk:add	1	2023-01-05 19:16:42	\N	f	f
63	61	B	删除说说	\N	\N	\N	web:talk:delete	2	2023-01-05 19:17:07	\N	f	f
64	61	B	修改说说	\N	\N	\N	web:talk:update	3	2023-01-05 19:17:36	\N	f	f
65	61	B	编辑说说	\N	\N	\N	web:talk:edit	4	2023-01-05 19:18:27	\N	f	f
66	61	B	上传说说图片	\N	\N	\N	web:talk:upload	5	2023-01-05 19:18:52	\N	f	f
67	46	B	修改网站配置	\N	\N	\N	web:site:update	1	2023-01-08 09:15:56	\N	f	f
68	46	B	上传网站配置图片	\N	\N	\N	web:site:upload	2	2023-01-08 14:53:16	\N	f	f
69	14	B	修改用户	\N	\N	\N	system:user:update	1	2023-01-09 17:03:18	\N	f	f
70	14	B	修改用户状态	\N	\N	\N	system:user:status	2	2023-01-09 17:03:51	\N	f	f
71	53	B	下线用户	\N	\N	\N	monitor:online:kick	1	2023-01-09 19:18:33	\N	f	f
73	1	C	文章列表	list	chart	/blog/article/list	blog:article:list	2	2023-01-10 17:37:29	2023-02-21 15:36:09	f	f
76	52	B	添加照片	\N	\N	\N	web:photo:add	1	2023-01-11 18:45:28	\N	f	f
77	52	B	删除照片	\N	\N	\N	web:photo:delete	2	2023-01-11 18:45:51	\N	f	f
78	52	B	修改照片	\N	\N	\N	web:photo:update	3	2023-01-11 18:46:12	\N	f	f
79	52	B	上传照片	\N	\N	\N	web:photo:upload	3	2023-01-11 18:46:48	\N	f	f
80	73	B	添加文章	\N	\N	\N	blog:article:add	1	2023-01-14 15:25:29	\N	f	f
81	73	B	物理删除文章	\N	\N	\N	blog:article:delete	2	2023-01-14 15:26:44	\N	f	f
82	73	B	逻辑删除文章	\N	\N	\N	blog:article:recycle	3	2023-01-14 15:28:32	\N	f	f
83	73	B	更新文章	\N	\N	\N	blog:article:update	4	2023-01-14 15:30:11	\N	f	f
84	73	B	编辑文章	\N	\N	\N	blog:article:edit	5	2023-01-14 15:32:34	2023-01-14 15:32:54	f	f
85	73	B	上传文章图片	\N	\N	\N	blog:article:upload	6	2023-01-14 15:34:05	2023-01-15 11:19:13	f	f
86	73	B	置顶文章	\N	\N	\N	blog:article:top	7	2023-01-14 15:35:33	\N	f	f
87	73	B	推荐文章	\N	\N	\N	blog:article:recommend	8	2023-01-14 15:36:08	\N	f	f
88	73	B	点赞文章	\N	\N	\N	blog:article:like	9	2023-01-14 15:36:39	\N	f	f
93	21	C	访问日志	visit	logininfor	/system/log/visit	log:visit:list	3	2023-01-28 19:04:09	2023-01-28 19:06:52	f	f
94	93	B	删除访问日志	\N	\N	\N	log:visit:delete	1	2023-01-28 19:05:31	\N	f	f
95	14	B	修改用户邮箱	\N	\N	\N	user:email:update	3	2023-01-31 09:26:22	\N	f	f
96	14	B	修改用户头像	\N	\N	\N	user:avatar:update	4	2023-01-31 09:27:03	\N	f	f
97	14	B	修改用户信息	\N	\N	\N	user:info:update	5	2023-01-31 09:27:37	\N	f	f
98	14	B	修改用户密码	\N	\N	\N	user:password:update	6	2023-01-31 09:28:10	\N	f	f
99	38	B	添加评论	\N	\N	\N	news:comment:add	1	2023-02-08 19:09:25	\N	f	f
100	38	B	删除评论	\N	\N	\N	news:comment:delete	2	2023-02-08 19:09:57	\N	f	f
101	38	B	审核评论	\N	\N	\N	news:comment:pass	3	2023-02-08 19:10:26	\N	f	f
102	38	B	点赞评论	\N	\N	\N	news:comment:like	4	2023-02-08 19:10:45	\N	f	f
103	61	B	点赞说说	\N	\N	\N	web:talk:like	6	2023-02-10 11:16:23	\N	f	f
104	21	C	任务日志	task	job	/system/log/task	log:task:list	3	2023-02-14 10:28:28	2023-02-14 10:28:41	f	f
105	104	B	删除任务日志	\N	\N	\N	log:task:delete	1	2023-02-14 11:21:06	\N	f	f
106	104	B	清空任务日志	\N	\N	\N	log:task:clear	2	2023-02-14 11:21:28	\N	f	f
108	1	C	发布文章	write	edit	/blog/article/write	blog:article:list	1	2023-02-21 13:32:22	2023-02-21 15:36:04	f	f
111	1	C	修改文章	write/:articleId	edit	/blog/article/write	blog:article:list	5	2023-02-21 15:40:11	2023-02-21 15:41:07	f	f
123	11	C	文件管理	file	file	/system/file/index	system:file:list	4	2023-03-09 10:57:29	\N	f	f
124	123	B	上传文件	\N	\N	\N	system:file:upload	1	2023-03-10 23:11:33	\N	f	f
125	123	B	新建文件夹	\N	\N	\N	system:file:createFolder	2	2023-03-10 23:12:11	\N	f	f
126	123	B	删除文件	\N	\N	\N	system:file:delete	3	2023-03-10 23:12:45	\N	f	f
\.


--
-- Data for Name: t_role; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.t_role (id, role_name, role_desc, is_disable, create_time, update_time) FROM stdin;
1	admin	管理员	f	2022-11-03 17:41:57	2023-03-10 23:12:59
2	user	普通用户	f	2022-11-03 17:42:17	2023-03-10 23:13:11
3	test	测试账号	f	2022-11-03 17:42:31	2023-03-10 23:13:17
\.


--
-- Data for Name: t_role_menu; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.t_role_menu (id, role_id, menu_id) FROM stdin;
5703	1	1
5704	1	108
5705	1	73
5706	1	80
5707	1	81
5708	1	82
5709	1	83
5710	1	84
5711	1	85
5712	1	86
5713	1	87
5714	1	88
5715	1	3
5716	1	5
5717	1	6
5718	1	7
5719	1	4
5720	1	8
5721	1	9
5722	1	10
5723	1	111
5724	1	36
5725	1	38
5726	1	99
5727	1	100
5728	1	101
5729	1	102
5730	1	37
5731	1	42
5732	1	45
5733	1	11
5734	1	12
5735	1	15
5736	1	16
5737	1	17
5738	1	60
5739	1	13
5740	1	18
5741	1	19
5742	1	20
5743	1	33
5744	1	14
5745	1	69
5746	1	70
5747	1	123
5748	1	124
5749	1	125
5750	1	126
5751	1	21
5752	1	22
5753	1	24
5754	1	23
5755	1	25
5756	1	93
5757	1	94
5758	1	104
5759	1	105
5760	1	106
5761	1	26
5762	1	53
5763	1	71
5764	1	27
5765	1	28
5766	1	29
5767	1	30
5768	1	31
5769	1	32
5770	1	34
5771	1	35
5772	1	39
5773	1	40
5774	1	41
5775	1	61
5776	1	62
5777	1	63
5778	1	64
5779	1	65
5780	1	66
5781	1	103
5782	1	51
5783	1	54
5784	1	55
5785	1	56
5786	1	57
5787	1	58
5788	1	52
5789	1	76
5790	1	77
5791	1	78
5792	1	79
5793	1	46
5794	1	67
5795	1	68
5796	2	1
5797	2	108
5798	2	73
5799	2	84
5800	2	88
5801	2	3
5802	2	4
5803	2	111
5804	2	36
5805	2	38
5806	2	99
5807	2	102
5808	2	37
5809	2	11
5810	2	12
5811	2	60
5812	2	13
5813	2	14
5814	2	95
5815	2	96
5816	2	97
5817	2	98
5818	2	123
5819	2	21
5820	2	22
5821	2	23
5822	2	93
5823	2	104
5824	2	26
5825	2	53
5826	2	27
5827	2	34
5828	2	35
5829	2	61
5830	2	65
5831	2	103
5832	2	51
5833	2	57
5834	2	52
5835	2	46
5836	3	1
5837	3	108
5838	3	73
5839	3	3
5840	3	4
5841	3	111
5842	3	36
5843	3	38
5844	3	37
5845	3	11
5846	3	12
5847	3	60
5848	3	13
5849	3	14
5850	3	123
5851	3	21
5852	3	22
5853	3	23
5854	3	93
5855	3	104
5856	3	26
5857	3	53
5858	3	27
5859	3	34
5860	3	35
5861	3	61
5862	3	65
5863	3	51
5864	3	57
5865	3	52
5866	3	46
\.


--
-- Data for Name: t_user; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.t_user (id, nickname, username, password, avatar, web_site, intro, email, ip_address, ip_source, login_type, is_disable, login_time, create_time, update_time) FROM stdin;
3	测试账号	test@qq.com	$argon2id$v=19$m=19456,t=2,p=1$KblZbGWHpF5MdL5pkFVncg$40UA21n4gJAz4xZrko9YsNkHaTZrCQVitB80wlvy74E	https://static.ttkwsd.top/config/0bca52afdb2b9998132355d716390c9f.png	https://www.ttkwsd.top	个人简介	test@qq.com	192.168.23.1	内网IP|内网IP	1	0	2023-02-24 10:45:59	2022-11-30 21:34:26	2023-02-24 10:45:59
1	阿冬	admin@qq.com	$argon2id$v=19$m=19456,t=2,p=1$KblZbGWHpF5MdL5pkFVncg$40UA21n4gJAz4xZrko9YsNkHaTZrCQVitB80wlvy74E	https://static.ttkwsd.top/config/9c65807710f54d9d5ad398a78216ebfb.jpg	\N	\N	1632167813@qq.com	192.168.23.1	内网IP|内网IP	1	0	2023-03-10 22:26:23	2022-11-29 21:45:48	2023-03-10 22:26:23
\.


--
-- Data for Name: t_user_role; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.t_user_role (id, user_id, role_id) FROM stdin;
50	3	3
54	1	1
\.


--
-- Name: t_menu_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.t_menu_id_seq', 127, false);


--
-- Name: t_role_menu_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.t_role_menu_id_seq', 5867, false);


--
-- Name: t_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.t_user_id_seq', 16, false);


--
-- Name: t_user_role_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.t_user_role_id_seq', 1, true);


--
-- Name: t_menu t_menu_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_menu
    ADD CONSTRAINT t_menu_pkey PRIMARY KEY (id);


--
-- Name: t_role_menu t_role_menu_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_role_menu
    ADD CONSTRAINT t_role_menu_pkey PRIMARY KEY (id);


--
-- Name: t_role t_role_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_role
    ADD CONSTRAINT t_role_pkey PRIMARY KEY (id);


--
-- Name: t_user t_user_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user
    ADD CONSTRAINT t_user_pkey PRIMARY KEY (id);


--
-- Name: t_user_role t_user_role_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user_role
    ADD CONSTRAINT t_user_role_pkey PRIMARY KEY (id);


--
-- Name: t_user_role fk_user_role_role; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user_role
    ADD CONSTRAINT fk_user_role_role FOREIGN KEY (role_id) REFERENCES public.t_role(id);


--
-- Name: t_user_role fk_user_role_user; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_user_role
    ADD CONSTRAINT fk_user_role_user FOREIGN KEY (user_id) REFERENCES public.t_user(id);


--
-- PostgreSQL database dump complete
--

\unrestrict gIeEQCmge4KiE7ZBX9PfDYVdOdP2k9bWMfxDhDKYJgNwbeVRdWFNsiqj9hzwgMY

