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
use crate::custom::FIGHTER_INT_1;


#[acmd_script( agent = "sonic", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn sonic_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 6.2;
		SIZE2[get_player_number(module_accessor)] = 4.0;
		SIZE3[get_player_number(module_accessor)] = 5.0;
		SIZE4[get_player_number(module_accessor)] = 7.0;
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 96, 100, 100, 0, 6.2, 0.0, 8.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 96, 100, 100, 0, 6.2, 0.0, 8.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 96, 100, 100, 0, 6.2, 0.0, 8.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 96, 100, 100, 0, 6.2, 0.0, 8.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("legr"), 4.0, 90, 100, 60, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("kneer"), 4.0, 95, 100, 35, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("kneer"), 4.0, 95, 100, 12, 0, 5.0, 7.0, 0.0, -1.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 3.0;
			SIZE2[get_player_number(module_accessor)] += 3.0;
			SIZE3[get_player_number(module_accessor)] += 3.0;
			SIZE4[get_player_number(module_accessor)] += 3.0;
		}
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 40, 0, 82, SIZE1[get_player_number(module_accessor)], 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legl"), 6.0, 85, 40, 0, 82, SIZE2[get_player_number(module_accessor)], 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.0, 85, 40, 0, 82, SIZE3[get_player_number(module_accessor)], 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("kneel"), 6.0, 85, 40, 0, 82, SIZE4[get_player_number(module_accessor)], 7.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn sonic_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		JostleModule::set_status(module_accessor, false);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 80, 80, 0, 50, 3.4, 7.5, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		JostleModule::set_status(module_accessor, true);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn sonic_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 99, 85, 0, 3.0, 0.0, 7.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 99, 85, 0, 3.0, 0.0, 7.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 99, 115, 0, 4.0, 0.0, 7.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 367, 99, 115, 0, 4.0, 0.0, 7.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 100, 100, 115, 0, 3.0, 0.0, 7.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 3.0, 100, 100, 115, 0, 3.0, 0.0, 7.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 3.0, 118, 100, 115, 0, 4.0, 0.0, 7.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 7, 0, Hash40::new("top"), 3.0, 118, 100, 115, 0, 4.0, 0.0, 7.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 15.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 15.0, -3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 18.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 18.0, -6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 18.0, -6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 7.0, 0.0, 20.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 6.0, 0.0, 15.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 4.5, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 4.5, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 8.0, 91, 68, 0, 66, 4.8, 0.0, 10.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn sonic_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.5);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.2);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 55, 107, 0, 30, 4.5, -1.5, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footr"), 11.0, 361, 97, 0, 45, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("footr"), 11.0, 361, 97, 0, 45, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.5, 55, 107, 0, 30, 4.5, -1.5, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footr"), 11.5, 280, 74, 0, 40, 6.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("footr"), 11.5, 280, 76, 0, 50, 6.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
	
}

#[acmd_script( agent = "sonic", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn sonic_attackairlw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10e97de698u64), Hash40::new("top"), 0.0, 9.5, 1.0, 180, -150, 90, 0.92, true);
		LAST_EFFECT_SET_RATE(fighter, 2.0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true, 0.9);
	}
	
}

#[acmd_script( agent = "sonic", script = "expression_landingairlw", category = ACMD_EXPRESSION, low_priority)]
unsafe fn sonic_landingairlw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "sonic", script = "game_specialsdashhi", category = ACMD_GAME, low_priority)]
unsafe fn sonic_specialsdashhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		JostleModule::set_status(module_accessor, false);
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 70, 0, 65, 3.8, 0.0, 4.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_captured_same_time_attack(module_accessor, 0, true);
		AttackModule::set_attack_keep_rumble(module_accessor, 0, true);
	}
	
}

#[acmd_script( agent = "sonic", script = "effect_specialsdashhi", category = ACMD_EFFECT, low_priority)]
unsafe fn sonic_specialsdashhi_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new_raw(0x0ee81887c4u64), Hash40::new("sphere"), 0, 0, -2, 0, 0, 0, 1, true);
		EffectModule::enable_sync_init_pos_last(module_accessor);
		FLASH(fighter, 1, 1, 1, 0.129999995);
		LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false);
	}
	wait(lua_state, 4.0);
	
}

#[acmd_script( agent = "sonic", script = "game_specialsdashlw", category = ACMD_GAME, low_priority)]
unsafe fn sonic_specialsdashlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		JostleModule::set_status(module_accessor, false);
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 70, 0, 65, 3.8, 0.0, 4.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_captured_same_time_attack(module_accessor, 0, true);
		AttackModule::set_attack_keep_rumble(module_accessor, 0, true);
	}
	
}

#[acmd_script( agent = "sonic", script = "effect_specialsdashlw", category = ACMD_EFFECT, low_priority)]
unsafe fn sonic_specialsdashlw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new_raw(0x0ee81887c4u64), Hash40::new("sphere"), 0, 4, -2, 0, 0, 0, 1, true);
		EffectModule::enable_sync_init_pos_last(module_accessor);
		FLASH(fighter, 1, 1, 1, 0.129999995);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		FLASH(fighter, 1, 1, 0.275000006, 0.195999995);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
	wait(lua_state, 2.0);
	
}

#[acmd_script( agent = "sonic", script = "effect_specialairlwstart", category = ACMD_EFFECT, low_priority)]
unsafe fn sonic_specialairlwstart_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "sonic", script = "effect_specialairlwhold", category = ACMD_EFFECT, low_priority)]
unsafe fn sonic_specialairlwhold_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x154e7cff56u64), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0ed4f69c49u64), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0ee81887c4u64), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
		EffectModule::enable_sync_init_pos_last(module_accessor);
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0c022d4f76u64), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
	}
	wait(lua_state, 17.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0c022d4f76u64), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
	}
	wait(lua_state, 17.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0c022d4f76u64), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
	}
	wait(lua_state, 10.0);
	
}

#[acmd_script( agent = "sonic", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn sonic_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.0, 4.0, Some(0.0), Some(6.0), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
		MotionModule::set_rate(module_accessor, 0.81);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn sonic_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.0, 4.0, Some(0.0), Some(6.0), Some(12.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
		MotionModule::set_rate(module_accessor, 0.78125);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn sonic_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
		MotionModule::set_rate(module_accessor, 0.88);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn sonic_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 20, 35, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.5, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 45, 100, 0, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(module_accessor, true, false);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, 2, 0);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
		lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 0.0, y: -5.0, z: 0.0 });
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(module_accessor);
    }
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
	}
    
}

#[acmd_script( agent = "sonic", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn sonic_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 4.0, 60.0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -2.0 * PostureModule::lr(module_accessor), 8);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 4, 0);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *EFFECT_SUB_ATTRIBUTE_EMIT, false);
		sv_animcmd::EFFECT_OFF(lua_state);
		fighter.clear_lua_stack();
		ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
//		ArticleModule::change_status(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		VisibilityModule::set_whole(module_accessor, false);
//		SA_SET(fighter, *SITUATION_KIND_AIR);
//		CORRECT(fighter, *GROUND_CORRECT_KIND_NONE);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 721;
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
	
}

#[acmd_script( agent = "sonic", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn sonic_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 4.0, 60.0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -2.0 * PostureModule::lr(module_accessor), 8);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 4, 0);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *EFFECT_SUB_ATTRIBUTE_EMIT, false);
		sv_animcmd::EFFECT_OFF(lua_state);
		fighter.clear_lua_stack();
		ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
//		ArticleModule::change_status(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		VisibilityModule::set_whole(module_accessor, false);
//		SA_SET(fighter, *SITUATION_KIND_AIR);
//		CORRECT(fighter, *GROUND_CORRECT_KIND_NONE);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 721;
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
	
}

extern "C" {
	#[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic17start_final_sonicERNS_7FighterE"]
	fn start_final_sonic(instance: &mut app::Fighter) -> u64;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic15end_final_sonicERNS_7FighterE"]
	fn end_final_sonic(instance: &mut app::Fighter) -> u64;
}

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
pub fn sonic_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let battle_object = app::sv_system::battle_object(fighter.lua_state_agent);
		let instance = mem::transmute::<&mut app::BattleObject, &mut app::Fighter>(battle_object);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let mut globals = fighter.globals_mut().clone();
		let BOOST_METER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
	
		if fighter_kind == FIGHTER_KIND_SONIC {
			if let L2CValueType::Void = globals["sonic_globals_set"].val_type {
				globals["bb_check"] = false.into();
				globals["bb_check2"] = false.into();
				globals["bb_damage"] = 4.0.into();
				globals["bb_momentum"] = 0.0.into();
				globals["bb_height"] = 0.into();
				globals["bb_frame"] = 0.0.into();
				globals["boost_startup_frame"] = 0.0.into();
				globals["can_build_meter"] = true.into();
				globals["sonic_globals_set"] = true.into();
			}
			if !sv_information::is_ready_go() {
				*BOOST_METER = 10;
			}
			if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
				start_final_sonic(instance);
			}
			if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
				if FINAL_TRANSFORM[get_player_number(module_accessor)] == 1 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_FINAL_END, true);	
				}
				ArticleModule::set_pos(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, *PostureModule::pos(module_accessor));
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
					end_final_sonic(instance);
				}
				//VisibilityModule::set_whole(module_accessor, false);
				FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					MotionModule::set_rate(module_accessor, 1.2);
				}
				*BOOST_METER = 60;
			}
			if motion_kind == hash40("throw_lw") && MotionModule::frame(module_accessor) == 44.0 {
				MotionModule::set_rate(module_accessor, 2.0);
			}
			if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				globals["can_build_meter"] = true.into();
			}
			else if globals["can_build_meter"].get_bool() {
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					*BOOST_METER += 1;
					globals["can_build_meter"] = false.into();
				}
			}
			if *BOOST_METER < 0 {
				*BOOST_METER = 0;
			}
			if *BOOST_METER > 60 {
				*BOOST_METER = 60;
			}
			if smashball::is_training_mode() {
				*BOOST_METER = 60;
			}
			else if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind  == *FIGHTER_STATUS_KIND_ENTRY {
				*BOOST_METER = 10;
			}
			if motion_kind == hash40("special_s_start") {
				if globals["boost_startup_frame"].get_num() < 8.0 {
					MotionModule::set_frame(module_accessor, 0.0, true);
					globals["boost_startup_frame"] = (globals["boost_startup_frame"].get_num() + 1.0).into();
				}
				else {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, true);
					globals["boost_startup_frame"] = 0.0.into();
				}
			}
			if motion_kind == hash40("special_s_dash_hi") || motion_kind == hash40("special_s_dash_lw") || motion_kind == hash40("special_s_dash_hop") {
				VisibilityModule::set_int64(module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
				WorkModule::unable_transition_term_group(module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
				MotionModule::set_rate(module_accessor, 3.0);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
			if motion_kind == hash40("special_s_dash_hop") {
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("special_s_dash_hi")}, 0.0, 1.0, true, 0.0, false, false);
			}
			if status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END {
				if situation_kind == SITUATION_KIND_AIR {
					if motion_kind == hash40("special_air_s_end_loop") || motion_kind == hash40("special_air_s_end_start") {
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
						WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
					}
				}
				else {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN, true);
				}
			}
			if motion_kind == hash40("fall") && globals["bb_check2"].get_bool() {
				if MotionModule::frame(module_accessor) == 0.0 {	
					if globals["bb_height"].get_int() == 1 {
						let bb_bounce1 = Vector3f{x: 0.0, y: 2.75, z: 0.0};
						KineticModule::add_speed(module_accessor, &bb_bounce1);
					}
					if globals["bb_height"].get_int() == 2 {
						let bb_bounce2 = Vector3f{x: 0.0, y: 2.75, z: 0.0};
						KineticModule::add_speed(module_accessor, &bb_bounce2);
					}
				}
				if MotionModule::frame(module_accessor) >= 0.0 && MotionModule::frame(module_accessor) <= 18.0 {
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
					WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
				}
				else {
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
					globals["bb_check2"] = false.into();
				}
			}
			else {
				if situation_kind == SITUATION_KIND_AIR && motion_kind != hash40("special_lw_hold") && motion_kind != 0x1cb5359b43u64 && motion_kind != 0x1c90a5cf37u64 && motion_kind != 0x1933420e2au64 && motion_kind != hash40("special_n_rebound") && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END {
					globals["bb_check2"] = false.into();
				}
			}
			if motion_kind == hash40("special_air_lw_start") {
				let bb_up = Vector3f{x: 0.0, y: 1.32, z: 0.0};
				if MotionModule::frame(module_accessor) >= 1.0 && MotionModule::frame(module_accessor) <= 2.0 {
					KineticModule::add_speed(module_accessor, &bb_up);
				}
				if MotionModule::frame(module_accessor) >= 10.0 {
					globals["bb_check"] = true.into();
					MotionModule::change_motion(module_accessor, Hash40{ hash: hash40("special_air_lw_hold") }, 0.0, 1.0, false, 0.0, false, false);
				}
			}
			if motion_kind == hash40("special_air_lw_hold") {
				globals["bb_check"] = true.into();
				if MotionModule::frame(module_accessor) == 1.0 {
					KineticModule::clear_speed_all(module_accessor);
					globals["bb_momentum"] = 0.0.into();
				}
				if MotionModule::frame(module_accessor) > 1.0 {
					globals["bb_momentum"] = (globals["bb_momentum"].get_num() + 2.64).into();
					let bb_down = Vector3f{x: 0.0, y: -0.66, z: 0.0};
					KineticModule::add_speed(module_accessor, &bb_down);
					globals["bb_damage"] = (globals["bb_damage"].get_num() + 0.9).into();
					if globals["bb_damage"].get_num() > 13.0 {
						globals["bb_damage"] = 13.0.into();
					}
					ATTACK(fighter, 0, 0, Hash40::new("top"), globals["bb_damage"].get_num(), 70, 80, 0, 80, 3.3, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
					ATTACK(fighter, 1, 0, Hash40::new("top"), globals["bb_damage"].get_num(), 270, 80, 0, 80, 3.3, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
						KineticModule::clear_speed_all(module_accessor);
						if MotionModule::frame(module_accessor) > globals["bb_frame"].get_num() + 2.0 || MotionModule::frame(module_accessor) < globals["bb_frame"].get_num() {
							MotionModule::change_motion(module_accessor, Hash40{ hash: hash40("special_n_rebound") }, 0.0, 1.0, false, 0.0, false, false);
						}
					}
					else {
						globals["bb_frame"] = MotionModule::frame(module_accessor).into();
					}
				}
			}
			else {
				globals["bb_damage"] = 4.0.into();
				globals["bb_momentum"] = 0.0.into();
				if motion_kind != hash40("special_lw_hold") && motion_kind != 0x1cb5359b43u64 && motion_kind != 0x1c90a5cf37u64 && motion_kind != 0x1933420e2au64 && motion_kind != hash40("special_n_rebound") && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD && status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END {
					globals["bb_check"] = false.into();
				}
			}
			if situation_kind == SITUATION_KIND_GROUND && globals["bb_check"].get_bool() == true {
				MotionModule::change_motion(module_accessor, Hash40{ hash: hash40("special_lw_hold") }, 0.0, 1.0, false, 0.0, false, false);
				globals["bb_check"] = false.into();
				globals["bb_check2"] = true.into();
			}
			if motion_kind == hash40("special_lw_hold") && globals["bb_check2"].get_bool() == true {
				globals["bb_height"] = 1.into();
				if MotionModule::frame(module_accessor) > 0.0 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
				}
			}
			if motion_kind == hash40("special_n_rebound") && globals["bb_check"].get_bool() {
				if MotionModule::frame(module_accessor) >= 0.0 && MotionModule::frame(module_accessor) <= 1.0 {
					globals["bb_height"] = 2.into();
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					globals["bb_check2"] = true.into();
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		sonic_attackhi3,
		sonic_attacklw3,
		sonic_attackairhi,
		sonic_attackairlw,
		sonic_attackairlw_effect,
		sonic_landingairlw_expression,
		sonic_specialsdashhi,
		sonic_specialsdashhi_effect,
		sonic_specialsdashlw,
		sonic_specialsdashlw_effect,
		sonic_specialairlwstart_effect,
		sonic_specialairlwhold_effect,
		sonic_catch,
		sonic_catchdash,
		sonic_catchturn,
		sonic_throwlw,
		sonic_finalstart,
		sonic_finalairstart
	);
	smashline::install_agent_frames!(sonic_functions);
}

