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
use crate::custom::COUNTERHIT_SUCCESS;
use crate::custom::COUNTERHIT_CHECK;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::custom::SUCCESSFUL_CLIP;


#[acmd_script( agent = "gaogaen", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "effect_attack11", category = ACMD_EFFECT, low_priority)]
unsafe fn gaogaen_attack11_effect(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "gaogaen", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_attack13(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if COUNTERHIT_SUCCESS[get_player_number(module_accessor)] {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 60, 0, 80, 2.8, 0.0, 8.0, 2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 361, 60, 0, 80, 3.4, 0.0, 8.5, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 20.0, 361, 60, 0, 80, 4.2, 0.0, 9.0, 7.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
			PLAY_SE_NO_3D(fighter, Hash40::new("se_item_homerunbat_l"));
			COUNTERHIT_SUCCESS[get_player_number(module_accessor)] = false;
			SUCCESSFUL_CLIP[get_player_number(module_accessor)] = 46;
			CAM_ZOOM_IN_arg5(fighter, 10.0, 0.0, 1.2, -0.3, 0.8 * PostureModule::lr(module_accessor));
			SlowModule::set_whole(module_accessor, 4, 0);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.9, 361, 60, 0, 80, 2.8, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 8.9, 361, 60, 0, 80, 3.4, 0.0, 8.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 8.9, 361, 60, 0, 80, 4.2, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 361, 85, 0, 33, 4.6, -3.4, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.0, 361, 85, 0, 33, 5.6, -11.2, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 361, 85, 0, 33, 4.6, -3.4, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("hip"), 10.6, 361, 85, 0, 33, 5.6, -11.2, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 30.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 11.0, 361, 73, 0, 66, 4.2, 1.8, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 361, 73, 0, 66, 5.2, 4.8, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_specialn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
		if COUNTERHIT_SUCCESS[get_player_number(module_accessor)] {
			AttackModule::set_is_critical_attack(module_accessor, true);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 27.0, 45, 47, 0, 85, 7.8, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 23.0, 45, 50, 0, 80, 7.4, 0.0, 11.0, -4.0, Some(0.0), Some(11.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 23.0, 45, 50, 0, 80, 6.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		}
		if COUNTERHIT_SUCCESS[get_player_number(module_accessor)] == false {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 45, 47, 0, 85, 5.8, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 5.4, 0.0, 11.0, -4.0, Some(0.0), Some(11.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		}
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		COUNTERHIT_SUCCESS[get_player_number(module_accessor)] = false;
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
		HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 42.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 56.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
	}
	frame(lua_state, 85.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_specialairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 45, 47, 0, 85, 4.6, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.2, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 45, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 42.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_target_category(module_accessor, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
		AttackModule::set_size(module_accessor, 0, 0.1);
		AttackModule::set_size(module_accessor, 1, 0.1);
		AttackModule::set_size(module_accessor, 2, 0.1);
	}
	frame(lua_state, 56.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
	}
	frame(lua_state, 85.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_speciallw", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 51, 100, 40, 40, 10.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.4, 54, 100, 38, 38, 20.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 18.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_speciallwturn", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_speciallwturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 51, 100, 40, 40, 10.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.4, 54, 100, 38, 38, 20.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 18.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 51, 100, 40, 40, 10.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.4, 54, 100, 38, 38, 20.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 18.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_specialairlwturn", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_specialairlwturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 51, 100, 40, 40, 10.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.4, 54, 100, 38, 38, 20.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 18.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.25);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.2, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 9.0, -4.0, Some(0.0), Some(9.0), Some(-15.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_catchattack", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_catchattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 80, 100, 30, 0, 6.5, 0.0, 10.0, 8.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
		AttackModule::set_catch_only_all(module_accessor, true, false);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_throwf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 32, 80, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 4.0, 30, 63, 0, 100, 8.0, -3.0, 7.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 6.0, 30, 62, 0, 100, 8.0, -4.5, 14.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 42.0);
	if is_excute(fighter) {
		ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 8.0, 30, 61, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 48.0);
	if is_excute(fighter) {
		ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 10.0, 30, 60, 0, 100, 8.0, -6.0, 22.0, 0.0, Some(1.0), Some(6.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		CHECK_FINISH_CAMERA(fighter, 16, 15);
	}
	frame(lua_state, 58.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_throwb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 47, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
		AttackModule::set_catch_only_all(module_accessor, true, false);
		CHECK_FINISH_CAMERA(fighter, -4, 5);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
		REVERSE_LR(fighter);
	}
	
}

#[acmd_script( agent = "gaogaen", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_finalstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 30.0, 60.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 55);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, 4, 6, 0);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "gaogaen", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn gaogaen_finalairstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 30.0, 60.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 55);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, 4, 6, 0);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
pub fn gaogaen_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if fighter_kind == FIGHTER_KIND_GAOGAEN {
			if motion_kind != hash40("attack_12") && motion_kind != hash40("attack_13") && motion_kind != hash40("special_n") {
				COUNTERHIT_SUCCESS[get_player_number(module_accessor)] = false;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		gaogaen_attack11,
		gaogaen_attack11_effect,
		gaogaen_attack13,
		gaogaen_attackairf,
		gaogaen_attackairb,
		gaogaen_specialn,
		gaogaen_specialairn,
		gaogaen_speciallw,
		gaogaen_speciallwturn,
		gaogaen_specialairlw,
		gaogaen_specialairlwturn,
		gaogaen_catch,
		gaogaen_catchdash,
		gaogaen_catchturn,
		gaogaen_catchattack,
		gaogaen_throwf,
		gaogaen_throwb,
		gaogaen_finalstart,
		gaogaen_finalairstart
	);
	smashline::install_agent_frames!(gaogaen_functions);
}

