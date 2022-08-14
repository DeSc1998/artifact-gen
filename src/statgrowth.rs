
use crate::stat::*;

#[derive(Clone)]
pub struct MainStatGrowth {
  pub base: Stat,
  pub growth: Stat,
}

impl MainStatGrowth {
  pub fn with_level(self, level: u8) -> Stat {
    self.base + self.growth.scale(level as f64)
  }
}

macro_rules! mainstat {
  ($type:ident, $base:expr, $growth:expr) => {
    MainStatGrowth {
      base: Stat {
        m_type: StatType::$type,
        value: $base,
      },
      growth: Stat {
        m_type: StatType::$type,
        value: $growth,
      },
    }
  };
}

pub const FLOWER_MAIN: MainStatGrowth = mainstat!(HealthFlat, 717.0, 203.0 + 3.0 / 20.0);
pub const PLUME_MAIN: MainStatGrowth = mainstat!(AttackFlat, 47.0, 13.2);
#[allow(dead_code)]
pub const SANDS_MAIN: [MainStatGrowth; 5] = [
  mainstat!(Health, 7.0, 1.98),
  mainstat!(Attack, 7.0, 1.98),
  mainstat!(Defance, 8.7, 2.48),
  mainstat!(ElemantalMastery, 28.0, 7.925),
  mainstat!(EnergyRecharge, 7.8, 2.2),
];
#[allow(dead_code)]
pub const GOBLET_MAIN: [MainStatGrowth; 5] = [
  mainstat!(Health, 7.0, 1.98),
  mainstat!(Attack, 7.0, 1.98),
  mainstat!(Defance, 8.7, 2.48),
  mainstat!(ElemantalMastery, 28.0, 7.925),
  mainstat!(ElemantalDamage, 7.0, 1.98),
];
#[allow(dead_code)]
pub const CIRCLET_MAIN: [MainStatGrowth; 7] = [
  mainstat!(Health, 7.0, 1.98),
  mainstat!(Attack, 7.0, 1.98),
  mainstat!(Defance, 8.7, 2.48),
  mainstat!(ElemantalMastery, 28.0, 7.925),
  mainstat!(Crit, 4.7, 1.32),
  mainstat!(CritDamage, 9.3, 2.645),
  mainstat!(Healing, 5.4, 1.525),
];

pub const WEIGTHED_SANDS_MAIN: [(MainStatGrowth, f32); 5] = [
  (mainstat!(Health, 7.0, 1.98), 26.68),
  (mainstat!(Attack, 7.0, 1.98), 26.68),
  (mainstat!(Defance, 8.7, 2.48), 26.68),
  (mainstat!(ElemantalMastery, 28.0, 7.925), 10.0),
  (mainstat!(EnergyRecharge, 7.8, 2.2), 10.0),
];
pub const WEIGTHED_GOBLET_MAIN: [(MainStatGrowth, f32); 5] = [
  (mainstat!(Health, 7.0, 1.98), 21.25),
  (mainstat!(Attack, 7.0, 1.98), 21.25),
  (mainstat!(Defance, 8.7, 2.48), 20.0),
  (mainstat!(ElemantalMastery, 28.0, 7.925), 2.5),
  (mainstat!(ElemantalDamage, 7.0, 1.98), 35.0),
];
pub const WEIGTHED_CIRCLET_MAIN: [(MainStatGrowth, f32); 7] = [
  (mainstat!(Health, 7.0, 1.98), 22.0),
  (mainstat!(Attack, 7.0, 1.98), 22.0),
  (mainstat!(Defance, 8.7, 2.48), 22.0),
  (mainstat!(ElemantalMastery, 28.0, 7.925), 4.0),
  (mainstat!(Crit, 4.7, 1.32), 10.0),
  (mainstat!(CritDamage, 9.3, 2.645), 10.0),
  (mainstat!(Healing, 5.4, 1.525), 1.0),
];

#[derive(Clone)]
pub struct SubStatGrowth {
  pub highest_roll: Stat,
}

impl SubStatGrowth {
  pub fn generate_roll(self) -> Stat {
    let scale = 1.0 - (rand::random::<u32>() % 4) as f64 / 10.0;
    self.highest_roll.scale(scale)
  }
}

macro_rules! substat {
  ($type:ident; $highest:literal) => {
    SubStatGrowth {
      highest_roll: Stat {
        m_type: StatType::$type,
        value: $highest,
      },
    }
  };
}

#[allow(dead_code)]
pub const ALL_SUBSTATS: [SubStatGrowth; 10] = [
  substat!(Attack; 5.83),
  substat!(AttackFlat; 19.45),
  substat!(Health; 5.83),
  substat!(HealthFlat; 298.75),
  substat!(Defance; 7.29),
  substat!(DefanceFlat; 23.15),
  substat!(EnergyRecharge; 6.48),
  substat!(ElemantalMastery; 23.31),
  substat!(Crit; 3.89),
  substat!(CritDamage; 7.77),
];

pub const WEIGTHED_ALL_SUBSTATS: [(SubStatGrowth, f32); 10] = [
  (substat!(Attack; 5.83), 4.0),
  (substat!(AttackFlat; 19.45), 6.0),
  (substat!(Health; 5.83), 4.0),
  (substat!(HealthFlat; 298.75), 6.0),
  (substat!(Defance; 7.29), 4.0),
  (substat!(DefanceFlat; 23.15), 6.0),
  (substat!(EnergyRecharge; 6.48), 4.0),
  (substat!(ElemantalMastery; 23.31), 4.0),
  (substat!(Crit; 3.89), 3.0),
  (substat!(CritDamage; 7.77), 3.0),
];