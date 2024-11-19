# Village Monitor

## Overview
This is simple toolset for environment monitoring using:
* Any kind of ARM64 board. I'm using Orange PI 3B. 
* ESP32 boards, using Micropython ecosystem
* DHT22 sensors


ESP32 boards pushes values of DHT22s sensors periodically, falling into deep sleep between pushes. They pushes values via simpe GET <webappURL>/temp?location= &temp= &humidity= request, then webapp, on receiving data, pushes it to influxdb. 

User may install grafana or any other visualisation software then and represent data with beautifull dashboards.


## Installation


On ubuntu, install rustup, libssl-dev, esptool, pkg-config, influxdb, influxdb-client
```
apt update
apt install rustup libssl-dev esptool pkg-config influxdb influxdb-client
```

Install rust:
```
rustup install stable
```


Clone and build 

```
git clone https://github.com/agv100/village_monitor.git
cd village_monitor
cargo build
```

Create influxdb database (Note, I include 111 everywhere as part of my street address. You may change this in code above  

```
influx
create database Sensors111
```

Install micropython onto the boards, according to ESP32 micropython installation.
Install mpremote: https://docs.micropython.org/en/latest/reference/mpremote.html  


Adjust dachaconnect.py file with correct SSID/WPA Password.  
For each of the controllers, adjust "location" setting in main.py, this is individual for each of controllers.  
Adjust IP address of server.   
Adjust DHT22 pin, if needed. I'm using DHT22 board, you may use DHT22 sensor and pullup resistor, see corresponding instruction over internet

Then, upload files to the controller
```
mpremote connect <port>
mpremote fs cp dachaconnect.py :dachaconnect.py
mpremote fs cp main.py :main.py
```

Run village_monitor app in foregroung, be sure that you see log messages of incoming temperature/humidity values:

```
./target/debug/village_monitor
2024-11-16 20:07:59.256739707 +00:00 Location:cherdak Temp:18.5 Humidity:47.7
2024-11-16 20:09:03.261042330 +00:00 Location:cherdak Temp:18.4 Humidity:47.8
2024-11-16 20:09:10.130993985 +00:00 Location:cherdak Temp:18.5 Humidity:47.8
2024-11-16 20:10:13.820369117 +00:00 Location:cherdak Temp:18.5 Humidity:47.8
```

Also check that data inserted into database:

```
influx
use Sensors111
select * from DHT22s
```

If everythin OK, install visualisation software compatible with InfluxDB as backend, i.e.  Grafana, configure visualisation with simple selects from DHT22s measurement.
