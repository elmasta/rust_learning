pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        
        self.members.push(Member {
            name: name.to_string(),
            age,
            role: Role::Associate,
        })
    }

    pub fn attack(&mut self, other_mob: &mut Mob) {
        let self_power = self.calculate_combat_power();
        let other_power = other_mob.calculate_combat_power();

        if self_power > other_power {
            // Remove last member from other_mob
            if let Some(member) = other_mob.members.pop() {
                println!("Attacker wins. Removing member {:?}", member);
            }
        } else if other_power > self_power {
            // Remove last member from self
            if let Some(member) = self.members.pop() {
                println!("Defender wins. Removing member {:?}", member);
            }
        } else {
            // Draw case, attacker loses
            if let Some(member) = self.members.pop() {
                println!("Draw. Attacker loses. Removing member {:?}", member);
            }
        }

        // Transfer cities and wealth if one mob has zero members
        if self.members.is_empty() && !other_mob.members.is_empty() {
            other_mob.cities.extend(self.cities.drain(..));
            other_mob.wealth += self.wealth;
            self.wealth = 0;
        } else if !self.members.is_empty() && other_mob.members.is_empty() {
            self.cities.extend(other_mob.cities.drain(..));
            self.wealth += other_mob.wealth;
            other_mob.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u32) {
        let stolen = target.wealth.min(value);
        self.wealth += stolen;
        target.wealth -= stolen;
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, value: u8) {
        let mut city_already_owned = false;

        for mob in mobs.clone().iter_mut() {
            for (city, _) in mob.cities.iter() {
                if city == &city_name {
                    city_already_owned = true;
                    break;
                }
            }
            if city_already_owned {
                break;
            }
        }

        if !city_already_owned {
            self.cities.push((city_name, value));
        }
    }

    fn calculate_combat_power(&self) -> u32 {
        self.members
            .iter()
            .map(|member| match member.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }
}
