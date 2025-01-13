#!/bin/bash


#获取值
echo "第一个参数是密码：$1"
echo "第二个参数是启动哪些服务：mysql8,rust,all：$2"

# 修改环境变量，并且保存（.env）
sed -i "0,/^DATABASE_URL=.*/s|^DATABASE_URL=.*|DATABASE_URL=mysql://root:$1@mysql8:3306/knowledge|" .env


sed -i "0,/^MYSQL_ROOT_PASSWORD=.*/s|^MYSQL_ROOT_PASSWORD=.*|MYSQL_ROOT_PASSWORD=$1|" .env

# 构建部署（根据传入值来确定部署哪些容器）
if [[ $2 = "mysql8" ]]; then
  echo "1开始部署$2！"
  docker compose up -d --build mysql8
elif [[ $2 = "rust" ]]; then
  echo "2开始部署$2！"
  docker compose up -d --build knowledge-base
else
  echo "3开始部署$2！"
  docker compose up -d --build
fi
echo "操作完成！"