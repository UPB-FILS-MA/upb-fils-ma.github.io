# WeatherSmart Clock
Smart Weather Clock with Alarm and Temperature Sensor

:::info 

**Author**: Nica Alexia-Stefania \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-alexianicaa

:::

## Description

I'll design a user-friendly alarm clock with a real-time clock (RTC) module, LCD display, push buttons, and a buzzer. Additionally, I'll integrate smart weather features including customizable alarms, a temperature sensor, and weather indication. Users can set multiple alarms and will be alerted by a buzzer when the set alarm time arrives.

## Motivation

The motivation behind this project is to create a multifunctional clock that not only provides accurate timekeeping and customizable alarms but also offers real-time weather information in a fun and intuitive way. I chose this project because the clock is essential in our lives; whether you need to go to work, attend an appointment, or manage your daily tasks, knowing the time is crucial. Additionally, the weather function helps a lot when you want to know how to dress yourself.

## Architecture 
![Architecture diagram](./architecture_diagram.png)

*Main Components used in the WeatherSmart Clock project:*

  **1-Rasberry Pi Pico:** Microcontroller unit responsible for managing all operations.
  
  **2-Temperature Sensor:** Monitors ambient temperature and communicates with the Rasberry Pi Pico
  
  **3-LCD Display:** Shows the current temperature in Celcius.
  
  **4-Buzzer:** Acts as the audible alarm.
  
  **5-RTC Module:** Real time clock module used for accurate timekeeping

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

In my project, the Raspberry Pi Pico microcontroller serves as the central processing unit, mandated for the task. Its low power consumption ensures efficiency throughout. In this project, it reads data from the DHT11 temperature sensor and the time from the DS3231 real time clock module, and displays them, while also being able to send electrical signals to a buzzer for an alarm effect.
  
### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Buzzer Pasiv de 5 V](https://components101.com/misc/buzzer-pinout-working-datasheet) |Passive buzzer | [1.40 RON](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=buzzer+pasiv&results=15)|
| [LCD Display](https://circuitdigest.com/article/16x2-lcd-display-module-pinout-datasheet)|Display|[9.83 RON](https://ardushop.ro/ro/electronica/36-lcd-1602.html?search_query=lcd+display&results=138)|
| [Push Button](https://components101.com/switches/push-button)|Push button|[0.63 RON](https://ardushop.ro/ro/home/97-buton-mic-push-button-trough-hole.html?search_query=buton&results=157)|
| [Temperature sensor](https://www.mouser.com/datasheet/2/758/DHT11-Technical-Data-Sheet-Translated-Version-1143054.pdf)|Temperature sensor|[14.30 RON](https://ardushop.ro/ro/home/121-modul-senzor-temperatura-i-umiditate-digital-dht11.html?search_query=dht11&results=3)|
| [RTC Module](https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231.pdf)|Real time clock module|[23.56 RON](https://ardushop.ro/ro/electronica/231-modul-rtc-de-precizie-ds3231-i2c.html?search_query=ds3231&results=1)|
| Breadboard|Breadboard|[10.14 RON](https://ardushop.ro/ro/electronica/33-breadboard-830.html?search_query=breadboard&results=31)|

## Software

| Library | Description | Usage |
|--------|--------|-------|
|[DHT11](https://docs.rs/dht11/latest/dht11/)|DHT11 rust library|Used to interact with DHT11 temperature sensor|
|[DS323x](https://lib.rs/crates/ds323x)|DS3231 rust library|Used to interact with DS3231 RTC module|
|[Embassy_rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html)|Peripheral access library|Used for accessing the peripherals|


## Links
1.[PM projects 2023](https://ocw.cs.pub.ro/courses/pm/prj2023) \
2.[Raspberry Pi Pico Project - Thermometer & Clock](https://www.youtube.com/watch?v=gBofy7MMdIY)
