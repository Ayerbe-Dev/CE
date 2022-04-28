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
use crate::custom::FIGHTER_BOOL_1;


#[acmd_script( agent = "simon", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn simon_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 5.0, 0.0, 25.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 4.0, 0.0, 25.5, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 4.0, 0.0, 25.5, 10.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		ATTACK(fighter, 4, 1, Hash40::new("top"), 2.0, 93, 100, 0, 60, 5.0, 0.0, 13.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 5, 1, Hash40::new("top"), 2.0, 93, 100, 0, 60, 5.0, 0.0, 13.5, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 6, 1, Hash40::new("top"), 2.0, 93, 100, 0, 60, 5.0, 0.0, 13.5, 10.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "simon", script = "game_attacklw32", category = ACMD_GAME, low_priority)]
unsafe fn simon_attacklw32(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		JostleModule::set_status(module_accessor, false);
		WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 7.0, 361, 55, 0, 90, 3.5, 0.0, 1.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footl"), 7.0, 361, 55, 0, 90, 3.0, -1.5, 1.5, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("hip"), 7.0, 361, 55, 0, 90, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 3.5, 50, 40, 0, 60, 2.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footl"), 3.5, 50, 40, 0, 60, 2.5, -1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::clear(module_accessor, 2, false);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		JostleModule::set_status(module_accessor, true);
	}
	
}

#[acmd_script( agent = "simon", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn simon_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 2.5, 4.5);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	for _ in 0..6 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 10, 0, 55, 4.0, 0.0, 14.5, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 93, 10, 0, 45, 4.0, 0.0, 14.5, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 367, 10, 0, 55, 4.0, 0.0, 14.5, -5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 88, 10, 0, 45, 4.0, 0.0, 14.5, -5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 93, 10, 0, 55, 4.0, 0.0, 4.5, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 5, 0, Hash40::new("top"), 1.0, 93, 10, 0, 65, 4.0, 0.0, 4.5, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 6, 0, Hash40::new("top"), 1.0, 88, 10, 0, 55, 4.0, 0.0, 4.5, -5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
			ATTACK(fighter, 7, 0, Hash40::new("top"), 1.0, 88, 10, 0, 65, 4.0, 0.0, 4.5, -5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 1.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 105, 0, 60, 11.0, 0.0, 10.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "simon", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn simon_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		SET_SPEED_EX(fighter, 0.0, 0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		KineticModule::suspend_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		SET_SPEED_EX(fighter, 1.6, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		JostleModule::set_status(module_accessor, false);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 280, 85, 0, 35, 5.5, 0.0, 1.0, -0.5, Some(0.0), Some(-1.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 72, 55, 0, 88, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-1.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		JostleModule::set_status(module_accessor, true);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
		KineticModule::resume_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	}
	
}

#[acmd_script( agent = "simon", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn simon_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 6.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 8.5, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		AttackModule::clear(module_accessor, 0, false);
		ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 61, 86, 0, 85, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
	}
	
}

#[acmd_script( agent = "simon", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn simon_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 6.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 8.5, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 61, 86, 0, 85, 6.0, 0.0, 23.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
	}
	
}

#[acmd_script( agent = "simon", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn simon_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.43);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 4.0, 6.0);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		GrabModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "simon", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn simon_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 6.0, 4.0);
		MotionModule::set_rate(module_accessor, 1.2);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		GrabModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "simon", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn simon_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.2);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		GrabModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "simon", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn simon_finalstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 4.0, 50.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 2, 0);
        FT_SET_FINAL_FEAR_FACE(fighter, 40);
        REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CAM_ZOOM_OUT(fighter);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 22.0, 0.0, 18.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 10.0, 0.0, 7.0, 15.0, Some(0.0), Some(11.0), Some(15.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_FINAL_HIT) {
            WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_STATUS_FINAL_WORK_ID_FLAG_START_COFFIN_ROT_Y);
        }
    }
    
}

#[acmd_script( agent = "simon", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn simon_finalairstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 4.0, 50.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 2, 0);
        FT_SET_FINAL_FEAR_FACE(fighter, 40);
        REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CAM_ZOOM_OUT(fighter);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 22.0, 0.0, 18.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 10.0, 0.0, 7.0, 15.0, Some(0.0), Some(11.0), Some(15.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_FINAL_HIT) {
            WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_STATUS_FINAL_WORK_ID_FLAG_START_COFFIN_ROT_Y);
        }
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
pub fn simon_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if fighter_kind == FIGHTER_KIND_SIMON {
			if motion_kind == hash40("attack_air_f") ||
			motion_kind == hash40("attack_air_f_hi") ||
			motion_kind == hash40("attack_air_f_lw") ||
			motion_kind == hash40("attack_air_b") ||
			motion_kind == hash40("attack_air_b_hi") ||
			motion_kind == hash40("attack_air_b_lw") {
				if MotionModule::frame(module_accessor) < 16.0 {
					MotionModule::set_rate(module_accessor, 1.2);
				}
				if MotionModule::frame(module_accessor) >= 16.0 {
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
			if motion_kind == hash40("attack_air_hi") {
				if MotionModule::frame(module_accessor) < 18.0 {
					MotionModule::set_rate(module_accessor, 1.3);
				}
				if MotionModule::frame(module_accessor) >= 18.0 {
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_SIMON_WHIP )]
fn simon_whip_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let BELMONT_POST_PARRY = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];

		if weapon_kind == WEAPON_KIND_SIMON_WHIP {
			if *BELMONT_POST_PARRY {
				StopModule::end_stop(module_accessor);
				*BELMONT_POST_PARRY = false;
			}

			if motion_kind == hash40("attack_air_f") ||
			motion_kind == hash40("attack_air_f_hi") ||
			motion_kind == hash40("attack_air_f_lw") ||
			motion_kind == hash40("attack_air_b") ||
			motion_kind == hash40("attack_air_b_hi") ||
			motion_kind == hash40("attack_air_b_lw") {
				if MotionModule::frame(module_accessor) < 16.0 {
					MotionModule::set_rate(module_accessor, 1.2);
				}
				if MotionModule::frame(module_accessor) >= 16.0 {
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
			if motion_kind == hash40("attack_air_hi") {
				if MotionModule::frame(module_accessor) < 18.0 {
					MotionModule::set_rate(module_accessor, 1.3);
				}
				if MotionModule::frame(module_accessor) >= 18.0 {
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
			if motion_kind == hash40("attack_hold") {
				if MotionModule::frame(module_accessor) == 0.0 {
					ATTACK(fighter, 4, 0, Hash40::new("hookshot22"), 1.0, 80, 35, 0, 40, 2.2, -1.5, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
					ATTACK(fighter, 5, 0, Hash40::new("hookshot20"), 1.0, 80, 35, 0, 40, 2.2, -1.5, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		simon_attackhi3,
		simon_attacklw32,
		simon_attackairn,
		simon_attackairlw,
		simon_specialhi,
		simon_specialairhi,
		simon_catch,
		simon_catchdash,
		simon_catchturn,
		simon_finalstart,
		simon_finalairstart
	);
	smashline::install_agent_frames!(simon_functions);
	smashline::install_agent_frames!(simon_whip_functions);
}

