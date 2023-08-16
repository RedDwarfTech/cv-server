
#### 请求cv生成列表 


本地环境：

```bash
curl http://127.0.0.1:8000/cv/gen/v1/cv -H "x-access-token:1" -H "app-id:1" -H "user-id:1" -H "x-request-id:1" -H "device-id:1"
```

生产环境：

```bash
curl -X POST http://10.98.93.22:11015/cv/gen/v1/list -H "x-access-token:1" -H "app-id:1" -H "user-id:1" -H "x-request-id:1" -H "device-id:1" -d "{}"|jq "."
```


#### 更新cv生成状态

本地环境：

```bash
curl -X PUT http://127.0.0.1:8000/cv/gen/v1/result -H "Content-Type: application/json" -H "x-access-token:1" -H "app-id:1" -H "user-id:1" -H "x-request-id:1" -H "device-id:1" -d '{"id": 1,"gen_status": 1}'
```

#### 获取源码

```bash
curl -X GET http://127.0.0.1:8000/cv/gen/v1/src?id=146 -H "Content-Type: application/json" -H "x-access-token:eyJhbGciOiJIUzUxMiJ9.eyJ1c2VySWQiOjkyLCJkZXZpY2VJZCI6ImYzOTQyZjU2MzUzZmFkODQyOTU2ZjQwZDNmMjY3Y2MxIiwiYXBwSWQiOiJ4U1c1YTRCYlZCIiwiZXQiOjAsInBpZCI6MTIsImV4cCI6MTY4NTI3NDMzM30.4uMrxm9iYt940RDL6UWFw2iN5y3p27w6H4ALtBBiE1ESnuyqBqznEIs1f240omN7bC7CpQoVhi73IZmxVTJjwA" -H "app-id:1" -H "user-id:96" -H "x-request-id:1" -H "device-id:1"
```
