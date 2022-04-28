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
use crate::custom::FIGHTER_INT_3;
use crate::custom::FIGHTER_INT_4;
use crate::custom::FIGHTER_INT_5;
use crate::custom::FIGHTER_INT_6;
use crate::custom::FIGHTER_INT_7;
use crate::custom::FIGHTER_INT_8;
use crate::custom::FIGHTER_INT_9;
use crate::custom::FIGHTER_INT_10;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_U32_1;
use crate::custom::FIGHTER_U8_1;
use crate::custom::FIGHTER_U8_2;
use crate::custom::FIGHTER_U8_3;
use crate::custom::FIGHTER_U8_4;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;


#[acmd_script( agent = "brave", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn brave_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 4.0, 4.0);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneel"), 7.0, 77, 62, 0, 74, 2.6, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneel"), 7.0, 77, 62, 0, 74, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("kneel"), 7.0, 77, 62, 0, 74, 3.3, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "brave", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn brave_attacks4s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 17.0);
	if *CRIT {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("sword1"), 36.0, 361, 76, 0, 31, 3.3, 8.8, 0.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -18, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("sword1"), 36.0, 361, 76, 0, 31, 3.8, 3.0, 0.0, -1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -18, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("armr"), 32.0, 361, 76, 0, 31, 2.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *CRIT == false {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 3, 0, Hash40::new("sword1"), 18.0, 361, 76, 0, 62, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 4, 0, Hash40::new("sword1"), 18.0, 361, 76, 0, 62, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("armr"), 16.0, 361, 76, 0, 62, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		*CRIT = false;
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 4.0, 4.0);
	}
	
}

#[acmd_script( agent = "brave", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn brave_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 12.0);
	if *CRIT {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 28.0, 87, 76, 0, 60, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 28.0, 87, 76, 0, 60, 5.2, 0.0, 16.0, -0.3, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -14, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("sword1"), 32.0, 90, 76, 0, 60, 4.8, 9.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 3, 0, Hash40::new("sword1"), 32.0, 90, 76, 0, 60, 4.8, 2.5, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *CRIT == false {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 87, 76, 0, 60, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("top"), 14.0, 87, 76, 0, 60, 5.2, 0.0, 16.0, -0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 6, 0, Hash40::new("sword1"), 16.0, 90, 76, 0, 60, 4.8, 9.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 7, 0, Hash40::new("sword1"), 16.0, 90, 76, 0, 60, 4.8, 2.5, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		*CRIT = false;
	}
	
}

#[acmd_script( agent = "brave", script = "game_attacklw4", category = ACMD_GAME, low_priority)]
unsafe fn brave_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let SEED = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
	let DSMASH = &mut FIGHTER_INT_10[get_player_number(module_accessor)];
	let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 9.0);
	if *SEED < 8 {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, *DSMASH as u64, 113, 0, 10, 4.0, 0.0, 5.0, 4.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 26.0, *DSMASH as u64, 100, 0, 10, 3.6, 0.0, 4.0, 11.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("sword1"), 26.0, *DSMASH as u64, 100, 0, 10, 3.6, 8.6, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *SEED >= 8 {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 11.0, *DSMASH as u64, 113, 0, 45, 4.0, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, *DSMASH as u64, 100, 0, 45, 3.6, 0.0, 4.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("sword1"), 13.0, *DSMASH as u64, 100, 0, 45, 3.6, 8.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 20.0);
	if *CRIT {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 46, 113, 0, 10, 4.0, 0.0, 5.0, -4.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -11, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 26.0, 46, 100, 0, 10, 3.6, 0.0, 4.0, -11.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("sword1"), 26.0, 46, 100, 0, 10, 3.6, 8.6, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *CRIT == false {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 11.0, 46, 113, 0, 45, 4.0, 0.0, 5.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 46, 100, 0, 45, 3.6, 0.0, 4.0, -11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("sword1"), 13.0, 46, 100, 0, 45, 3.6, 8.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		*CRIT = false;
	}
	
}

#[acmd_script( agent = "brave", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn brave_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 14.0);
	if *CRIT {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 20.0, 361, 65, 0, 36, 2.5, 1.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("sword1"), 24.0, 361, 60, 0, 36, 3.5, 9.0, 0.0, -2.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("sword1"), 24.0, 361, 60, 0, 36, 4.0, 2.5, 0.0, -2.75, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *CRIT == false {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 3, 0, Hash40::new("armr"), 10.0, 361, 97, 0, 36, 2.5, 1.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 4, 0, Hash40::new("sword1"), 12.0, 361, 89, 0, 36, 3.5, 9.0, 0.0, -2.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("sword1"), 12.0, 361, 89, 0, 36, 4.0, 2.5, 0.0, -2.75, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		*CRIT = false;
	}
	frame(lua_state, 42.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "brave", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn brave_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 18.0);
	if *CRIT {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 24.0, 36, 65, 0, 32, 2.5, 1.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 1, 0, Hash40::new("sword1"), 28.0, 36, 60, 0, 32, 3.5, 9.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 2, 0, Hash40::new("sword1"), 28.0, 36, 60, 0, 32, 4.0, 2.5, 0.0, -2.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		}
	}
	if *CRIT == false {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
			WorkModule::off_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
			ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.0, 36, 96, 0, 32, 2.5, 1.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 4, 0, Hash40::new("sword1"), 14.0, 36, 89, 0, 32, 3.5, 9.0, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
			ATTACK(fighter, 5, 0, Hash40::new("sword1"), 14.0, 36, 89, 0, 32, 4.0, 2.5, 0.0, -2.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		*CRIT = false;
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "brave", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn brave_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 2.0, 3.0, 3.0, 5.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 73, 93, 0, 49, 4.0, 1.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 73, 93, 0, 49, 5.0, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "brave", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn brave_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "brave", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn brave_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "brave", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn brave_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "brave", script = "game_speciallw19", category = ACMD_GAME, low_priority)]
unsafe fn brave_speciallw19(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 1.0, 361, 220, 10, 10, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 6.0, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "brave", script = "game_specialairlw19", category = ACMD_GAME, low_priority)]
unsafe fn brave_specialairlw19(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 1.0, 361, 220, 10, 10, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 1.0, 361, 220, 10, 10, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup_metal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 6.0, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "brave", script = "game_speciallw20", category = ACMD_GAME, low_priority)]
unsafe fn brave_speciallw20(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "brave", script = "game_specialairlw20", category = ACMD_GAME, low_priority)]
unsafe fn brave_specialairlw20(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_PLAY_MISS_SE);
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.3, 8.8, 0.0, -2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 35.0, 55, 46, 0, 110, 3.8, 3.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BRAVE_CRITICALHIT, *ATTACK_REGION_SWORD);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "brave", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn brave_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 30.0, 60.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 50);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), true);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 56.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 8.0, 0.0, 3.0, 5.0, Some(0.0), Some(3.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 18.0, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 18.0, 0.0, 11.0, 15.0, Some(0.0), Some(22.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 65.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_STATUS_FINAL_FLAG_END_FINAL);
	}
	
}

#[acmd_script( agent = "brave", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn brave_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 30.0, 60.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 50);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), true);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 56.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 8.0, 0.0, 3.0, 5.0, Some(0.0), Some(3.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 18.0, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 25, 0, 25, 18.0, 0.0, 11.0, 15.0, Some(0.0), Some(22.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 65.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BRAVE_STATUS_FINAL_FLAG_END_FINAL);
	}
	
}

pub unsafe fn spell_check(spell_seed: u8) -> i32 {
	if spell_seed > 0 && spell_seed <= 4 {
		return 0;
	}
	else if spell_seed >= 5 && spell_seed <= 22 {
		return 1;
	}
	else if spell_seed >= 23 && spell_seed <= 38 {
		return 2;
	}
	else if spell_seed >= 39 && spell_seed <= 56 {
		return 3;
	}
	else if spell_seed >= 57 && spell_seed <= 70 {
		return 4;
	}
	else if spell_seed >= 71 && spell_seed <= 80 {
		return 5;
	}
	else if spell_seed >= 81 && spell_seed <= 86 {
		return 6;
	}
	else if spell_seed >= 87 && spell_seed <= 100 {
		return 7;
	}
	else if spell_seed >= 101 && spell_seed <= 104 {
		return 8;
	}
	else if spell_seed >= 105 && spell_seed <= 108 {
		return 9;
	}
	else if spell_seed >= 109 && spell_seed <= 122 {
		return 10;
	}
	else if spell_seed >= 123 && spell_seed <= 136 {
		return 11;
	}
	else if spell_seed >= 137 && spell_seed <= 150 {
		return 12;
	}
	else if spell_seed >= 151 && spell_seed <= 165 {
		return 13;
	}
	else if spell_seed >= 166 && spell_seed <= 176 {
		return 14;
	}
	else if spell_seed >= 177 && spell_seed <= 189 {
		return 15;
	}
	else if spell_seed >= 190 && spell_seed <= 205 {
		return 16;
	}
	else if spell_seed >= 206 && spell_seed <= 221 {
		return 17;
	}
	else if spell_seed >= 222 && spell_seed <= 226 {
		return 18;
	}
	else if spell_seed >= 227 && spell_seed <= 242 {
		return 19;
	}
	else {
		return 20;
	}
}

pub unsafe fn spell_ok(player_number: usize, spell: u8) -> bool {
	let PREV_SPELL_1 = &mut FIGHTER_INT_6[player_number];
	let PREV_SPELL_2 = &mut FIGHTER_INT_7[player_number];
	let PREV_SPELL_3 = &mut FIGHTER_INT_8[player_number];
	let PREV_SPELL_4 = &mut FIGHTER_INT_9[player_number];
	if spell_check(spell) != *PREV_SPELL_1
	&& spell_check(spell) != *PREV_SPELL_2
	&& spell_check(spell) != *PREV_SPELL_3
	&& spell_check(spell) != *PREV_SPELL_4 {
		return true;
	}
	else {
		return false;
	}
}

#[fighter_frame( agent = FIGHTER_KIND_BRAVE )]
pub fn brave_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();
		let SEED = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let SPELL_RNG = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let SPELL_1 = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
		let SPELL_2 = &mut FIGHTER_INT_3[get_player_number(module_accessor)];
		let SPELL_3 = &mut FIGHTER_INT_4[get_player_number(module_accessor)];
		let SPELL_4 = &mut FIGHTER_INT_5[get_player_number(module_accessor)];
		let PREV_SPELL_1 = &mut FIGHTER_INT_6[get_player_number(module_accessor)];
		let PREV_SPELL_2 = &mut FIGHTER_INT_7[get_player_number(module_accessor)];
		let PREV_SPELL_3 = &mut FIGHTER_INT_8[get_player_number(module_accessor)];
		let PREV_SPELL_4 = &mut FIGHTER_INT_9[get_player_number(module_accessor)];
		let SPELL_SEED_ID = &mut FIGHTER_U32_1[get_player_number(module_accessor)];
		let SPELL_SEED_1 = &mut FIGHTER_U8_1[get_player_number(module_accessor)];
		let SPELL_SEED_2 = &mut FIGHTER_U8_2[get_player_number(module_accessor)];
		let SPELL_SEED_3 = &mut FIGHTER_U8_3[get_player_number(module_accessor)];
		let SPELL_SEED_4 = &mut FIGHTER_U8_4[get_player_number(module_accessor)];
		let DSMASH = &mut FIGHTER_INT_10[get_player_number(module_accessor)];
		let CRIT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_BRAVE {
			if smashball::is_training_mode() {
				if motion_kind == hash40("appeal_lw_l") && MotionModule::frame(module_accessor) == 0.0 {
					if *SPELL_RNG {
						*SPELL_RNG = false;
					}
					else {
						*SPELL_RNG = true;
					}
				}
			}
			else {
				if (motion_kind == hash40("appeal_lw_l") || motion_kind == hash40("appeal_lw_r")) && MotionModule::frame(module_accessor) == 0.0 {
					if *SPELL_RNG {
						*SPELL_RNG = false;
					}
					else {
						*SPELL_RNG = true;
					}
				}
			}
			if !sv_information::is_ready_go() {
				*SPELL_1 = 21;
				*SPELL_2 = 21;
				*SPELL_3 = 21;
				*SPELL_4 = 21;
				*PREV_SPELL_1 = 21;
				*PREV_SPELL_2 = 21;
				*PREV_SPELL_3 = 21;
				*PREV_SPELL_4 = 21;
			}
			*SPELL_SEED_ID = smash::crc32::crc32(&[*SEED as u8]);
			*SPELL_SEED_1 = ((*SPELL_SEED_ID >> 24) & 0xff) as u8;
			*SPELL_SEED_2 = ((*SPELL_SEED_ID >> 16) & 0xff) as u8;
			*SPELL_SEED_3 = ((*SPELL_SEED_ID >> 8) & 0xff) as u8;
			*SPELL_SEED_4 = ((*SPELL_SEED_ID) & 0xff) as u8;

			if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT {
				*PREV_SPELL_1 = *SPELL_1;
				*PREV_SPELL_2 = *SPELL_2;
				*PREV_SPELL_3 = *SPELL_3;
				*PREV_SPELL_4 = *SPELL_4;
			}
			if motion_kind != hash40("attack_s4_s") && motion_kind != hash40("attack_hi4") && motion_kind != hash40("attack_lw4") && motion_kind != hash40("attack_air_f") && motion_kind != hash40("attack_air_b") && motion_kind != hash40("attack_s4_hold") && motion_kind != hash40("attack_hi4_hold") && motion_kind != hash40("attack_lw4_hold") {
				*CRIT = false;
			}
			if status_kind == *FIGHTER_STATUS_KIND_ENTRY || (smashball::is_training_mode() == true && (motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_lw_l"))) || globals["crit_ok"].get_int() as i32 == 1 {
				if globals["seed_check"].get_bool() == false {
					globals["seed_id"] = (sv_math::rand(hash40("fighter"), 16)).into();
					if globals["seed_id"].get_int() as i32 > 15 {
						globals["seed_id"] = (globals["seed_id"].get_int() as i32 - 16).into();
					}
					if globals["seed_id"].get_int() as i32 == 0 {
						*SEED = 23;
					}
					if globals["seed_id"].get_int() as i32 == 1 {
						*SEED = 141;
					}
					if globals["seed_id"].get_int() as i32 == 2 {
						*SEED = 69;
					}
					if globals["seed_id"].get_int() as i32 == 3 {
						*SEED = 0;
					}
					if globals["seed_id"].get_int() as i32 == 4 {
						*SEED = 245;
					}
					if globals["seed_id"].get_int() as i32 == 5 {
						*SEED = 113;
					}
					if globals["seed_id"].get_int() as i32 == 6 {
						*SEED = 84;
					}
					if globals["seed_id"].get_int() as i32 == 7 {
						*SEED = 210;
					}
					if globals["seed_id"].get_int() as i32 == 8 {
						*SEED = 99;
					}
					if globals["seed_id"].get_int() as i32 == 9 {
						*SEED = 55;
					}
					if globals["seed_id"].get_int() as i32 == 10 {
						*SEED = 40;
					}
					if globals["seed_id"].get_int() as i32 == 11 {
						*SEED = 195;
					}
					if globals["seed_id"].get_int() as i32 == 12 {
						*SEED = 222;
					}
					if globals["seed_id"].get_int() as i32 == 13 {
						*SEED = 166;
					}
					if globals["seed_id"].get_int() as i32 == 14 {
						*SEED = 156;
					}
					if globals["seed_id"].get_int() as i32 == 15 {
						*SEED = 232;
					}
					if smashball::is_training_mode() == true {
						if motion_kind == hash40("appeal_lw_r") {
							DamageModule::add_damage(module_accessor, *SEED as f32, 0);
						}
					}
					globals["seed_check"] = true.into();
				}
			}
			if (motion_kind != hash40("appeal_lw_r")) && (motion_kind != hash40("appeal_lw_l")) && (status_kind != *FIGHTER_STATUS_KIND_ENTRY) {
				globals["seed_check"] = false.into();
			}
			if *SEED > 255 {
				*SEED -= 256;
			}
			if *SEED < 0 {
				*SEED += 256;
			}
			if *CRIT {
				globals["crit_ok"] = (globals["crit_ok"].get_int() as i32 + 1).into();
			}
			else {
				globals["crit_ok"] = 0.into();
			}
			if motion_kind == hash40("atack_11") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 19;
				}
			}
			if motion_kind == hash40("atack_12") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED *= 3;
				}
			}
			if motion_kind == hash40("atack_13") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 1;
				}
			}
			if motion_kind == hash40("attack_s3_s") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 6;
				}
			}
			if motion_kind == hash40("attack_s3_s2") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 20;
				}
			}
			if motion_kind == hash40("attack_hi3") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 123;
				}
			}
			if motion_kind == hash40("attack_lw3") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 87;
				}
			}
			if motion_kind == hash40("attack_s4_s") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 5;
					if *SEED > 255 {
						*SEED -= 256;
					}
					if *SEED < 0 {
						*SEED += 256;
					}
					if *SEED < 8 {
						*CRIT = true;
					}
				}
			}
			if motion_kind == hash40("attack_hi4") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 3;
					if *SEED > 255 {
						*SEED -= 256;
					}
					if *SEED < 0 {
						*SEED += 256;
					}
					if *SEED < 8 {
						*CRIT = true;
					}
				}
			}
			if globals["seed_changed"].get_bool() == false {
				if motion_kind == hash40("attack_lw4") {
					*DSMASH = app::sv_math::rand(hash40("fighter"), 368) + 1;
					if MotionModule::frame(module_accessor) >= 9.0 && MotionModule::frame(module_accessor) < 19.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
							*SEED += *DSMASH;
							if *SEED > 255 {
								*SEED -= 256;
							}
							if *SEED < 0 {
								*SEED += 256
							}
							globals["seed_changed"] = true.into();
						}
						else {
							*SEED += 11;
							if *SEED > 255 {
								*SEED -= 256;
							}
							if *SEED < 0 {
								*SEED += 256;
							}
							if *SEED < 8 {
								*CRIT = true;
							}
							globals["seed_changed"] = true.into();
						}
					}
				}
			}
			if motion_kind == hash40("attack_dash") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED *= 2;
				}
			}
			if motion_kind == hash40("attack_air_n") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 69;
				}
			}
			if motion_kind == hash40("attack_air_f") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 8;
					if *SEED > 255 {
						*SEED -= 256;
					}
					if *SEED < 0 {
						*SEED += 256;
					}
					if *SEED < 8 {
						*CRIT = true;
					}
				}
			}
			if motion_kind == hash40("attack_air_b") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 8;
					if *SEED > 255 {
						*SEED -= 256;
					}
					if *SEED < 0 {
						*SEED += 256;
					}
					if *SEED < 8 {
						*CRIT = true;
					}
				}
			}
			if motion_kind == hash40("attack_air_hi") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 32;
				}
			}
			if motion_kind == hash40("attack_air_lw") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 226;
				}
			}
			if motion_kind == hash40("throw_f") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 176;
				}
			}
			if motion_kind == hash40("throw_b") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 101;
				}
			}
			if motion_kind == hash40("throw_hi") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED /= 2;
				}
			}
			if motion_kind == hash40("throw_lw") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 50;
				}
			}
			if motion_kind == hash40("special_n1") || motion_kind == hash40("special_air_n1") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
						*SEED += 9;
					}
					else {
						*SEED += WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32;
					}
				}
			}
			if motion_kind == hash40("special_n2") || motion_kind == hash40("special_air_n2") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
						*SEED += 18;
					}
					else {
						*SEED += (WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32) * 2;
					}
				}
			}
			if motion_kind == hash40("special_n3") || motion_kind == hash40("special_air_n3") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
						*SEED += 27;
					}
					else {
						*SEED += (WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32) * 3;
					}
				}
			}
			if motion_kind == hash40("special_s1") || motion_kind == hash40("special_air_s1") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
						*SEED -= 12;
					}
					else {
						*SEED -= WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32;
					}
				}
			}
			if motion_kind == hash40("special_s2") || motion_kind == hash40("special_air_s2") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
						*SEED -= 24;
					}
					else {
						*SEED -= (WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32) * 2;
					}
				}
			}
			if motion_kind == hash40("special_s3") || motion_kind == hash40("special_air_s3") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
						*SEED -= 36;
					}
					else {
						*SEED -= (WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32) * 3;
					}
				}
			}
			if motion_kind == hash40("special_hi1") || motion_kind == hash40("special_air_hi1") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 7;
				}
			}
			if motion_kind == hash40("special_hi2") || motion_kind == hash40("special_air_hi2") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 14;
				}
			}
			if motion_kind == hash40("special_hi3") || motion_kind == hash40("special_air_hi3") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 21;
				}
			}
			if motion_kind == hash40("special_hi_empty") || motion_kind == hash40("special_air_hi_empty") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32;
				}
			}
			if motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_hi_l") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= WorkModule::get_float(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) as i32;
				}
			}
			if motion_kind == hash40("guard_off") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED -= 1;
				}
			}
			if motion_kind == hash40("just_shield_off") {
				*SEED = 187;
			}
			if motion_kind == hash40("appeal_s_r") || motion_kind == hash40("appeal_s_l") {
				if MotionModule::frame(module_accessor) == 0.0 {
					*SEED += 66;
				}
			}
			if motion_kind != hash40("attack_lw4") || (motion_kind == hash40("attack_lw4") && MotionModule::frame(module_accessor) == 0.0) {
				globals["seed_changed"] = false.into();
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		brave_attacklw3,
		brave_attacks4s,
		brave_attackhi4,
		brave_attacklw4,
		brave_attackairf,
		brave_attackairb,
		brave_attackairhi,
		brave_catch,
		brave_catchdash,
		brave_catchturn,
		brave_speciallw19,
		brave_specialairlw19,
		brave_speciallw20,
		brave_specialairlw20,
		brave_finalstart,
		brave_finalairstart
	);
	smashline::install_agent_frames!(brave_functions);
}

