import wifi

import socket
from machine import Pin,deepsleep
import dht

wifi.dachaconnect()


d=dht.DHT22(Pin(25))

# Adjust
location = "cherdak"
host = "192.168.0.104"
port = 3000

while True:
    d.measure()
    temp = d.temperature()
    humidity = d.humidity()
    s = socket.socket()
    s.connect((host, port))

    path =f"temp?location={location}&temp={temp}&humidity={humidity}"
    s.send(bytes('GET /%s HTTP/1.0\r\nHost: %s\r\n\r\n' % (path, host), 'utf8'))
    deepsleep(290000)
