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


#[acmd_script( agent = "samusd", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn samusd_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.0, 0.0, 10.0, 6.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.3, 0.0, 10.0, 8.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.5, 0.0, 10.0, 12.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn samusd_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.5;
		SIZE2[get_player_number(module_accessor)] = 5.0;
		SIZE3[get_player_number(module_accessor)] = 5.0;
		SIZE4[get_player_number(module_accessor)] = 6.5;
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 100, 30, 0, 60, SIZE1[get_player_number(module_accessor)], -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, SIZE2[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, SIZE3[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		let hitVec = Vector2f{ x: 4.0, y: 22.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 4.0 as u32, false);
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 2, Hash40{ hash: hash40("top") }, &hitVec, 4.0 as u32, false);
		AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 2, true, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
			SIZE3[get_player_number(module_accessor)] += 2.0;
			SIZE4[get_player_number(module_accessor)] += 2.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 130, 30, 0, 30, SIZE1[get_player_number(module_accessor)], -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, SIZE2[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 160, 30, 0, 30, SIZE3[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		let hitVec = Vector2f{ x: -1.0, y: 23.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 10.0 as u32, false);
		AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 2, true, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
			SIZE3[get_player_number(module_accessor)] += 2.0;
			SIZE4[get_player_number(module_accessor)] += 2.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 130, 30, 0, 40, SIZE1[get_player_number(module_accessor)], -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, SIZE2[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 160, 30, 0, 30, SIZE3[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		let hitVec = Vector2f{ x: -6.0, y: 22.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 10.0 as u32, false);
		AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 2, true, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
			SIZE3[get_player_number(module_accessor)] += 2.0;
			SIZE4[get_player_number(module_accessor)] += 2.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 120, 30, 0, 30, SIZE1[get_player_number(module_accessor)], -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 368, 30, 0, 0, SIZE2[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 170, 30, 0, 30, SIZE3[get_player_number(module_accessor)], 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		let hitVec = Vector2f{ x: -10.0, y: 20.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 10.0 as u32, false);
		AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
		AttackModule::set_no_finish_camera(module_accessor, 2, true, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
			SIZE3[get_player_number(module_accessor)] += 2.0;
			SIZE4[get_player_number(module_accessor)] += 2.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 80, 150, 0, 50, SIZE1[get_player_number(module_accessor)], -3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 80, 150, 0, 50, SIZE4[get_player_number(module_accessor)], 7.8, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn samusd_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.0, 367, 100, 25, 0, 6.0, 9.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 18, 100, 30, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	for _ in 0..3 {
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 4.0);
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.0);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 1.6, 367, 100, 35, 0, 6.0, 9.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
			ATTACK(fighter, 1, 0, Hash40::new("armr"), 1.6, 18, 100, 40, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			MotionModule::set_rate(module_accessor, 4.0);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 361, 140, 0, 40, 5.5, 2.0, 0.0, 2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 5.0, 361, 140, 0, 40, 7.2, 9.0, 0.0, 2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn samusd_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 6.5;
		SIZE2[get_player_number(module_accessor)] = 5.5;
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 3.0, 84, 100, 90, 0, SIZE1[get_player_number(module_accessor)], -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.0, 367, 100, 90, 0, SIZE1[get_player_number(module_accessor)], -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("toer"), 3.0, 367, 100, 25, 0, SIZE2[get_player_number(module_accessor)], 1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, -10.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, -10.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, -10.0, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.0;
			SIZE2[get_player_number(module_accessor)] += 1.0;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.3, 84, 100, 90, 0, SIZE1[get_player_number(module_accessor)], -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.3, 367, 100, 90, 0, SIZE1[get_player_number(module_accessor)], -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("toer"), 1.3, 367, 100, 25, 0, SIZE2[get_player_number(module_accessor)], 1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, -10.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, -10.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, -10.0, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.0;
			SIZE2[get_player_number(module_accessor)] += 1.0;
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.0, 70, 160, 0, 40, SIZE1[get_player_number(module_accessor)], -1.0, 1.0, 2.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("toer"), 4.0, 70, 160, 0, 40, SIZE2[get_player_number(module_accessor)], 1.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn samusd_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.2;
		SIZE2[get_player_number(module_accessor)] = 4.0;
		WorkModule::on_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
		WorkModule::on_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.5;
			SIZE2[get_player_number(module_accessor)] += 1.5;
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 92, 100, 180, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 1.0, 98, 100, 180, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 1.0, 92, 100, 80, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 7, 0, Hash40::new("top"), 1.0, 98, 100, 80, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.5;
			SIZE2[get_player_number(module_accessor)] += 1.5;
		}
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 4, false);
		AttackModule::clear(module_accessor, 5, false);
		AttackModule::clear(module_accessor, 6, false);
		AttackModule::clear(module_accessor, 7, false);
		WorkModule::on_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 84, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 84, 100, 40, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.5;
			SIZE2[get_player_number(module_accessor)] += 1.5;
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 82, 100, 40, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 1.0, 82, 100, 20, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 7, 0, Hash40::new("top"), 1.0, 90, 100, 20, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 4, false);
		AttackModule::clear(module_accessor, 5, false);
		AttackModule::clear(module_accessor, 6, false);
		AttackModule::clear(module_accessor, 7, false);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 190, 0, 56, 10.0, 0.0, 6.5, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn samusd_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
		SIZE1[get_player_number(module_accessor)] = 4.2;
		SIZE2[get_player_number(module_accessor)] = 4.0;
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 115, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 96, 100, 115, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 93, 100, 60, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 96, 100, 60, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 1.5;
			SIZE2[get_player_number(module_accessor)] += 1.5;
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 96, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 1.0, 98, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 6, 0, Hash40::new("top"), 1.0, 96, 100, 40, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
		ATTACK(fighter, 7, 0, Hash40::new("top"), 1.0, 98, 100, 40, 0, SIZE2[get_player_number(module_accessor)], 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x13135c5462"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 4, false);
		AttackModule::clear(module_accessor, 5, false);
		AttackModule::clear(module_accessor, 6, false);
		AttackModule::clear(module_accessor, 7, false);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 240, 0, 50, 10.0, 0.0, 6.5, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	
}

#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME, low_priority)]
unsafe fn samusd_missile_homing(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME, low_priority)]
unsafe fn samusd_supermissile_ready(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 65, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd_supermissile", script = "game_straight", category = ACMD_GAME, low_priority)]
unsafe fn samusd_supermissile_straight(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 65, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
	}
	
}

#[acmd_script( agent = "samusd_laser2", script = "game_upper", category = ACMD_GAME, low_priority)]
unsafe fn samusd_laser2_upper(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	wait(lua_state, 17.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 325, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 123.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd_laser2", script = "game_lower", category = ACMD_GAME, low_priority)]
unsafe fn samusd_laser2_lower(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	wait(lua_state, 17.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 45, 100, 24, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 123.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd_laser", script = "game_shoot", category = ACMD_GAME, low_priority)]
unsafe fn samusd_laser_shoot(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	wait(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 20, 100, 16, 0, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 128.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 183, 0, 45, 11.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "samusd", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn samusd_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 50.0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER, false, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
}

#[acmd_script( agent = "samusd", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn samusd_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 50.0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER, false, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
}

#[acmd_script( agent = "samusd", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn samusd_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ArticleModule::shoot(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		WorkModule::set_float(module_accessor, 10.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, false, 0);
		WorkModule::set_float(module_accessor, -10.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, false, 0);
		ArticleModule::shoot(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
}

#[acmd_script( agent = "samusd", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn samusd_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ArticleModule::shoot(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		WorkModule::set_float(module_accessor, 10.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, false, 0);
		WorkModule::set_float(module_accessor, -10.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
		ArticleModule::generate_article(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, false, 0);
		ArticleModule::shoot(module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_LASER2, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
}

pub fn install() {
	install_acmd_scripts!(
		samusd_attack11,
		samusd_attackhi4,
		samusd_attackairf,
		samusd_attackairhi,
		samusd_specialhi,
		samusd_specialairhi,
		samusd_missile_homing,
		samusd_supermissile_ready,
		samusd_supermissile_straight,
		samusd_laser_shoot,
		samusd_laser2_upper,
		samusd_laser2_lower,
		samusd_finalstart,
		samusd_finalairstart,
		samusd_final,
		samusd_finalair
	);
}

