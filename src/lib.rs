mod characteristics;
use characteristics::characteristics::{Mach, Reynolds};
mod gas;
use gas::gas::Gas;
mod liquid;
use liquid::liquid::Liquid;
mod solid;
use solid::solid::Solid;

pub struct Air{
    density: f64,
    isentropic_expansion_factor: f64
}

impl Default for Air{
    fn default()->Self{
        Air{
            density: 1.225,
            isentropic_expansion_factor: 1.4
        }
    }
}

impl Air{
    pub fn new(density: f64, isentropic_expansion_factor: f64)->Self{
        Air{
            density: density,
            isentropic_expansion_factor: isentropic_expansion_factor
        }
    }
        
    pub fn characteristic_velocity(&self)->f64{
        let characteristic_velocity = isentropic_expansion_factor / density;
        return characteristic_velocity 
    }
}

impl Gas for Air{

}