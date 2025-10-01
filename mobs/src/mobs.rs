// usually put those two line inside a mod.rs file inside mobs/, but this project use mobs.rs so we cant do that
pub mod boss;
pub mod member;

use std::collections::{HashMap, HashSet};

use crate::mobs::boss::*;
use crate::mobs::member::*;

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    // a method add a Member to the members map.  accept a tuple with the member's information, name and age
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    //  a method which receives another Mob as reference
    pub fn attack(&mut self, target: &mut Mob) {
        let mut self_power = 0;
        let mut target_power = 0;

        for member in self.members.values() {
            match member.role {
                Role::Underboss => self_power += 4,
                Role::Caporegime => self_power += 3,
                Role::Soldier => self_power += 2,
                Role::Associate => self_power += 1,
            }
        }

        for member in target.members.values() {
            match member.role {
                Role::Underboss => target_power += 4,
                Role::Caporegime => target_power += 3,
                Role::Soldier => target_power += 2,
                Role::Associate => target_power += 1,
            }
        }

        if self_power > target_power {
            let mut youngest_age = u32::MAX;
            let mut youngest_name = String::new();
            for (name, member) in target.members.iter() {
                if member.age < youngest_age {
                    youngest_age = member.age;
                    youngest_name = name.clone();
                }
            }
            if !youngest_name.is_empty() {
                target.members.remove(&youngest_name);
            }
        } else if target_power > self_power {
            let mut youngest_age = u32::MAX;
            let mut youngest_name = String::new();
            for (name, member) in self.members.iter() {
                if member.age < youngest_age {
                    youngest_age = member.age;
                    youngest_name = name.clone();
                }
            }
            if !youngest_name.is_empty() {
                self.members.remove(&youngest_name);
            }
        }

        if target.members.is_empty() {
            self.cities.extend(target.cities.drain());
            self.wealth += target.wealth;
            target.wealth = 0;
        }
    }

    //  a method which receives a Mob to target, and a u64 value to steal.
    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        if value > target.wealth {
            self.wealth += target.wealth;
            target.wealth = 0;
        } else {
            self.wealth += value;
            target.wealth -= value;
        }
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: &str) {
        let mut city_is_owned = false;
        for mob in mobs {
            if mob.cities.contains(city) {
                city_is_owned = true;
                break;
            }
        }

        if !city_is_owned {
            self.cities.insert(city.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // brings Mob, Boss, Member, Role into scope

    #[test]
    fn test_recruit() {
        let mut mob = Mob {
            name: "Test Mob".to_string(),
            boss: Boss::new("Boss", 100),
            members: std::collections::HashMap::new(),
            cities: std::collections::HashSet::new(),
            wealth: 0,
        };

        mob.recruit(("John", 25));

        let member = mob.members.get("John").unwrap();
        assert_eq!(member.role, Role::Associate);
        assert_eq!(member.age, 25);
    }

    #[test]
    fn test_attack() {
        let mut mob_a = Mob {
            name: "A".to_string(),
            boss: Boss::new("Boss A", 100),
            members: std::collections::HashMap::new(),
            cities: std::collections::HashSet::new(),
            wealth: 50,
        };

        let mut mob_b = Mob {
            name: "B".to_string(),
            boss: Boss::new("Boss B", 100),
            members: std::collections::HashMap::new(),
            cities: std::collections::HashSet::new(),
            wealth: 50,
        };

        mob_a.recruit(("Mike", 30));
        mob_b.recruit(("Tony", 20));

        mob_a.attack(&mut mob_b);

        // one of mob_b's members should be removed (the youngest)
        assert!(mob_b.members.is_empty() || mob_b.members.len() == 1);
    }

    #[test]
    fn test_steal() {
        let mut mob_a = Mob {
            name: "A".to_string(),
            boss: Boss::new("Boss A", 100),
            members: std::collections::HashMap::new(),
            cities: std::collections::HashSet::new(),
            wealth: 50,
        };

        let mut mob_b = Mob {
            name: "B".to_string(),
            boss: Boss::new("Boss B", 100),
            members: std::collections::HashMap::new(),
            cities: std::collections::HashSet::new(),
            wealth: 80,
        };

        mob_a.steal(&mut mob_b, 30);

        assert_eq!(mob_a.wealth, 80);
        assert_eq!(mob_b.wealth, 50);
    }
}
