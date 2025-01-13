#!/bin/bash

# 容器名称
CONTAINER_NAME="meinerust"  # 替换为实际的容器名称

# 步骤 1：拉取最新的 Git 代码
echo "拉取最新的 Git 代码..."
cd /root/rust
rm -rf ${CONTAINER_NAME}
git clone git@gitee.com:wei___peng/${CONTAINER_NAME}.git

# 步骤 2：构建 Docker 镜像
echo "构建 Docker 镜像..."
cd ${CONTAINER_NAME}
docker build -t ${CONTAINER_NAME} .

# 步骤 3：停止并删除旧的 Docker 容器（如果有的话）
# 获取容器ID
CONTAINER_ID=$(docker ps -aq --filter "name=${CONTAINER_NAME}")


IMAGE_ID
if [[ -n $CONTAINER_ID ]]; then
  IMAGE_ID=$(docker inspect --format '{{.Image}}' $CONTAINER_ID)
  echo "找到镜像 ${CONTAINER_NAME}，ID：$IMAGE_ID"
  echo "停止并删除旧的 Docker 容器..."
  docker stop ${CONTAINER_NAME}
  docker rm ${CONTAINER_NAME}
  docker rmi ${IMAGE_ID}

fi
# 步骤 4：运行新的 Docker 容器
echo "运行新的 Docker 容器..."
docker run -d --name meinerust -p 7878:7878 --restart=unless-stopped ${CONTAINER_NAME}

echo "操作完成！"