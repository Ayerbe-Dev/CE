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
use crate::custom::FINAL_HIT;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;


#[acmd_script( agent = "yoshi", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn yoshi_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.9);
	frame(lua_state, 7.0);
	FT_MOTION_RATE(fighter, 1);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail2"), 9.0, 100, 65, 0, 72, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail2"), 9.0, 100, 65, 0, 72, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail2"), 9.0, 100, 65, 0, 72, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail2"), 7.0, 100, 45, 0, 72, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail2"), 7.0, 100, 45, 0, 72, 4.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail1"), 7.0, 100, 45, 0, 72, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "yoshi", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn yoshi_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 35, 4.8, 0.0, 3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 86, 0, 40, 4.8, 0.0, 4.3, 7.5, Some(0.0), Some(3.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 100, 0, 30, 3.6, 0.0, 4.5, 5.0, Some(0.0), Some(3.7), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::clear(module_accessor, 1, false);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 30, 3.0, 0.0, 4.8, 3.5, Some(0.0), Some(4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "yoshi", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn yoshi_finaldash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 130, 50, 0, 8.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(16.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "yoshi", script = "game_finalhit", category = ACMD_GAME, low_priority)]
unsafe fn yoshi_finalhit(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, *FIGHTER_ATTACK_KIND_VISUAL_SCENE as u64, 1, Hash40::new("top"), 4.0, 70, 40, 0, 90, 12.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(20.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
			AttackModule::set_no_dead_all(module_accessor, true, false);    
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, *FIGHTER_ATTACK_KIND_VISUAL_SCENE as u64, 1, Hash40::new("top"), 4.0, 70, 40, 0, 90, 12.0, 0.0, 15.0, 5.0, Some(0.0), Some(15.0), Some(20.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, *FIGHTER_ATTACK_KIND_VISUAL_SCENE as u64, 1, Hash40::new("top"), 4.0, 70, 40, 0, 90, 12.0, 0.0, 20.0, 5.0, Some(0.0), Some(20.0), Some(20.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_YOSHI )]
pub fn yoshi_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();

		if fighter_kind == FIGHTER_KIND_YOSHI {
			if let L2CValueType::Void = globals["eggroll_grounded"].val_type {
				globals["eggroll_grounded"] = false.into();
			}
			if motion_kind == hash40("special_air_s_start") || status_kind == *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP {
				if situation_kind == *SITUATION_KIND_GROUND {
					globals["eggroll_grounded"] = true.into();
				}
				if globals["eggroll_grounded"].get_bool() == false {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, true);
					}
				}
			}
			else {
				globals["eggroll_grounded"] = false.into();
			}
			if status_kind == *FIGHTER_YOSHI_STATUS_KIND_FINAL_DASH && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				FINAL_HIT[get_player_number(module_accessor)] = true;
			}
			else if status_kind != *FIGHTER_YOSHI_STATUS_KIND_FINAL_HIT {
				FINAL_HIT[get_player_number(module_accessor)] = false;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		yoshi_attackhi3,
		yoshi_attackairn,
		yoshi_finaldash,
		yoshi_finalhit
	);
	smashline::install_agent_frames!(yoshi_functions);
}

