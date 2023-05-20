
#### 请求cv生成列表 


本地环境：

```bash
curl http://127.0.0.1:8000/cv/gen/v1/cv -H "x-access-token:1" -H "app-id:1" -H "user-id:1" -H "x-request-id:1" -H "device-id:1"
```

生产环境：

```bash
curl -X POST http://10.98.93.22:11015/cv/gen/v1/list -H "x-access-token:1" -H "app-id:1" -H "user-id:1" -H "x-request-id:1" -H "device-id:1" -d "{}"|jq "."
```