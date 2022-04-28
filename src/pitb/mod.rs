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


#[acmd_script( agent = "pitb", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attacks3s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 100, 0, 40, 3.5, 0.0, 7.5, 20.0, Some(0.0), Some(7.5), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attacks3", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attacks3s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x0f666250b2u64), Hash40::new_raw(0x0fff6b0108u64), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.200000003, Hash40::new("swordr1"), 0.0, 15.3999996, -1.20000005, true, Hash40::new_raw(0x0a702f9287u64), Hash40::new("swordr1"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.39999998, 0.100000001);
        EffectModule::set_rgb_partial_last(module_accessor, 0.8, 0.0, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 365, 100, 85, 0, 4.0, 0.0, 24.0, 2.3, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        let hitVec = Vector2f{ x: 3.8, y: 23.5 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40{ hash: hash40("top") }, &hitVec, 10.0 as u32, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 83, 30, 0, 70, 6.0, 0.0, 24.0, 5.0, Some(0.0), Some(22.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 83, 30, 0, 70, 6.0, 0.0, 10.0, 2.3, Some(0.0), Some(24.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        MotionModule::set_rate(module_accessor, 1.8);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attacks4s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 100, 100, 20, 0, 6.0, 0.0, 7.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 70, 100, 20, 0, 6.0, 0.0, 7.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 35, 100, 53, 0, 6.0, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 0, 100, 15, 0, 6.0, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 10.0, 361, 122, 0, 42, 6.0, 0.0, 7.5, 13.5, Some(0.0), Some(7.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 10.0, 361, 122, 0, 42, 6.0, 0.0, 8.5, 13.5, Some(0.0), Some(8.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        SIZE1[get_player_number(module_accessor)] = 4.0;
        SIZE2[get_player_number(module_accessor)] = 5.0;
        SIZE3[get_player_number(module_accessor)] = 5.2;
        SIZE4[get_player_number(module_accessor)] = 5.8;
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 365, 100, 20, 40, 4.0, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 120, 100, 20, 40, 4.5, 0.0, 24.0, 8.0, Some(0.0), Some(24.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 120, 100, 20, 40, 4.5, 0.0, 24.0, -8.0, Some(0.0), Some(24.0), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 367, 100, 33, 40, 4.5, 0.0, 24.0, 8.0, Some(0.0), Some(24.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 105, 100, 33, 40, 4.5, 0.0, 24.0, 8.0, Some(0.0), Some(24.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        let hitVec = Vector2f{ x: 0.0, y: 24.5 };
        AttackModule::set_vec_target_pos(module_accessor, 3, Hash40{ hash: hash40("top") }, &hitVec, 10.0 as u32, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 140, 0, 4.0, 0.0, 14.0, 10.0, Some(0.0), Some(14.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 123, 100, 140, 0, 4.0, 0.0, 14.0, 10.0, Some(0.0), Some(14.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 2, false);
        AttackModule::clear(module_accessor, 3, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 140, 0, 4.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 123, 100, 140, 0, 4.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
            SIZE3[get_player_number(module_accessor)] += 3.0;
            SIZE4[get_player_number(module_accessor)] += 3.0;
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 98, 100, 20, 0, SIZE1[get_player_number(module_accessor)], 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 2.0, 120, 100, 50, 0, SIZE3[get_player_number(module_accessor)], 0.0, 24.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 2, 1, Hash40::new("top"), 2.0, 120, 100, 50, 0, SIZE3[get_player_number(module_accessor)], 0.0, 24.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 3, 1, Hash40::new("top"), 2.0, 120, 100, 50, 0, SIZE3[get_player_number(module_accessor)], 0.0, 18.0, -9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 3.0;
            SIZE2[get_player_number(module_accessor)] += 3.0;
            SIZE3[get_player_number(module_accessor)] += 3.0;
            SIZE4[get_player_number(module_accessor)] += 3.0;
        }
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 2, Hash40::new("top"), 8.0, 89, 111, 0, 66, SIZE4[get_player_number(module_accessor)], 0.0, 34.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 2, Hash40::new("top"), 8.0, 89, 111, 0, 66, SIZE2[get_player_number(module_accessor)], 0.0, 24.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 2, 2, Hash40::new("top"), 8.0, 89, 111, 0, 66, SIZE2[get_player_number(module_accessor)], 0.0, 31.0, 7.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 3, 2, Hash40::new("top"), 8.0, 89, 111, 0, 66, SIZE2[get_player_number(module_accessor)], 0.0, 31.0, -8.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 367, 100, 50, 0, 4.5, 0.0, 4.6, 0.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.7, 367, 100, 30, 0, 4.5, 0.0, 13.0, 1.2, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.7, 367, 100, 30, 0, 4.5, 0.0, 12.5, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.7, 140, 100, 30, 0, 4.5, 0.0, 12.5, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 0.7, 367, 100, 65, 0, 4.5, 0.0, 4.6, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 4.5, 361, 100, 0, 60, 9.5, 0.0, 9.5, 3.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attackairn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14abd82a05u64), Hash40::new("bowr"), 0.0, 5.5, 0.0, 0.0, 45.0, 90.0, 1.0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x14abd82a05u64), false, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 100, 0, 20, 2.5, 0.0, 5.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 100, 0, 20, 2.5, 0.0, 5.0, 6.0, Some(0.0), Some(5.0), Some(21.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attackairf", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attackairf_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0dea206f78u64), Hash40::new("top"), -7, 6, 2.29999995, -12, -42, 168, 1.5, true);
        EffectModule::set_rgb_partial_last(module_accessor, 0.8, 0.0, 1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0a702f9287u64), true, true);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn pitb_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.223);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            MotionModule::set_rate(module_accessor, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 2.8, 0.0, 9.6, 0.0, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 20, 0, 3.8, 0.0, 19.0, 0.0, None, None, None, 0.8, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 60, 0, 3.8, 0.0, 16.799999, -8.0, Some(0.0), Some(9.6), Some(0.0), 0.8, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 60, 0, 3.8, 0.0, 16.799999, 8.0, Some(0.0), Some(9.6), Some(0.0), 0.8, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 143, 0, 50, 4.5, 0.0, 9.6, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 80, 143, 0, 50, 6.1, 0.0, 16.799999, -8.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 80, 143, 0, 50, 6.1, 0.0, 16.799999, 8.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 80, 143, 0, 50, 6.1, 0.0, 17.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attackairhi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14abd82a05u64), Hash40::new("swordr2"), 0.0, 0.0, 0.0, -90.0, 90.0, 0.0, 1.0, true);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x14abd82a05u64), false, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_specialnholds", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialnholds_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    for _ in 0..12 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_specialairnholds", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialairnholds_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialnholdhi", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialnholdhi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    for _ in 0..12 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_specialairnholdhi", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialairnholdhi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialnhitos", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialnhitos_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialnstohi", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialnstohi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialnstos", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialnstos_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialairnhitos", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialairnhitos_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialairnstohi", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialairnstohi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "effect_specialairnstos", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_specialairnstos_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
}

#[acmd_script( agent = "pitb", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn pitb_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        JostleModule::set_status(module_accessor, false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(module_accessor, 2.0);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.0);
        WorkModule::on_flag(module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        JostleModule::set_status(module_accessor, true);
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_speciallwhold", category = ACMD_GAME, low_priority)]
unsafe fn pitb_speciallwhold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 0, 100, 80, 0, 2.7, 0.0, 0.0, 5.0, Some(5.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 0, 100, 80, 0, 2.7, 0.0, 0.0, -5.0, Some(-5.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_specialairlwhold", category = ACMD_GAME, low_priority)]
unsafe fn pitb_specialairlwhold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 0, 100, 80, 0, 2.7, 0.0, 0.0, 5.0, Some(5.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 0, 100, 80, 0, 2.7, 0.0, 0.0, -5.0, Some(-5.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn pitb_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.1, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn pitb_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 2.5, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn pitb_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.1, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-16.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "pitb_bowarrow", script = "game_fly", category = ACMD_GAME, low_priority)]
unsafe fn pitb_bowarrow_fly(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        if WorkModule::is_flag(owner_module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_FLAG_DIR_S) {
            let added_pos = Vector3f{ x: 0.0, y: 1.2, z: 0.0 };
            PostureModule::add_pos(module_accessor, &added_pos);
        }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 45, 88, 0, 4, 1.3, 0.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::enable_safe_pos(module_accessor);
    }
    
}

#[acmd_script( agent = "pitb_bowarrow", script = "effect_fly", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_bowarrow_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x114907c472u64), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x12a497a9d9u64), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn pitb_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 3.0, 30.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 30);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 5, 0);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 1.0, 80, 100, 50, 0, 7.0, 0.0, 300.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_damage_shake_scale(module_accessor, 0.3);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SEARCH(fighter, 0, 0, Hash40::new("haver"), 7.0, 0.0, 300.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
        WorkModule::on_flag(module_accessor, *FIGHTER_PITB_STATUS_FINAL_FLAG_SHOOT);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_PITB_FINAL, 0, 39.0, 361, 53, 0, 63, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn pitb_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 3.0, 30.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 30);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 5, 0);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 1.0, 80, 100, 50, 0, 7.0, 0.0, 300.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_damage_shake_scale(module_accessor, 0.3);
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SEARCH(fighter, 0, 0, Hash40::new("haver"), 7.0, 0.0, 300.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        }
        WorkModule::on_flag(module_accessor, *FIGHTER_PITB_STATUS_FINAL_FLAG_SHOOT);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_PITB_FINAL, 0, 39.0, 361, 53, 0, 63, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
pub fn pitb_functions(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = app::utility::get_kind(module_accessor) as i32;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let mut globals = fighter.globals_mut().clone();
        
        if fighter_kind == FIGHTER_KIND_PITB {
            if let L2CValueType::Void = globals["staff_visible"].val_type {
                globals["staff_visible"] = false.into();
            }
            if sv_information::is_ready_go() == false {
                globals["staff_visible"] = false.into();
            }
            if ArticleModule::is_exist(module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOW) {
                ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOW, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
            if motion_kind == hash40("attack_lw3")
                || motion_kind == hash40("attack_s3_s")
                || motion_kind == hash40("attack_air_n")
                || motion_kind == hash40("attack_air_f")
                || motion_kind == hash40("attack_air_hi")
                || motion_kind == hash40("special_n_start")
                || motion_kind == hash40("special_air_n_start")
                || motion_kind == hash40("appeal_lw_r")
                || motion_kind == hash40("appeal_lw_l") {
                    globals["staff_visible"] = true.into();
            }
            if globals["staff_visible"].get_bool() {
                if motion_kind != hash40("guard_on")
                    && motion_kind != hash40("appeal_hi")
                    && motion_kind != hash40("appeal_s_r")
                    && motion_kind != hash40("appeal_s_l")
                    && motion_kind != hash40("appeal_hi_r")
                    && motion_kind != hash40("appeal_hi_r")
                    && motion_kind != hash40("catch")
                    && motion_kind != hash40("catch_dash")
                    && motion_kind != hash40("catch_turn")
                    && motion_kind != hash40("attack_11")
                    && motion_kind != hash40("attack_hi3")
                    && motion_kind != hash40("attack_s4_s")
                    && motion_kind != hash40("attack_hi4")
                    && motion_kind != hash40("attack_lw4")
                    && motion_kind != hash40("attack_air_b")
                    && motion_kind != hash40("attack_air_lw")
                    && motion_kind != hash40("special_lw_hold")
                    && motion_kind != hash40("special_air_lw_hold")
                    && motion_kind != hash40("special_s_start")
                    && motion_kind != hash40("special_air_s_start") {
                    VisibilityModule::set_int64(module_accessor, hash40("weapon") as i64, hash40("weapon_final") as i64);
                    VisibilityModule::set_status_default_int64(module_accessor, hash40("weapon") as i64, hash40("weapon_final") as i64);
                }
                else {
                    globals["staff_visible"] = false.into();
                }
            }
            if (motion_kind == hash40("special_n_fire_s")
            || motion_kind == hash40("special_n_fire_hi")) && MotionModule::frame(module_accessor) >= 52.0 {
                MotionModule::change_motion(module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
            if (motion_kind == hash40("special_air_n_fire_s")
            || motion_kind == hash40("special_air_n_fire_hi")) && MotionModule::frame(module_accessor) >= 52.0 {
                MotionModule::change_motion(module_accessor, Hash40{hash: hash40("fall")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        pitb_attacks3s,
        pitb_attacks3s_effect,
        pitb_attackhi3,
        pitb_attacks4s,
        pitb_attackhi4,
        pitb_attackairn,
        pitb_attackairn_effect,
        pitb_attackairf,
        pitb_attackairf_effect,
        pitb_attackairhi,
        pitb_attackairhi_effect,
        pitb_specialnholds_effect,
        pitb_specialairnholds_effect,
        pitb_specialnholdhi_effect,
        pitb_specialairnholdhi_effect,
        pitb_specialnhitos_effect,
        pitb_specialnstohi_effect,
        pitb_specialnstos_effect,
        pitb_specialairnhitos_effect,
        pitb_specialairnstohi_effect,
        pitb_specialairnstos_effect,
        pitb_specialhi,
        pitb_speciallwhold,
        pitb_specialairlwhold,
        pitb_catch,
        pitb_catchdash,
        pitb_catchturn,
        pitb_bowarrow_fly,
        pitb_bowarrow_fly_effect,
        pitb_final,
        pitb_finalair
    );
    smashline::install_agent_frames!(pitb_functions);
}

