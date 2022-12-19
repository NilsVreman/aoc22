use parser;
use std::collections::HashMap;

/// Ore = 0, Clay = 2, Obsidian = 4, Geode = 8;

// MAC = Machine
const MAC_ORE: u8 = 1 << 0;
const MAC_CLY: u8 = 1 << 1;
const MAC_OBS: u8 = 1 << 2;
const MAC_GEO: u8 = 1 << 3;
const MAC_ALL: u8 = MAC_ORE | MAC_CLY | MAC_OBS | MAC_GEO;
const MAC_NUM: u8 = 4;

#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct State {
    time: u8,
    mat: [u16; MAC_NUM as usize], // Materials
    mac: [u16; MAC_NUM as usize], // Machines
    bld: u8,             // Buildable
}

impl State {
    pub fn new(time: u8) -> Self {
        Self {
            time: time,
            mat: [0, 0, 0, 0],
            mac: [1, 0, 0, 0],
            bld: MAC_ALL,
        }
    }
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: u16,
    ore_cost: u16,
    cly_cost: u16,
    obs_cost: (u16, u16),
    geo_cost: (u16, u16),
    max_ore_cost: u16,
    max_cly_cost: u16,
    max_obs_cost: u16,
}

impl Blueprint {
    pub fn new(l: &str) -> Self {
        let ps = parser::Parser::build("Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", "{}");
        let v = ps.parse(&l).expect("Couldn't parse Blueprint line");

        let ore_cost = v[1].parse::<u16>().unwrap();
        let cly_cost = v[2].parse::<u16>().unwrap();
        let obs_cost = (v[3].parse::<u16>().unwrap(), v[4].parse::<u16>().unwrap());
        let geo_cost = (v[5].parse::<u16>().unwrap(), v[6].parse::<u16>().unwrap());
        Self {
            id:                     v[0].parse::<u16>().unwrap(),
            ore_cost:               ore_cost,
            cly_cost:               cly_cost,
            obs_cost:               obs_cost,
            geo_cost:               geo_cost,
            max_ore_cost:           ore_cost.max(cly_cost).max(obs_cost.0).max(geo_cost.0),
            max_cly_cost:           obs_cost.1,
            max_obs_cost:           geo_cost.1,
        }
    }
}

pub fn execute_bp(bp: &Blueprint, s: &State, mem: &mut HashMap<State, u16>) -> u16 {
    if s.time == 0 {
        return s.mat[3];
    }

    let mat_new: [u16; MAC_NUM as usize] = s.mat.iter()
        .zip(s.mac.iter())
        .map(|(x, y)| x + y)
        .collect::<Vec<u16>>()
        .try_into()
        .expect("No idea why this does not work");

    let s_new = State {
        time: s.time,
        mat: mat_new,
        mac: s.mac,
        bld: s.bld,
    };

    // Check if we have gone through this state already. If we haven't...
    if !mem.contains_key(&s_new) {
        let mut max_geo = 0;
        let mut bld_new = 0;

        // if we don't have enough resources to build another geode machine: say that we want to build one
        if (s.mat[0] < bp.geo_cost.0) || (s.mat[2] < bp.geo_cost.1) {
            bld_new |= MAC_GEO;
        // else
        } else {
            let geodes = execute_bp(
                &bp,
                &State {
                    time: s.time - 1,
                    mat: [mat_new[0] - bp.geo_cost.0, mat_new[1], mat_new[2] - bp.geo_cost.1, mat_new[3]],
                    mac: [s.mac[0], s.mac[1], s.mac[2], s.mac[3]+1],
                    bld: MAC_ALL,
                },
                mem);
            max_geo = max_geo.max(geodes);
        } 

        // if we don't have enough resources to build another ore machine: say that we want to build one
        if s.mat[0] < bp.ore_cost {
            bld_new |= MAC_ORE;
            // else if we want to build another ore machine AND we *need* to build one
        } else if (bp.max_ore_cost > s.mac[0]) && (s.bld & MAC_ORE) != 0 {
            let geodes = execute_bp(
                &bp,
                &State {
                    time: s.time - 1,
                    mat: [mat_new[0] - bp.ore_cost, mat_new[1], mat_new[2], mat_new[3]],
                    mac: [s.mac[0]+1, s.mac[1], s.mac[2], s.mac[3]],
                    bld: MAC_ALL,
                },
                mem);
            max_geo = max_geo.max(geodes);
        }

        // if we don't have enough resources to build another clay machine: say that we want to build one
        if s.mat[0] < bp.cly_cost {
            bld_new |= MAC_CLY;
            // else if we want to build another clay machine AND we *need* to build one
        } else if bp.max_cly_cost > s.mac[1] && (s.bld & MAC_CLY) != 0 {
            let geodes = execute_bp(
                &bp,
                &State {
                    time: s.time - 1,
                    mat: [mat_new[0] - bp.cly_cost, mat_new[1], mat_new[2], mat_new[3]],
                    mac: [s.mac[0], s.mac[1]+1, s.mac[2], s.mac[3]],
                    bld: MAC_ALL,
                },
                mem);
            max_geo = max_geo.max(geodes);
        }

        // if we don't have enough resources to build another obsidian machine: say that we want to build one
        if s.mat[0] < bp.obs_cost.0 || s.mat[1] < bp.obs_cost.1 {
            bld_new |= MAC_OBS;
            // else if we want to build another obsidian machine AND we *need* to build one
        } else if (bp.max_obs_cost > s.mac[2]) && (s.bld & MAC_OBS) != 0 {
            let geodes = execute_bp(
                &bp,
                &State {
                    time: s.time - 1,
                    mat: [mat_new[0] - bp.obs_cost.0, mat_new[1] - bp.obs_cost.1, mat_new[2], mat_new[3]],
                    mac: [s.mac[0], s.mac[1], s.mac[2]+1, s.mac[3]],
                    bld: MAC_ALL,
                },
                mem);
            max_geo = max_geo.max(geodes);
        }

        // Build all machines that we *need*
        if bld_new != 0 {
            let geodes = execute_bp(
                &bp,
                &State {
                    time: s.time - 1,
                    mat: mat_new,
                    mac: s.mac,
                    bld: bld_new,
                },
                mem);
            max_geo = max_geo.max(geodes);
        }

        // Insert this state into the memory
        mem.insert(s_new.clone(), max_geo);
    }

    // Get the value from memory
    *mem.get(&s_new).unwrap()
}
