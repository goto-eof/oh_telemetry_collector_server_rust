```
 ________  ___  ___          _________  ________  ________      
|\   __  \|\  \|\  \        |\___   ___\\   ____\|\   ____\     
\ \  \|\  \ \  \\\  \       \|___ \  \_\ \  \___|\ \  \___|_    
 \ \  \\\  \ \   __  \           \ \  \ \ \  \    \ \_____  \   
  \ \  \\\  \ \  \ \  \           \ \  \ \ \  \____\|____|\  \  
   \ \_______\ \__\ \__\           \ \__\ \ \_______\____\_\  \ 
    \|_______|\|__|\|__|            \|__|  \|_______|\_________\
                                                    \|_________|
                                                                
                                                                
```                                                              

# POC | Open Hospital - Telemetry Collector Server (Rust)
See also [oh_telemetry_collector_server_java](https://github.com/goto-eof/oh_telemetry_collector_server_java) and [oh_telemetry_collector_test_load](https://github.com/goto-eof/oh_telemetry_collector_load_test).

[Here](https://openhospital.atlassian.net/browse/OP-952) is the Jira ticket.

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
http://localhost:8017/telemetry
```


### DB connection

```
postgres://127.0.0.1:5437/postgres
username: postgres
password: postgres
```
