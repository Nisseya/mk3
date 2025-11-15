# Rust ESP Project: Connected thermometer 

## Why: 

I have to monitor some places that need to have specific temperature / humidity on a budget (<5€)

-> To grow plants inside,
-> To hatch eggs
Etc

Also wanted to practice a bit Rust, since my current projects are on Python and I can't migrate them because it's group projects.

## Hardware:

- ESP32c3mini motherboard (found on AliExpress)
- DHT11 Sensor (found on AliExpress)

How to plug DHT11 to ESP32c3Mini:
- (-) on 3.3V (or reversed I don't remember)
- (+) on G
- Middle on GPIO4
 
## Features:

Connect to Wifi

Read Sensor Values 

Send them to a remote server

For now it's on my own server, but if not done yet, it will be a .env with a .env.sample file


## How to use:

1. Flash the board:

```bash
cargo espflash flash --monitor --port /dev/ttyACM0 --flash-size 4mb   --partition-table partitions.csv
```

Don't forget to change the port

### Setup port on WSL2 (just in case)

[Use usbipd from windows host (Pwshell Admin)](https://learn.microsoft.com/en-us/windows/wsl/connect-usb#attach-a-usb-device):


```bash 
usbipd list #to see the ports
usbipd bind --busid <port>
usbipd attach --wsl --busid <port>
```

You'll have to reattach each time you unplug the cable 