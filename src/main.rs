use std::time::{Duration, Instant};

use args::{Args, Commands};
use clap::Parser;
use environment::Environment;
use error::Error;
use philipshue::{
    light::{HueLight, StateChange},
    DeviceType, Discover, DiscoveryUrl, Hue, HueBridge, HueError,
};
use tokio::time::sleep;

mod args;
mod environment;
mod error;

/// `light` could be a light number or the name of the light
async fn get_light_by_light_number_or_name(
    hue: &Hue,
    light: &str,
) -> Result<(String, HueLight), Error> {
    if light.parse::<u16>().is_ok() {
        // light is the `light_number`
        Ok((light.to_string(), hue.get_light(light).await?))
    } else {
        let lights = hue.lights().await?;
        Ok(lights
            .into_iter()
            .find(|(_, hue_light)| hue_light.name.to_lowercase() == light.to_lowercase())
            .ok_or(Error::NoLightFound)?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut environment = Environment::new().await?;

    if let Commands::Login {
        bridge_url,
        device_type,
    } = args.command
    {
        // discover bridge or get bridge by set bridge url
        let bridge = match bridge_url {
            Some(url) => HueBridge::new(url),
            None => DiscoveryUrl::discover_one()
                .await
                .map_err(Error::HueBridgeDiscover)?
                .into(),
        };

        // login to bridge
        let device_type = match device_type {
            Some(d) => d,
            None => {
                format!("cli#{}", whoami::hostname())
            }
        };
        match bridge.login_to_hue(DeviceType::new(device_type)).await {
            Ok(hue) => Ok(environment.set_hue(hue).await?),
            Err(HueError::ApiError(api_error)) => {
                if api_error.error_type == 101 {
                    Err(Error::LinkButtonNotPressed)
                } else {
                    Err(HueError::ApiError(api_error))?
                }
            }
            Err(e) => Err(e)?,
        }?;
        println!("Login was successful, the login credentials were saved in your home directory.");
    } else if let Commands::GetLight { light } = args.command {
        let hue = environment.get_hue()?;
        let (light_number, light) = get_light_by_light_number_or_name(hue, &light).await?;
        println!("Light {light_number}:");
        println!("{light}");
    } else if matches!(args.command, Commands::GetLights)
        || matches!(args.command, Commands::ListLights)
    {
        // get or list lights
        let hue = environment.get_hue()?;
        let lights = hue.lights().await?;
        println!("Lights: ");
        for (light_number, light) in lights {
            if matches!(args.command, Commands::GetLights) {
                // print full light
                println!("Light {light_number}:");
                println!("{light}");
            } else {
                // Commands::ListLights
                // print just number and name
                println!("{light_number}: {}", light.name);
            }
        }
    } else if let Commands::ChangeState {
        change_state_args,
        state,
    } = args.command
    {
        let hue = environment.get_hue()?;
        let (light_number, _) =
            get_light_by_light_number_or_name(hue, &change_state_args.light).await?;
        let mut state_change: StateChange = change_state_args.into();
        // change_state_args does not include the state so we need to add it here
        if let Some(state) = state {
            state_change = state_change.on(state.into());
        }
        if state_change.is_empty() {
            println!("The state change is empty so you don't want to change anything about the state of the light. Done.");
        } else {
            hue.set_light_state(&light_number, state_change).await?;
            println!("Light state was set.");
        }
    } else if let Commands::StayOn {
        change_state_args,
        stay_on_for,
    } = args.command
    {
        let hue = environment.get_hue()?;
        let (light_number, _) =
            get_light_by_light_number_or_name(hue, &change_state_args.light).await?;
        let mut state_change: StateChange = change_state_args.clone().into();
        state_change = state_change.on(true); // turn the light on

        let stay_on_for_duration = Duration::from_secs(stay_on_for);
        // wait after each loop iteration for transition_time, if set, or otherwise 10 seconds
        let loop_interval =
            Duration::from_secs(change_state_args.transition_time.unwrap_or(100) as u64 / 10);
        let start = Instant::now();
        println!("Starting stay on loop for {stay_on_for} seconds.");
        loop {
            if Instant::now() - start > stay_on_for_duration {
                // the loop run for stay_on_for seconds, break
                break;
            }
            // set the light state
            hue.set_light_state(&light_number, state_change.clone())
                .await?;

            // wait for loop_interval
            sleep(loop_interval).await;
        }
    }

    Ok(())
}
