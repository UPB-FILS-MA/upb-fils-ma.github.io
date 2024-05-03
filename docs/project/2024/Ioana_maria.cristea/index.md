# Smart Fan
Displays room temp/pressure, auto-activates at thresholds.

:::info 

**Author**: Cristea Ioana-Maria \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-Ella23411

:::

## Description

A fan that monitors and displays the temperature and pressure in the room on a LCD screen, then activates the fan after a certain threshold :


+ The system utilizes a sensor to detect both temperature and pressure.

+ The LCD screen showcases the temperature and pressure readings.

+ The fan is triggered once the threshold is exceeded.

+ A buzzer is activated once the threshold is surpassed.

+ After the threshold is exceeded, the fan's arm oscillates from side to side.


## Motivation

**This project** holds a special significance for me because it allowed me to bridge the gap between theory and practice. Throughout my time in the laboratory, I gained valuable insights into the fundamentals of electronics and programming. However, there were certain aspects that I didn't have the chance to explore fully.

By undertaking the development of a smart fan, I saw an opportunity to not only apply the knowledge I acquired in the lab but also to expand my skill set in areas that were previously uncharted territory for me. Combining hardware with software was a challenge I eagerly embraced, and through this project, I gained invaluable hands-on experience in integrating the two seamlessly.

## Architecture 
The **Raspberry Pi Pico** serves as the central control unit, directing and managing all other components utilized within the project.


The **LCD** serves as an interface for showcasing the sensor readings, it is connected to the Pico through I2C

The **servomotor** is connected to an improvised arm, allowing the fan to rotate 90° in either direction.

The **buzzer**, when the temperature threshold is surpassed, emits an  alarm, acting as an alert mechanism.

The **pressure and temperature** sensor detects changes in temperature and pressure, supplying input to the Pico.

A **DC Motors** that controls the fan

![diagram](proiectma.jpg)



## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

Detail in a few words the hardware used.

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
| [BMP280](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) | Temperature Sensor| [8,49 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-presiune/1666-modul-senzor-de-presiune-barometric-bmp280.html?search_query=bmp280&results=11) |
| Active Buzzer | Active Buzzer | [1,49 RON](https://www.optimusdigital.ro/ro/audio-buzzere/635-buzzer-activ-de-3-v.html?search_query=buzzer+activ&results=18) |
| [LCD Display](https://www.waveshare.com/datasheet/LCD_en_PDF/LCD1602.pdf) | LCD Display| [30 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/62-lcd-1602-cu-interfata-i2c-si-backlight-galben-verde.html?search_query=lcd+1602&results=17) |
| DC Motor | DC Motor | [3,50 RON](https://ardushop.ro/ro/motoare-si-drivere/437-motoras-curent-continuu.html?gad_source=1&gclid=Cj0KCQjwltKxBhDMARIsAG8KnqXmbO_Pw93I5tCpBIvBfZewX5Zezb63wDD63KWlIPhqzm3f4ITGaoYaAr0nEALw_wcB) |
| [Servomotor](https://datasheetspdf.com/datasheet/SG90.html) | Servomotor | [14 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html?search_query=servomotor&results=119) |




## Software

| Library | Description | Usage |
|---------|-------------|-------|


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://www.youtube.com/watch?v=8kfROSQYDWI)
2. [link](https://www.youtube.com/watch?v=OdqcJoFi_Bk)

