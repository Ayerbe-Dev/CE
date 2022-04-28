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
use crate::custom::FIGHTER_INT_1;
use crate::custom::FIGHTER_INT_2;


#[acmd_script( agent = "miifighter", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 367, 10, 0, 39, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.0, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 367, 10, 0, 47, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 6.0, 40, 100, 0, 35, 5.0, 4.4, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 40, 100, 0, 35, 4.3, -1.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		if *SMALL_STATE == 1 {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialn3", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialn3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_TURN);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_TURN);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legl"), 25.0, 361, 80, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legl"), 23.0, 361, 80, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialairn3", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairn3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_TURN);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_TURN);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_DIR_DECIDE);
		WorkModule::set_int(module_accessor, 1, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_BAKURETU_KICK_AIR_PHASE);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legl"), 21.25, 361, 80, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legl"), 19.549999, 361, 80, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 63.0);
	if is_excute(fighter) {
		WorkModule::set_int(module_accessor, 2, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_BAKURETU_KICK_AIR_PHASE);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialn3turn", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialn3turn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.43);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legl"), 28.0, 361, 77, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legl"), 26.0, 361, 77, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialairn3turn", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairn3turn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.43);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 48.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_BAKURETU_KICK_DIR_DECIDE);
		WorkModule::set_int(module_accessor, 1, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_BAKURETU_KICK_AIR_PHASE);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legl"), 23.799999, 361, 77, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("legl"), 22.1, 361, 77, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 63.0);
	if is_excute(fighter) {
		WorkModule::set_int(module_accessor, 2, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_BAKURETU_KICK_AIR_PHASE);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialhi2", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialhi2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.5;
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_AIR_START);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 75, 100, 70, 0, 6.2, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 75, 75, 70, 0, 4.0, 0.0, 14.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_FINISH_ANGLE);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, 55, 100, 0, 65, 6.0, 0.0, 9.0, 10.0, Some(0.0), Some(5.0), Some(2.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_ATTACK_ANGLE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 48.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_END_FLAG_NORMAL_ACCEL_Y);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_ROLL_BACK_ANGLE);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_specialairhi2", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairhi2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.5;
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, 4.5, 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 60, 0, 4.5, 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, 4.5, 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, 4.5, 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 70, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 80, 100, 60, 0, SIZE1[get_player_number(module_accessor)], 0.0, 13.5, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 68, 100, 130, 0, SIZE1[get_player_number(module_accessor)], 0.0, 6.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] = 6.2;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_FINISH_ANGLE);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, 43, 100, 0, 65, 6.0, 0.0, 9.0, 10.0, Some(0.0), Some(5.0), Some(2.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_ATTACK_ANGLE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 48.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_END_FLAG_NORMAL_ACCEL_Y);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_SYOTEN_KICK_FLAG_SET_ROLL_BACK_ANGLE);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_speciallw1", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_speciallw1(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 10.0);
	if is_excute(fighter) {
		HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
	}
	for i in 0..10 {
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			ModelModule::set_scale(module_accessor, ModelModule::scale(module_accessor) - 0.01);
		}
	}
	if is_excute(fighter) {
		HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let SMALL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
		*SMALL_STATE = 1;
		*SMALL_TIMER = 480;
		StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
	}
	frame(lua_state, 43.0);
	if is_excute(fighter) {
		StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), true);
	}
	frame(lua_state, 80.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
}

#[acmd_script( agent = "miifighter", script = "game_specialairlw1", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairlw1(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 10.0);
	if is_excute(fighter) {
		HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
	}
	for i in 0..10 {
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			ModelModule::set_scale(module_accessor, ModelModule::scale(module_accessor) - 0.01);
		}
	}
	if is_excute(fighter) {
		HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let SMALL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
		*SMALL_STATE = 1;
		*SMALL_TIMER = 480;
	}
	frame(lua_state, 65.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
}

#[acmd_script( agent = "miifighter", script = "game_specialairlw2kick", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairlw2kick(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		SET_SPEED_EX(fighter, -4, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 35, 90, 0, 65, 5.8, 4.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 60, 90, 0, 65, 5.8, 4.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 0.455);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_speciallw3", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_speciallw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
	}
	frame(lua_state, 8.0);
	FT_MOTION_RATE(fighter, 0.8);
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
	}
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, true);
	}
}

#[acmd_script( agent = "miifighter", script = "game_specialairlw3", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_specialairlw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
	}
	frame(lua_state, 8.0);
	FT_MOTION_RATE(fighter, 0.8);
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
	}
	FT_MOTION_RATE(fighter, 0.5);
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, true);
	}
}

#[acmd_script( agent = "miifighter", script = "game_speciallw3catch", category = ACMD_GAME, low_priority )]
unsafe fn miifighter_speciallw3catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
			PostureModule::reverse_lr(module_accessor);
			PostureModule::update_rot_y_lr(module_accessor);
		}
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 11.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(4.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
	}
}

#[acmd_script( agent = "miifighter", script = "game_specialairlw3catch", category = ACMD_GAME, low_priority )]
unsafe fn miifighter_specialairlw3catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
			PostureModule::reverse_lr(module_accessor);
			PostureModule::update_rot_y_lr(module_accessor);
		}
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 11.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(4.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
	}
}

#[acmd_script( agent = "miifighter", script = "game_speciallw3throw", category = ACMD_GAME, low_priority )]
unsafe fn miifighter_speciallw3throw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(fighter);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 74, 50, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, 5, 1);
		sv_animcmd::FT_CATCH_STOP(lua_state);
		fighter.clear_lua_stack();
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 85, 0, 80, 4.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(-11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
		fighter.clear_lua_stack();
		lua_args!(fighter, 5, 1);
		sv_animcmd::FT_CATCH_STOP(lua_state);
		fighter.clear_lua_stack();
        CHECK_FINISH_CAMERA(fighter, 14, 0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        MotionModule::set_rate(module_accessor, 2.5);
    }
    
}

#[acmd_script( agent = "miifighter", script = "game_specialairlw3throw", category = ACMD_GAME, low_priority )]
unsafe fn miifighter_specialairlw3throw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(fighter);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 270, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, 5, 1);
		sv_animcmd::FT_CATCH_STOP(lua_state);
		fighter.clear_lua_stack();
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 85, 0, 80, 4.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(-11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
		fighter.clear_lua_stack();
		lua_args!(fighter, 5, 1);
		sv_animcmd::FT_CATCH_STOP(lua_state);
		fighter.clear_lua_stack();
        CHECK_FINISH_CAMERA(fighter, 14, 0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        MotionModule::set_rate(module_accessor, 2.5);
    }
    
}

#[acmd_script( agent = "miifighter", script = "effect_specialairlw3throw", category = ACMD_EFFECT, low_priority )]
unsafe fn miifighter_specialairlw3throw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		COL_PRI(fighter, 101);
		FLASH(fighter, 1, 1, 1, 0);
		EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
		LAST_EFFECT_SET_RATE(fighter, 0.8);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
		LAST_EFFECT_SET_RATE(fighter, 0.8);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		FLASH(fighter, 1, 1, 1, 0);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		COL_NORMAL(fighter);
	}
}

#[acmd_script( agent = "miifighter", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 30);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 5, 0);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 8.0, 0.0, 7.0, -8.0, Some(0.0), Some(7.0), Some(3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 9.0, 0.0, 8.0, 7.0, Some(0.0), Some(-4.0), Some(7.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 16.5, 0.0, 13.5, 4.0, Some(0.0), Some(13.5), Some(11.5), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_attack_keep_rumble(module_accessor, 0, true);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_FINAL_FLAG_ATTACK_START);
		AttackModule::set_no_damage_orbit_all(module_accessor, true, false);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_FINAL_FLAG_DISABLE_GOLD_EYE);
	}
	
}

#[acmd_script( agent = "miifighter", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn miifighter_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 30);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 5, 0);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 8.0, 0.0, 7.0, -8.0, Some(0.0), Some(7.0), Some(3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 9.0, 0.0, 8.0, 7.0, Some(0.0), Some(-4.0), Some(7.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 100, 0, 50, 16.5, 0.0, 13.5, 4.0, Some(0.0), Some(13.5), Some(11.5), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_attack_keep_rumble(module_accessor, 0, true);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_FINAL_FLAG_ATTACK_START);
		AttackModule::set_no_damage_orbit_all(module_accessor, true, false);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_MIIFIGHTER_STATUS_FINAL_FLAG_DISABLE_GOLD_EYE);
	}
	
}

pub unsafe fn get_frame_to_set_rate(module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 { 
	//I could probably just do !is_attack && COUNTERHIT == false but i am very lazy and also fair is a move
	let motion_kind = MotionModule::motion_kind(module_accessor);

	if motion_kind == hash40("attack_11") 
	|| motion_kind == hash40("attack_12") {
		return 3.0;
	}
	else if motion_kind == hash40("attack_100_end") {
		return 6.0;
	}
	else if motion_kind == hash40("attack_dash") 
	|| motion_kind == hash40("attack_air_f") {
		return 17.0;
	}
	else if motion_kind == hash40("attack_s3_s") 
	|| motion_kind == hash40("attack_s3_hi") 
	|| motion_kind == hash40("attack_s3_lw") {
		return 8.0;
	}
	else if motion_kind == hash40("attack_hi3") {
		return 12.0;
	}
	else if motion_kind == hash40("attack_lw3") {
		return 8.0;
	}
	else if motion_kind == hash40("attack_s4_s") 
	|| motion_kind == hash40("attack_s4_hi") 
	|| motion_kind == hash40("attack_s4_lw") 
	|| motion_kind == hash40("attack_air_lw") {
		return 18.0;
	}
	else if motion_kind == hash40("attack_hi4") {
		return 14.0;
	}
	else if motion_kind == hash40("attack_lw4")
	|| motion_kind == hash40("attack_air_hi") {
		return 10.0;
	}
	else if motion_kind == hash40("attack_air_n") {
		return 29.0;
	}
	else if motion_kind == hash40("attack_air_b") {
		return 9.0;
	}
	else {
		return 0.0;
	}
}

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
pub fn miifighter_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		
		if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
			let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
			let SMALL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];

			if !sv_information::is_ready_go() {
				*SMALL_STATE = 0;
				*SMALL_TIMER = 0;
				AttackModule::set_power_mul(module_accessor, 1.0);
				AttackModule::set_attack_scale(module_accessor, 1.0, false);
				ModelModule::set_scale(module_accessor, 1.15);
			}

			if *SMALL_TIMER == 0 {
				if *SMALL_STATE == 1 {
					*SMALL_STATE = 2;
					*SMALL_TIMER = 480;
				}				
				else if *SMALL_STATE == 2 {
					AttackModule::set_power_mul(module_accessor, 1.0);
					AttackModule::set_attack_scale(module_accessor, 1.0, false);
					ModelModule::set_scale(module_accessor, 1.15);
					*SMALL_STATE = 0;
				}
			}
			else {
				*SMALL_TIMER -= 1;
			}

			if *SMALL_STATE == 1 {
				AttackModule::set_power_mul(module_accessor, 0.9603);
				AttackModule::set_attack_scale(module_accessor, 0.9, false);
				let rate_frame = get_frame_to_set_rate(module_accessor);
				if rate_frame != 0.0 && MotionModule::frame(module_accessor) >= rate_frame {
					MotionModule::set_rate(module_accessor, 1.219);
				}
				if *SMALL_TIMER <= 10 {
					ModelModule::set_scale(module_accessor, ModelModule::scale(module_accessor) + 0.01);
				}
			}
			if *SMALL_STATE == 2 {
				AttackModule::set_power_mul(module_accessor, 1.0403);
				AttackModule::set_attack_scale(module_accessor, 1.1, false);
				let rate_frame = get_frame_to_set_rate(module_accessor);
				if rate_frame != 0.0 && MotionModule::frame(module_accessor) >= rate_frame {
					MotionModule::set_rate(module_accessor, 0.787);
				}
				if *SMALL_TIMER >= 470 {
					ModelModule::set_scale(module_accessor, ModelModule::scale(module_accessor) + 0.01);
				}
				else if *SMALL_TIMER <= 10 {
					ModelModule::set_scale(module_accessor, ModelModule::scale(module_accessor) - 0.01);
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		miifighter_attackairf,
		miifighter_specialn3,
		miifighter_specialairn3,
		miifighter_specialn3turn,
		miifighter_specialairn3turn,
		miifighter_specialhi2,
		miifighter_specialairhi2,
		miifighter_speciallw1,
		miifighter_specialairlw1,
		miifighter_specialairlw2kick,
		miifighter_speciallw3,
		miifighter_specialairlw3,
		miifighter_speciallw3catch,
		miifighter_specialairlw3catch,
		miifighter_speciallw3throw,
		miifighter_specialairlw3throw,
		miifighter_specialairlw3throw_effect,
		miifighter_catch,
		miifighter_catchdash,
		miifighter_catchturn,
		miifighter_finalstart,
		miifighter_finalairstart
	);
	install_agent_frames!(
		miifighter_functions
	);
}

