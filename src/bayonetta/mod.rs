use smash::lua2cpp::L2CAgentBase;
use smash_script::{*, macros::*};
use smashline::*;
use smash::hash40;
use smash::app::{self, lua_bind::*, sv_animcmd::{frame, wait}, *};
use smash::lib::{lua_const::{self, *}, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
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
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use crate::custom::FIGHTER_BOOL_4;
use crate::custom::FINAL_WAIT;
use crate::custom::FINAL_HIT;
use crate::custom::START_FS;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;


#[acmd_script( agent = "bayonetta", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10, 3, 10, 5, true);
		MotionModule::set_rate(module_accessor, 1.5);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 361, 8, 0, 36, 2.8, 0.0, 10.0, 7.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.4, 361, 8, 0, 36, 2.6, 0.0, 10.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.4, 361, 8, 0, 32, 2.2, 0.0, 5.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.4, 180, 6, 0, 32, 2.8, 0.0, 10.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 1.4, 361, 6, 0, 32, 2.8, 0.0, 10.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 4, 9.0, false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attack13(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 10, 5, true);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10);
		MotionModule::set_rate(module_accessor, 1.667);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 8, 0, 40, 6.5, 0.0, 7.0, 10.0, Some(0.0), Some(10.5), Some(10.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 10.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attacks3s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 5, true);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 58, 15, 0, 55, 4.0, 0.0, 12.5, 8.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 45, 15, 0, 45, 4.0, 0.0, 12.5, 8.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 70, 15, 0, 50, 4.0, 0.0, 12.5, 16.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 70, 15, 0, 35, 4.0, 0.0, 12.5, 16.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 50, 15, 0, 60, 3.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.176);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 10, 5, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 105, 105, 120, 0, 5.0, 0.0, 7.0, 9.5, Some(0.0), Some(10.0), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 105, 105, 120, 0, 5.0, 0.0, 7.0, 9.5, Some(0.0), Some(10.0), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 1, false);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 105, 100, 65, 65, 4.5, 0.0, 18.0, 7.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		AttackModule::clear(module_accessor, 0, false);
		ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 93, 60, 0, 55, 5.5, 0.0, 25.5, -1.0, Some(0.0), Some(25.5), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 6.0, 93, 60, 0, 55, 2.0, 0.0, 20.0, -3.0, Some(0.0), Some(20.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.36);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 15, 5, true);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.714);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 71, 0, 78, 5.0, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 50, 71, 0, 78, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 65, 0, 80, 4.5, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 50, 65, 0, 80, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairnhold", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairnhold(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 3.0, 54, 85, 0, 60, 4.8, 3.0, 0.0, 1.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.0, 54, 85, 0, 60, 3.3, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 14.0, -6.0, Some(0.0), Some(14.0), Some(-66.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x1cd9e295e5"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(73.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x1cd9e295e5"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		ATTACK(fighter, 4, 1, Hash40::new("top"), 0.5, 330, 0, 0, 0, 2.5, 0.0, 3.2, 2.0, Some(0.0), Some(-26.799999), Some(54.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("0x1cd9e295e5"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 2, 12, 4);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 3, 15, 4);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 4, 3, 4);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2.5);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
		AttackModule::clear(module_accessor, 4, false);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 2, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 14.0, 6.0, Some(0.0), Some(14.0), Some(66.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x1cd9e295e5u64), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, -13.0, Some(0.0), Some(11.0), Some(-73.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x1cd9e295e5u64), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		ATTACK(fighter, 4, 1, Hash40::new("top"), 0.5, 330, 0, 0, 0, 2.5, 0.0, 3.2, -2.0, Some(0.0), Some(-26.799999), Some(-54.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x1cd9e295e5u64), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 2, 12, 4);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 3, 15, 4);
		notify_event_msc_cmd!(fighter, 0x36db1a34c9u64, 4, 3, 4);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 2.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 2.5);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 2, false);
		AttackModule::clear(module_accessor, 3, false);
		AttackModule::clear(module_accessor, 4, false);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let FAIR_CHECK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
	let FAIR_CHECK2 = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		*FAIR_CHECK = false;
		*FAIR_CHECK2 = false;
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10, 3, 10, 0, true);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 50, 10, 0, 65, 2.8, 0.0, 9.2, 3.2, Some(0.0), Some(9.2), Some(7.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 28, 10, 0, 55, 2.8, 0.0, 14.8, 3.2, Some(0.0), Some(14.8), Some(7.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 70, 10, 0, 55, 5.8, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(11.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 45, 10, 0, 45, 5.8, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(11.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 8.0, false);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			*FAIR_CHECK = true;
		}
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf2", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairf2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let FAIR_CHECK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
	let FAIR_CHECK2 = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10, 3, 10, 0, true);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.3, 56, 10, 0, 55, 3.7, 0.0, 8.5, 6.0, Some(0.0), Some(8.5), Some(11.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.3, 44, 10, 0, 50, 3.7, 0.0, 15.5, 6.0, Some(0.0), Some(15.5), Some(11.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.3, 82, 10, 0, 50, 7.4, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(15.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 3.3, 60, 10, 0, 40, 7.4, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(15.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 8.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 8.0, false);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			if *FAIR_CHECK {
				*FAIR_CHECK2 = true;
			}
			if *FAIR_CHECK == false {
				*FAIR_CHECK2 = false;
			}
		}
		if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			*FAIR_CHECK2 = false;
		}
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf3", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairf3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let FAIR_CHECK2 = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 10, 3, 10, 5, true);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 10);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 12.0);
	if *FAIR_CHECK2 == true && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 54, 68, 0, 60, 4.2, -5.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 54, 68, 0, 60, 6.2, 1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("kneer"), 10.0, 54, 68, 0, 60, 6.2, 5.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		}
	}
	if *FAIR_CHECK2 == false || ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 54, 68, 0, 60, 4.2, -5.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 54, 68, 0, 60, 6.2, 1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.0, 54, 68, 0, 60, 6.2, 5.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let BAIR_CHECK = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			*BAIR_CHECK = true;
		}
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10, 3, 10, 5, true);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	if MotionModule::frame(module_accessor) >= 1.0 && MotionModule::frame(module_accessor) <= 20.0 {
		if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			*BAIR_CHECK = false;
		}
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 106, 0, 12, 6.2, 0.0, 13.0, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 106, 0, 12, 4.8, 0.0, 13.0, -14.0, Some(0.0), Some(11.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			*BAIR_CHECK = false;
		}
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		if *BAIR_CHECK {
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 361, 283, 0, 12, 6.2, 0.0, 13.0, -14.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 1, Hash40::new("top"), 2.0, 361, 283, 0, 12, 4.8, 0.0, 13.0, -14.0, Some(0.0), Some(11.0), Some(-4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
				MotionModule::set_rate(module_accessor, 0.9);
			}
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				AttackModule::clear_all(module_accessor);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
				WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
			}
		}
		if *BAIR_CHECK == false {
			AttackModule::clear_all(module_accessor);
			WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
			WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
			WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 39.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20, 3, 15, 0, false);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
		notify_event_msc_cmd!(fighter, 0x2b7cb92b79u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.428);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.333);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 5.5, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 4.2, 2.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.5, 55, 80, 0, 40, 3.0, -1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 3.0, false);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CHECK_HOLD);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2bfb02b69au64, false);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_landingairlw", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_landingairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let DAIR_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	
	frame(lua_state, 1.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK) == false {
		if is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 3, 0, true);
			if *DAIR_CHECK {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 127, 0, 80, 8.5, 0.0, 8.0, -5.0, Some(0.0), Some(8.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 127, 0, 80, 3.5, 0.0, 2.5, 12.0, Some(0.0), Some(2.5), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			}
			if *DAIR_CHECK == false {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 127, 0, 80, 8.5, 0.0, 8.0, -5.0, Some(0.0), Some(8.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 45, 127, 0, 80, 3.5, 0.0, 2.5, 12.0, Some(0.0), Some(2.5), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			}
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 1, false);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_specials", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_specials(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 45, 8, 0, 100, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.875);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 47, 8, 0, 95, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.875);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.5, 50, 8, 0, 83, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.875);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 68, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.14);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HOLD_END_CHECK);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
		MotionModule::set_rate(module_accessor, 1.19);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_specialsholdend", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_specialsholdend(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
		notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 80, 8, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.333);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 98, 65, 0, 64, 5.3, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 98, 65, 0, 64, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("waist"), 5.0, 90, 65, 0, 64, 4.5, 0.0, -0.8, -1.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 0);
	}
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-15.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FINAL_WAIT[get_player_number(module_accessor)] = false;
		FINAL_HIT[get_player_number(module_accessor)] = false;
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_FINAL_FLAG_START_CLIMAX_GAUGE);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_FINAL_FLAG_SYNC);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FINAL_WAIT[get_player_number(module_accessor)] = false;
		FINAL_HIT[get_player_number(module_accessor)] = false;
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_FINAL_FLAG_START_CLIMAX_GAUGE);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_FINAL_FLAG_SYNC);
	}
	
}

#[acmd_script( agent = "bayonetta", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
}

#[acmd_script( agent = "bayonetta", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn bayonetta_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 3, 0);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
}

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
pub fn bayonetta_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();
		let DAIR_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_BAYONETTA {
			if let L2CValueType::Void = globals["bahs"].val_type {
				globals["bahs"] = false.into();
			}
			if let L2CValueType::Void = globals["bahs_check"].val_type {
				globals["bahs_check"] = false.into();
			}
			if let L2CValueType::Void = globals["wt_check"].val_type {
				globals["wt_check"] = false.into();
			}
			START_FS[get_player_number(module_accessor)] = false;
	
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
					globals["bahs"] = false.into();
				}
				if MotionModule::frame(module_accessor) == 1.0 {
					globals["bahs_check"] = false.into();
				}
				if MotionModule::frame(module_accessor) >= 32.0 {
					if globals["bahs"].get_bool() {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END, true);
					}
				}
				if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
					globals["bahs_check"] = true.into();
				}
				if globals["bahs_check"].get_bool() {
					WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
					WorkModule::off_flag(module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_FORBID);
					WorkModule::on_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_BEFORE_GUARD);
				}
			}
			else {
				globals["bahs"] = true.into();
			}
			if motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi") {
				if MotionModule::frame(module_accessor) == 0.0 {
					globals["wt_check"] = false.into();
				}
				if MotionModule::frame(module_accessor) >= 7.0 || MotionModule::frame(module_accessor) <= 8.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
						let wt_height = Vector3f{ x: 0.0, y: 0.2, z: 0.0 };
						KineticModule::add_speed(module_accessor, &wt_height);
						globals["wt_check"] = true.into();
					}
				}
				if globals["wt_check"].get_bool() {
					if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
						let wt_height_2 = Vector3f{ x: 0.0, y: -0.05, z: 0.0 };
						KineticModule::add_speed(module_accessor, &wt_height_2);
						globals["wt_check"] = false.into();
					}
				}
			}
			if motion_kind == hash40("attack_air_lw") {
				if MotionModule::frame(module_accessor) == 0.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
						*DAIR_CHECK = true;
					}
				}
				if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
					*DAIR_CHECK = false;
				}
			}

			if status_kind == *FIGHTER_STATUS_KIND_APPEAL && MotionModule::frame(module_accessor) < 4.0 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			}

			if FINAL_WAIT[get_player_number(module_accessor)] {
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					FINAL_HIT[get_player_number(module_accessor)] = true;
					FINAL_WAIT[get_player_number(module_accessor)] = false;
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FINAL, true);
				}
				else if CancelModule::is_enable_cancel(module_accessor) {
					if motion_kind == hash40("attack_11") || motion_kind == hash40("attack_12") || motion_kind == hash40("attack_13")
					|| motion_kind == hash40("attack_100_end") || motion_kind == hash40("attack_dash") || motion_kind == hash40("attack_s3_s")
					|| motion_kind == hash40("attack_s3_s2") || motion_kind == hash40("attack_s3_s3") || motion_kind == hash40("attack_hi3")
					|| motion_kind == hash40("attack_lw3") || motion_kind == hash40("attack_s4_s") || motion_kind == hash40("attack_s4_hi")
					|| motion_kind == hash40("attack_s4_lw") || motion_kind == hash40("attack_hi4") || motion_kind == hash40("attack_lw4")
					|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F 
					|| status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || motion_kind == hash40("special_s") || motion_kind == hash40("special_s_hold_end")
					|| motion_kind == hash40("special_air_s_u") || motion_kind == hash40("special_air_s_d") || motion_kind == hash40("special_air_s_d_landing")
					|| motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi") {
						FINAL_WAIT[get_player_number(module_accessor)] = false;
						notify_event_msc_cmd!(fighter, 0x1e0aba2d68u64, 7);
					}
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		bayonetta_attack11,
		bayonetta_attack13,
		bayonetta_attacks3s,
		bayonetta_attackhi3,
		bayonetta_attackdash,
		bayonetta_attackairnhold,
		bayonetta_attackairf,
		bayonetta_attackairf2,
		bayonetta_attackairf3,
		bayonetta_attackairb,
		bayonetta_attackairhi,
		bayonetta_landingairlw,
		bayonetta_specials,
		bayonetta_specialsholdend,
		bayonetta_catch,
		bayonetta_catchdash,
		bayonetta_catchturn,
		bayonetta_final,
		bayonetta_finalair,
		bayonetta_finalstart,
		bayonetta_finalairstart
	);
	smashline::install_agent_frames!(bayonetta_functions);
}

