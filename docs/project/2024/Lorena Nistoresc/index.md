# Project Name

Smart Door Lock System 
Author: Nistorescu Maria-Lorena
GitHub Project Link*: (https://github.com/LolitaMary/SmartDoorLockSystem)

:::

## Description

The project is about a smart door lock system using Raspberry Pi Pico W. 
The door can be unlocked in 3 ways: RFID MODULE, SENSORS( knock knock_ knock knock ), WEB APPLICATION ( turn on/ off).
I intend to make a function that sends me back an e-mail which says how many times the door was unlocked in a day. Incredible, right?

## Motivation

Why did I choose this project?

Well, I went in a trip to Italy 2 months ago and I stayed to an airbnb that had a smart door lock system with password.

I knew about the project at school, and at that moment seemed very interesting to me the idea of a digital lock.
So I wanted to implement it and, maybe use it every day life.


## Architecture 

![SDOOR](https://github.com/UPB-FILS-MA/upb-fils-ma.github.io/assets/120091173/9e9a5397-384d-43d5-ad6c-95f6c4b8de39)

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

-Rapspberry Pi Pico W (micronctroller)
-RFID MODULE (for unlocking the door)
-TTPP223
-2N2222
-BUZZER 5V
-RELAY MODULE
-ELECTRIC DOOR LOCK


### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 

| [Device](link://to/device) | This is used ... | [price](link://to/store) |



-->

| Device | Usage | Price |
|--------|--------|-------|

| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Incuietoare electrica 12V](https://cleste.ro/incuietoare-electrica-12v.html) | [30,74 RON] 
| [Modul RFID RC522,similar cu MFRC522](https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=Modul+RFID+RC522+%28similar+cu+MFRC522%29&submit_search= )| [  9.99 RON ] |
| [Buzzer Pasiv de 5 V](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=Buzzer+Pasiv+de+5+V%09+%09&results=15)| [ 1,40 RON ] |
| [Fire Colorate Mamă-Mamă (40p, 30 cm)](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html?search_query=%09Fire+Colorate+Mama-Mama+%2840p%2C+30+cm%29+%09&results=10) | [ 6.98RON ] |
| [Fire Colorate Tată-Tată (40p, 15 cm)]( https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html?search_query=%09Fire+Colorate+Tata-Tata+%2840p%2C+15+cm%29&results=10) | [ 6.99 RON  ] |
|  [Rezistor 0.25W 22KΩ](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/858-rezistor-025w-18k.html?search_query=rezistor&results=120 ) | [  0,10 RON ] |
|  [Tranzistor NPN 2n2222 TO-92](https://www.optimusdigital.ro/ro/componente-electronice-tranzistoare/935-tranzistor-s9013-npn-50-pcs-set.html?search_query=Tranzistor+NPN+2n2222+TO-92&results=9) | [ 0,34 RON]
|  [Modul cu 4 Relee, Albastru]( https://www.optimusdigital.ro/ro/electronica-de-putere-module-cu-releu/478-modul-releu-cu-4-canale-albastru.html?search_query=Modul+cu+4+Relee%2C+Albastru&results=3) | [ 13,88RON]
|  [Breadboard HQ (830 Puncte)](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html?search_query=Breadboard+HQ+%28830+Puncte%29&results=12)|  [ 9.98 RON ] |
|  [Cablaj de test 50 x 70 mm](https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=Cablaj+de+test+50+x+70+mm&submit_search=) | [ 2,49 RON  ]|
|  [Buton cu o singura atingere, TTP223, Cu pini, 3 V, MulticoloR](https://www.emag.ro/buton-cu-o-singura-atingere-ttp223-cu-pini-3-v-multicolor-ttp223-mod-1ch/pd/DC6798MBM/)| IT IS USED FOR UNLOCKING THE DOOR| [18,33 RON] shipping more expensive than the component |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://www.youtube.com/watch?v=LfptYHFc6xU&t=609s)
2. [link](https://youtube.com/shorts/94idGywsd70?si=_esn9RnLSelw9FPq)
3. [link](https://www.youtube.com/watch?v=VP0qLUOdvuU)
...
