# README

## Introduce

本仓库是浙江大学短学期Rust课程作业的内容，使用axum框架包装RPC请求，实现从http协议访问RPC资源和函数的效果。

由于使用了一些原mini-redis的RPC实现的接口和参数，需要将项目仓库放到和原RPC的仓库的同一目录下，以便顺利获取依赖。

## How to use

在根目录使用cargo run，请确保localhost的3000端口没有被占用

## Features

本服务包含以下接口

1. ping_http 

| 方法  | URL                      | 说明                            |
| --- | ------------------------ | ----------------------------- |
| Get | localhost:3000/ping_http | 若成功连接http服务器，返回pong from http |

2. ping_rpc

| 方法  | URL                     | 说明                          |
| --- | ----------------------- | --------------------------- |
| Get | localhost:3000/ping_rpc | 若成功连接rpc服务器，返回pong from rpc |

3. get_rpc

| 方法   | URL                    | 参数类型                  |
| ---- | ---------------------- | --------------------- |
| Post | localhost:3000/get_rpc | x-www-form-urlencoded |

| 参数名 | 描述     | 类型     | 备注  |
| --- | ------ | ------ | --- |
| key | 需要查询的键 | string | 必填  |

| 返回    | 描述    | 类型   | 备注           |
| ----- | ----- | ---- | ------------ |
| value | 键对应的值 | text | 若不存在，返回(nil) |

4. set_rpc

| 方法   | URL                    | 参数类型                  |
| ---- | ---------------------- | --------------------- |
| Post | localhost:3000/set_rpc | x-www-form-urlencoded |

| 参数名   | 描述     | 类型     | 备注  |
| ----- | ------ | ------ | --- |
| key   | 需要设置的键 | string | 必填  |
| value | 需要设置的值 | string | 必填  |

| 返回     | 描述   | 类型   | 备注          |
| ------ | ---- | ---- | ----------- |
| Status | 是否正常 | text | 正常则返回set ok |

5. del_rpc

| 方法   | URL                    | 参数类型                  |
| ---- | ---------------------- | --------------------- |
| Post | localhost:3000/get_rpc | x-www-form-urlencoded |

| 参数名 | 描述     | 类型     | 备注  |
| --- | ------ | ------ | --- |
| key | 需要查询的键 | string | 必填  |

| 返回     | 描述   | 类型   | 备注                    |
| ------ | ---- | ---- | --------------------- |
| Status | 是否正常 | text | 正常则返回del ok，否则返回error |

## Test

测试使用reqwest，在另外的仓库里，可参见作业提交表格

https://github.com/Wtbg/mini-redis-httpclient

## Beg for score

球球助教给好分
