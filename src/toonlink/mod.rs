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


#[acmd_script( agent = "toonlink", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, 45, 60, 0, 50, 2.5, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.0, 45, 60, 0, 50, 3.0, 1.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.0, 45, 60, 0, 50, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 2.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attacks4s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {  }frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.33);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		JostleModule::set_status(module_accessor, false);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 20.0, 40, 110, 0, 25, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 20.0, 40, 110, 0, 25, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 20.0, 40, 110, 0, 25, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 20.0, 40, 110, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		JostleModule::set_status(module_accessor, true);
	}
	frame(lua_state, 39.0);
	if is_excute(fighter) {  }
}

#[acmd_script( agent = "toonlink", script = "game_attacklw4", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.3;
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		JostleModule::set_status(module_accessor, false);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 180, 100, 80, 0, 4.0, 0.0, 2.2, 16.0, Some(0.0), Some(2.9), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 4.0;
		}
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 42, 127, 0, 60, SIZE1[get_player_number(module_accessor)], 0.0, 4.2, -16.0, Some(0.0), Some(4.2), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		JostleModule::set_status(module_accessor, true);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.14);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.43);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.0, 65, 70, 0, 70, 4.7, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 65, 70, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 65, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 3.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.62);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 43.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.5, 361, 100, 0, 20, 4.5, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.5, 361, 100, 0, 20, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.5, 361, 100, 0, 20, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 8.5, 361, 100, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, 361, 100, 0, 20, 4.5, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.0, 361, 100, 0, 20, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.0, 361, 100, 0, 20, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 7.0, 361, 100, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 13.0, 361, 98, 0, 35, 5.5, 4.0, -2.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 13.0, 361, 98, 0, 35, 5.5, 1.0, -2.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 13.0, 361, 98, 0, 35, 4.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 39.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 60, 117, 0, 24, 4.5, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.0, 60, 117, 0, 24, 5.5, 2.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.0, 60, 117, 0, 24, 4.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 0.6);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_specialhistart", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_specialhistart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
}

#[acmd_script( agent = "toonlink", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_throwhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 92, 100, 74, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, true);
	}
	frame(lua_state, 27.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword2"), 5.0, 20, 130, 0, 50, 3.84, 6.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword2"), 4.0, 20, 130, 0, 50, 3.84, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
		AttackModule::set_catch_only_all(module_accessor, true, false);
		CHECK_FINISH_CAMERA(fighter, 0, 20);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.7);
		lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 0.0, y: 7.0, z: 0.0 });
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "toonlink_bowarrow", script = "game_fly", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_bowarrow_fly(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 50, 0, 8, 1.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(module_accessor);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 30.0, 361, 90, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 8.0, 34.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 10, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		SlowModule::clear_whole(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			WorkModule::on_flag(module_accessor, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_FINAL_REQ_CATCH);
		}
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		GrabModule::clear_all(module_accessor);
		SearchModule::clear_all(module_accessor);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.3);
	}
	
}

#[acmd_script( agent = "toonlink", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn toonlink_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 30.0, 361, 90, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 8.0, 34.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart02.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 10, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 22, 100, 50, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		SlowModule::clear_whole(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			WorkModule::on_flag(module_accessor, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_FINAL_REQ_CATCH);
		}
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		GrabModule::clear_all(module_accessor);
		SearchModule::clear_all(module_accessor);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.3);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
pub fn toonlink_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();

		if fighter_kind == FIGHTER_KIND_TOONLINK {
			if let L2CValueType::Void = globals["hurricane_frame"].val_type {
				globals["hurricane_frame"] = 0.0.into();
			}
			if motion_kind == hash40("special_hi_start") {
				MotionModule::set_rate(module_accessor, 0.5);
			}
			if motion_kind == hash40("attack_s4_hold") {
				VisibilityModule::set_int64(module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
				VisibilityModule::set_int64(module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
			}
			if motion_kind == hash40("special_hi") {
				if MotionModule::frame(module_accessor) < 44.0 && !AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL) { //Do not add speed during hitlag or after the multihits clear
					if ControlModule::get_stick_x(module_accessor).abs() >= 0.6 { 
						globals["hurricane_frame"] = 10.0.into(); //It takes 10 frames to come to a full stop after releasing
						KineticModule::clear_speed_all(module_accessor); //Speed shouldn't stack
						let hurricaneVec = Vector3f{x: ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) * 1.8 / (MotionModule::frame(module_accessor) / 15.0), y: 0.0, z: 0.0};
						KineticModule::add_speed(module_accessor, &hurricaneVec);
					}
					else {
						if globals["hurricane_frame"].get_num() != 0.0 {
							let hurricaneSlow = Vector3f{x: 1.0/1.02, y: 0.0, z: 0.0};
							KineticModule::mul_speed(module_accessor, &hurricaneSlow, *FIGHTER_KINETIC_ENERGY_ID_NONE);
							globals["hurricane_frame"] = (globals["hurricane_frame"].get_num() - 1.0).into();
						}
						else {
							KineticModule::clear_speed_all(module_accessor);
						}
					}
				}
			}
			else {
				globals["hurricane_frame"] = 0.0.into();
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_TOONLINK_BOWARROW )]
fn toonlink_bowarrow_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let _motion_kind = MotionModule::motion_kind(module_accessor);
		let owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);

		if weapon_kind == *WEAPON_KIND_TOONLINK_BOWARROW {
			if owner_motion_kind != hash40("attack_s4_s") && owner_motion_kind != hash40("attack_s4_hold") {
				//VisibilityModule::set_whole(module_accessor, false);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		toonlink_attacklw3,
		toonlink_attacks4s,
		toonlink_attacklw4,
		toonlink_attackdash,
		toonlink_attackairn,
		toonlink_attackairf,
		toonlink_attackairb,
		toonlink_specialhistart,
		toonlink_throwhi,
		toonlink_bowarrow_fly,
		toonlink_finalstart,
		toonlink_finalairstart
	);
	smashline::install_agent_frames!(toonlink_functions);
	smashline::install_agent_frames!(toonlink_bowarrow_functions);
}

