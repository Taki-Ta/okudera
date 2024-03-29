# 介绍
这是一个由 [salvo-cli](https://github.com/salvo-rs/salvo-cli) 生成的项目,你可以按照以下命令来运行程序以及测试(非sqlite数据库的请先按照教程修改数据库连接串,完成数据的初始工作)。
😄 最新版的 Salvo 依赖 Rust 版本 1.75。如果编译失败，请尝试使用 `rustup update` 来升级版本。
``` shell
//运行项目
cargo run 
//运行测试
cargo test
```
# 小贴士
- 如果你选择的sqlite或者已经初始化了users表的数据,请使用账号 zhangsan 密码123 来登录。
- 程序数据库连接串在config/config.toml里,但是如果你使用的是sqlx或者seaorm,库本身读取.env文件的配置来生成实体,运行迁移,验证。所以当你修改数据库连接串时,需要同时修改两个地方。
# orm 的文档或主页链接
🎯 您选择了sqlx，文档可以在这里查看:https://github.com/launchbadge/sqlx
## sqlx_cli
SQLx的相关命令行实用程序,用于管理数据库、迁移以及使用sqlx::query!()等启用“脱机”模式。 https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
## 数据初始化
你选择的是sqlite数据库,数据库已初始化完毕,在data文件夹下。
# 项目目录说明
# okudera
- **目录:** okudera 
- *文件:* .env         (存储数据库连接串的配置文件)
- **目录:** assets         (静态资源如图片、JavaScript脚本和CSS样式表)
    - *文件:* favicon.ico 
    - **目录:** js 
        - *文件:* alpinejs.js 
        - *文件:* sweetalert2.js 
        - *文件:* tailwindcss.js 
- *文件:* Cargo.toml         (Rust项目的依赖和配置信息)
- **目录:** config         (包含所有配置文件的文件夹)
    - **目录:** certs 
        - *文件:* cert.pem 
        - *文件:* key.pem 
    - *文件:* config.yml 
- **目录:** data         (包含数据库文件或初始化数据sql文件的目录)
    - *文件:* demo.db 
- **目录:** migrations         (数据库迁移脚本的存放位置)
    - **目录:** 2021-10-20-000000_create_users_table 
        - *文件:* up.sql 
- **目录:** src         (源代码目录)
    - *文件:* app_error.rs 
    - *文件:* app_writer.rs 
    - *文件:* config.rs 
    - *文件:* db.rs 
    - **目录:** dtos 
        - *文件:* mod.rs 
        - *文件:* user.rs 
    - **目录:** entities 
        - *文件:* mod.rs 
        - *文件:* users.rs 
    - *文件:* main.rs 
    - **目录:** middleware 
        - *文件:* cors.rs 
        - *文件:* handle_404.rs 
        - *文件:* jwt.rs 
        - *文件:* mod.rs 
    - **目录:** routers 
        - *文件:* demo.rs 
        - *文件:* mod.rs 
        - *文件:* static_routers.rs 
        - *文件:* user.rs 
    - **目录:** services 
        - *文件:* mod.rs 
        - *文件:* user.rs 
    - **目录:** utils 
        - *文件:* mod.rs 
        - *文件:* rand_utils.rs 
- **目录:** templates         (存放网页模板文件的文件夹)
    - *文件:* handle_404.html 
    - *文件:* hello.html 
    - *文件:* login.html 
    - *文件:* user_list.html 
    - *文件:* user_list_page.html 

# 关于赛风(salvo)
你可以在 https://salvo.rs/ 📖查看salvo的文档以及更多例子,如果我们的工具帮到你,欢迎start [salvo](https://github.com/salvo-rs/salvo) 和 [salvo-cli](https://github.com/salvo-rs/salvo-cli),这将给我们很大激励。❤️