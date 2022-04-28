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
use crate::custom::FINAL_TRANSFORM;


#[acmd_script( agent = "lucas", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 35, 0, 35, 2.4, 0.0, 5.5, 5.8, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 28, 0, 20, 2.6, 0.0, 5.5, 8.4, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 20, 0, 20, 2.8, 0.0, 5.5, 11.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 28, 0, 20, 2.6, 0.0, 5.5, 8.4, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 361, 20, 0, 20, 2.8, 0.0, 5.5, 11.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 30, 0, 25, 3.0, 0.0, 5.5, 5.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 28, 0, 20, 3.2, 0.0, 6.0, 8.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 20, 0, 20, 3.0, 0.0, 6.5, 12.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 361, 80, 0, 64, 4.5, 0.0, 6.5, 6.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 361, 80, 0, 64, 5.5, 0.0, 7.0, 12.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 84, 45, 0, 20, 2.8, 0.0, 3.5, 4.5, Some(0.0), Some(3.6), Some(4.7), 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 84, 45, 0, 20, 2.8, 0.0, 3.0, 7.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 40, 45, 0, 10, 2.8, 0.0, 2.0, 10.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 70, 0, 20, 6.9, 0.0, 5.0, 0.0, None, None, None, 0.44, 2.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 60, 140, 0, 40, 10.2, 0.0, 3.7, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 68.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.0, 280, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 12.0, 280, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 3, 0, Hash40::new("kneer"), 7.0, 361, 60, 0, 10, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 12.0, 361, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 3, 0, Hash40::new("kneer"), 7.0, 361, 60, 0, 10, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 1, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn lucas_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 5.5;
        SIZE2[get_player_number(module_accessor)] = 4.2;
        SIZE3[get_player_number(module_accessor)] = 3.2;
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    for _ in 0..3 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 363, 10, 0, 42, SIZE2[get_player_number(module_accessor)], 0.0, -4.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 270, 100, 20, 0, SIZE3[get_player_number(module_accessor)], 0.0, 1.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                SIZE1[get_player_number(module_accessor)] += 2.0;
                SIZE2[get_player_number(module_accessor)] += 2.0;
                SIZE3[get_player_number(module_accessor)] += 2.0;
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
        wait(lua_state, 5.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 270, 110, 0, 10, SIZE1[get_player_number(module_accessor)], 0.0, -4.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 130, 0, 10, 3.5, 0.0, 1.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_specialairs", category = ACMD_GAME, low_priority)]
unsafe fn lucas_specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.6);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        let added_speed = Vector3f{ x: -0.9, y: 0.1, z: 0.0 };
        KineticModule::add_speed(module_accessor, &added_speed);
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, 0);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn lucas_specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        SIZE1[get_player_number(module_accessor)] = 3.5;
        SIZE2[get_player_number(module_accessor)] = 4.3;
        SIZE3[get_player_number(module_accessor)] = 6.0;
        SIZE4[get_player_number(module_accessor)] = 12.0;
        GroundModule::select_cliff_hangdata(module_accessor, *FIGHTER_LUCAS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, 3.0, -3.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, -2.0, 0.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        JostleModule::set_status(module_accessor, false);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    wait(lua_state, 1.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.0, 367, 50, 50, 30, SIZE1[get_player_number(module_accessor)], 0.0, 5.0, 6.5, Some(0.0), Some(-1.0), Some(6.5), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.0, 366, 50, 0, 50, SIZE2[get_player_number(module_accessor)], 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.0, 366, 50, 0, 50, SIZE3[get_player_number(module_accessor)], 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                SIZE1[get_player_number(module_accessor)] += 0.5;
                SIZE2[get_player_number(module_accessor)] += 0.5;
                SIZE3[get_player_number(module_accessor)] += 0.5;
                SIZE4[get_player_number(module_accessor)] += 0.5;
            }
            AttackModule::set_attack_composition_speed(module_accessor, 0, true);
            AttackModule::set_attack_composition_speed(module_accessor, 1, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
    }
    for _ in 0..5 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.5, 367, 100, 120, 0, SIZE1[get_player_number(module_accessor)], 0.0, 5.0, 6.5, Some(0.0), Some(-1.0), Some(6.5), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.5, 363, 100, 130, 0, SIZE2[get_player_number(module_accessor)], 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            ATTACK(fighter, 2, 0, Hash40::new("rot"), 1.5, 363, 100, 180, 0, SIZE3[get_player_number(module_accessor)], 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                SIZE1[get_player_number(module_accessor)] += 0.5;
                SIZE2[get_player_number(module_accessor)] += 0.5;
                SIZE3[get_player_number(module_accessor)] += 0.5;
                SIZE4[get_player_number(module_accessor)] += 0.5;
            }
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 50, 74, 0, 90, SIZE4[get_player_number(module_accessor)], 1.0, -1.0, 0.0, None, None, None, 1.3, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_LAST_ATTACK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(module_accessor, true);
        GroundModule::select_cliff_hangdata(module_accessor, *FIGHTER_LUCAS_CLIFF_HANG_DATA_START as u32);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn lucas_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40{hash: hash40("catch") }, false, 0.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.5, Some(0.0), Some(0.0), Some(-6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("armr"), 3.0, 3.5, 0.5, 0.0, Some(8.5), Some(3.8), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        GrabModule::set_rebound(module_accessor, false);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "lucas", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn lucas_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40{hash: hash40("catch_dash") }, false, 0.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("armr"), 3.0, 3.5, 0.5, 0.0, Some(8.5), Some(3.8), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        GrabModule::set_rebound(module_accessor, false);
    }
    frame(lua_state, 80.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "lucas", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn lucas_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40{hash: hash40("catch_turn") }, false, 0.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("armr"), 3.0, 3.5, 0.5, 0.0, Some(8.5), Some(3.8), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        GrabModule::set_rebound(module_accessor, false);
    }
    frame(lua_state, 76.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "lucas_pkstarstorm", script = "game_move", category = ACMD_GAME, low_priority)]
unsafe fn lucas_pkstarstorm_move(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 38, 90, 0, 80, 12.0, 0.0, 12.0, 0.0, Some(0.0), Some(40.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, false, -1.0, false);
    }
    
}

#[acmd_script( agent = "lucas", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn lucas_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, false, 0);
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, false, 0);
        let kumatora_pos = Vector3f{x: PostureModule::pos_x(module_accessor) - 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		let bonnie_pos = Vector3f{x: PostureModule::pos_x(module_accessor) + 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		ArticleModule::set_pos(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, kumatora_pos);
		ArticleModule::set_pos(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, bonnie_pos);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 5.0, 60.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
    }
    frame(lua_state, 51.0);
    if is_excute(fighter) {
        FINAL_TRANSFORM[get_player_number(module_accessor)] = 303;
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_FINAL_START);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(module_accessor);
    }
    frame(lua_state, 354.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "lucas", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn lucas_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, false, 0);
        ArticleModule::generate_article(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, false, 0);
        let kumatora_pos = Vector3f{x: PostureModule::pos_x(module_accessor) - 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		let bonnie_pos = Vector3f{x: PostureModule::pos_x(module_accessor) + 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		ArticleModule::set_pos(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, kumatora_pos);
		ArticleModule::set_pos(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, bonnie_pos);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 5.0, 60.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
    }
    frame(lua_state, 51.0);
    if is_excute(fighter) {
        FINAL_TRANSFORM[get_player_number(module_accessor)] = 303;
        WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_FINAL_START);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(module_accessor);
    }
    frame(lua_state, 354.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_KUMATORA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_BONNIE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
pub fn lucas_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
            if !WorkModule::is_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_FINAL_START) {
                WorkModule::on_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_FINAL_START);
            }
            FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
        }
        else {
            WorkModule::off_flag(module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_FINAL_START);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucas_attack11,
        lucas_attack12,
        lucas_attack13,
        lucas_attacklw3,
        lucas_attackairn,
        lucas_attackairb,
        lucas_attackairlw,
        lucas_specialairs,
        lucas_specialairhi,
        lucas_catch,
        lucas_catchdash,
        lucas_catchturn,
        lucas_final,
        lucas_finalair,
        lucas_pkstarstorm_move
    );
    smashline::install_agent_frames!(
		lucas_functions
	);
}

