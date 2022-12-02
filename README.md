# POC | Open Hospital  - Telemetry Collector Server
See also [oh_telemetry_collector_server_java](https://github.com/goto-eof/oh_telemetry_collector_server_java) (has the same features, but it was developed in Java).

### Run 

```bash
docker-compose up
```

### Use postman collection for making requests

Import postman.json

### The server responds on

```
http://localhost:8013/telemetry
```


### DB connection

```
postgres://127.0.0.1:5432/postgres
username: postgres
password: postgres
```
