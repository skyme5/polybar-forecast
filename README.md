# Weather forecast for Polybar
![preview](https://github.com/kamek-pf/polybar-forecast/blob/master/screenshots/preview.png)

This is a simple weather forecast module for Polybar. You will need Weather Icons and/or Material Icons for this to work properly. Both are available in the AUR:

- [Weather Icons](https://aur.archlinux.org/packages/ttf-weather-icons/)
- [Material Icons](https://aur.archlinux.org/packages/ttf-material-icons/)

## Usage

Following command-line options are available

```text
 -k, --api-key             Your unique openweathermap.org API key.
 -c, --city-id             City ID.
 -u, --units               Units of measurement. standard, metric and imperial
                           units are available. If you do not use the units
                           parameter, standard units will be applied by default.
 -l, --language            You can use this parameter to get the output in your
                           language. (Default: en.)
 -f, --enable-forcast      Include forcast in result.
```

To find city id read [this](https://www.dmopress.com/openweathermap-howto/#:~:text=Get%20your%20OpenWeatherMap%20City%20ID).

## Build

```
cargo build --release
```

then you'll find the binary `target/release/polybar-forecast`.

## Polybar integration
You can define your new module like this :

```
[module/weather]
type = custom/script
exec = ~/polybar-forecast --api-key=API_KEY --city-id=CITY_ID --units=metric
exec-if = ping openweathermap.org -c 1

;label = %output:0:15:...%
label = %output%
interval = 10
label-font = 2
```

Don't forget to add Weather Icons to your config or it won't render correctly :
```
font-1 = Weather Icons:size=12;0
```

You can change line height in `font-1` to adjust vertical align of text.
