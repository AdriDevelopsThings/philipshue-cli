# philipshue-cli
A cli interface for the philipshue api

## Installation
Download the binary from the releasese page for your operating system or install it with cargo: 
```
cargo install --git https://github.com/adridevelopsthings/philipshue-cli
```

### Docker
It's also possible to run philipshue-cli with docker:

```
docker run --rm -v ./config:/config ghcr.io/adridevelopsthings/philipshue-cli philipshue-cli login --device-type testdevice
```

# Usage
```
Usage: philipshue-cli <COMMAND>

Commands:
  login         Login to the hue bridge.
  get-light     Get all data including it's state from a light.
  list-lights   List all lights, their light number and name.
  get-lights    List the data from all lights. If you just want to list all light numbers and names use 'list-lights' instead.
  change-state  Change the state (like if the light is on or off) of a light.
  stay-on       Turn the light on and (re-)set the state for n seconds every transition-time, if set, otherwise 10 seconds.
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## Login
```
Login to the hue bridge.

Usage: philipshue-cli login [OPTIONS] --device-type <DEVICE_TYPE>

Options:
  -b, --bridge-url <BRIDGE_URL>    Set the bridge url if discovering is not possible or doesn't work. (not required)
  -d, --device-type <DEVICE_TYPE>  Set the device type to any string. The bridge will associate your credentials with this string.
```

## Get light
```
Get all data including it's state from a light.

Usage: philipshue-cli get-light --light <LIGHT>

Options:
  -l, --light <LIGHT>  The light number or light name.
```

## List lights
```
List all lights, their light number and name.

Usage: philipshue-cli list-lights
```

## Get lights
```
List the data from all lights. If you just want to list all light numbers and names use 'list-lights' instead.

Usage: philipshue-cli get-lights
```

## Change light state
```
Change the state (like if the light is on or off) of a light.

Usage: philipshue-cli change-state [OPTIONS] --light <LIGHT>

Options:
  -l, --light <LIGHT>                      The light number or light name.
  -s, --state <STATE>                      [possible values: on, off]
  -b, --brightness <BRIGHTNESS>            
      --saturation <SATURATION>            
      --hue <HUE>                          
  -t, --transition-time <TRANSITION_TIME>  Transition time in seconds.
```

## Stay on
```
Turn the light on and (re-)set the state for n seconds every transition-time, if set, otherwise 10 seconds.

Usage: philipshue-cli stay-on [OPTIONS] --light <LIGHT> --stay-on-for <STAY_ON_FOR>

Options:
  -l, --light <LIGHT>                      The light number or light name.
  -b, --brightness <BRIGHTNESS>            
      --saturation <SATURATION>            
      --hue <HUE>                          
  -t, --transition-time <TRANSITION_TIME>  Transition time in seconds.
  -s, --stay-on-for <STAY_ON_FOR>          Stay on for n seconds.
```

# License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


at your option.