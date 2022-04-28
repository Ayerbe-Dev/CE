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
use crate::custom::{get_player_number, get_attacker_number, change_aegis_without_special_lw};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;
use crate::custom::FIGHTER_INT_1;

#[acmd_script( agent = "elight", script = "game_landingairn", category = ACMD_GAME, low_priority )]
unsafe fn elight_landingairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 64, 105, 0, 65, 3.0, 0.0, 3.0, -9.0, Some(0.0), Some(3.0), Some(9.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 0, 0.2);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "elight", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn elight_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 2.5, 0.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 2.5, 0.0, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 7.0, 0.0, 7.0, -5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 4, false);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 3.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	FT_MOTION_RATE(fighter, 0.75);
	frame(lua_state, 43.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 79.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	
}

#[acmd_script( agent = "elight", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn elight_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
        if MotionModule::is_changing(module_accessor) {
            WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 2.5, 0.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 4.0, -1.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 3.5, -2.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 2.5, -3.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 6.0, 81, 70, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
            ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
        if MotionModule::is_changing(module_accessor) {
            WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "elight", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn elight_specialn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 10, 0, 5.0, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 5.0, 0.0, 6.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 9.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 5.0, 0.0, 6.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 9.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 10, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 20, 10, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 95, 100, 20, 10, 2.8, 11.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 95, 100, 20, 10, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 100, 100, 20, 10, 6.0, 0.0, 10.0, 15.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 90, 100, 20, 10, 6.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 10, 0, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 3.5, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 15.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 18.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 42, 84, 0, 75, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 42.0);
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 52.0);
	FT_MOTION_RATE(fighter, 1.0);
	
}

#[acmd_script( agent = "elight", script = "game_specialn2", category = ACMD_GAME, low_priority)]
unsafe fn elight_specialn2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 10.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 110, 100, 20, 10, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 366, 100, 20, 0, 7.0, 0.0, 10.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 15, 100, 20, 15, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 15, 100, 20, 15, 3.5, 8.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 100, 100, 20, 15, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 50, 10, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 2.0, 366, 100, 50, 20, 6.0, 0.0, 11.0, 22.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 7, 0, Hash40::new("top"), 2.0, 15, 100, 20, 15, 4.0, 0.0, 11.0, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 6, 2);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 6.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 2);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 14.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 366, 100, 50, 0, 5.0, 0.0, 13.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 2);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	
}

#[acmd_script( agent = "elight", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn elight_specialairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 10, 7.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(15.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 2.0, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 16.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 10, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 10, 3.5, 6.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 10, 2.8, 9.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 10, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 90, 100, 30, 10, 5.0, 0.0, 7.0, 17.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 45, 100, 40, 20, 5.0, 0.0, 7.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 2.0, 366, 100, 80, 30, 5.0, 0.0, 13.0, 8.0, Some(0.0), Some(13.0), Some(17.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 4, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 5, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 366, 100, 60, 10, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 366, 100, 60, 10, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 366, 100, 60, 10, 2.8, 10.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 60, 10, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 50, 10, 6.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(16.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 3.5, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 15.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 6.0, 42, 84, 0, 75, 2.8, 18.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 42, 84, 0, 75, 7.0, 0.0, 9.0, 13.0, Some(0.0), Some(9.0), Some(22.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 42.0);
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 52.0);
	FT_MOTION_RATE(fighter, 1.0);
	frame(lua_state, 54.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	
}

#[acmd_script( agent = "elight", script = "game_specialairn2", category = ACMD_GAME, low_priority)]
unsafe fn elight_specialairn2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 10, 7.0, 0.0, 7.0, 10.0, Some(0.0), Some(7.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 100, 15, 0, 2.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(20.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 10.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 90, 100, 60, 20, 5.0, 0.0, 5.0, 20.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 4.0, 1.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 3.5, 8.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 366, 100, 80, 0, 2.8, 14.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 90, 100, 30, 10, 5.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 366, 100, 80, 10, 5.0, 0.0, 13.0, 8.0, Some(0.0), Some(13.0), Some(24.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 4.0, 2.5, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 3.5, 7.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 12.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 2.0, 90, 100, 10, 0, 2.8, 15.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 366, 100, 80, 0, 5.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(22.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 4.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.5, 42, 78, 0, 55, 2.8, 15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 8.5, 42, 78, 0, 55, 7.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		if ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
			ArticleModule::add_motion_partial(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false);
		}
		if MotionModule::is_changing(module_accessor) {
			WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
		}
	}
	frame(lua_state, 54.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL);
	}
	
}

#[acmd_script( agent = "elight", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn elight_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "elight", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn elight_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "elight", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn elight_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -6.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "elight", script = "game_throwf", category = ACMD_GAME, low_priority )]
unsafe fn elight_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.5, 20, 75, 0, 58, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 130, 0, 30, 6.0, 0.0, 11.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 13, 8);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
		lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 5.0, y: 0.0, z: 0.0 });
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
pub fn elight_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if fighter_kind == *FIGHTER_KIND_ELIGHT {
			if status_kind == *FIGHTER_STATUS_KIND_THROW {
				if motion_kind == hash40("throw_f") {
					if MotionModule::frame(module_accessor) >= 11.0 {
						change_aegis_without_special_lw(fighter);
					}
				}
				if motion_kind == hash40("throw_b") {
					if MotionModule::frame(module_accessor) >= 17.0 {
						REVERSE_LR(fighter);
						change_aegis_without_special_lw(fighter);
					}
				}
				if motion_kind == hash40("throw_hi") {
					if MotionModule::frame(module_accessor) >= 9.0 {
						change_aegis_without_special_lw(fighter);
					}
				}
				if motion_kind == hash40("throw_lw") {
					if MotionModule::frame(module_accessor) >= 25.0 {
						change_aegis_without_special_lw(fighter);
					}
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		elight_landingairn,
		elight_attackairb,
		elight_attackairhi,
		elight_specialn,
		elight_specialn2,
		elight_specialairn,
		elight_specialairn2,
		elight_catch,
		elight_catchdash,
		elight_catchturn,
		elight_throwf
	);
	smashline::install_agent_frames!(elight_functions);
}

