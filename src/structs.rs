use bevy::prelude::*;
#[derive(Debug)]
enum Data {
    FuelRod(FuelRod),
    ControlRod(ControlRod),
    StartupNeutronSource(StartupNeutronSource),
    ShortControlRod(ShortControlRod),
    AutomaticControlRod(AutomaticControlRod),
}
#[derive(Debug, Component)]
pub(crate) struct Rod {
    number: u32,
    position: Vec2,
    rod_type: String,
    data: Data

}
impl Rod {
    pub(crate) fn new(position: Vec2, number: u32, string: String) -> Rod {
        let data = if string == "control rod" {
            Data::ControlRod(ControlRod::default())
        } else if string == "startup neutron source" {
            Data::StartupNeutronSource(StartupNeutronSource::default())
        } else if string == "short control rod" {
            Data::ShortControlRod(ShortControlRod::default())
        } else if string == "automatic control rod" {
            Data::AutomaticControlRod(AutomaticControlRod::default())
        } else {
            Data::FuelRod(FuelRod::default())
        };
        Rod {
            position,
            number,
            rod_type: string,
            data,
        }
    }
}

#[derive(Debug, Default)]
struct StartupNeutronSource {
    neutron_rate: f32,
    source: MaterialData,
    activity_level: f32,
    operating: bool,
    energy_spectrum: String, // change to enum
    lifetime: f32,
    date: f32,
}

#[derive(Debug, Default)]
struct ControlRod {
    control_rod_data: ControlRodData,
}

#[derive(Debug, Default)]
struct ShortControlRod {
    control_rod_data: ControlRodData,
}

#[derive(Debug, Default)]
struct ControlRodData {
    insertion_depth: f32,
    insertion_rate: f32,
    set_point: f32,
    absorption_rate: f32,
    is_manual_control: bool, // manuel or emergency
    material_data: MaterialData,
    date: f32,
    wear_level: f32,
    emergency_position: Option<f32>
    

}
#[derive(Debug, Default)]
struct AutomaticControlRod {
    control_rod_data: ControlRodData,
    response_time: f32,
}

#[derive(Debug, Default)]
struct FuelRod {
    internal_pressure: f32,
    internal_temperature: f32,
    burnup: f32,
    coolant_flow_rate: f32,
    material_data: MaterialData,
}
#[derive(Debug, Default)]
struct StackData {
    stack_length: f32,
    stack_diameter: f32, // cladding + fuel rod diameter
    inner_diameter: f32, // fuel rod diameter
    temperature: f32,
    stack_material: MaterialData,
    cladding: Cladding,
}

#[derive(Debug, Default)]
struct MaterialData {
    composition: String, // change to enum
    density: f32,
    melting_point: f32,
    thermal_conductivity: f32,

}

#[derive(Debug, Default)] 
struct Cladding {
    material_data: MaterialData,
    wear_level: f32,
    corrosion_resistance: f32,
}

