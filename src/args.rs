use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use philipshue::light::StateChange;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Login to the hue bridge.")]
    Login {
        #[arg(
            short,
            long,
            help = "Set the bridge url if discovering is not possible or doesn't work."
        )]
        bridge_url: Option<String>,
        #[arg(
            short,
            long,
            help = "Set the device type to any string. The bridge will associate your credentials with this string. If you don't set the device-type USERNAME@HOSTNAME will be used"
        )]
        device_type: Option<String>,
    },
    #[command(about = "Get all data including it's state from a light.")]
    GetLight {
        #[arg(help = "The light number or light name.")]
        light: String,
    },
    #[command(about = "List all lights, their light number and name.")]
    ListLights,
    #[command(
        about = "List the data from all lights. If you just want to list all light numbers and names use 'list-lights' instead."
    )]
    GetLights,
    #[command(about = "Change the state (like if the light is on or off) of a light.")]
    ChangeState {
        #[command(flatten)]
        change_state_args: ChangeStateArgs,
        #[arg(short, long)]
        state: Option<LightState>,
    },
    #[command(
        about = "Turn the light on and (re-)set the state for n seconds every transition-time, if set, otherwise 10 seconds."
    )]
    StayOn {
        #[command(flatten)]
        change_state_args: ChangeStateArgs,
        #[arg(short, long, help = "Stay on for n seconds.")]
        stay_on_for: u64,
    },
}

#[derive(ClapArgs, Clone)]
pub struct ChangeStateArgs {
    #[arg(help = "The light number or light name.")]
    pub light: String,
    #[arg(short, long)]
    pub brightness: Option<u8>,
    #[arg(long)]
    pub saturation: Option<u8>,
    #[arg(long)]
    pub hue: Option<u16>,
    #[arg(short, long, help = "Transition time in seconds.")]
    pub transition_time: Option<u16>,
}

#[derive(Clone, ValueEnum)]
pub enum LightState {
    On,
    Off,
}

impl From<ChangeStateArgs> for StateChange {
    fn from(value: ChangeStateArgs) -> Self {
        let mut state_change = StateChange::new();
        if let Some(brightness) = value.brightness {
            state_change = state_change.bri(brightness);
        }
        if let Some(saturation) = value.saturation {
            state_change = state_change.sat(saturation);
        }
        if let Some(light_hue) = value.hue {
            state_change = state_change.hue(light_hue);
        }
        if let Some(transition_time) = value.transition_time {
            state_change = state_change.transition_time(transition_time);
        }
        state_change
    }
}

impl From<LightState> for bool {
    fn from(val: LightState) -> Self {
        match val {
            LightState::On => true,
            LightState::Off => false,
        }
    }
}
