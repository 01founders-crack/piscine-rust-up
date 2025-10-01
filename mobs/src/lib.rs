use std::collections::{HashMap, HashSet};

#[path = "mobs/boss.rs"]
pub mod boss;
#[path = "mobs/member.rs"]
pub mod member;

pub use boss::Boss;
pub use member::{Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    fn get_power_score(&self) -> u32 {
        self.members
            .values()
            .map(|member| match member.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power = self.get_power_score();
        let other_power = other.get_power_score();

        if self_power > other_power {
            if !other.members.is_empty() {
                let youngest_age = other.members.values().map(|m| m.age).min().unwrap_or(0);
                other.members.retain(|_, m| m.age != youngest_age);

                if other.members.is_empty() {
                    self.wealth += other.wealth;
                    other.wealth = 0;
                    self.cities.extend(other.cities.drain());
                }
            }
        } else {
            if !self.members.is_empty() {
                let youngest_age = self.members.values().map(|m| m.age).min().unwrap_or(0);
                self.members.retain(|_, m| m.age != youngest_age);

                if self.members.is_empty() {
                    other.wealth += self.wealth;
                    self.wealth = 0;
                    other.cities.extend(self.cities.drain());
                }
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen_amount = amount.min(target.wealth);
        self.wealth += stolen_amount;
        target.wealth -= stolen_amount;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city: String) {
        if !other_mobs.iter().any(|mob| mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_boss() {
        assert_eq!(
            Boss::new("Scarface", 43),
            Boss {
                name: "Scarface".to_owned(),
                age: 43
            }
        );
    }

    #[inline]
    fn mobs() -> (Mob, Mob) {
        (
            Mob {
                name: "Hairy Giants".to_owned(),
                boss: Boss::new("Louie HaHa", 36),
                cities: ["San Francisco"].map(str::to_owned).into(),
                members: [
                    (
                        "Benny Eggs",
                        Member {
                            role: Role::Soldier,
                            age: 28,
                        },
                    ),
                    (
                        "Jhonny",
                        Member {
                            role: Role::Associate,
                            age: 17,
                        },
                    ),
                    (
                        "Greasy Thumb",
                        Member {
                            role: Role::Soldier,
                            age: 30,
                        },
                    ),
                    (
                        "No Finger",
                        Member {
                            role: Role::Caporegime,
                            age: 32,
                        },
                    ),
                ]
                .map(|(n, d)| (n.to_owned(), d))
                .into(),
                wealth: 100000,
            },
            Mob {
                name: "Red Thorns".to_owned(),
                boss: Boss::new("Big Tuna", 30),
                cities: ["San Jose"].map(str::to_owned).into(),
                members: [
                    (
                        "Knuckles",
                        Member {
                            role: Role::Soldier,
                            age: 25,
                        },
                    ),
                    (
                        "Baldy Dom",
                        Member {
                            role: Role::Caporegime,
                            age: 36,
                        },
                    ),
                    (
                        "Crazy Joe",
                        Member {
                            role: Role::Underboss,
                            age: 23,
                        },
                    ),
                ]
                .map(|(n, d)| (n.to_owned(), d))
                .into(),
                wealth: 70000,
            },
        )
    }

    #[test]
    fn mob_recruit() {
        let (mut mob, _) = mobs();
        mob.recruit(("Rusty", 37));

        assert_eq!(
            mob.members.get("Rusty"),
            Some(&Member {
                role: Role::Associate,
                age: 37,
            })
        );

        mob.recruit(("Pedro", 14));

        assert_eq!(
            mob.members.get("Pedro"),
            Some(&Member {
                role: Role::Associate,
                age: 14,
            })
        );
    }

    #[test]
    fn member_get_promotion() {
        let (mut mob, _) = mobs();

        let member = mob.members.get_mut("Benny Eggs").unwrap();

        member.get_promotion();
        assert_eq!(member.role, Role::Caporegime);
        member.get_promotion();
        assert_eq!(member.role, Role::Underboss);
    }

    #[test]
    #[should_panic]
    fn member_get_promotion_panic() {
        let (_, mut mob) = mobs();

        let member = mob.members.get_mut("Crazy Joe").unwrap();

        member.get_promotion();
    }

    #[test]
    fn mob_steal() {
        let (mut a, mut b) = mobs();
        b.steal(&mut a, 10000);
        assert_eq!(a.wealth, 90000);
        assert_eq!(b.wealth, 80000);

        b.steal(&mut a, 1000000);
        assert_eq!(a.wealth, 0);
        assert_eq!(b.wealth, 170000);
    }

    #[test]
    fn mob_attack() {
        let (mut a, mut b) = mobs();
        a.attack(&mut b);

        assert_eq!(a.members.len(), 3);
        assert_eq!(b.members.len(), 3);
    }

    #[test]
    fn same_combat_power() {
        let (mut a, mut b) = mobs();

        a.recruit(("Stitches", 28));
        a.attack(&mut b);

        assert_eq!(a.members.len(), 4);
        assert_eq!(b.members.len(), 3);
    }

    #[test]
    fn no_members_mob() {
        let (mut a, mut b) = mobs();
        b.attack(&mut a);
        a.attack(&mut b);
        b.attack(&mut a);
        b.attack(&mut a);

        assert_eq!(a.members.len(), 0);
        assert_eq!(a.cities.len(), 0);
        assert_eq!(a.wealth, 0);

        assert!(b
            .cities
            .is_superset(&["San Jose", "San Francisco"].map(str::to_owned).into()));
        assert_eq!(b.wealth, 170000);
    }

    #[test]
    fn mob_conquer_city() {
        let (mut a, mut b) = mobs();

        b.conquer_city(&[&a], "Las Vegas".to_owned());
        assert!(b.cities.contains("Las Vegas"));

        a.conquer_city(&[&b], "Las Vegas".to_owned());
        assert_eq!(a.cities.len(), 1);
    }
}
