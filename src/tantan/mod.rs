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
use crate::custom::FIGHTER_INT_1;
use crate::custom::FIGHTER_INT_2;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;

#[acmd_script( agent = "tantan", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn tantan_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 15.0, 60.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 50);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final02.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 58.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(22.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(30.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(38.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 61.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 22.0, Some(0.0), Some(7.0), Some(46.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 62.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 28.0, Some(0.0), Some(7.0), Some(54.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 63.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 32.0, Some(0.0), Some(7.0), Some(59.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 64.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 34.0, Some(0.0), Some(7.0), Some(65.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 65.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 36.0, Some(0.0), Some(7.0), Some(69.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 66.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 38.0, Some(0.0), Some(7.0), Some(71.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 68.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "tantan", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn tantan_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 15.0, 60.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 50);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final02.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 58.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(22.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(30.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(38.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 61.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 22.0, Some(0.0), Some(7.0), Some(46.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 62.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 28.0, Some(0.0), Some(7.0), Some(54.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 63.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 32.0, Some(0.0), Some(7.0), Some(59.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 64.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 34.0, Some(0.0), Some(7.0), Some(65.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 65.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 36.0, Some(0.0), Some(7.0), Some(69.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 66.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 70, 0, 70, 7.0, 0.0, 7.0, 38.0, Some(0.0), Some(7.0), Some(71.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 60164, *ATTACK_REGION_OBJECT);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 68.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
pub fn tantan_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let CAN_ATTACK_JUMP = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let CAN_DOUBLE_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let L_ARM_HIT = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let R_ARM_HIT = &mut FIGHTER_INT_2[get_player_number(module_accessor)];

		if fighter_kind == *FIGHTER_KIND_TANTAN {
			if WorkModule::is_flag(module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_LUA_TEMP_IS_ATTACK_BOTH) { //If we're attacking with both arms, check if whichever one we launched first has connected
				if *L_ARM_HIT != 0 || *R_ARM_HIT != 0 || CancelModule::is_enable_cancel(module_accessor) {
					*CAN_ATTACK_JUMP = true;
				}
				else {
					*CAN_ATTACK_JUMP = false; //If it hasn't, we aren't allowed to jump cancel attacks
				}
			}
			else {
				*CAN_ATTACK_JUMP = true; //But if we are only attacking with one arm, feel free
			}

			if (status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_JUMP) 
			|| (status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_JUMP_AERIAL) { 
				//If we're in the attack jump status, and we didn't get there from a real jump, check if whichever arm we launched first has connected
				if *L_ARM_HIT != 0 || *R_ARM_HIT != 0 || CancelModule::is_enable_cancel(module_accessor) {
					*CAN_DOUBLE_ATTACK = true;
				}
				else {
					*CAN_DOUBLE_ATTACK = false; //If it hasn't, we aren't allowed to attack with the second arm
				}
			}
			else {
				*CAN_DOUBLE_ATTACK = true; //But if we aren't jumping at all, feel free
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_TANTAN_PUNCH1 )]
pub fn tantan_punch1_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let L_ARM_HIT = &mut FIGHTER_INT_1[get_player_number(owner_module_accessor)];
		let R_ARM_HIT = &mut FIGHTER_INT_2[get_player_number(owner_module_accessor)];

		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) {
				*L_ARM_HIT = owner_status_kind;
			}
			else {
				*R_ARM_HIT = owner_status_kind;
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) && StatusModule::prev_status_kind(owner_module_accessor, 0) != *L_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 1) != *L_ARM_HIT {
				*L_ARM_HIT = 0;
			}
			else if StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT {
				*R_ARM_HIT = 0;
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_TANTAN_PUNCH2 )]
pub fn tantan_punch2_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let L_ARM_HIT = &mut FIGHTER_INT_1[get_player_number(owner_module_accessor)];
		let R_ARM_HIT = &mut FIGHTER_INT_2[get_player_number(owner_module_accessor)];

		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) {
				*L_ARM_HIT = owner_status_kind;
			}
			else {
				*R_ARM_HIT = owner_status_kind;
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) && StatusModule::prev_status_kind(owner_module_accessor, 0) != *L_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 1) != *L_ARM_HIT {
				*L_ARM_HIT = 0;
			}
			else if StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT {
				*R_ARM_HIT = 0;
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_TANTAN_PUNCH3 )]
pub fn tantan_punch3_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let L_ARM_HIT = &mut FIGHTER_INT_1[get_player_number(owner_module_accessor)];
		let R_ARM_HIT = &mut FIGHTER_INT_2[get_player_number(owner_module_accessor)];

		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) {
				*L_ARM_HIT = owner_status_kind;
			}
			else {
				*R_ARM_HIT = owner_status_kind;
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT) && StatusModule::prev_status_kind(owner_module_accessor, 0) != *L_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 1) != *L_ARM_HIT {
				*L_ARM_HIT = 0;
			}
			else if StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT && StatusModule::prev_status_kind(owner_module_accessor, 0) != *R_ARM_HIT {
				*R_ARM_HIT = 0;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		tantan_final,
		tantan_finalair
	);
	smashline::install_agent_frames!(tantan_functions);
	smashline::install_agent_frames!(
		tantan_punch1_functions,
		tantan_punch2_functions,
		tantan_punch3_functions
	);
}

