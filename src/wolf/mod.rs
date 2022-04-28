// porter? what porter? who's blujay?
use smash::lua2cpp::L2CAgentBase;
use smash_script::{*, macros::*};
use smashline::*;
use smash::hash40;
use smash::app::{self, lua_bind::*, sv_animcmd::{frame, wait}, *};
use smash::lib::{lua_const::*, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
use smash::phx::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use crate::custom::SIZE1;
use crate::custom::SIZE2;
use crate::custom::SIZE3;
use crate::custom::SIZE4;
use crate::custom::SIZE5;
use crate::custom::SIZE6;
use crate::custom::SIZE7;
use crate::custom::SIZE8;
use crate::custom::SIZE9;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;


#[acmd_script( agent = "wolf", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 3.1, 0.0, 8.0, 5.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 3.3, 0.0, 8.0, 6.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 20, 0, 20, 3.7, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(10.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 3.7, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(10.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 3.5, 0.0, 8.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 4.5, 0.0, 8.0, 12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 55, 140, 0, 40, 4.0, 0.0, 9.0, 2.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 55, 140, 0, 40, 4.3, 0.0, 9.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 55, 140, 0, 40, 5.5, 0.0, 9.0, 11.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attacks3s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 6.0;
        SIZE2[get_player_number(module_accessor)] = 4.0;
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 4.0, 0.0, 6.5, 16.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 6.0, 11.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
        AttackModule::clear(module_accessor, 2, false);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 16.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 6.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 6.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attacks3hi", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 6.0;
        SIZE2[get_player_number(module_accessor)] = 4.0;
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 4.0, 0.0, 11.0, 14.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 9.0, 11.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 8.0, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
        AttackModule::clear(module_accessor, 2, false);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE1[get_player_number(module_accessor)], 0.0, 13.0, 14.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 10.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 7.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attacks3lw", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 6.0;
        SIZE2[get_player_number(module_accessor)] = 4.0;
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 4.0, 0.0, 3.0, 16.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 4.5, 10.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 60, 70, 0, 10, 3.0, 0.0, 6.0, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
        AttackModule::clear(module_accessor, 2, false);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE1[get_player_number(module_accessor)], 0.0, 3.0, 16.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 4.5, 10.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 361, 106, 0, 55, SIZE2[get_player_number(module_accessor)], 0.0, 6.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn wolf_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 8.0;
        SIZE2[get_player_number(module_accessor)] = 7.0;
        SIZE3[get_player_number(module_accessor)] = 6.0;
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 110, 0, 0, 70, 5.0, 0.0, 7.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 110, 0, 0, 70, 5.0, 0.0, 7.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 125, 0, 0, 80, 5.0, 0.0, 7.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 125, 0, 0, 80, 5.0, 0.0, 7.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
            SIZE3[get_player_number(module_accessor)] += 3.0;
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE1[get_player_number(module_accessor)], 0.0, 23.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE2[get_player_number(module_accessor)], 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE3[get_player_number(module_accessor)], 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE1[get_player_number(module_accessor)], 0.0, 26.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE2[get_player_number(module_accessor)], 0.0, 21.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 95, 65, 0, 85, SIZE3[get_player_number(module_accessor)], 0.0, 16.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn wolf_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn wolf_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 2.4, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
        MotionModule::set_rate(module_accessor, 0.862);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn wolf_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
        MotionModule::set_rate(module_accessor, 0.869);
    }
    
}

#[acmd_script( agent = "wolf", script = "game_finallockon", category = ACMD_GAME, low_priority)]
unsafe fn wolf_finallockon(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 20, -20);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		SLOW_OPPONENT(fighter, 4.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 2, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 25);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finallockon.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 200, 0, 20.0, 0.0, 15.0, 22.0, Some(0.0), Some(15.0), Some(38.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);

	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_FOX_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
		fighter.clear_lua_stack();
		lua_args!(fighter, 0);
		sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	
}

#[acmd_script( agent = "wolf", script = "game_finalairlockon", category = ACMD_GAME, low_priority)]
unsafe fn wolf_finalairlockon(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 20, -20);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_WOLFEN, false, 0);
		SLOW_OPPONENT(fighter, 4.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 2, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 25);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finallockon.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 200, 0, 20.0, 0.0, 15.0, 22.0, Some(0.0), Some(15.0), Some(38.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_FOX_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
		fighter.clear_lua_stack();
		lua_args!(fighter, 0);
		sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	
}

pub fn install() {
    install_acmd_scripts!(
        wolf_attack11,
        wolf_attack12,
        wolf_attack13,
        wolf_attacks3s,
        wolf_attacks3hi,
        wolf_attacks3lw,
        wolf_attackhi4,
        wolf_catch,
        wolf_catchdash,
        wolf_catchturn,
        wolf_finallockon,
        wolf_finalairlockon
    );
}

