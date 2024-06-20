use strum::{Display, EnumString};

#[derive(Default, Debug, EnumString, Display, PartialEq, Eq, Hash)]
pub enum MeansOfDeath {
    #[default]
    #[strum(serialize = "MOD_UNKNOWN")]
    Unknown,
    #[strum(serialize = "MOD_SHOTGUN")]
    Shotgun,
    #[strum(serialize = "MOD_GAUNTLET")]
    Gauntlet,
    #[strum(serialize = "MOD_MACHINEGUN")]
    Machinegun,
    #[strum(serialize = "MOD_GRENADE")]
    Grenade,
    #[strum(serialize = "MOD_GRENADE_SPLASH")]
    GrenadeSplash,
    #[strum(serialize = "MOD_ROCKET")]
    Rocket,
    #[strum(serialize = "MOD_ROCKET_SPLASH")]
    RocketSplash,
    #[strum(serialize = "MOD_PLASMA")]
    Plasma,
    #[strum(serialize = "MOD_PLASMA_SPLASH")]
    PlasmaSplash,
    #[strum(serialize = "MOD_RAILGUN")]
    Railgun,
    #[strum(serialize = "MOD_LIGHTNING")]
    Lightning,
    #[strum(serialize = "MOD_BFG")]
    Bfg,
    #[strum(serialize = "MOD_BFG_SPLASH")]
    BfgSplash,
    #[strum(serialize = "MOD_WATER")]
    Water,
    #[strum(serialize = "MOD_SLIME")]
    Slime,
    #[strum(serialize = "MOD_LAVA")]
    Lava,
    #[strum(serialize = "MOD_CRUSH")]
    Crush,
    #[strum(serialize = "MOD_TELEFRAG")]
    Telefrag,
    #[strum(serialize = "MOD_FALLING")]
    Falling,
    #[strum(serialize = "MOD_SUICIDE")]
    Suicide,
    #[strum(serialize = "MOD_TARGET_LASER")]
    TargetLaser,
    #[strum(serialize = "MOD_TRIGGER_HURT")]
    TriggerHurt,
    #[strum(serialize = "MOD_NAIL")]
    Nail,
    #[strum(serialize = "MOD_CHAINGUN")]
    Chaingun,
    #[strum(serialize = "MOD_PROXIMITY_MINE")]
    ProximityMine,
    #[strum(serialize = "MOD_KAMIKAZE")]
    Kamikaze,
    #[strum(serialize = "MOD_JUICED")]
    Juiced,
    #[strum(serialize = "MOD_GRAPPLE")]
    Grapple,
}
