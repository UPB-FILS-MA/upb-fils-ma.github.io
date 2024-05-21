GreenGuardian: SmartPlantCare System
General description and use-cases
The main purpose of our system is to track soil moisture and ligh for our plants. Depending on on different conditions we will switch ON or OFF fan, water pipe and additiona lights.

Use-case-1: If the average moisture during last 1 minute is less or equal to 50%, we treat this condition as DRY ground and switching ON water pipe for 15 seconds. We indicate this condition by switching ON YELLOW led.

Use-case-2: If average moisture is betwen 51% and 75% we treat this condition as NORMAL and do nothing. We indicate this condition by switching ON GREEN led.

Use-case-3: If average moisture is greater than 75% we treat this condition as WET and swithing ON fan to make groupd drier. We indicate this condition by switching ON RED led.

Use-case-4: Every 10 second we will measure immediate ground humidity and display using LCD display.

Use-case-5: If light sensor detects that light is swicthed OFF for 30 seconds, we will enable additional light using LED lamps.

Peripherials description
1.We use LIGHT sensor attached to a single pin as a digital Input sensor to detected if light is switched on or off.
2.We use 3 LED sensors(yellow, green, red) attached to Output pins to signal different soil moisture conditions (dry, normal, wet).
3.We use moisture sensor attached to a pin with analog-digital converter to read immediate RAW soil moisture value. Later this value recalculated to percentage using calibrated values.
4.We use LCD display with 16 characters and 2 rows. The LCD display attached to appropriate pins (rs, e, d4, d5, d6, d7). We do not use I2C but send signals directly to LCD pins to display characters as needed. LCD will display immediate humidity and also information related to watering pipe and fan.
5.We use water pipe attached to a pin that allow us increase ground humidity if needed.
6.We use fan attached to an approriate pin to decrease ground humidity and make it more dry if needed.