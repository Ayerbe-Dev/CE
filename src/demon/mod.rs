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
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::TOTAL_FIGHTER;


#[acmd_script( agent = "demon", script = "game_squat", category = ACMD_GAME, low_priority)]
unsafe fn demon_squat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

#[acmd_script( agent = "demon", script = "game_attackstep2f", category = ACMD_GAME, low_priority )]
unsafe fn demon_attackstep2f(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("handr"), 14.0, 81, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 5, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 0, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 1, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 2, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 3, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 4, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 5, 13.0, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 4, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 5, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("handr"), 14.0, 81, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 0, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 1, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 2, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 3, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 4, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 5, 13.0, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 4, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 5, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("handr"), 14.0, 81, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, 61400, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 0, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 1, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 2, 18.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 3, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 4, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 5, 13.0, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 4, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(module_accessor, 5, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 0, false);
        AttackModule::clear(module_accessor, 1, false);
        AttackModule::clear(module_accessor, 2, false);
        AttackModule::clear(module_accessor, 3, false);
        AttackModule::clear(module_accessor, 4, false);
        AttackModule::clear(module_accessor, 5, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 6, false);
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    FT_MOTION_RATE(fighter, 0.9);
    
}

#[acmd_script( agent = "demon", script = "game_attackstand6", category = ACMD_GAME, low_priority )]
unsafe fn demon_attackstand6(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR);
        WorkModule::on_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 48, 65, 0, 72, 2.0, 0.0, 15.0, 11.25, None, None, None, 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 48, 65, 0, 72, 4.0, 0.0, 14.0, 10.5, None, None, None, 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.5, 48, 65, 0, 72, 3.5, 0.0, 13.25, 6.5, None, None, None, 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 13.5, 48, 65, 0, 72, 3.5, 0.0, 12.5, 3.0, None, None, None, 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 13.5, 48, 65, 0, 72, 3.0, 0.0, 11.5, 8.0, Some(0.0), Some(3.0), Some(3.0), 0.35, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackstand1", category = ACMD_GAME, low_priority )]
unsafe fn demon_attackstand1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 10.5, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 11.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 4.2, 0.0, 10.5, 2.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 2, 10.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.1);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 7.25, 7.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 11.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 4.2, 0.0, 10.5, 2.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 61404, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(module_accessor, 2, 10.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.1);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
pub fn demon_functions(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
    
        if fighter_kind == FIGHTER_KIND_DEMON {
            if status_kind == *FIGHTER_STATUS_KIND_DASH {
                if MotionModule::frame(module_accessor) >= 10.0 && ControlModule::get_stick_y(module_accessor) <= -0.66 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT_WAIT, true);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH || status_kind == *FIGHTER_DEMON_STATUS_KIND_DASH_BACK {
                if MotionModule::frame(module_accessor) >= 5.0 && ControlModule::get_stick_y(module_accessor) <= -0.66 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT_WAIT, true);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SQUAT_RV {
                if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) <= -0.85 && ControlModule::get_flick_x(module_accessor) <= 2 {
                    if TOTAL_FIGHTER > 2 {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH, true);
                    }
                    else {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, true);
                    }
                }
            }
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_squat,
        demon_attackstep2f,
        demon_attackstand1,
        demon_attackstand6
    );
    smashline::install_agent_frames!(demon_functions);
}

