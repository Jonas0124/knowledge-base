# Knowledge Base (知识库)

## 核心依赖

+ [阿里云百炼](https://help.aliyun.com/product/2400256.html)
+ [actix-web](https://actix.rs/docs/)
+ [elasticsearch](https://docs.rs/elasticsearch/latest/elasticsearch/)

### 环境安装

+ [rust](https://www.rust-lang.org/zh-CN/)
+ [docker-desktop](https://www.docker.com/products/docker-desktop/)
+ [utoipa](https://github.com/juhaku/utoipa)

### 系统模块

- [ ] 用户模块
  - [x] 用户登录
  - [ ] 密码修改
- [ ] 知识库管理
  - [ ] 知识库列表
  - [ ] 知识库新增、修改、删除
  - [ ] 百炼联调
- [ ] Chat模块
  - [ ] 对话列表
  - [ ] Chat列表
  - [ ] 发起对话
- [ ] 超管模块
  - [ ] 系统设置
  - [x] 用户管理
    - [x] 创建用户
    - [x] 用户列表
    - [x] 重置密码
  - [ ] 知识库管理



  发版步骤:
  1.代码推送到生产环境(复制三个环境文件)，
  docker run --rm -v "C:\Users\weipe\RustroverProjects\knowledge-base:/app" -w /app rust:1.83 cargo build --release
  ，上传二进制包

2.执行命令 bash ./release.sh 部署类型 密码

