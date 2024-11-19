#
#

def dachaconnect():
    import network
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    if not wlan.isconnected():
        wlan.disconnect()
        print('connecting to network...')
        wlan.connect('ADJUST', 'ADJUST')
        while not wlan.isconnected():
            pass
#    print('network config:', wlan.ipconfig('addr4'))
