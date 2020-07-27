// this file is auto-generated by build.rs

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		air_quality::AirQualityCharacteristic,
		status_active::StatusActiveCharacteristic,
		status_fault::StatusFaultCharacteristic,
		status_tampered::StatusTamperedCharacteristic,
		status_low_battery::StatusLowBatteryCharacteristic,
		name::NameCharacteristic,
		ozone_density::OzoneDensityCharacteristic,
		nitrogen_dioxide_density::NitrogenDioxideDensityCharacteristic,
		sulphur_dioxide_density::SulphurDioxideDensityCharacteristic,
		pm2_5_density::Pm2_5DensityCharacteristic,
		pm10_density::Pm10DensityCharacteristic,
		voc_density::VocDensityCharacteristic,
		carbon_monoxide_level::CarbonMonoxideLevelCharacteristic,
		carbon_dioxide_level::CarbonDioxideLevelCharacteristic,
	},
    HapType,
};

/// Air Quality Sensor Service.
#[derive(Debug, Default)]
pub struct AirQualitySensorService {
    /// ID of the Air Quality Sensor Service.
    id: u64,
    /// `HapType` of the Air Quality Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Air Quality Characteristic (required).
	pub air_quality: AirQualityCharacteristic,

	/// Status Active Characteristic (optional).
	pub status_active: Option<StatusActiveCharacteristic>,
	/// Status Fault Characteristic (optional).
	pub status_fault: Option<StatusFaultCharacteristic>,
	/// Status Tampered Characteristic (optional).
	pub status_tampered: Option<StatusTamperedCharacteristic>,
	/// Status Low Battery Characteristic (optional).
	pub status_low_battery: Option<StatusLowBatteryCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Ozone Density Characteristic (optional).
	pub ozone_density: Option<OzoneDensityCharacteristic>,
	/// Nitrogen Dioxide Density Characteristic (optional).
	pub nitrogen_dioxide_density: Option<NitrogenDioxideDensityCharacteristic>,
	/// Sulphur Dioxide Density Characteristic (optional).
	pub sulphur_dioxide_density: Option<SulphurDioxideDensityCharacteristic>,
	/// PM2.5 Density Characteristic (optional).
	pub pm2_5_density: Option<Pm2_5DensityCharacteristic>,
	/// PM10 Density Characteristic (optional).
	pub pm10_density: Option<Pm10DensityCharacteristic>,
	/// VOC Density Characteristic (optional).
	pub voc_density: Option<VocDensityCharacteristic>,
	/// Carbon Monoxide Level Characteristic (optional).
	pub carbon_monoxide_level: Option<CarbonMonoxideLevelCharacteristic>,
	/// Carbon Dioxide Level Characteristic (optional).
	pub carbon_dioxide_level: Option<CarbonDioxideLevelCharacteristic>,
}

impl AirQualitySensorService {
    /// Creates a new Air Quality Sensor Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::AirQualitySensor,
			air_quality: AirQualityCharacteristic::new(id + 1 + 0, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for AirQualitySensorService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.air_quality,
		];
		if let Some(c) = &self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_low_battery {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.ozone_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.nitrogen_dioxide_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.sulphur_dioxide_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.pm2_5_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.pm10_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.voc_density {
		    characteristics.push(c);
		}
		if let Some(c) = &self.carbon_monoxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.carbon_dioxide_level {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.air_quality,
		];
		if let Some(c) = &mut self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_low_battery {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.ozone_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.nitrogen_dioxide_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.sulphur_dioxide_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.pm2_5_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.pm10_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.voc_density {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.carbon_monoxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.carbon_dioxide_level {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for AirQualitySensorService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}