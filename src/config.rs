use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use fimg::Image;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The satellite to source imagery from.
    /// 
    /// Options include:
    /// 
    /// - auto (default; picks a satellite from the system time zone)
    /// 
    /// - GOES East (covers most of North and South America)
    /// 
    /// - GOES West (Pacific Ocean and parts of the western US)
    /// 
    /// - Himawari (Oceania and East Asia)
    /// 
    /// - Meteosat 9 (Africa, Middle East, India, Central Asia)
    /// 
    /// - Meteosat 10 (Atlantic Ocean, Africa, Europe)
    #[arg(short, long, env = "SATPAPER_SATELLITE", default_value = "auto")]
    pub satellite: Satellite,
    /// The X resolution/width of the generated wallpaper.
    #[arg(short = 'x', long, env = "SATPAPER_RESOLUTION_X")]
    pub resolution_x: u32,
    /// The Y resolution/height of the generated wallpaper.
    #[arg(short = 'y', long, env = "SATPAPER_RESOLUTION_Y")]
    pub resolution_y: u32,
    /// The size of the "disk" (Earth) relative to the generated wallpaper's
    /// smaller dimension.
    /// 
    /// Values in the 90-95 range are the best if you want maximum detail.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=100), env = "SATPAPER_DISK_SIZE")]
    pub disk_size: u32,
    /// Where generated wallpapers should be saved.
    /// 
    /// Satpaper will output to a file called "satpaper_latest.png" at this path.
    #[arg(short, long, env = "SATPAPER_TARGET_PATH")]
    pub target_path: PathBuf,
    /// Command to run to change the wallpaper. This overrides automatic update handling.
    /// 
    /// The command will be ran as `sh -c "{wallpaper_command} file://{path}"`. 
    #[arg(short, long, env = "SATPAPER_WALLPAPER_COMMAND")]
    pub wallpaper_command: Option<String>,
    /// Whether or not to only run once.
    /// 
    /// By default, Satpaper is designed to run in the background - it stays resident once launched
    /// and periodically attempts to update your wallpaper.
    /// 
    /// With --once set, Satpaper will instead generate one wallpaper and terminate, without
    /// affecting your existing wallpaper or staying resident.
    /// 
    /// This is ideal if you want to use Satpaper as a simple wallpaper generator or as part of a larger script/program.
    #[arg(short, long, env = "SATPAPER_ONCE", default_value_t = false)]
    pub once: bool,
    /// A background image to use instead of the default pure black.
    /// 
    /// For best results, the image should match the specified resolution, 
    /// but Satpaper will resize the image to fit if need be.
    #[arg(short, long, env = "SATPAPER_BACKGROUND_IMAGE")]
    pub background_image: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Satellite {
    /// Automatically pick a satellite from the system time zone.
    Auto,
    GOESEast,
    GOESWest,
    Himawari,
    Meteosat9,
    Meteosat10,
}

/// Maps an IANA time zone prefix to the satellite with the best coverage.
///
/// The longest matching prefix wins, so a specific override such as
/// `America/Los_Angeles` takes precedence over its region default `America/`.
/// Add entries here to teach Satpaper about more regions. Time zones that do
/// not match anything fall back to GOES East.
const TZ_MAP: &[(&str, Satellite)] = &[
    // Europe, Africa and the (eastern) Atlantic -> Meteosat 10.
    ("Europe/", Satellite::Meteosat10),
    ("Africa/", Satellite::Meteosat10),
    ("Atlantic/", Satellite::Meteosat10),
    ("Antarctica/", Satellite::Meteosat10),
    // Middle East, India, the Indian Ocean and Central Asia -> Meteosat 9.
    ("Indian/", Satellite::Meteosat9),
    ("Asia/Kabul", Satellite::Meteosat9),
    ("Asia/Karachi", Satellite::Meteosat9),
    ("Asia/Kolkata", Satellite::Meteosat9),
    ("Asia/Calcutta", Satellite::Meteosat9),
    ("Asia/Colombo", Satellite::Meteosat9),
    ("Asia/Kathmandu", Satellite::Meteosat9),
    ("Asia/Dhaka", Satellite::Meteosat9),
    ("Asia/Dubai", Satellite::Meteosat9),
    ("Asia/Muscat", Satellite::Meteosat9),
    ("Asia/Qatar", Satellite::Meteosat9),
    ("Asia/Riyadh", Satellite::Meteosat9),
    ("Asia/Kuwait", Satellite::Meteosat9),
    ("Asia/Bahrain", Satellite::Meteosat9),
    ("Asia/Baghdad", Satellite::Meteosat9),
    ("Asia/Tehran", Satellite::Meteosat9),
    ("Asia/Jerusalem", Satellite::Meteosat9),
    ("Asia/Tel_Aviv", Satellite::Meteosat9),
    ("Asia/Beirut", Satellite::Meteosat9),
    ("Asia/Damascus", Satellite::Meteosat9),
    ("Asia/Amman", Satellite::Meteosat9),
    ("Asia/Yerevan", Satellite::Meteosat9),
    ("Asia/Baku", Satellite::Meteosat9),
    ("Asia/Tbilisi", Satellite::Meteosat9),
    ("Asia/Tashkent", Satellite::Meteosat9),
    ("Asia/Ashgabat", Satellite::Meteosat9),
    ("Asia/Dushanbe", Satellite::Meteosat9),
    ("Asia/Bishkek", Satellite::Meteosat9),
    ("Asia/Almaty", Satellite::Meteosat9),
    ("Asia/Qyzylorda", Satellite::Meteosat9),
    ("Asia/Qostanay", Satellite::Meteosat9),
    ("Asia/Aqtobe", Satellite::Meteosat9),
    ("Asia/Atyrau", Satellite::Meteosat9),
    ("Asia/Aqtau", Satellite::Meteosat9),
    ("Asia/Oral", Satellite::Meteosat9),
    ("Asia/Urumqi", Satellite::Meteosat9),
    ("Asia/Yekaterinburg", Satellite::Meteosat9),
    ("Asia/Omsk", Satellite::Meteosat9),
    ("Asia/Novosibirsk", Satellite::Meteosat9),
    ("Asia/Barnaul", Satellite::Meteosat9),
    ("Asia/Tomsk", Satellite::Meteosat9),
    ("Asia/Novokuznetsk", Satellite::Meteosat9),
    ("Asia/Krasnoyarsk", Satellite::Meteosat9),
    // East Asia, the Russian Far East and Oceania -> Himawari.
    ("Australia/", Satellite::Himawari),
    ("Antarctica/McMurdo", Satellite::Himawari),
    ("Asia/Tokyo", Satellite::Himawari),
    ("Asia/Seoul", Satellite::Himawari),
    ("Asia/Pyongyang", Satellite::Himawari),
    ("Asia/Shanghai", Satellite::Himawari),
    ("Asia/Chongqing", Satellite::Himawari),
    ("Asia/Harbin", Satellite::Himawari),
    ("Asia/Hong_Kong", Satellite::Himawari),
    ("Asia/Macau", Satellite::Himawari),
    ("Asia/Taipei", Satellite::Himawari),
    ("Asia/Manila", Satellite::Himawari),
    ("Asia/Singapore", Satellite::Himawari),
    ("Asia/Kuala_Lumpur", Satellite::Himawari),
    ("Asia/Kuching", Satellite::Himawari),
    ("Asia/Brunei", Satellite::Himawari),
    ("Asia/Jakarta", Satellite::Himawari),
    ("Asia/Pontianak", Satellite::Himawari),
    ("Asia/Makassar", Satellite::Himawari),
    ("Asia/Jayapura", Satellite::Himawari),
    ("Asia/Ulaanbaatar", Satellite::Himawari),
    ("Asia/Choibalsan", Satellite::Himawari),
    ("Asia/Hovd", Satellite::Himawari),
    ("Asia/Irkutsk", Satellite::Himawari),
    ("Asia/Chita", Satellite::Himawari),
    ("Asia/Yakutsk", Satellite::Himawari),
    ("Asia/Khandyga", Satellite::Himawari),
    ("Asia/Vladivostok", Satellite::Himawari),
    ("Asia/Ust-Nera", Satellite::Himawari),
    ("Asia/Sakhalin", Satellite::Himawari),
    ("Asia/Magadan", Satellite::Himawari),
    ("Asia/Srednekolymsk", Satellite::Himawari),
    ("Asia/Kamchatka", Satellite::Himawari),
    ("Asia/Anadyr", Satellite::Himawari),
    // The Americas -> GOES East, with the far west/Pacific on GOES West.
    ("America/", Satellite::GOESEast),
    ("US/", Satellite::GOESEast),
    ("Canada/", Satellite::GOESEast),
    ("Brazil/", Satellite::GOESEast),
    ("Chile/", Satellite::GOESEast),
    ("Mexico/", Satellite::GOESEast),
    ("America/Los_Angeles", Satellite::GOESWest),
    ("America/Vancouver", Satellite::GOESWest),
    ("America/Whitehorse", Satellite::GOESWest),
    ("America/Dawson", Satellite::GOESWest),
    ("America/Inuvik", Satellite::GOESWest),
    ("America/Boise", Satellite::GOESWest),
    ("America/Juneau", Satellite::GOESWest),
    ("America/Sitka", Satellite::GOESWest),
    ("America/Metlakatla", Satellite::GOESWest),
    ("America/Yakutat", Satellite::GOESWest),
    ("America/Anchorage", Satellite::GOESWest),
    ("America/Nome", Satellite::GOESWest),
    ("America/Adak", Satellite::GOESWest),
    ("America/Tijuana", Satellite::GOESWest),
    ("US/Pacific", Satellite::GOESWest),
    ("US/Alaska", Satellite::GOESWest),
    ("US/Aleutian", Satellite::GOESWest),
    ("US/Hawaii", Satellite::GOESWest),
    ("Canada/Pacific", Satellite::GOESWest),
    ("Canada/Yukon", Satellite::GOESWest),
    ("Canada/Mountain", Satellite::GOESWest),
    ("Mexico/BajaNorte", Satellite::GOESWest),
    // The eastern Pacific -> GOES West; the rest of Oceania -> Himawari.
    ("Pacific/", Satellite::Himawari),
    ("Pacific/Honolulu", Satellite::GOESWest),
    ("Pacific/Midway", Satellite::GOESWest),
    ("Pacific/Pago_Pago", Satellite::GOESWest),
    ("Pacific/Tahiti", Satellite::GOESWest),
    ("Pacific/Marquesas", Satellite::GOESWest),
    ("Pacific/Gambier", Satellite::GOESWest),
    ("Pacific/Easter", Satellite::GOESWest),
    ("Pacific/Pitcairn", Satellite::GOESWest),
    ("Pacific/Niue", Satellite::GOESWest),
    ("Pacific/Rarotonga", Satellite::GOESWest),
];

impl Satellite {
    /// Resolve [`Satellite::Auto`] into a concrete satellite. Every other
    /// value is returned unchanged.
    pub fn resolved(self) -> Self {
        match self {
            Self::Auto => Self::from_system_timezone(),
            concrete => concrete,
        }
    }

    /// Pick a satellite based on the machine's current time zone.
    ///
    /// This is what [`Satellite::Auto`] resolves to. Falls back to GOES East
    /// (the widest single-disk view) when the time zone cannot be determined.
    pub fn from_system_timezone() -> Self {
        match system_time_zone() {
            Some(tz) => Self::from_timezone(&tz),
            None => {
                log::warn!("Could not determine the system time zone; defaulting to GOES East");
                Self::GOESEast
            }
        }
    }

    /// Map an IANA time zone name (e.g. `"Europe/Berlin"`) to the satellite
    /// with the best coverage for that region.
    ///
    /// The longest matching prefix in [`TZ_MAP`] wins, so a specific override
    /// takes precedence over its broader region default.
    pub fn from_timezone(timezone: &str) -> Self {
        let timezone = timezone.trim();

        TZ_MAP
            .iter()
            .filter(|(prefix, _)| timezone.starts_with(prefix))
            .max_by_key(|(prefix, _)| prefix.len())
            .map_or(Self::GOESEast, |(_, satellite)| *satellite)
    }

    pub fn id(self) -> &'static str {
        use Satellite::*;

        match self.resolved() {
            Auto => unreachable!("Auto is resolved to a concrete satellite before use"),
            GOESEast => "goes-19",
            GOESWest => "goes-18",
            Himawari => "himawari",
            Meteosat9 => "meteosat-9",
            Meteosat10 => "meteosat-0deg"
        }
    }

    pub fn max_zoom(self) -> u32 {
        use Satellite::*;

        match self.resolved() {
            Auto => unreachable!("Auto is resolved to a concrete satellite before use"),
            GOESEast | GOESWest | Himawari => 4,
            Meteosat9 | Meteosat10 => 3,
        }
    }

    pub fn tile_image(self) -> Image<Box<[u8]>, 3> {
        Image::alloc(self.tile_size(), self.tile_size()).boxed()
    }

    pub fn tile_count(self) -> u32 {
        use Satellite::*;

        match self.resolved() {
            Auto => unreachable!("Auto is resolved to a concrete satellite before use"),
            GOESEast | GOESWest | Himawari => 16,
            Meteosat9 | Meteosat10 => 8,
        }
    }

    pub fn tile_size(self) -> u32 {
        use Satellite::*;

        match self.resolved() {
            Auto => unreachable!("Auto is resolved to a concrete satellite before use"),
            GOESEast | GOESWest => 678,
            Himawari => 688,
            Meteosat9 | Meteosat10 => 464,
        }
    }
}

impl Config {
    pub fn disk(&self) -> u32 {
        let smaller_dim = self.resolution_x.min(self.resolution_y);

        let disk_dim = smaller_dim as f32 * (self.disk_size as f32 / 100.0);
        disk_dim.floor() as u32
    }
}

/// Best-effort discovery of the machine's IANA time zone name.
///
/// Checks `$TZ` first, then the `/etc/localtime` symlink (what
/// `timedatectl set-timezone` updates) and finally `/etc/timezone`.
pub fn system_time_zone() -> Option<String> {
    if let Ok(tz) = std::env::var("TZ") {
        let tz = tz.trim();
        if !tz.is_empty() {
            return Some(tz.to_string());
        }
    }

    if let Ok(target) = std::fs::read_link("/etc/localtime") {
        if let Some(tz) = target.to_str().and_then(zone_name_from_path) {
            return Some(tz);
        }
    }

    if let Ok(tz) = std::fs::read_to_string("/etc/timezone") {
        let tz = tz.trim();
        if !tz.is_empty() {
            return Some(tz.to_string());
        }
    }

    None
}

/// Extract `"Area/Location"` from a path such as
/// `/usr/share/zoneinfo/Europe/Berlin`.
fn zone_name_from_path(path: &str) -> Option<String> {
    let (_, zone) = path.split_once("zoneinfo/")?;
    let zone = zone.trim_start_matches('/');

    if zone.is_empty() || zone == "UTC" || zone == "localtime" {
        return None;
    }

    Some(zone.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_time_zones_to_satellites() {
        use Satellite::*;

        assert_eq!(Satellite::from_timezone("Europe/Berlin"), Meteosat10);
        assert_eq!(Satellite::from_timezone("Africa/Nairobi"), Meteosat10);
        assert_eq!(Satellite::from_timezone("Asia/Tokyo"), Himawari);
        assert_eq!(Satellite::from_timezone("Australia/Sydney"), Himawari);
        assert_eq!(Satellite::from_timezone("America/Chicago"), GOESEast);
        assert_eq!(Satellite::from_timezone("America/Los_Angeles"), GOESWest);
        assert_eq!(Satellite::from_timezone("Pacific/Honolulu"), GOESWest);
        assert_eq!(Satellite::from_timezone("Asia/Kolkata"), Meteosat9);
        assert_eq!(Satellite::from_timezone("Asia/Dubai"), Meteosat9);
        // Unknown time zones fall back to GOES East.
        assert_eq!(Satellite::from_timezone("UTC"), GOESEast);
        assert_eq!(Satellite::from_timezone("Made/Up"), GOESEast);
    }

    #[test]
    fn parses_localtime_symlink() {
        assert_eq!(
            zone_name_from_path("/usr/share/zoneinfo/America/Chicago").as_deref(),
            Some("America/Chicago")
        );
        assert_eq!(
            zone_name_from_path("/var/db/timezone/zoneinfo/Japan").as_deref(),
            Some("Japan")
        );
        assert_eq!(zone_name_from_path("/etc/localtime"), None);
    }

    #[test]
    fn auto_resolves_to_concrete() {
        // Whatever the host time zone is, `Auto` must not stay `Auto`.
        assert_ne!(Satellite::Auto.resolved(), Satellite::Auto);
    }
}
