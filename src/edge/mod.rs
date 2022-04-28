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


#[acmd_script( agent = "edge", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn edge_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.7, Some(0.0), Some(8.5), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "edge", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn edge_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.7, Some(0.0), Some(7.0), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "edge", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn edge_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.0, -4.7, Some(0.0), Some(7.0), Some(-17.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "edge", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn edge_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 30.0, 60.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 50);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 59.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 8.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 8.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 8.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 8.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 11.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(43.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 11.0, 0.0, 18.0, 12.0, Some(0.0), Some(23.0), Some(22.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 11.0, 0.0, 10.0, 12.0, Some(0.0), Some(21.0), Some(35.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 62.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 60, 70, 0, 70, 16.0, 0.0, 20.0, 10.0, Some(0.0), Some(28.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 63.0);
    if is_excute(fighter) {
        AttackModule::clear(module_accessor, 1, false);
        AttackModule::clear(module_accessor, 3, false);
    }
    frame(lua_state, 65.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        edge_catch,
        edge_catchdash,
        edge_catchturn,
        edge_final
    );
}

