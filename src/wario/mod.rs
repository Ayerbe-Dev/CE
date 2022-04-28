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
use crate::custom::FINAL_TRANSFORM;
use std::mem;


#[acmd_script( agent = "wario", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn wario_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 20, 0, 40, 3.2, 0.0, 6.3, 7.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 20, 0, 40, 3.2, 0.0, 5.6, 9.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 180, 20, 0, 38, 3.8, 0.0, 4.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
	}
	
}

#[acmd_script( agent = "wario", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn wario_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 92, 130, 0, 48, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 92, 130, 0, 48, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 92, 130, 0, 48, 4.0, 0.0, 14.5, 3.5, Some(0.0), Some(14.5), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 92, 130, 0, 48, 4.0, 0.0, 10.5, 3.5, Some(0.0), Some(10.5), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "wario", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn wario_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 6.0, 40, 100, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 40, 100, 0, 30, 3.0, 1.0, 0.0, 0.0, Some(2.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("hip"), 6.0, 40, 100, 0, 30, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 5.0, 4.0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		ATK_POWER(fighter, 0, 3);
		ATK_POWER(fighter, 1, 3);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 4.0, 70, 100, 0, 65, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	frame(lua_state, 27.0);
	if is_excute(fighter) {
		ATK_POWER(fighter, 0, 5);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "wario", script = "game_speciallwlr", category = ACMD_GAME, low_priority)]
unsafe fn wario_speciallwlr(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 45, 68, 0, 50, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		WorkModule::on_flag(module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "wario", script = "game_specialairlwlr", category = ACMD_GAME, low_priority)]
unsafe fn wario_specialairlwlr(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 45, 68, 0, 50, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		WorkModule::on_flag(module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "wario", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn wario_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(10.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.96);
	}
	
}

#[acmd_script( agent = "wario", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn wario_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
		MotionModule::set_rate(module_accessor, 0.97);
	}
	
}

#[acmd_script( agent = "wario", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn wario_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-15.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "wario", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn wario_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.1);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, 0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 20.0, 55.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 75);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 901;
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
}

#[acmd_script( agent = "wario", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn wario_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.1);
		ArticleModule::generate_article(module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, 0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 20.0, 55.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 75);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 721;
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
}

#[acmd_script( agent = "wario", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn wario_finaldash(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "wario", script = "game_finaldashend", category = ACMD_GAME, low_priority)]
unsafe fn wario_finaldashend(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "wario", script = "game_finalairdashend", category = ACMD_GAME, low_priority)]
unsafe fn wario_finalairdashend(fighter: &mut L2CAgentBase) {}

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
pub fn wario_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;

		if fighter_kind == *FIGHTER_KIND_WARIO {
			if FINAL_TRANSFORM[get_player_number(module_accessor)] > 0 {
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					MotionModule::set_rate(module_accessor, 1.5);
				}
				if FINAL_TRANSFORM[get_player_number(module_accessor)] != 1 {
					FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
				}
				else {
					if CancelModule::is_enable_cancel(module_accessor) {
						FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_WARIO_STATUS_KIND_FINAL_END, true);
					}
				}
			}
			if status_kind == *FIGHTER_WARIO_STATUS_KIND_FINAL_END {
				if PostureModule::lr(module_accessor) >= 0.0 {
					PostureModule::set_lr(module_accessor, 1.0);
				}
				else {
					PostureModule::set_lr(module_accessor, -1.0);
				}
				PostureModule::update_rot_y_lr(module_accessor);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		wario_attack11,
		wario_attackhi3,
		wario_attackairn,
		wario_speciallwlr,
		wario_specialairlwlr,
		wario_catch,
		wario_catchdash,
		wario_catchturn,
		wario_finalstart,
		wario_finalairstart,
		wario_finaldash,
		wario_finaldashend,
		wario_finalairdashend
	);
	smashline::install_agent_frames!(
		wario_functions
	);
}

