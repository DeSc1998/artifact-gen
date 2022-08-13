
use colored::*;

#[derive(Clone, Copy, PartialEq)]
pub enum StatType {
  Health,
  Defance,
  Attack,
  HealthFlat,
  DefanceFlat,
  AttackFlat,
  EnergyRecharge,
  ElemantalMastery,
  ElemantalDamage,
  Crit,
  CritDamage,
  Healing,
}

impl std::fmt::Display for StatType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let out = match self {
      StatType::Health => {
        format!("{}", "Health %".green().bold())
      }
      StatType::Defance => {
        format!("{}", "Defence %".yellow().bold())
      }
      StatType::Attack => {
        format!("{}", "Attack %".red().bold())
      }
      StatType::HealthFlat => {
        format!("{}", "Health".green())
      }
      StatType::DefanceFlat => {
        format!("{}", "Defence".yellow())
      }
      StatType::AttackFlat => {
        format!("{}", "Attack".red())
      }
      StatType::EnergyRecharge => {
        format!("{}", "Energy Recharge".cyan().bold())
      }
      StatType::ElemantalMastery => {
        format!("{}", "Elemantal Mastery".magenta())
      }
      StatType::Crit => {
        format!("{}", "Crit. Rate".red())
      }
      StatType::CritDamage => {
        format!("{}", "Crit. Damage".red().bold())
      }
      StatType::ElemantalDamage => {
        format!("{}", "Elemental Damage %".magenta().bold())
      }
      StatType::Healing => {
        format!("{}", "Healing Bonus %".green())
      }
    };
    f.write_fmt(format_args!("{}", out))
  }
}

#[derive(Clone, Copy)]
pub struct Stat {
  pub m_type: StatType,
  pub value: f64,
}

impl Stat {
  pub fn scale(self, factor: f64) -> Stat {
    Stat {
      m_type: self.m_type,
      value: self.value * factor,
    }
  }
}

impl std::fmt::Display for Stat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let val_out = format!("{:.1}", self.value);
    f.write_fmt(format_args!("{}({})", self.m_type, val_out.red()))
  }
}

impl std::ops::Add for Stat {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    if self.m_type == other.m_type {
      Stat {
        m_type: self.m_type,
        value: self.value + other.value,
      }
    } else {
      self
    }
  }
}

impl PartialEq for Stat {
  fn eq(&self, other: &Self) -> bool {
    self.m_type == other.m_type
  }
}