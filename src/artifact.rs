use crate::stat::*;
use crate::statgrowth::*;
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
  Flower,
  Plume,
  Sands,
  Goblet,
  Circlet,
}

pub const ALL_TYPES: [Type; 5] = [
  Type::Flower,
  Type::Plume,
  Type::Sands,
  Type::Goblet,
  Type::Circlet,
];

fn pick_from_array<T, const N: usize>(array: [T; N]) -> T {
  use rand::seq::IteratorRandom;
  let mut rng = rand::thread_rng();
  array.into_iter().choose(&mut rng).unwrap()
}

fn pick_from_vec_slice<T>(vec: &mut Vec<T>) -> &mut T {
  use rand::seq::IteratorRandom;
  let mut rng = rand::thread_rng();
  vec.into_iter().choose(&mut rng).unwrap()
}

fn not_any_of<T: PartialEq>(ss: T, r: Vec<T>) -> bool {
  match r.into_iter().find(|elem| *elem == ss) {
    Some(_) => false,
    None => true,
  }
}

fn exclude_weigthed_substats(all: Vec<(SubStatGrowth, f32)>, exclude: Vec<Stat>) -> Vec<(SubStatGrowth, f32)> {
  all
    .into_iter()
    .filter(|(e, _)| not_any_of(e.highest_roll, exclude.clone()))
    .collect()
}

fn sum_weigths(stats: Vec<(SubStatGrowth, f32)>) -> f32 {
  stats.into_iter().map( |(_, w)| w ).fold(0.0, |acc, elem| acc + elem)
}

fn pick_weigthed_substats( stats: Vec<(SubStatGrowth, f32)> ) -> SubStatGrowth {
  let base = sum_weigths(stats.clone()) as u32;
  let mut roll = (rand::random::<u32>() % base) as f32;
  for (stat, weigth) in stats.clone() {
    if roll - weigth < 0.0 {
      return stat;
    } else {
      roll -= weigth;
    }
  }
  stats[0].0.clone()
}

#[derive(Clone)]
pub struct Artifact {
  pub raity: u8,
  pub level: u8,
  pub m_type: Type,
  pub main_stat: Stat,
  pub substats: Vec<Stat>,
}

impl Artifact {
  fn to_statgrowth(t: Type, s: Stat) -> MainStatGrowth {
    match t {
      Type::Flower => FLOWER_MAIN,
      Type::Plume => PLUME_MAIN,
      Type::Sands => SANDS_MAIN.into_iter().find(|ms| ms.base == s).unwrap(),
      Type::Goblet => GOBLET_MAIN.into_iter().find(|ms| ms.base == s).unwrap(),
      Type::Circlet => CIRCLET_MAIN.into_iter().find(|ms| ms.base == s).unwrap(),
    }
  }

  fn add_substat(level: u8, main_stat: Stat, substats: &mut Vec<Stat>) {
    if level % 4 == 0 {
      if substats.len() < 4 {
        let mut cloned = substats.clone();
        cloned.push( main_stat );
        let new_substat = pick_weigthed_substats(exclude_weigthed_substats(
          WEIGTHED_ALL_SUBSTATS.into_iter().collect(),
          cloned,
        ));
        substats.push(new_substat.generate_roll());
      } else {
        let chosen_substat: &mut Stat = pick_from_vec_slice(substats);
        let stat = ALL_SUBSTATS
          .into_iter()
          .find(|ss| *chosen_substat == ss.highest_roll)
          .unwrap();
        let roll = stat.generate_roll();
        *chosen_substat = *chosen_substat + roll;
      }
    }
  }

  pub fn add_levels(&mut self, level: u8) {
    let tmp_lvl = self.level;
    self.level += level;
    let sg = Artifact::to_statgrowth(self.m_type, self.main_stat);
    self.main_stat = sg.with_level(self.level);
    let new_substats = (self.level / 4) - (tmp_lvl / 4);
    for _ in 0..new_substats {
      Artifact::add_substat(4, self.main_stat.clone(), &mut self.substats);
    }
  }
}

impl std::fmt::Display for Artifact {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let t = format!("type: {:?}", self.m_type).bold();
    let r = format!("{:*<1$}", "", self.raity as usize).yellow().bold();
    f.write_fmt(format_args!("{} +{} | {}\n", t, self.level, r))?;
    let m = format!("main stat: {}", self.main_stat);
    f.write_fmt(format_args!("  {}\n  sub stats:\n", m))?;
    for s in self.substats.clone() {
      f.write_fmt(format_args!("    {}\n", s))?;
    }
    Ok(())
  }
}

fn generate_substats(count: u8, mut used_stats: Vec<Stat>) -> Vec<Stat> {
  if count > 0 {
    let all_stats = WEIGTHED_ALL_SUBSTATS.into_iter().collect();
    let filtered_stats = exclude_weigthed_substats(all_stats, used_stats.clone());
    let random_stat = pick_weigthed_substats(filtered_stats);
    used_stats.insert(0, random_stat.generate_roll());
    generate_substats(count - 1, used_stats)
  } else {
    // exclude main stat
    let length = used_stats.len();
    used_stats.into_iter().take(length - 1).collect()
  }
}

fn pick_weigthed_main_stat<const N: usize>( stats: [(MainStatGrowth, f32); N] ) -> MainStatGrowth {
  let mut tmp = (rand::random::<u32>() % 10000) as f32 / 100.0;
  for (stat, weigth) in stats.clone() {
    if tmp - weigth < 0.0 {
      return stat;
    } else {
      tmp -= weigth;
    }
  }
  stats[N - 1].0.clone()
}

fn generate_main_stat(t: Type) -> Stat {
  use Type::*;
  match t {
    Flower => FLOWER_MAIN.clone().base,
    Plume => PLUME_MAIN.clone().base,
    Sands => pick_weigthed_main_stat(WEIGTHED_SANDS_MAIN).base,
    Circlet => pick_weigthed_main_stat(WEIGTHED_CIRCLET_MAIN).base,
    Goblet => pick_weigthed_main_stat(WEIGTHED_GOBLET_MAIN).base,
  }
}

fn number_of_substats(rarity: u8) -> u8 {
  if rarity > 1 {
    rarity - 2 + rand::random::<u8>() % 2
  } else {
    0
  }
}

pub fn generate_arifact(r: u8) -> Artifact {
  let t = pick_from_array(ALL_TYPES);
  let main = generate_main_stat(t.clone());
  Artifact {
    raity: r,
    level: 0,
    m_type: t,
    main_stat: main.clone(),
    substats: generate_substats(number_of_substats(r), vec![main]),
  }
}
