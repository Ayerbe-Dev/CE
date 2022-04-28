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

#[acmd_script( agent = "reflet", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn reflet_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 1.8, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 1.8, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword"), 2.0, 290, 25, 0, 25, 3.0, 2.0, 0.0, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("sword"), 2.0, 290, 25, 0, 25, 3.5, 3.0, 4.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "reflet", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn reflet_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 3.3, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 4.0, 0.0, 8.5, 12.0, Some(0.0), Some(8.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 180, 20, 0, 20, 4.0, 0.0, 8.5, 15.5, Some(0.0), Some(8.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 4.0, 0.0, 8.5, 15.5, Some(0.0), Some(8.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("sword"), 1.5, 361, 25, 0, 25, 4.0, 2.0, 0.0, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("sword"), 1.5, 361, 25, 0, 25, 4.5, 3.0, 4.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        if WorkModule::get_int(module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) >= 1 {
            WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if WorkModule::get_float(module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) >= 0.1 {
            WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}


#[acmd_script( agent = "reflet", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn reflet_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.5, 4.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 0.0, 5.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 70, 60, 0, 40, 3.5, 0.0, 3.0, 11.0, Some(0.0), Some(5.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 5.0, 5.0);
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "reflet", script = "game_specialhi2", category = ACMD_GAME, low_priority)]
unsafe fn reflet_specialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 18.0);
    if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if is_excute(fighter) {
            ArticleModule::generate_article(module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
            WorkModule::on_flag(module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
            notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    
}

#[acmd_script( agent = "reflet", script = "game_specialairhi2", category = ACMD_GAME, low_priority)]
unsafe fn reflet_specialairhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 18.0);
    if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if is_excute(fighter) {
            ArticleModule::generate_article(module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
            WorkModule::on_flag(module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
            notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    
}

#[acmd_script( agent = "reflet_thunder", script = "game_shoot0", category = ACMD_GAME, low_priority)]
unsafe fn reflet_thunder_shoot0(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_beams") }, 0, false, 0 as u32);
    ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 50, 20, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
    AttackModule::enable_safe_pos(module_accessor);
    
}

#[acmd_script( agent = "reflet", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn reflet_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "reflet", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn reflet_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "reflet", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn reflet_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-15.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "reflet_chrom", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn reflet_chrom_finaldash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 22, 100, 50, 0, 8.0, 0.0, 7.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
pub fn reflet_functions(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = app::utility::get_kind(module_accessor) as i32;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let mut globals = fighter.globals_mut().clone();

        if fighter_kind == FIGHTER_KIND_REFLET {
            if let L2CValueType::Void = globals["robin_wind_meter"].val_type {
                globals["robin_wind_meter"] = 0.into();
            }
            if let L2CValueType::Void = globals["robin_levin_reset"].val_type {
                globals["robin_levin_reset"] = false.into();
            }        
            if sv_information::is_ready_go() == false {
                globals["robin_levin_reset"] = true.into();
            }
            else {
                if globals["robin_levin_reset"].get_bool() {
                    WorkModule::set_int(module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                    globals["robin_levin_reset"] = false.into();
                }
            }
            if motion_kind == hash40("special_air_lw_end") {
                if MotionModule::frame(module_accessor) == 12.0 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
            if motion_kind == hash40("special_air_hi") || motion_kind == hash40("special_hi") {
                if MotionModule::frame(module_accessor) == 2.0 {
                    globals["robin_wind_meter"] = (WorkModule::get_int(module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT)).into();
                }
            }
            if motion_kind == hash40("special_air_hi2") || motion_kind == hash40("special_hi2"){
                if MotionModule::frame(module_accessor) == 1.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::set_int(module_accessor, globals["robin_wind_meter"].get_int() as i32, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
                    }
                }
            }
        }
    
    }
}

pub fn install() {
    install_acmd_scripts!(
        reflet_attack11,
        reflet_attack12,
        reflet_attacklw3,
        reflet_specialhi2,
        reflet_specialairhi2,
        reflet_thunder_shoot0,
        reflet_catch,
        reflet_catchdash,
        reflet_catchturn,
        reflet_chrom_finaldash
    );
    smashline::install_agent_frames!(reflet_functions);
}

