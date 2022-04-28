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
use crate::custom::jump_checker_buffer;
use crate::custom::GIGA_GRAB;
use crate::custom::GIGA_GRABBED;
use crate::custom::GIGA_OFFSET;
use crate::custom::AERIAL_KIND;
use crate::custom::GIGA_GRAB_TARGET;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use std::mem;	


#[acmd_script( agent = "koopag", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 3.125);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 6.0, 361, 18, 0, 30, 5.6, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 361, 18, 0, 30, 5.6, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 6.0, 361, 18, 0, 30, 5.6, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 9.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 9.0, false);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 3.0);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 3.0);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 3.0);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attack12(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.4);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 45, 55, 0, 70, 7.0, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 45, 55, 0, 70, 7.0, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 8.0, 45, 55, 0, 70, 7.0, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 47.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attacks3s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.5);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.5);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 13.0, 361, 85, 0, 50, 8.4, 6.5, -0.200000003, 0.699999988, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 13.0, 361, 85, 0, 50, 7.6, -2.0, -0.800000012, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 2.7);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.4);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 12.0, 80, 90, 0, 65, 8.4, 7.0, -0.200000003, 0.699999988, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.0, 80, 90, 0, 65, 6.75, 8.0, -0.800000012, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 8.0, 100, 60, 0, 65, 6.75, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 24.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	wait(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 4.0);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("footl"), 4.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 12.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail2"), 12.0, 80, 60, 0, 90, 7.6, -4.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail2"), 12.0, 80, 60, 0, 90, 7.5, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail2"), 12.0, 80, 60, 0, 90, 7.3, 7.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 3, 0, Hash40::new("tail2"), 12.0, 80, 60, 0, 90, 7.3, 9.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 3.0, false);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
}

#[acmd_script( agent = "koopag", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority)]
unsafe fn koopag_attacklw3_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_scratch"), Hash40::new("koopag_scratch"), Hash40::new("top"), 5, 10, 22, 1, 27, -175.100006, 5.70000005, true, *EF_FLIP_YZ);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 3.0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("trans"), 15.0, 80, 60, 0, 100, 8.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("trans"), 15.0, 80, 60, 0, 100, 8.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("trans"), 15.0, 80, 60, 0, 100, 8.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 4, 0, Hash40::new("trans"), 15.0, 80, 60, 0, 100, 8.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("trans"), 12.0, 80, 60, 0, 70, 8.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("trans"), 12.0, 80, 60, 0, 70, 8.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("trans"), 12.0, 80, 60, 0, 70, 8.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("trans"), 12.0, 80, 60, 0, 70, 8.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("trans"), 9.0, 74, 80, 0, 40, 10.4, 0.0, 6.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("trans"), 9.0, 74, 80, 0, 40, 10.4, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("trans"), 9.0, 74, 80, 0, 40, 10.4, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("trans"), 9.0, 74, 80, 0, 40, 10.4, 0.0, 6.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attacks4s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		ATTACK(fighter, 0, 0, Hash40::new("head"), 28.0, 45, 85, 0, 60, 4.5, 7.0, 3.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("head"), 28.0, 45, 85, 0, 60, 4.5, 7.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 51.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("head"), 28.0, 45, 85, 0, 60, 9.0, 11.0, -6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("head"), 28.0, 45, 85, 0, 60, 11.8, 11.0, -8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 0, 0, Hash40::new("head"), 28.0, 45, 85, 0, 60, 4.5, -5.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.5);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.5);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.33);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
		HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 18.0, 90, 25, 0, 105, 11.2, -9.0, 5.69999981, -13.5, Some(6.0), Some(-3.70000005), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 18.0, 90, 25, 0, 105, 12.3, -3.0, 1.79999995, -4.5, Some(6.0), Some(-3.70000005), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 70, 75, 0, 100, 5.6, 0.0, -28.0, -14.0, Some(0.0), Some(-28.0), Some(22.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attacklw4", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 15.0);
	for _ in 0..5 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 120, 40, 0, 35, 7.3, 0.0, 10.0, -15.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 120, 40, 0, 35, 7.3, 0.0, 10.0, 15.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 160, 40, 0, 35, 6.1, 0.0, 24.0, -7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 160, 40, 0, 35, 6.1, 0.0, 24.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 367, 40, 0, 35, 8.4, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 100, 60, 0, 110, 7.3, 0.0, 10.0, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 100, 60, 0, 110, 7.3, 0.0, 10.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 100, 60, 0, 110, 6.1, 0.0, 24.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 100, 60, 0, 110, 6.1, 0.0, 24.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 100, 60, 0, 110, 8.4, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.3);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 367, 100, 70, 0, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 100, 0, 50, 15.0, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
unsafe fn koopag_attackairn_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	
}

#[acmd_script( agent = "koopag", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority)]
unsafe fn koopag_attackairn_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		VisibilityModule::set_int64(module_accessor, hash40("body") as i64, hash40("body_shell") as i64);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		VisibilityModule::set_int64(module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.75);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("handl"), 12.0, 361, 80, 0, 30, 8.4, 7.0, -0.200000003, 0.699999988, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 80, 0, 30, 6.7, 8.0, -0.800000012, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.0, 361, 80, 0, 30, 6.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "koopag", script = "expression_landingairf", category = ACMD_EXPRESSION, low_priority)]
unsafe fn koopag_landingairf_expression(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.714);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("waist"), 15.0, 24, 40, 0, 90, 14.0, 3.5, -3.70000005, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("waist"), 8.0, 24, 40, 0, 90, 14.0, 3.5, -3.70000005, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.6);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("head"), 13.0, 85, 70, 0, 50, 7.8, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("head"), 13.0, 85, 70, 0, 50, 7.8, 0.0, -7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 0.8);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn koopag_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 15.0);
	for _ in 0..7 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 40, 0, 35, 9.3, 0.0, 0.0, -10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 40, 0, 35, 9.3, 0.0, 0.0, 10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 40, 0, 35, 8.1, 0.0, -5.0, -6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 40, 0, 35, 8.1, 0.0, -5.0, 6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 1.5, 367, 40, 0, 35, 10.4, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 5, 0, Hash40::new("top"), 1.5, 80, 40, 0, 35, 9.3, 0.0, 0.0, -10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 6, 0, Hash40::new("top"), 1.5, 80, 40, 0, 35, 9.3, 0.0, 0.0, 10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 7, 0, Hash40::new("top"), 1.5, 80, 40, 0, 35, 8.1, 0.0, -5.0, -6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 8, 0, Hash40::new("top"), 1.5, 80, 40, 0, 35, 8.1, 0.0, -5.0, 6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 9, 0, Hash40::new("top"), 1.5, 80, 40, 0, 35, 10.4, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 9.3, 0.0, 0.0, -10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 9.3, 0.0, 0.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 8.1, 0.0, -5.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 8.1, 0.0, -5.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 10.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn koopag_attackairlw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 15.0);
	for _ in 0..7 {
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
		}
		wait(lua_state, 3.0);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
	}
	
}

#[acmd_script( agent = "koopag", script = "expression_attackairlw", category = ACMD_EXPRESSION, low_priority)]
unsafe fn koopag_attackairlw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(module_accessor, Hash40::new("shellmodel"), true);
	}
	frame(lua_state, 53.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(module_accessor, Hash40::new("shellmodel"), false);
	}

}

#[acmd_script( agent = "koopag", script = "game_landingairlw", category = ACMD_GAME, low_priority)]
unsafe fn koopag_landingairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 4.0, 361, 100, 0, 50, 15.6, 0.0, 4.0, 6.0, Some(0.0), Some(4.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "expression_landingairlw", category = ACMD_EXPRESSION, low_priority)]
unsafe fn koopag_landingairlw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(module_accessor, Hash40::new("shellmodel"), false);
	}

}

#[acmd_script( agent = "koopag", script = "game_specialnstart", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialnstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 43.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 120.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	frame(lua_state, 133.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
	
}


#[acmd_script( agent = "koopag", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialn(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_specialnend", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialnend(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_specialairnstart", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairnstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairn(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_specialairnend", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairnend(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_specialscatch", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialscatch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.95);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 16.0, 361, 70, 0, 70, 11.0, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 16.0, 361, 70, 0, 70, 11.0, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 16.0, 361, 70, 0, 70, 11.0, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 2.65);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_specialsaircatch", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairscatch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.95);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		let giga_speed = Vector3f{ x: 1.8, y: 0.0, z: 0.0 };
		KineticModule::add_speed(module_accessor, &giga_speed);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 16.0, 60, 55, 0, 70, 11.0, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 16.0, 60, 55, 0, 70, 11.0, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 16.0, 60, 55, 0, 70, 11.0, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 2.5);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.75);
		KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		KineticModule::clear_speed_all(module_accessor);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		KineticModule::enable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 10.8, 0.0, 10.0, -4.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 10.8, 0.0, 10.0, 8.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
		HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
		KineticModule::set_consider_ground_friction(module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG2);
	}
	wait(lua_state, 33.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
		AttackModule::clear(module_accessor, 1, false);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 11.8, 0.0, 10.0, -4.0, None, None, None, 0.699999988, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 11.8, 0.0, 10.0, 8.0, None, None, None, 0.699999988, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 51.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		WorkModule::off_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		KineticModule::set_consider_ground_friction(module_accessor, true, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 60, 0, 80, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 60, 0, 80, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 7.0);
	for _ in 0..5 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 84, 20, 0, 50, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 20, 0, 50, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		wait(lua_state, 1.0);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 6.0, 74, 120, 0, 50, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		WorkModule::off_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		WorkModule::off_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
		WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_speciallw", category = ACMD_GAME, low_priority)]
unsafe fn koopag_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if GIGA_GRAB[get_player_number(module_accessor)] != 8 {
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 0.25);
		}
		frame(lua_state, 23.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.0);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 80, 100, 100, 0, 10.1, 0.0, 10.0, 1.0, Some(0.0), Some(10.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		}
		wait(lua_state, 5.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
		}
		frame(lua_state, 53.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 40.0, 76, 100, 0, 60, 25.0, 0.0, 20.0, 0.0, None, None, None, 10.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		}
	}
	else {
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 4.5);
		}
		frame(lua_state, 28.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
		}
		frame(lua_state, 53.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
		}
	}
	
}

#[acmd_script( agent = "koopag", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn koopag_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if GIGA_GRAB[get_player_number(module_accessor)] != 8 {
		frame(lua_state, 32.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 76, 60, 0, 110, 20.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		}
	}
	else {
		frame(lua_state, 32.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 76, 60, 0, 110, 20.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		}
	}
	
}

#[acmd_script( agent = "koopag", script = "game_speciallwlanding", category = ACMD_GAME, low_priority)]
unsafe fn koopag_speciallwland(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if GIGA_GRAB[get_player_number(module_accessor)] != 8 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 76, 40, 0, 120, 26.0, 0.0, 5.0, -5.5, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.0);
		}
	}
	else {
		MotionModule::set_rate(module_accessor, 4.5);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_turn", category = ACMD_GAME, low_priority)]
unsafe fn koopag_turn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "koopag", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn koopag_turndash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn koopag_dash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_cliffcatch", category = ACMD_GAME, low_priority)]
unsafe fn koopag_cliffcatch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "koopag", script = "game_cliffclimb", category = ACMD_GAME, low_priority)]
unsafe fn koopag_cliffclimb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "koopag", script = "game_cliffjump1", category = ACMD_GAME, low_priority)]
unsafe fn koopag_cliffjump1(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	
}

#[acmd_script( agent = "koopag", script = "game_cliffattack", category = ACMD_GAME, low_priority)]
unsafe fn koopag_cliffattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 10.0, 361, 100, 90, 0, 12.3, 2.5, -3.70000005, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("hip"), 10.0, 361, 100, 90, 0, 11.2, -5.4000001, -1.79999995, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_downattackd", category = ACMD_GAME, low_priority)]
unsafe fn koopag_downattackd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 12.0, 0.0, 5.5, 22.0, Some(0.0), Some(5.5), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 12.0, 0.0, 5.5, 20.5, Some(0.0), Some(5.5), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_downattacku", category = ACMD_GAME, low_priority)]
unsafe fn koopag_downattacku(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 12.0, 0.0, 5.5, -22.0, Some(0.0), Some(5.5), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 12.0, 0.0, 5.5, 21.5, Some(0.0), Some(5.5), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_jumpsquat", category = ACMD_GAME, low_priority)]
unsafe fn koopag_jumpsquat(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "koopag", script = "game_turnrun", category = ACMD_GAME, low_priority)]
unsafe fn koopag_turnrun(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP);
	}
}

#[acmd_script( agent = "koopag_breath", script = "game_move", category = ACMD_GAME, low_priority)]
unsafe fn koopag_breath_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 55, 60, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		AttackModule::enable_safe_pos(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn koopag_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
//		CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(17.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA );
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 12.0, 0.0, 9.0, 18.8, Some(0.0), Some(9.0), Some(23.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		GrabModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_catchattack", category = ACMD_GAME, low_priority)]
unsafe fn koopag_catchattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("head"), 2.2, 361, 0, 0, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn koopag_throwf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("head"), 7.0, 361, 80, 0, 80, 10.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn koopag_throwb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 7.0, 361, 80, 0, 80, 10.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn koopag_throwhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.4);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 90, 80, 0, 80, 10.0, 0.0, 9.0, 8.8, Some(0.0), Some(9.0), Some(28.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "koopag", script = "game_throwlw", category = ACMD_GAME, low_priority)]
unsafe fn koopag_throwlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 270, 80, 0, 80, 10.0, 0.0, 9.0, 8.8, Some(0.0), Some(9.0), Some(28.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

unsafe fn dtilt_input(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
	if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
	&& ControlModule::get_stick_y(module_accessor) <= -0.25 
	&& (
		ControlModule::get_stick_y(module_accessor) > -0.72 || ControlModule::get_flick_y(module_accessor) > 4
	) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
		return true;
	}
	else {
		return false;
	}
}

#[status_script(agent = "koopag", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)] //Properly allow Giga Bowser to use Nair and Dair
unsafe fn koopag_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	smash::lua2cpp::L2CFighterCommon_status_AttackAir(fighter)
}

#[fighter_frame( agent = FIGHTER_KIND_KOOPAG )]
pub fn koopag_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
		let cat2 = fighter.global_table[CMD_CAT2].get_int() as i32;
		let pad_flag = fighter.global_table[PAD_FLAG].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();
		let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_KOOPAG {
			if let L2CValueType::Void = globals["giga_globals_set"].val_type {
				globals["giga_buffer_timer"] = 0.0.into();
				*GIGA_DTILT = false;
				*GIGA_DASH_ATTACK = false;
				globals["giga_dash"] = false.into();
				globals["giga_neutral_b"] = false.into();
				globals["giga_side_b"] = false.into();
				globals["giga_lr"] = 0.0.into();
				globals["giga_guard_grab"] = false.into();
				globals["giga_hitlag"] = false.into();
				globals["giga_gravity"] = 0.0.into();
				globals["giga_situation"] = 0.into();
				globals["giga_globals_set"] = true.into();
			}
			if sv_information::is_ready_go() == false {
				GIGA_GRABBED[get_player_number(module_accessor)] = 0;
				GIGA_GRAB[get_player_number(module_accessor)] = 0;
			}
			if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
				if dtilt_input(module_accessor) {
					*DTILT_INPUT = true;
					globals["giga_buffer_timer"] = 6.0.into();
				}
				else {
					*DTILT_INPUT = false;
				}
			}
			if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
				if globals["giga_buffer_timer"].get_num() > 0.0 {
					globals["giga_buffer_timer"] = (globals["giga_buffer_timer"].get_num() - 1.0).into();
				}
				else {
					*DTILT_INPUT = false;
				}
			}

			if situation_kind == SITUATION_KIND_GROUND {
				if *DTILT_INPUT && WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) 
				&& motion_kind != hash40("attack_lw3") 
				&& status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 
				&& motion_kind != hash40("rebound") {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					ControlModule::clear_command(module_accessor, true);
					*GIGA_DTILT = true;
				}
			}
			if status_kind == *FIGHTER_STATUS_KIND_RUN {
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
					StatusModule::change_status_force(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					*GIGA_DASH_ATTACK = true;
				}
			}
	
			if motion_kind == hash40("attack_dash") {
				PostureModule::update_rot_y_lr(module_accessor);
				if MotionModule::frame(module_accessor) >= 58.0 {
					if *DTILT_INPUT {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("attack_lw3")}, 0.0, 1.0, false, 0.0, false, false);
						*DTILT_INPUT = false;	
					}
					else {
						if ControlModule::get_stick_y(module_accessor) < -0.66 {
							MotionModule::change_motion(module_accessor, Hash40{hash: hash40("squat")}, 0.0, 1.0, false, 0.0, false, false);
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
						}
						else {
							MotionModule::change_motion(module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						}
					}
				}
			}
			if motion_kind == hash40("attack_lw3") {
				PostureModule::update_rot_y_lr(module_accessor);
				if MotionModule::frame(module_accessor) >= 36.0 {
					if *DTILT_INPUT {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("attack_lw3")}, 0.0, 1.0, false, 0.0, false, false);
						*DTILT_INPUT = false;
					}
					else {
						if ControlModule::get_stick_y(module_accessor) < -0.66 {
							MotionModule::change_motion(module_accessor, Hash40{hash: hash40("squat")}, 0.0, 1.0, false, 0.0, false, false);
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
						}
						else {
							MotionModule::change_motion(module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						}
						CancelModule::enable_cancel(module_accessor);
					}
				}
			}
	
			//Allows Air Neutral and Side B to cancel on landing (and prevents the animations from restarting on landing)
	
			if motion_kind == hash40("special_s_catch") || motion_kind == hash40("special_s_air_catch") {
				if motion_kind == hash40("special_s_air_catch") {
					globals["giga_side_b"] = true.into();
				}
				if motion_kind == hash40("special_s_catch") && globals["giga_side_b"].get_bool() {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
				}
			}
			else {
				globals["giga_side_b"] = false.into();
			}

			if motion_kind == hash40("special_air_n_start") || motion_kind == hash40("special_n_start") {
				if motion_kind == hash40("special_air_n_start") {
					globals["giga_neutral_b"] = true.into();
				}
				if motion_kind == hash40("special_n_start") && globals["giga_neutral_b"].get_bool() {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
				}
			}
			else {
				globals["giga_neutral_b"] = false.into();
			}
	
			//Up Smash Canceling
	
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					if jump_checker_buffer(module_accessor, cat) {
						if MotionModule::frame(module_accessor) <= 26.0 && MotionModule::frame(module_accessor) >= 18.0 {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						}
					}
				}
			}

			//Grab
			if situation_kind == SITUATION_KIND_GROUND {
				let mut pos = *PostureModule::pos(module_accessor);
				let offset = ModelModule::joint_global_offset_from_top(module_accessor, Hash40{hash: hash40("handl")}, &mut pos);		
				GIGA_OFFSET[get_player_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor) + offset.x, y: PostureModule::pos_y(module_accessor) + offset.y + 8.0, z: PostureModule::pos_z(module_accessor) + offset.z};
				
				if ((WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT) 
					|| status_kind == *FIGHTER_STATUS_KIND_WALK || CancelModule::is_enable_cancel(module_accessor)) && (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
					&& ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD)) && (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0) 
				|| globals["giga_guard_grab"].get_bool() {
					globals["giga_lr"] = (PostureModule::lr(module_accessor)).into();
					globals["giga_guard_grab"] = false.into();
					MotionModule::change_motion(module_accessor, Hash40 { hash: hash40("catch") }, 0.0, 1.0, false, 0.0, false, false);
				}
				if motion_kind == hash40("catch") {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if MotionModule::frame(module_accessor) > 38.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					}
					if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
						AttackModule::clear_all(module_accessor);
						GIGA_GRAB[get_player_number(module_accessor)] = 2;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					}
				}
				if GIGA_GRAB[get_player_number(module_accessor)] == 2 {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_pull")}, 0.0, 1.0, false, 0.0, false, false);
					GIGA_GRAB[get_player_number(module_accessor)] = 1;
				}
				if motion_kind == hash40("catch_pull") {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if MotionModule::frame(module_accessor) >= 19.0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_wait")}, 0.0, 1.0, true, 0.0, false, false);
					}
				}
				if GIGA_GRAB[get_player_number(module_accessor)] == 9 {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					GIGA_GRAB[get_player_number(module_accessor)] = 0;					
				}
				if motion_kind == hash40("catch_wait") || (motion_kind == hash40("catch_pull") && MotionModule::frame(module_accessor) >= 4.0) {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if GIGA_GRABBED[get_player_number(module_accessor)] == 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					}
					if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_attack")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 3;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_f")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 4;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_b")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 5;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_hi")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 6;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_lw")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 7;
					}
				}
				if motion_kind == hash40("catch_attack") {
					if GIGA_GRABBED[get_player_number(module_accessor)] == 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					}
					if MotionModule::frame(module_accessor) >= 13.0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_wait")}, 0.0, 1.0, true, 0.0, false, false);
					}
				}

				if motion_kind == hash40("throw_f") {
					if MotionModule::frame(module_accessor) >= 58.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}

				if motion_kind == hash40("throw_b") {
					if MotionModule::frame(module_accessor) >= 46.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}

				if motion_kind == hash40("throw_hi") {
					if MotionModule::frame(module_accessor) >= 50.0 {
						CancelModule::enable_cancel(module_accessor);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}
				if motion_kind == hash40("throw_lw") {
					if MotionModule::frame(module_accessor) >= 9.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
						GIGA_GRAB[get_player_number(module_accessor)] = 8;
					}
				}
			}
			if motion_kind != hash40("throw_lw") && motion_kind != hash40("special_lw") && motion_kind != hash40("special_air_lw") && motion_kind != hash40("special_lw_land") && GIGA_GRAB[get_player_number(module_accessor)] == 8 {
				GIGA_GRAB[get_player_number(module_accessor)] = 0;
				GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
			}
				
			/*
			Shield code. Actually shielding can be enabled by turning off the disable_guard flag, but shieldgrabs need to be added manually, Giga Bowser's shield
			doesn't break when it hits 0 by default, and since multiple animations are shared, the guard_on anim needs to be interrupted with the standard guard status			
			*/

			if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON && MotionModule::frame(module_accessor) >= 9.0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD, true);
			}

			if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) || (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
				&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) < 2 
				&& (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK, true);
					globals["giga_guard_grab"] = true.into();
				}
			}

			if status_kind == *FIGHTER_STATUS_KIND_GUARD {
				MotionModule::set_frame(module_accessor, 0.0, true);
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
					if WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) <= WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, true);
					}
				}
			}

			//Enabling certain status changes

			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);		
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		koopag_attack11,
		koopag_attack12,
		koopag_attacks3s,
		koopag_attackhi3,
		koopag_attacklw3,
		koopag_attacklw3_effect,
		koopag_attackdash,
		koopag_attacks4s,
		koopag_attackhi4,
		koopag_attacklw4,
		koopag_attackairn,
		koopag_attackairn_effect,
		koopag_attackairn_expression,
		koopag_attackairf,
		koopag_landingairf_expression,
		koopag_attackairb,
		koopag_attackairhi,
		koopag_attackairlw,
		koopag_attackairlw_effect,
		koopag_attackairlw_expression,
		koopag_landingairlw,
		koopag_landingairlw_expression,
		koopag_specialnstart,
		koopag_specialn,
		koopag_specialnend,
		koopag_specialairnstart,
		koopag_specialairn,
		koopag_specialairnend,
		koopag_specialscatch,
		koopag_specialairscatch,
		koopag_specialhi,
		koopag_specialairhi,
		koopag_speciallw,
		koopag_specialairlw,
		koopag_speciallwland,
		koopag_turn,
		koopag_turndash,
		koopag_dash,
		koopag_cliffcatch,
		koopag_cliffclimb,
		koopag_cliffjump1,
		koopag_cliffattack,
		koopag_downattackd,
		koopag_downattacku,
		koopag_jumpsquat,
		koopag_turnrun,
		koopag_breath_move,
		koopag_catch,
		koopag_catchattack,
		koopag_throwf,
		koopag_throwb,
		koopag_throwhi,
		koopag_throwlw,
	);
	smashline::install_agent_frames!(koopag_functions);
	smashline::install_status_scripts!(koopag_attackair_main);
}

