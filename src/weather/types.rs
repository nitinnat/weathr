use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherCondition {
    Clear,
    PartlyCloudy,
    Cloudy,
    Overcast,
    Fog,
    Drizzle,
    Rain,
    FreezingRain,
    Snow,
    SnowGrains,
    RainShowers,
    SnowShowers,
    Thunderstorm,
    ThunderstormHail,
    Hail,
    Flood,
    Tsunami,
    Volcano,
    Godzilla,
    Meteor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RainIntensity {
    Drizzle,
    Light,
    Heavy,
    Storm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnowIntensity {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FogIntensity {
    Light,
    Medium,
    Heavy,
}

impl WeatherCondition {
    pub fn rain_intensity(&self) -> RainIntensity {
        match self {
            Self::Drizzle => RainIntensity::Drizzle,
            Self::Rain | Self::RainShowers => RainIntensity::Light,
            Self::FreezingRain => RainIntensity::Heavy,
            Self::Thunderstorm => RainIntensity::Heavy,
            Self::ThunderstormHail => RainIntensity::Storm,
            _ => RainIntensity::Light,
        }
    }

    pub fn snow_intensity(&self) -> SnowIntensity {
        match self {
            Self::SnowGrains => SnowIntensity::Light,
            Self::SnowShowers => SnowIntensity::Medium,
            Self::Snow => SnowIntensity::Heavy,
            _ => SnowIntensity::Light,
        }
    }

    pub fn fog_intensity(&self) -> FogIntensity {
        match self {
            Self::Fog => FogIntensity::Medium,
            _ => FogIntensity::Light,
        }
    }

    pub fn is_raining(&self) -> bool {
        matches!(
            self,
            Self::Drizzle
                | Self::Rain
                | Self::RainShowers
                | Self::FreezingRain
                | Self::Thunderstorm
                | Self::ThunderstormHail
        )
    }

    pub fn is_snowing(&self) -> bool {
        matches!(self, Self::Snow | Self::SnowGrains | Self::SnowShowers)
    }

    pub fn is_thunderstorm(&self) -> bool {
        matches!(self, Self::Thunderstorm | Self::ThunderstormHail)
    }

    pub fn is_cloudy(&self) -> bool {
        matches!(self, Self::PartlyCloudy | Self::Cloudy | Self::Overcast)
    }

    pub fn is_foggy(&self) -> bool {
        matches!(self, Self::Fog)
    }

    pub fn is_hail(&self) -> bool {
        matches!(self, Self::Hail)
    }

    pub fn is_flood(&self) -> bool {
        matches!(self, Self::Flood)
    }

    pub fn is_tsunami(&self) -> bool {
        matches!(self, Self::Tsunami)
    }

    pub fn is_volcano(&self) -> bool {
        matches!(self, Self::Volcano)
    }

    pub fn is_godzilla(&self) -> bool {
        matches!(self, Self::Godzilla)
    }

    pub fn is_meteor(&self) -> bool {
        matches!(self, Self::Meteor)
    }

    pub fn is_disaster(&self) -> bool {
        matches!(
            self,
            Self::Hail
                | Self::Flood
                | Self::Tsunami
                | Self::Volcano
                | Self::Godzilla
                | Self::Meteor
        )
    }
}

impl std::str::FromStr for WeatherCondition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "clear" => Ok(Self::Clear),
            "partly_cloudy" | "partly-cloudy" => Ok(Self::PartlyCloudy),
            "cloudy" => Ok(Self::Cloudy),
            "overcast" => Ok(Self::Overcast),
            "fog" => Ok(Self::Fog),
            "drizzle" => Ok(Self::Drizzle),
            "rain" => Ok(Self::Rain),
            "freezing_rain" | "freezing-rain" => Ok(Self::FreezingRain),
            "snow" => Ok(Self::Snow),
            "snow_grains" | "snow-grains" => Ok(Self::SnowGrains),
            "rain_showers" | "rain-showers" => Ok(Self::RainShowers),
            "snow_showers" | "snow-showers" => Ok(Self::SnowShowers),
            "thunderstorm" => Ok(Self::Thunderstorm),
            "thunderstorm_hail" | "thunderstorm-hail" => Ok(Self::ThunderstormHail),
            "hail" => Ok(Self::Hail),
            "flood" => Ok(Self::Flood),
            "tsunami" => Ok(Self::Tsunami),
            "volcano" => Ok(Self::Volcano),
            "godzilla" => Ok(Self::Godzilla),
            "meteor" | "meteor-impact" | "meteor_impact" => Ok(Self::Meteor),
            _ => Err(format!(
                "Unknown weather condition: '{}'. Valid options: clear, partly_cloudy, cloudy, overcast, fog, drizzle, rain, freezing_rain, snow, snow_grains, rain_showers, snow_showers, thunderstorm, thunderstorm_hail, hail, flood, tsunami, volcano, godzilla, meteor",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindSpeedUnit {
    Kmh,
    Ms,
    Mph,
    Kn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecipitationUnit {
    Mm,
    Inch,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct WeatherData {
    pub condition: WeatherCondition,
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub humidity: f64,
    pub precipitation: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub cloud_cover: f64,
    pub pressure: f64,
    pub visibility: Option<f64>,
    pub is_day: bool,
    pub moon_phase: Option<f64>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(default)]
pub struct WeatherUnits {
    pub temperature: TemperatureUnit,
    pub wind_speed: WindSpeedUnit,
    pub precipitation: PrecipitationUnit,
}

impl WeatherUnits {
    pub fn imperial() -> Self {
        Self {
            temperature: TemperatureUnit::Fahrenheit,
            wind_speed: WindSpeedUnit::Mph,
            precipitation: PrecipitationUnit::Inch,
        }
    }

    pub fn metric() -> Self {
        Self::default()
    }
}

impl Default for WeatherUnits {
    fn default() -> Self {
        Self {
            temperature: TemperatureUnit::Celsius,
            wind_speed: WindSpeedUnit::Kmh,
            precipitation: PrecipitationUnit::Mm,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct WeatherLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeatherConditions {
    pub is_raining: bool,
    pub is_snowing: bool,
    pub is_thunderstorm: bool,
    pub is_cloudy: bool,
    pub is_foggy: bool,
    pub is_day: bool,
    pub is_hail: bool,
    pub is_flood: bool,
    pub is_tsunami: bool,
    pub is_volcano: bool,
    pub is_godzilla: bool,
    pub is_meteor: bool,
    pub is_disaster: bool,
}

impl Default for WeatherConditions {
    fn default() -> Self {
        Self {
            is_raining: false,
            is_snowing: false,
            is_thunderstorm: false,
            is_cloudy: false,
            is_foggy: false,
            is_day: true,
            is_hail: false,
            is_flood: false,
            is_tsunami: false,
            is_volcano: false,
            is_godzilla: false,
            is_meteor: false,
            is_disaster: false,
        }
    }
}
