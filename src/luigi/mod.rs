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


#[acmd_script( agent = "luigi", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn luigi_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		KineticModule::clear_speed_all(module_accessor);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 361, 72, 0, 32, 4.8, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legr"), 5.0, 361, 72, 0, 32, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn luigi_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 15.0, 90, 80, 0, 40, 4.5, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 90, 80, 0, 40, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 90, 100, 0, 20, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 90, 100, 0, 20, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn luigi_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.75, 361, 100, 0, 30, 5.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 12.75, 361, 100, 0, 30, 3.84, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn luigi_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 270, 80, 0, 30, 3.0, 0.0, 1.0, 2.8, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 270, 80, 0, 30, 3.0, 0.0, 1.0, 2.8, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, -2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 0, -2.0, false);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 361, 100, 0, 20, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 20, 5.2, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_specialswallend", category = ACMD_GAME, low_priority)]
unsafe fn luigi_specialswallend(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_WALL_FLAG_DETACH);
		JostleModule::set_status(module_accessor, true);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn luigi_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 88, 77, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 88, 77, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13b3061dd0u64), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13b3061dd0u64), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
		WorkModule::off_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		SA_SET(fighter, *SITUATION_KIND_AIR);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn luigi_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 90, 77, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 90, 77, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13b3061dd0u64), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13b3061dd0u64), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
		WorkModule::off_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		SA_SET(fighter, *SITUATION_KIND_AIR);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_speciallw", category = ACMD_GAME, low_priority)]
unsafe fn luigi_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, -5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, 5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 60, 0, 25, 6.0, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 50, 0, 19.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		let hitVec = Vector2f{ x: 0.0, y: 8.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 0, Hash40{ hash: hash40("top") }, &hitVec, 9.0 as u32, false);
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 9.0 as u32, false);
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 2, Hash40{ hash: hash40("top") }, &hitVec, 9.0 as u32, false);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.67);
	}
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 140, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 70, 140, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 90, 140, 0, 85, 6.5, 0.0, 4.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
		WorkModule::off_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn luigi_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		let hitVec = Vector2f{ x: 0.0, y: 8.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 0, Hash40{ hash: hash40("top") }, &hitVec, 8.0 as u32, false);
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{ hash: hash40("top") }, &hitVec, 8.0 as u32, false);
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 2, Hash40{ hash: hash40("top") }, &hitVec, 8.0 as u32, false);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.67);
	}
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 89, 130, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
		WorkModule::off_flag(module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
		WorkModule::on_flag(module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_appeallwr", category = ACMD_GAME, low_priority)]
unsafe fn luigi_appeallwr(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 100, 180, 0, 3.0, 0.0, -1.0, 5.5, Some(0.0), Some(-3.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_appeallwl", category = ACMD_GAME, low_priority)]
unsafe fn luigi_appeallwl(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 100, 180, 0, 3.0, 0.0, -1.0, 5.5, Some(0.0), Some(-3.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "luigi", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn luigi_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SlowModule::set_whole(module_accessor, 10, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		FT_START_CUTIN(fighter);
		if PostureModule::lr(module_accessor) == 1.0 {
			REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
		}
		else {
			REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		SLOW_OPPONENT(fighter, 100.0, 73.0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SLOW_OPPONENT(fighter, 100.0, 73.0);
	}
	frame(lua_state, 73.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 8.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(80.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 8.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(80.0), 20.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
			EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0.0, WorkModule::rnd_float(module_accessor, 20.0, 80.0, 0), 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
		}
	}
	frame(lua_state, 110.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
}

#[acmd_script( agent = "luigi", script = "effect_finalstart", category = ACMD_EFFECT, low_priority)]
unsafe fn luigi_finalstart_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		EffectModule::req_screen(module_accessor, Hash40::new("bg_luigi_final"), false, false, false);
	}
}

#[acmd_script( agent = "luigi", script = "sound_finalstart", category = ACMD_SOUND, low_priority)]
unsafe fn luigi_finalstart_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		PLAY_SE_NO_3D(fighter, Hash40::new("vc_luigi_009"));
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		PLAY_SE_NO_3D(fighter, Hash40::new("se_common_elec_l_damage"));
		PLAY_SE_NO_3D(fighter, Hash40::new("se_common_elec_ll_damage"));
	}
	frame(lua_state, 71.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_luigi_final01"));
	}
}

#[acmd_script( agent = "luigi", script = "expression_finalstart", category = ACMD_EXPRESSION, low_priority)]
unsafe fn luigi_finalstart_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

}

#[acmd_script( agent = "luigi", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn luigi_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SlowModule::set_whole(module_accessor, 10, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		FT_START_CUTIN(fighter);
		if PostureModule::lr(module_accessor) == 1.0 {
			REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
		}
		else {
			REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
		}
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SLOW_OPPONENT(fighter, 4.0, 53.0);
	}
	frame(lua_state, 73.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 8.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(80.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 8.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(80.0), 20.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 20, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
			EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0.0, WorkModule::rnd_float(module_accessor, 20.0, 80.0, 0), 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
		}
	}
	frame(lua_state, 110.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
}

#[acmd_script( agent = "luigi", script = "effect_finalairstart", category = ACMD_EFFECT, low_priority)]
unsafe fn luigi_finalairstart_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		EffectModule::req_screen(module_accessor, Hash40::new("bg_luigi_final"), false, false, false);
	}
}

#[acmd_script( agent = "luigi", script = "sound_finalairstart", category = ACMD_SOUND, low_priority)]
unsafe fn luigi_finalairstart_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		PLAY_SE_NO_3D(fighter, Hash40::new("vc_luigi_009"));
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		PLAY_SE_NO_3D(fighter, Hash40::new("se_common_elec_l_damage"));
		PLAY_SE_NO_3D(fighter, Hash40::new("se_common_elec_ll_damage"));
	}
	frame(lua_state, 71.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_luigi_final01"));
	}
}

#[acmd_script( agent = "luigi", script = "expression_finalairstart", category = ACMD_EXPRESSION, low_priority)]
unsafe fn luigi_finalairstart_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

}

#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
pub fn luigi_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();

		if fighter_kind == FIGHTER_KIND_LUIGI {
			if let L2CValueType::Void = globals["luigi_dash_cancel"].val_type {
				globals["luigi_dash_cancel"] = false.into();
			}
			if let L2CValueType::Void = globals["cyclone_check"].val_type {
				globals["cyclone_check"] = false.into();
			}

			if motion_kind == hash40("attack_lw3") {
				if MotionModule::frame(module_accessor) == 1.0 {
					globals["luigi_dash_cancel"] = true.into();
				}
				if AttackModule::is_attack(module_accessor, 0, false) {
					globals["luigi_dash_cancel"] = false.into();
				}
				if globals["luigi_dash_cancel"].get_bool() {
					let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
					if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
					}
					if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH, true);
					}
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
					}
				}
			}
			else {
				globals["luigi_dash_cancel"] = false.into();
			}
			if motion_kind == hash40("special_lw") || motion_kind == hash40("special_air_lw") {
				if globals["cyclone_check"].get_bool() == false {
					if MotionModule::frame(module_accessor) >= 9.0 && MotionModule::frame(module_accessor) <= 44.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
						}
						else {
							if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
								let cyclone_rise = Vector3f{x: 0.0, y: 1.3, z: 0.0};
								KineticModule::add_speed(module_accessor, &cyclone_rise);
							}
						}
					}
					else {
						if MotionModule::frame(module_accessor) > 44.0 {
							globals["cyclone_check"] = true.into();
						}
					}
				}
			}
		}
		if situation_kind == SITUATION_KIND_GROUND || situation_kind == SITUATION_KIND_CLIFF || StopModule::is_damage(module_accessor) {
			globals["cyclone_check"] = false.into();
		}
	}
}


pub fn install() {
	install_acmd_scripts!(
		luigi_attacklw3,
		luigi_attackairn,
		luigi_attackairf,
		luigi_attackairlw,
		luigi_specialswallend,
		luigi_specialhi,
		luigi_specialairhi,
		luigi_speciallw,
		luigi_specialairlw,
		luigi_appeallwr,
		luigi_appeallwl,
		luigi_finalstart,
		luigi_finalstart_effect,
		luigi_finalstart_sound,
		luigi_finalstart_expression,
		luigi_finalairstart,
		luigi_finalairstart_effect,
		luigi_finalairstart_sound,
		luigi_finalairstart_expression
	);
	smashline::install_agent_frames!(luigi_functions);
}

