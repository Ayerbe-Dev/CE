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
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_FLOAT_1;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;


#[acmd_script( agent = "ken", script = "game_attack13w", category = ACMD_GAME, low_priority)]
unsafe fn ken_attack13w(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 70, 3.0, 0.0, 11.0, 11.5, Some(0.0), Some(11.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 70, 1.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attack11s", category = ACMD_GAME, low_priority)]
unsafe fn ken_attack11s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 70, 0, 70, 3.0, 0.0, 11.0, 11.5, Some(0.0), Some(11.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 45, 70, 0, 70, 1.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attack11nears", category = ACMD_GAME, low_priority)]
unsafe fn ken_attack11nears(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 80, 100, 120, 0, 3.0, 1.7, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 12.0, 80, 100, 120, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 80, 100, 120, 0, 5.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 80, 100, 120, 0, 3.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(10.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 3, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attacks3w", category = ACMD_GAME, low_priority)]
unsafe fn ken_attacks3sw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attacknearw", category = ACMD_GAME, low_priority)]
unsafe fn ken_attacknearw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 2.0, 2.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 180, 100, 10, 0, 3.2, 0.0, 12.5, 9.0, Some(0.0), Some(7.5), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 75, 23, 0, 16, 3.2, 0.0, 12.5, 9.0, Some(0.0), Some(7.5), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 9.0, false);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attackhi3w", category = ACMD_GAME, low_priority)]
unsafe fn ken_attackhi3w(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 6.0, 4.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 75, 0, 19, 5.0, 0.0, 16.0, 4.8, Some(2.0), Some(14.5), Some(4.8), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 75, 0, 19, 5.0, 0.0, 16.0, 6.0, Some(2.0), Some(14.5), Some(6.0), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 75, 75, 0, 19, 5.0, 0.0, 16.0, 4.8, Some(2.0), Some(14.5), Some(4.8), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 75, 75, 0, 19, 5.0, 0.0, 16.0, 6.0, Some(2.0), Some(14.5), Some(6.0), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 4.0, 4.0);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specials", category = ACMD_GAME, low_priority)]
unsafe fn ken_specials(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 5.5, 3.0, 9.0, 3.0);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
	}
	wait(lua_state, 1.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.666);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 160, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 45, 160, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.666);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 35, 30, 0, 62, 4.5, 0.0, 12.5, 9.0, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 35, 30, 0, 31, 4.5, 0.0, 12.5, 12.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.666);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 35, 30, 0, 62, 4.5, 0.0, 12.5, 9.0, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 35, 30, 0, 31, 4.5, 0.0, 12.5, 12.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 135, 120, 0, 30, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 135, 120, 0, 30, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 145, 30, 0, 62, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 145, 30, 0, 62, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 145, 30, 0, 62, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 145, 30, 0, 62, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specialairs", category = ACMD_GAME, low_priority)]
unsafe fn ken_specialairs(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	wait(lua_state, 1.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 0.6);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 45, 160, 0, 26, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 0.6);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 35, 30, 0, 62, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 0.6);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 35, 30, 0, 62, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 45, 120, 0, 30, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 35, 30, 0, 62, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 35, 30, 0, 62, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		}
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn ken_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.6;
		SIZE2[get_player_number(module_accessor)] = 5.5;
	}
	frame(lua_state, 5.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 6.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 121, 0, 78, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 70, 126, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specialhicommand", category = ACMD_GAME, low_priority)]
unsafe fn ken_specialhicommand(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		SIZE1[get_player_number(module_accessor)] = 4.6;
		SIZE2[get_player_number(module_accessor)] = 5.5;
	}
	frame(lua_state, 5.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 6.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 121, 0, 78, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 70, 126, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn ken_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.6;
		SIZE2[get_player_number(module_accessor)] = 5.5;
	}
	frame(lua_state, 5.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 6.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 55, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 54, 0, 80, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 150, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 117, 0, 78, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 87, 0, 76, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_specialairhicommand", category = ACMD_GAME, low_priority)]
unsafe fn ken_specialairhicommand(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		SIZE1[get_player_number(module_accessor)] = 4.6;
		SIZE2[get_player_number(module_accessor)] = 5.5;
	}
	frame(lua_state, 5.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 6.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 55, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 54, 0, 80, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 150, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
			}
		}
	}
	frame(lua_state, 9.0);
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 117, 0, 78, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 87, 0, 76, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_attackcommand1", category = ACMD_GAME, low_priority)]
unsafe fn ken_attackcommand1(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 75, 80, 0, 10, 4.0, 0.0, 13.5, 6.5, Some(0.0), Some(4.0), Some(4.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 75, 80, 0, 10, 4.0, 0.0, 13.5, 6.5, Some(0.0), Some(13.5), Some(0.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 2.0, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BRANCH);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 63, 30, 0, 40, 3.5, 0.0, 4.0, 4.0, Some(0.0), Some(17.799999), Some(9.5), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 63, 30, 0, 40, 3.5, 0.0, 16.0, 0.0, Some(0.0), Some(17.799999), Some(7.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ken_hadoken", script = "game_movew", category = ACMD_GAME, low_priority)]
unsafe fn ken_hadoken_movew(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		}
	}
	wait(lua_state, 7.0);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	
}

#[acmd_script( agent = "ken_hadoken", script = "game_movem", category = ACMD_GAME, low_priority)]
unsafe fn ken_hadoken_movem(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		}
	}
	wait(lua_state, 6.0);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	
}

#[acmd_script( agent = "ken_hadoken", script = "game_moves", category = ACMD_GAME, low_priority)]
unsafe fn ken_hadoken_moves(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 5.0, false);
	}
	wait(lua_state, 5.0);
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
		}
	}
	
}

#[acmd_script( agent = "ken", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn ken_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.5, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ken", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn ken_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ken", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn ken_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-14.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ken", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn ken_throwhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 88, 75, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, 6, 20);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.3);
		lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 5.0, y: 4.0, z: 0.0 });
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 30, 2.6, -1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 30, 3.7, 5.3, 0.5, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	wait(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "ken", script = "game_appeallwr", category = ACMD_GAME, low_priority)]
unsafe fn ken_appeallwr(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 1.0, 65, 100, 20, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 1.0, 65, 100, 20, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_appeallwl", category = ACMD_GAME, low_priority)]
unsafe fn ken_appeallwl(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 1.0, 65, 100, 20, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 1.0, 65, 100, 20, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ken", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn ken_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 0, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 10.0, 70.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 40);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), true);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 15.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 15.0, y: 1.0 };
        let vec2 = Vector2f{ x: 19.0, y: 5.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 9, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 20.0, y: 2.0 };
        let vec2 = Vector2f{ x: 20.0, y: 4.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 13, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
     }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 22.0, y: 4.0 };
        let vec2 = Vector2f{ x: 22.0, y: 7.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 10, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
     }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 22.0, y: 4.0 };
        let vec2 = Vector2f{ x: 26.0, y: 5.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 14, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 77.0);
    if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);		
		}			
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(module_accessor);
	}
}

#[acmd_script( agent = "ken", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn ken_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 0, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 10.0, 70.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 40);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), true);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 15.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 15.0, y: 1.0 };
        let vec2 = Vector2f{ x: 19.0, y: 5.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 9, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 20.0, y: 2.0 };
        let vec2 = Vector2f{ x: 20.0, y: 4.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 13, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
     }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 22.0, y: 4.0 };
        let vec2 = Vector2f{ x: 22.0, y: 7.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 10, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
     }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
        AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new_raw(0x192e7100d1u64));
        AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new_raw(0x192e7100d1u64));
        let vec1 = Vector2f{ x: 22.0, y: 4.0 };
        let vec2 = Vector2f{ x: 26.0, y: 5.0 };
        AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec1, 14, false);
        AttackModule::set_vec_target_pos(module_accessor, 1, Hash40::new("top"), &vec2, 15, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
	frame(lua_state, 75.0);
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);		
	}
    frame(lua_state, 76.0);
    if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
		}
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(module_accessor);
    }
    
}

#[acmd_script( agent = "ken_shinryuken", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn ken_shinryuken_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.3, 110, 120, 130, 0, 11.0, 0.0, -20.0, 0.0, Some(0.0), Some(80.0), Some(0.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 100, 0, 11.0, 0.0, 80.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *WEAPON_KEN_SHINRYUKEN_INSTANCE_WORK_ID_FLAG_ADD_ATTACK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 110, 85, 70, 0, 11.0, 0.0, -20.0, 0.0, Some(0.0), Some(80.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 367, 100, 100, 0, 11.0, 0.0, 80.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *WEAPON_KEN_SHINRYUKEN_INSTANCE_WORK_ID_FLAG_ADD_ATTACK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 70, 63, 0, 105, 11.0, 0.0, -20.0, 0.0, Some(0.0), Some(80.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 70, 63, 0, 105, 11.0, 0.0, 80.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
pub fn ken_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let PJAB_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];
		let PJAB_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let PTILT_CHECK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_KEN {
			for i in 0 .. TOTAL_FIGHTER {
				let o = i as usize;
				if (PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(i))).abs() < 14.0 && (PostureModule::pos_y(module_accessor) - PostureModule::pos_y(get_boma(i))).abs() < 5.0 && o != get_player_number(module_accessor) {
					*PTILT_CHECK = true;
					break;
				}		
				else {
					*PTILT_CHECK = false;
				}	
			}
			for i in 0 .. TOTAL_FIGHTER {
				let o = i as usize;
				if (PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(i))).abs() < 15.0 && (PostureModule::pos_y(module_accessor) - PostureModule::pos_y(get_boma(i))).abs() < 25.0 && o != get_player_number(module_accessor) {
					*PJAB_CHECK = true;
					break;
				}		
				else {
                    *PJAB_CHECK = false;
				}	
			}

			if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && MotionModule::frame(module_accessor) < 4.0 && !AttackModule::is_attack(module_accessor, 0, false) && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {				
				if motion_kind == hash40("attack_11_s") || motion_kind == hash40("attack_11_near_s") {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_11_w"), 1.0, 1.0, false, 0.0, false, false);
				}
				if motion_kind == hash40("attack_s3_s_s") {
					if *PTILT_CHECK {
						MotionModule::change_motion(module_accessor, Hash40::new("attack_near_w"), 1.0, 1.0, false, 0.0, false, false);					
					}
					else {
						MotionModule::change_motion(module_accessor, Hash40::new("attack_s3_s_w"), 1.0, 1.0, false, 0.0, false, false);					
					}
				}

				if motion_kind == hash40("attack_hi3_s") {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_hi3_w"), 1.0, 1.0, false, 0.0, false, false);					
				}
				if motion_kind == hash40("attack_lw3_s") {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_lw3_w"), 1.0, 1.0, false, 0.0, false, false);
				}
			}

			if motion_kind == hash40("attack_11_near_s") || motion_kind == hash40("attack_11_w") {
				if motion_kind == hash40("attack_11_near_s") {
					if MotionModule::frame(module_accessor) > 6.0 && !CancelModule::is_enable_cancel(module_accessor) {
						*PJAB_FRAME = MotionModule::frame(module_accessor);
					}
					else {
						*PJAB_FRAME = -1.0;
					}
				}
				else if *PJAB_FRAME != -1.0 {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_11_near_s"), *PJAB_FRAME, 1.0, false, 0.0, false, false);
				}
			}
			else {
				*PJAB_FRAME = -1.0;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		ken_attack13w,
		ken_attack11s,
		ken_attack11nears,
		ken_attacks3sw,
		ken_attacknearw,
		ken_attackhi3w,
		ken_specials,
		ken_specialairs,
		ken_specialhi,
		ken_specialhicommand,
		ken_specialairhi,
		ken_specialairhicommand,
		ken_attackcommand1,
		ken_hadoken_movew,
		ken_hadoken_movem,
		ken_hadoken_moves,
		ken_catch,
		ken_catchdash,
		ken_catchturn,
		ken_throwhi,
		ken_appeallwr,
		ken_appeallwl,
		ken_final,
		ken_finalair,
		ken_shinryuken_final
	);
	smashline::install_agent_frames!(ken_functions);
}

