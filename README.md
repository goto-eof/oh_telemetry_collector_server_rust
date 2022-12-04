# POC | Open Hospital - Telemetry Collector Server (Rust)
See also [oh_telemetry_collector_server_java](https://github.com/goto-eof/oh_telemetry_collector_server_java) (has the same features, but it was developed in Java).

<p align="center" width="100%">
    <img width="50%" src="dev-stack.png"> 
</p>
 
### Run 

```bash
docker-compose up
```

### Use postman collection for making requests

Import postman.json

### The server responds on

```
http://localhost:8013/telemetry:8013
```


### DB connection

```
postgres://127.0.0.1:5432/postgres
username: postgres
password: postgres
```
