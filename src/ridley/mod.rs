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
use crate::custom::FIGHTER_FLOAT_1;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;
use crate::custom::estimate_frame;


#[acmd_script( agent = "ridley", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn ridley_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("wingr2"), 7.0, 88, 130, 0, 30, 6.5, 1.5, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("wingr3"), 9.0, 88, 110, 0, 30, 5.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("wingr6"), 7.0, 88, 130, 0, 30, 6.5, -5.7, 2.0, -3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 0, Hash40::new("wingr6"), 7.0, 88, 130, 0, 30, 6.5, 0.0, 2.0, -2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 7.0, 88, 130, 0, 30, 7.0, 0.0, 13.0, 0.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
		AttackModule::clear(module_accessor, 4, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_NORMAL);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn ridley_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 17.0, 82, 78, 0, 58, 4.5, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 17.0, 82, 78, 0, 58, 4.5, 0.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("footr"), 17.0, 82, 78, 0, 58, 6.0, -1.5, -1.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 9.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_NORMAL);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_speciallwstab", category = ACMD_GAME, low_priority)]
unsafe fn ridley_speciallwstab(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) == 0 {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 361, 100, 20, 0, 2.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_DISABLE_CRITICAL_SPECIAL_LW) ==false {
			ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 361, 30, 0, 50, 2.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_specialairlwstab", category = ACMD_GAME, low_priority)]
unsafe fn ridley_specialairlwstab(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) == 0 {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 361, 100, 20, 0, 2.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_DISABLE_CRITICAL_SPECIAL_LW) ==false {
			ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 361, 30, 0, 50, 2.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn ridley_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 7.5, 6.0, Some(0.0), Some(7.5), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn ridley_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 7.5, 7.0, Some(0.0), Some(7.5), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn ridley_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 7.5, -5.0, Some(0.0), Some(7.5), Some(-18.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ridley", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn ridley_finaldash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, *FIGHTER_ATTACK_KIND_RIDLEY_FINAL as u64, 0, Hash40::new("top"), 15.0, 50, 100, 130, 0, 13.0, 0.0, 13.0, -5.0, Some(0.0), Some(13.0), Some(5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
pub fn ridley_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let DOMINANCE_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_RIDLEY {
			if motion_kind == hash40("jump_aerial_f3") {
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("jump_aerial_f2")}, 1.0, 1.0, false, 0.0, false, false);
			}
			if motion_kind == hash40("dominance") {
				if MotionModule::frame(module_accessor) >= 50.0 {
					CancelModule::enable_cancel(module_accessor);
				}
			}
			if motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_lw_l") {
				if MotionModule::frame(module_accessor) >= 23.0 && MotionModule::frame(module_accessor) < 69.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
						*DOMINANCE_FRAME = MotionModule::frame(module_accessor);
						MotionModule::set_rate(module_accessor, 0.25);
						MotionModule::set_frame(module_accessor, 100.0, true);
					}
				}
				if estimate_frame(module_accessor, 90.0) {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
				}
				if estimate_frame(module_accessor, 104.0) {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("dominance")}, *DOMINANCE_FRAME, 1.0, false, 0.0, false, false);	
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
			if AttackModule::is_attack(module_accessor, 0, false) {
				AttackModule::set_attack_scale(module_accessor, 1.06, false);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		ridley_attackhi3,
		ridley_attackhi4,
		ridley_speciallwstab,
		ridley_specialairlwstab,
		ridley_catch,
		ridley_catchdash,
		ridley_catchturn,
		ridley_finaldash
	);
	smashline::install_agent_frames!(ridley_functions);
}

