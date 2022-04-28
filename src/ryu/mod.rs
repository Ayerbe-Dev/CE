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
use crate::custom::SUCCESSFUL_CLIP;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use crate::custom::FIGHTER_BOOL_4;
use crate::custom::FIGHTER_BOOL_5;
use crate::custom::FIGHTER_BOOL_6;
use crate::custom::FIGHTER_BOOL_7;
use crate::custom::FIGHTER_BOOL_8;
use crate::custom::FIGHTER_FLOAT_1;
use crate::custom::FIGHTER_FLOAT_2;
use crate::custom::FIGHTER_FLOAT_3;
use crate::custom::FIGHTER_INT_1;
use crate::custom::FIGHTER_VEC3F_1;
use crate::custom::estimate_frame;
use crate::custom::is_majin;
use crate::custom::MAJIN_GRAB;
use crate::custom::MAJIN_OFFSET;
use crate::custom::SUB_METER;
use crate::custom::READY_GO;
use crate::custom::MAJIN_DEMON_TARGET;
use crate::custom::SUB_STICK;
use crate::req_shinsyoryu_hit_effect;


#[acmd_script( agent = "ryu", script = "game_attack13w", category = ACMD_GAME)]
unsafe fn ryu_attack13w(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 70, 3.0, 0.0, 11.0, 11.5, Some(0.0), Some(11.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 70, 1.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
	}
	
}

#[acmd_script( agent = "ryu", script = "game_attack11s", category = ACMD_GAME)]
unsafe fn ryu_attack11s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 90, 120, 30, 0, 4.0, 0.0, 11.0, 3.4, Some(0.0), Some(20.5), Some(3.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 1, Hash40::new("kneer"), 6.0, 30, 20, 0, 35, 3.0, 6.3, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, true);
		}
		frame(lua_state, 119.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 9.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 110, 0, 40, 4.5, 0.0, 17.0, 11.9, Some(0.0), Some(14.0), Some(5.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_attack11s", category = ACMD_EFFECT)]
unsafe fn ryu_attack11s_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), 0, 12, 3, -90, 0, 0, 0.850000024, 0, 0, 0, 0, 0, 0, true, 0.400000006);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0ece5db9cbu64), false, true);
			//FOOT_EFFECT(fighter, None, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f3b34c494u64), Hash40::new_raw(0x0f3b34c494u64), Hash40::new("top"), 0, 13.5, 5.5, 0, -14, -90, 0.800000012, true, 0, 0.5);
			LAST_EFFECT_SET_RATE(fighter, 1.5);
		}
		frame(lua_state, 118.0);
		if is_excute(fighter) {
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0e558e2c34u64), Hash40::new_raw(0x0e558e2c34u64), Hash40::new("top"), -2, 12, 3, 30, -20, -170, 1.10000002, true, 0, 0.349999994);
			LAST_EFFECT_SET_RATE(fighter, 1.35000002);
		}
		frame(lua_state, 9.0);
		if is_excute(fighter) {
			//FOOT_EFFECT(fighter, None, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attack11s", category = ACMD_SOUND)]
unsafe fn ryu_attack11s_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
	else {
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attack11s", category = ACMD_EXPRESSION)]
unsafe fn ryu_attack11s_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 9.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
		}		
	}
}

#[acmd_script( agent = "ryu", script = "game_attack11nears", category = ACMD_GAME)]
unsafe fn ryu_attack11nears(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 90, 120, 30, 0, 4.0, 0.0, 11.0, 3.4, Some(0.0), Some(20.5), Some(3.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 1, Hash40::new("kneer"), 6.0, 30, 20, 0, 35, 3.0, 6.3, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 2.0, true);
		}
		frame(lua_state, 119.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
	}
	else {
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
			ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 200, 100, 20, 0, 3.0, 1.7, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("bust"), 12.0, 200, 100, 20, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 200, 100, 20, 0, 5.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 200, 100, 20, 0, 3.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(10.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	
}

#[acmd_script( agent = "ryu", script = "effect_attack11nears", category = ACMD_EFFECT)]
unsafe fn ryu_attack11nears_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), 0, 12, 3, -90, 0, 0, 0.850000024, 0, 0, 0, 0, 0, 0, true, 0.400000006);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0ece5db9cbu64), false, true);
			//FOOT_EFFECT(fighter, None, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f3b34c494u64), Hash40::new_raw(0x0f3b34c494u64), Hash40::new("top"), 0, 13.5, 5.5, 0, -14, -90, 0.800000012, true, 0, 0.5);
			LAST_EFFECT_SET_RATE(fighter, 1.5);
		}
		frame(lua_state, 118.0);
		if is_excute(fighter) {
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			//FOOT_EFFECT(fighter, None, Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0e558e2c34u64), Hash40::new_raw(0x0e558e2c34u64), Hash40::new("top"), 1, 14.5, 3, -5, -13, 109, 0.699999988, true, 0, 0.349999994);
			LAST_EFFECT_SET_RATE(fighter, 1.60000002);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 11.5, 15, 0, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 360, false, 0.699999988);
		}
	}	
}

#[acmd_script( agent = "ryu", script = "sound_attack11nears", category = ACMD_SOUND)]
unsafe fn ryu_attack11nears_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
	else {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x142e922876u64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attack11nears", category = ACMD_EXPRESSION)]
unsafe fn ryu_attack11nears_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "game_attacks3s", category = ACMD_GAME)]
unsafe fn ryu_attacks3ss(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 118.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 3.0, 270, 100, 15, 0, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("bust"), 3.0, 270, 100, 15, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 270, 100, 15, 0, 4.2, 0.0, 11.0, 10.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 120.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 6.0, 46, 90, 0, 60, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("bust"), 6.0, 46, 90, 0, 60, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 46, 90, 0, 60, 4.2, 0.0, 7.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			FT_MOTION_RATE(fighter, 1.1);
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 1.0);
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 1.0);
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 3.0, 270, 100, 15, 0, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("bust"), 3.0, 270, 100, 15, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 270, 100, 15, 0, 4.2, 0.0, 11.0, 10.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 18.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 6.0, 46, 90, 0, 60, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("bust"), 6.0, 46, 90, 0, 60, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 46, 90, 0, 60, 4.2, 0.0, 7.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_attacks3s", category = ACMD_EFFECT)]
unsafe fn ryu_attacks3ss_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 117.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0e558e2c34u64), Hash40::new_raw(0x0e558e2c34u64), Hash40::new("trans"), 3, 9, -2, 7, -60, -64, 1.0, true, 0, 0.349999994);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		frame(lua_state, 120.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 8, 7.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, false, 0.400000006);
			FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 2, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0e558e2c34u64), Hash40::new_raw(0x0e558e2c34u64), Hash40::new("trans"), 3, 9, -2, 7, -60, -64, 0.899999976, true, 0, 0.349999994);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		frame(lua_state, 17.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 8, 7.5, 0, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 360, false, 0.400000006);
			FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 2, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 0, false);
		}
	}	
}

#[acmd_script( agent = "ryu", script = "sound_attacks3s", category = ACMD_SOUND)]
unsafe fn ryu_attacks3ss_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x0f7774411fu64));
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x15cba5f59eu64));
		}
	}
	else {
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x0f7774411fu64));
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x15cba5f59eu64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacks3s", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacks3ss_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 116.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 9);
		}
		frame(lua_state, 118.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 9);
		}
		frame(lua_state, 18.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_attacks3w", category = ACMD_GAME)]
unsafe fn ryu_attacks3sw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		MotionModule::set_rate(module_accessor, 1.25);
	}
	
}

#[acmd_script( agent = "ryu", script = "game_attacknearw", category = ACMD_GAME)]
unsafe fn ryu_attacknearw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 100, 30, 0, 5.0, 0.0, 10.0, 9.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KNEE);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 114.0);
		if is_excute(fighter) {
			CancelModule::enable_cancel(module_accessor);
		}
		frame(lua_state, 119.0);
		if is_excute(fighter) {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 2.0, 2.0);
			MotionModule::set_rate(module_accessor, 1.0);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 180, 100, 10, 0, 3.2, 0.0, 12.5, 9.0, Some(0.0), Some(7.5), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 75, 23, 0, 16, 3.2, 0.0, 12.5, 9.0, Some(0.0), Some(7.5), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	
}

#[acmd_script( agent = "ryu", script = "effect_attacknearw", category = ACMD_EFFECT)]
unsafe fn ryu_attacknearw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), -2, 6, -1, -30, 5, 0, 0.75, 0, 0, 0, 0, 0, 0, true, 0, 0.800000012);
			//FOOT_EFFECT(fighter, None, Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			EFFECT(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 10, 11.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
		}
	}
	else {
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 9, 13, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 360, false, 0.600000024);
		}
	}	
}

#[acmd_script( agent = "ryu", script = "sound_attacknearw", category = ACMD_SOUND)]
unsafe fn ryu_attacknearw_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x14599518e0u64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14144bfe44u64));
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x14599518e0u64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14144bfe44u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacknearw", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacknearw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_attackhi3w", category = ACMD_GAME)]
unsafe fn ryu_attackhi3w(fighter: &mut L2CAgentBase) {
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
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 90, 75, 0, 19, 5.0, 0.0, 16.0, 4.8, Some(2.0), Some(14.5), Some(4.8), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 75, 0, 19, 5.0, 0.0, 16.0, 6.0, Some(2.0), Some(14.5), Some(6.0), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 75, 75, 0, 19, 5.0, 0.0, 16.0, 4.8, Some(2.0), Some(14.5), Some(4.8), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 75, 75, 0, 19, 5.0, 0.0, 16.0, 6.0, Some(2.0), Some(14.5), Some(6.0), 1.5, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
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

#[acmd_script( agent = "ryu", script = "game_attacklw3w", category = ACMD_GAME)]
unsafe fn ryu_attacklw3w(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			MotionModule::set_rate(module_accessor, 1.0);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 9, 0, 2.8, 0.0, 6.5, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 361, 100, 11, 0, 2.5, 0.0, 6.5, 3.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 361, 100, 5, 0, 2.5, 0.0, 6.5, 6.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 1.6, 361, 100, 5, 0, 2.8, 0.0, 6.5, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 1.0);
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
			FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.0, 5.0);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 65, 100, 9, 0, 3.5, 0.0, 3.0, 9.0, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 65, 100, 11, 0, 2.8, 0.0, 3.3, 11.5, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 60, 100, 5, 0, 2.5, 0.0, 1.5, 13.2, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 1.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 1.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 2, 1.0, false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_attacklw3w", category = ACMD_EFFECT)]
unsafe fn ryu_attacklw3w_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 0.649999976, 0, 0, 0, 0, 0, 360, false, 0.300000012);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 1.5, 12, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 360, true, 0.400000006);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "sound_attacklw3w", category = ACMD_SOUND)]
unsafe fn ryu_attacklw3w_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x13bbe84a5eu64));
		}
	}
	else {
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x13bbe84a5eu64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacklw3w", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacklw3w_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
		}
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attacks"), 0);
		}
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attacks"), 0);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "game_attacklw3s", category = ACMD_GAME)]
unsafe fn ryu_attacklw3s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 55, 16, 0, 66, 4.0, 3.5, -3.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 64, 16, 0, 73, 4.0, 6.5, -3.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 121.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 0.82);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 55, 16, 0, 66, 4.0, 0.0, 2.8, 12.0, Some(0.0), Some(3.8), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 55, 16, 0, 66, 3.3, 0.0, 2.2, 15.7, Some(0.0), Some(3.8), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 10.0);
		FT_MOTION_RATE(fighter, 1);
		frame(lua_state, 21.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_attacklw3s", category = ACMD_EFFECT)]
unsafe fn ryu_attacklw3s_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), 0, 2, 5.5, 7.30000019, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.699999988);
			FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, -2, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 1.5, 17.2, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 360, true, 0.400000006);
			//FOOT_EFFECT(fighter, null, top, 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), 0, 2, 5.5, 7.30000019, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.699999988);
			FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, -2, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			EFFECT_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 1.5, 16.5, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 360, true, 0.400000006);
			//FOOT_EFFECT(fighter, null, top, 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attacklw3s", category = ACMD_SOUND)]
unsafe fn ryu_attacklw3s_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 7.0);
		if is_excute(fighter){
			PLAY_SE(fighter, Hash40::new_raw(0x1341e7773du64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14144bfe44u64));
		}
	}
	else {
		frame(lua_state, 108.0);
		if is_excute(fighter){
			PLAY_SE(fighter, Hash40::new_raw(0x1341e7773du64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14144bfe44u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacklw3s", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacklw3s_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 102.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
		frame(lua_state, 115.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn ryu_attacks4s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 98, 0, 26, 3.5, 0.0, 10.5, 0.5, Some(0.0), Some(11.5), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 98, 0, 26, 3.5, 0.0, 5.5, 0.5, Some(0.0), Some(6.5), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 17.5, 361, 98, 0, 26, 3.5, 0.0, 12.5, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	else {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("kneel"), 17.5, 361, 98, 0, 26, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn ryu_attacks4s_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -8, 10.5, 1, -15, 0, 0, 0.7, true, *EF_FLIP_YZ);
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_attack_line"), Hash40::new("ryu_attack_line"), Hash40::new("top"), -8, 10.5, 1, -15, 0, 0, 0.7, true, *EF_FLIP_YZ);
			EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 11, 7, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
			LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0f998ceac2u64), Hash40::new_raw(0x0f998ceac2u64), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
			EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
			LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attacks4", category = ACMD_SOUND)]
unsafe fn ryu_attacks4s_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_smash_s01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_smash_s01"));
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_smash_s01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_smash_s01"));
		}
		wait(lua_state, 38.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new("se_ryu_step_left_m"));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacks4", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacks4s_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -10, 7, 20, 14, 80);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
		}
		frame(lua_state, 128.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -10, 7, 20, 14, 80);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
		}
		frame(lua_state, 28.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_attacks4charge", category = ACMD_EFFECT)]
unsafe fn ryu_attacks4charge_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
		}
		wait(lua_state, 5.0);
	}
	else {
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
		}
		wait(lua_state, 5.0);
	}
}

#[acmd_script( agent = "ryu", script = "sound_attacks4charge", category = ACMD_SOUND)]
unsafe fn ryu_attacks4charge_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_smash_s01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_smash_s01"));
		}
		wait(lua_state, 38.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new("se_ryu_step_left_m"));
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new("se_common_smash_start"));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_smash_s01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_smash_s01"));
		}
		wait(lua_state, 38.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new("se_ryu_step_left_m"));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacks4charge", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacks4charge_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, 0.3, 0.1, -1, hash40("invalid"));
			AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -8, 4, 16, 8, 80);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, 0);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 161.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x119a789590u64), 0, true, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, 0.3, 0.1, -1, hash40("invalid"));
			AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -8, 4, 16, 8, 80);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, 0);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 61.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x119a789590u64), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ryu_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 0.5);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.0);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 9.0);
		if is_excute(fighter) {
			HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
			HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 70, 120, 100, 0, 5.3, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.5, 70, 120, 100, 0, 4.3, -2.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 70, 120, 100, 0, 4.8, 0.0, 21.5, 5.0, Some(0.0), Some(17.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 9.0);
		if is_excute(fighter) {
			HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
			HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 83, 82, 0, 32, 5.3, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.5, 83, 86, 0, 32, 4.3, -2.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 83, 86, 0, 32, 4.8, 0.0, 21.5, 5.0, Some(0.0), Some(17.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ryu_attacklw4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 35, 47, 0, 50, 4.5, 0.0, 4.5, 9.2, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 4.5, 0.0, 4.5, 9.2, Some(0.0), Some(4.5), Some(-1.8), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 4.5, 0.0, 4.5, 9.2, Some(0.0), Some(4.5), Some(-1.8), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 114.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 141.0);
		if is_excute(fighter) {
			CancelModule::enable_cancel(module_accessor);
		}
		frame(lua_state, 142.0);
		if is_excute(fighter) {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 35, 47, 0, 50, 3.6, 0.0, 2.5, 12.0, Some(0.0), Some(3.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 14.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn ryu_attacklw4_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f3b34c494u64), Hash40::new_raw(0x0f3b34c494u64), Hash40::new("top"), 0, 4, -1, 0, -60, 15, 1.10000002, true, 0, 0.400000006);
			LAST_EFFECT_SET_RATE(fighter, 2);
		}
	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f16adf88cu64), Hash40::new_raw(0x0f16adf88cu64), Hash40::new("top"), 1, 1.5, 0, 0, -70, -5, 1, true, 0, 0.5);
			LAST_EFFECT_SET_RATE(fighter, 2);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			LANDING_EFFECT_FLIP(fighter, Hash40::new_raw(0x0fe5950916u64), Hash40::new_raw(0x0f1f9a3475u64), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 0, false, 0);
			LAST_EFFECT_SET_ALPHA(fighter, 0.699999988);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn ryu_attacklw4_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
//			STOP_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
//			PLAY_SE(fighter, Hash40::new_raw(0x1069dcccd2u64));
//			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x15feb83dc8u64));
		}
	}
	else {
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1069dcccd2u64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x15feb83dc8u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacklw4", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacklw4_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 1);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 0.800000012, 110, 8, 0.800000012, 0, 4, 32, 8, 80);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
		}
		frame(lua_state, 111.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 13);
		}
		frame(lua_state, 119.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 1);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 0.800000012, 110, 8, 0.800000012, 0, 4, 32, 8, 80);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 13);
		}
		frame(lua_state, 19.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_attacklw4charge", category = ACMD_EFFECT)]
unsafe fn ryu_attacklw4charge_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 105.0);
		for _ in 0..60 {
			if is_excute(fighter) {
				FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
			}
			wait(lua_state, 5.0);
		}
	}
	else {
		frame(lua_state, 5.0);
		for _ in 0..60 {
			if is_excute(fighter) {
				FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
			}
			wait(lua_state, 5.0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attacklw4charge", category = ACMD_SOUND)]
unsafe fn ryu_attacklw4charge_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
		}
	}
	else {
		frame(lua_state, 102.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
		}		
	}
}

#[acmd_script( agent = "ryu", script = "expression_attacklw4charge", category = ACMD_EXPRESSION)]
unsafe fn ryu_attacklw4charge_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
			physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.5, 0.100000001, -1, 0.899999976, 0.600000024, -1);
			AREA_WIND_2ND_arg10(fighter, 0, 0.800000012, 180, 8, 0.800000012, -8, 4, 16, 8, 80);
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x110371c42au64), 0, true, 0);
		}
		frame(lua_state, 159.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x119a789590u64), 0, true, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
			physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.5, 0.100000001, -1, 0.899999976, 0.600000024, -1);
			AREA_WIND_2ND_arg10(fighter, 0, 0.800000012, 180, 8, 0.800000012, -8, 4, 16, 8, 80);
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x110371c42au64), 0, true, 0);
		}
		frame(lua_state, 61.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x119a789590u64), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn ryu_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 100, 0, 20, 4.3, 0.0, 4.2, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 45, 100, 0, 20, 4.3, 0.0, 8.7, -1.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 45, 90, 0, 13, 3.3, 0.0, 4.2, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 45, 90, 0, 13, 3.3, 0.0, 8.7, -1.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 14.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 22.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 34.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 20, 4.3, 0.0, 4.2, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 100, 0, 20, 4.3, 0.0, 8.7, -1.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 90, 0, 13, 3.0, 0.0, 4.2, 4.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 90, 0, 13, 3.0, 0.0, 8.7, -1.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 14.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 32.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 34.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn ryu_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 102.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.0, 50, 70, 0, 82, 4.3, 6.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.0, 50, 35, 0, 65, 3.8, 6.3, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 50, 70, 0, 82, 4.3, 6.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 50, 35, 0, 65, 3.8, 6.3, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 132.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			FT_MOTION_RATE(fighter, 1.34);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 5.5);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 61, 56, 0, 82, 4.3, 0.0, 5.5, 10.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 70, 25, 0, 65, 3.8, 0.0, 5.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 0.6);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 61, 56, 0, 82, 4.0, 0.0, 5.5, 10.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 25, 0, 65, 4.6, 0.0, 5.5, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			FT_MOTION_RATE(fighter, 1.15);
			AttackModule::clear_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 35.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_attackairf", category = ACMD_EFFECT)]
unsafe fn ryu_attackairf_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f3b34c494u64), Hash40::new_raw(0x0f3b34c494u64), Hash40::new("top"), 0, 4, -1, -45, -10, 15, 1.40000002, true, 0, 0.400000006);
			LAST_EFFECT_SET_RATE(fighter, 1.5);
		}
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 1, 14, 12, 0, 0, 0, 1, false, 0.800000012);
		}
	}
	else {
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 1, 5, 12, 0, 0, 0, 1, false, 0.800000012);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_attackairf", category = ACMD_SOUND)]
unsafe fn ryu_attackairf_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
	else {
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1336e047abu64));
			PLAY_SEQUENCE(fighter, Hash40::new_raw(0x14634cced2u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attackairf", category = ACMD_EXPRESSION)]
unsafe fn ryu_attackairf_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
		}
	}
	else {
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn ryu_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			SET_SPEED_EX(fighter, 2, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("kneel"), 10.0, 65, 60, 0, 65, 4.2, 5.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		}
		frame(lua_state, 134.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
	}
	else {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 300, 80, 0, 10, 2.7, 0.0, 3.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 84, 14, 0, 60, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 48, 80, 0, 20, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 5.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 5.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 33.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_attackairlw", category = ACMD_EFFECT)]
unsafe fn ryu_attackairlw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.8000002, 0, 43, 0, 0, 0.800000012, true, 0, 0.5);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), -0.5, 10.6999998, 2.5, 43, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true, 0, 0.800000012);
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 2.5, 9, 0, 0, 0, 0.800000012, false, 0.5);
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.8000002, 0, 50, 0, 0, 0.800000012, true, 0, 0.5);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), -0.5, 10.6999998, 2.5, 50, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true, 0, 0.800000012);
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 2.5, 9, 0, 0, 0, 0.800000012, false, 0.5);
		}

	}
}

#[acmd_script( agent = "ryu", script = "sound_attackairlw", category = ACMD_SOUND)]
unsafe fn ryu_attackairlw_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x102605217bu64));
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x142e922876u64));
		}
	}
	else {
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x102605217bu64));
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x142e922876u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_attackairlw", category = ACMD_EXPRESSION)]
unsafe fn ryu_attackairlw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
	}
	else {
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 6.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_specialn", category = ACMD_EFFECT)]
unsafe fn ryu_specialn_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
		if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
			frame(lua_state, 3.0);
			if is_excute(fighter) {
				if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
					EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
			}
			frame(lua_state, 4.0);
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
				LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				EffectModule::enable_sync_init_pos_last(module_accessor);
			}
			wait(lua_state, 4.0);
			if is_excute(fighter) {
				FLASH(fighter, 0.39199999, 1, 1, 0.352999985);
			} 
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
				EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10364628c9u64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
				LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);    
			}
			frame(lua_state, 12.0);
			if is_excute(fighter) {
				FOOT_EFFECT(fighter, Hash40::new_raw(0x11f7d0b8cau64), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false);
			}
		}
		else {
			frame(lua_state, 3.0);
			if is_excute(fighter) {
				if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
					EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
			}
			frame(lua_state, 4.0);
			if is_excute(fighter) {
				if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x13941a1597u64), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
				}
				else {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
				}
				EffectModule::enable_sync_init_pos_last(module_accessor);
			}
			wait(lua_state, 4.0);
			if is_excute(fighter) {
				FLASH(fighter, 0.39199999, 1, 1, 0.352999985);
			} 
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1320d7902bu64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
				}
				else {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10364628c9u64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);    
				}
				EffectModule::enable_sync_init_pos_last(module_accessor);
				COL_NORMAL(fighter);
			}
			frame(lua_state, 12.0);
			if is_excute(fighter) {
				FOOT_EFFECT(fighter, Hash40::new_raw(0x11f7d0b8cau64), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false);
			}
		}
	}
	else {
		frame(lua_state, 12.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x0b41410a4du64), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialairn", category = ACMD_GAME)]
unsafe fn ryu_specialairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		wait(lua_state, 1.0);
		FT_MOTION_RATE(fighter, 0.916);
		frame(lua_state, 110.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
		}
		FT_MOTION_RATE(fighter, 1.0);
		frame(lua_state, 114.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 122.0);
		if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 128.0);
		if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
	}
	else {
		wait(lua_state, 1.0);
		FT_MOTION_RATE(fighter, 0.916);
		frame(lua_state, 10.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
		}
		FT_MOTION_RATE(fighter, 1.0);
		frame(lua_state, 14.0);
		if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 22.0);
		if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 28.0);
		if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_specialairn", category = ACMD_EFFECT)]
unsafe fn ryu_specialairn_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
		if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
			frame(lua_state, 103.0);
			if is_excute(fighter) {
				if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
					EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
			}
			frame(lua_state, 104.0);
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10828bad75u64), Hash40::new("top"), 0, 3.5, 10, 0, 0, 0, 1, true);
				LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				EffectModule::enable_sync_init_pos_last(module_accessor);
			}
			wait(lua_state, 4.0);
			if is_excute(fighter) {
				FLASH(fighter, 0.39199999, 1, 1, 0.352999985);
			} 
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
				EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10364628c9u64), Hash40::new("top"), 0, 3.5, 10, 45, 0, 0, 1, true);
				LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
			}
		}
		else {
			frame(lua_state, 3.0);
			if is_excute(fighter) {
				if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
					EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
			}
			frame(lua_state, 4.0);
			if is_excute(fighter) {
				if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x13941a1597u64), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
				}
				else {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10828bad75u64), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
				}
				EffectModule::enable_sync_init_pos_last(module_accessor);
			}
			wait(lua_state, 4.0);
			if is_excute(fighter) {
				FLASH(fighter, 0.39199999, 1, 1, 0.352999985);
			} 
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1320d7902bu64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
				}
				else {
					EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10364628c9u64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);    
				}
				EffectModule::enable_sync_init_pos_last(module_accessor);
				COL_NORMAL(fighter);
			}
		}
	}
	else {
		if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
			frame(lua_state, 112.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x0b41410a4du64), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true);
			}
		}
		else {
			frame(lua_state, 12.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x0b41410a4du64), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true);
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_specialairn", category = ACMD_SOUND)]
unsafe fn ryu_specialairn_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let status_kind = StatusModule::status_kind(module_accessor);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 111.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x122cf85ae1u64));
			}
		}
		else {
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
				frame(lua_state, 101.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x1681dd5a8eu64));
					PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
				}
				wait(lua_state, 10.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
				}
				wait(lua_state, 2.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new("vc_ryu_final04_02"));
				}
			}
			else {
				frame(lua_state, 101.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
				}
				wait(lua_state, 10.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
				}
				wait(lua_state, 2.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new("vc_ryu_final04"));
				}
			}
		}
	}
	else {
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 11.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x122cf85ae1u64));
			}
		}
		else {
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
				frame(lua_state, 1.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x1681dd5a8eu64));
					PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
				}
				wait(lua_state, 10.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
				}
				wait(lua_state, 2.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x1ae15be958u64));
				}
			}
			else if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
				frame(lua_state, 1.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x1681dd5a8eu64));
					PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
				}
				wait(lua_state, 3.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x1ad8d6d59du64));
				}
				wait(lua_state, 6.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
				}
			}
			else {
				frame(lua_state, 1.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
				}
				wait(lua_state, 10.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
				}
				wait(lua_state, 2.0);
				if is_excute(fighter) {
					PLAY_SE(fighter, Hash40::new_raw(0x12c2f63bcdu64));
				}
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialairn", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialairn_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let status_kind = StatusModule::status_kind(module_accessor);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 1.20000005, 110, 8, 0.800000012, -2, 8, 24, 16, 80);
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 111.0);
			if is_excute(fighter) {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
			}
		}
		else {
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
				frame(lua_state, 111.0);
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackl"), 0, false, 0);
				}
			}
			else {
				frame(lua_state, 111.0);
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
				}
			}
		}
		frame(lua_state, 142.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 166.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 1.20000005, 110, 8, 0.800000012, -2, 8, 24, 16, 80);
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 11.0);
			if is_excute(fighter) {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
			}
		}
		else {
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
				frame(lua_state, 11.0);
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackl"), 0, false, 0);
				}
			}
			else if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
				frame(lua_state, 11.0);
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackl"), 0, false, 0);
				}
			}
			else {
				frame(lua_state, 11.0);
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
				}
			}
		}
		frame(lua_state, 142.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 166.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialn2", category = ACMD_GAME)]
unsafe fn ryu_specialn2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let SHAKU_INPUT = &mut FIGHTER_FLOAT_3[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
		frame(lua_state, 124.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
			*SHAKU_INPUT = 0.0;
		}
		frame(lua_state, 125.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 134.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 142.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_specialn2", category = ACMD_EFFECT)]
unsafe fn ryu_specialn2_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) && !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 114.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x13941a1597u64), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
			EffectModule::enable_sync_init_pos_last(module_accessor);
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			FLASH(fighter, 1, 0.862999976, 0.234999999, 0.50999999);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1320d7902bu64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, true);
		}
		frame(lua_state, 123.0);
		if is_excute(fighter) {
			COL_NORMAL(fighter);
		}
	}
	else {
		frame(lua_state, 123.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x11f7d0b8cau64), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false);
			EFFECT(fighter, Hash40::new_raw(0x0b41410a4du64), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "sound_specialn2", category = ACMD_SOUND)]
unsafe fn ryu_specialn2_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 19.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x122cf85ae1u64));
			}
		}
		else {
			frame(lua_state, 101.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x1681dd5a8eu64));
				PLAY_SE(fighter, Hash40::new_raw(0x128138ed22u64));
			}
			wait(lua_state, 3.0);
			if is_excute(fighter) {
				PLAY_SEQUENCE(fighter, Hash40::new("seq_majin_rnd_special_n02"));
			}
			wait(lua_state, 6.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialn2", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialn2_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 1.20000005, 110, 8, 0.800000012, -2, 8, 24, 16, 80);
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
			frame(lua_state, 123.0);
			if is_excute(fighter) {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0d487813b4u64), 0, false, 0);
			}
		}
		else {
			frame(lua_state, 123.0);
			if is_excute(fighter) {
				if is_excute(fighter) {
					ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackl"), 0, false, 0);
				}
			}
		}
		frame(lua_state, 142.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 166.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialsstart", category = ACMD_GAME)]
unsafe fn ryu_specialsstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 1.0, 3.5, 8.5, 8.5);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 3.0, 3.5, 8.5, 4.5);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
			AttackModule::set_size(module_accessor, 0, 0.1);
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 1.0, 3.5, 8.5, 8.5);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
			AttackModule::set_size(module_accessor, 0, 0.1);
		}
		frame(lua_state, 10.0);
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_specials", category = ACMD_GAME)]
unsafe fn ryu_specials(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 5.5, 3.0, 9.0, 3.0);
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 50, 40, 0, 60, 6.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 6.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				}
				else {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 100, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
					ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 25, 90, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				}
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 55, 40, 0, 70, 6.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
					AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
				}
				else if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 30, 120, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
					ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 25, 90, 0, 26, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				}
				else {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 35, 30, 0, 62, 4.5, 0.0, 12.5, 9.0, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
					ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 25, 30, 0, 31, 4.5, 0.0, 12.5, 12.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
				}
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 160, 0, 36, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 160, 0, 36, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 160, 0, 36, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
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
	else {
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 5.5, 3.0, 9.0, 3.0);
			if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND {
				HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
				HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
				HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
			}
			 else {
				HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
				HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
				HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				JostleModule::set_status(module_accessor, false);
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 55, 60, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 55, 60, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				JostleModule::set_status(module_accessor, false);
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 42, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 50, 42, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
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
	
}

#[acmd_script( agent = "ryu", script = "game_specialairsstart", category = ACMD_GAME)]
unsafe fn ryu_specialairsstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 1.0, 3.5, 8.5, 8.5);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 55, 40, 0, 70, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 3.0, 3.5, 8.5, 4.5);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
			AttackModule::set_size(module_accessor, 0, 0.1);
		}
	}
	else {
		frame(lua_state, 7.0);
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 1.0, 3.5, 8.5, 8.5);
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
			AttackModule::set_size(module_accessor, 0, 0.1);
		}
		frame(lua_state, 10.0);
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_specialairs", category = ACMD_GAME)]
unsafe fn ryu_specialairs(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				FT_MOTION_RATE(fighter, 0.6);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				FT_MOTION_RATE(fighter, 0.6);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				FT_MOTION_RATE(fighter, 0.6);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 20, 0, 100, 4.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::set_target_category(module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
			AttackModule::set_size(module_accessor, 0, 0.1);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
	else {
		if is_excute(fighter) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(module_accessor, 5.5, 3.0, 9.0, 3.0);
			if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND {
				HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
			}
			 else {
				HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				JostleModule::set_status(module_accessor, false);
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 55, 60, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 55, 60, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				JostleModule::set_status(module_accessor, false);
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.666);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 70, 71, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 50, 42, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 50, 42, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 50, 49, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
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
	
}

#[acmd_script( agent = "ryu", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn ryu_specialhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 4.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 120, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 70, 100, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	else {
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 70, 64, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 69, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.0, 70, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 70, 64, 0, 80, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 20.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_specialhicommand", category = ACMD_GAME)]
unsafe fn ryu_specialhicommand(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 4.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 4.0;
					SIZE2[get_player_number(module_accessor)] += 4.0;
				}
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 120, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 70, 100, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
			HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 80, 64, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 69, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 80, 64, 0, 80, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		}
		frame(lua_state, 20.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_specialairhi", category = ACMD_GAME)]
unsafe fn ryu_specialairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 4.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 3.0;
					SIZE2[get_player_number(module_accessor)] += 3.0;
				}
			}
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 3.0;
					SIZE2[get_player_number(module_accessor)] += 3.0;
				}
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.5, 70, 120, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.5, 70, 80, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	else {
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 49, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 54, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 80, 60, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 70, 70, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 20.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "game_specialairhicommand", category = ACMD_GAME)]
unsafe fn ryu_specialairhicommand(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
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
				ATTACK(fighter, 0, 0, Hash40::new("top"), 4.2, 100, 100, 90, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 3.0;
					SIZE2[get_player_number(module_accessor)] += 3.0;
				}
			}
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 80, 100, 100, 0, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					SIZE1[get_player_number(module_accessor)] += 3.0;
					SIZE2[get_player_number(module_accessor)] += 3.0;
				}
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 95, 100, 95, 0, SIZE1[get_player_number(module_accessor)], 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.5, 70, 120, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 3.5, 70, 80, 0, 80, SIZE2[get_player_number(module_accessor)], 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
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
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
			HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		}
		frame(lua_state, 6.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 49, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 54, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 80, 60, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		wait(lua_state, 1.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 9.0);
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 70, 70, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S {
			if is_excute(fighter) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
				ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
		}
		frame(lua_state, 20.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
	}	
}

#[acmd_script( agent = "ryu", script = "game_speciallwstepf", category = ACMD_GAME)] //Landing Demon Flip
unsafe fn ryu_speciallwstepf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			if *DEMON_FLIP_INPUT {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 280, 20, 0, 80, 3.6, 0.0, 2.5, 12.0, Some(0.0), Some(3.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 280, 20, 0, 80, 2.5, 0.0, 3.0, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 280, 20, 0, 80, 2.5, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			 else {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 280, 20, 0, 80, 3.6, 0.0, 2.5, 12.0, Some(0.0), Some(3.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 280, 20, 0, 80, 2.5, 0.0, 3.0, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 280, 20, 0, 80, 2.5, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			}
			AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
			AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
			AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(lua_state, 4.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			MotionModule::set_rate(module_accessor, 0.8);
		}
	}
	else {
		if is_excute(fighter) {
			SEARCH(fighter, 0, 0, Hash40::new("top"), 10.0, 0.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_BODY_HEAD, false);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
		}	
	}
}

#[acmd_script( agent = "ryu", script = "effect_speciallwstepf", category = ACMD_EFFECT)]
unsafe fn ryu_speciallwstepf_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new_raw(0x0f3b34c494u64), Hash40::new_raw(0x0f3b34c494u64), Hash40::new("top"), 1, 1.5, 0, 0, -70, -5, 1, true, 0, 0.5);
			LAST_EFFECT_SET_RATE(fighter, 2.0);
		}
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			LANDING_EFFECT_FLIP(fighter, Hash40::new_raw(0x0fe5950916u64), Hash40::new_raw(0x0f1f9a3475u64), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.899999976, 0, 0, 0, 0, 0, 0, false, 0);
			LAST_EFFECT_SET_ALPHA(fighter, 0.699999988);
		}
	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x11345bc2deu64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.10000002, 0, 0, 0, 0, 0, 0, false);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "sound_speciallwstepf", category = ACMD_SOUND)]
unsafe fn ryu_speciallwstepf_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			STOP_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1069dcccd2u64));
			PLAY_SE(fighter, Hash40::new("vc_ryu_attack03"));
		}	
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1285d1fd55u64));
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new_raw(0x136cfa04e3u64));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new_raw(0x121cf0616du64));
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "expression_speciallwstepf", category = ACMD_EXPRESSION)]
unsafe fn ryu_speciallwstepf_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 102.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
		}
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 1);
		}
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			AREA_WIND_2ND_arg10(fighter, 0, 0.800000012, 110, 8, 0.800000012, 0, 4, 32, 8, 80);
		}
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 13);
		}
		frame(lua_state, 119.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_dash"), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_speciallwstepb", category = ACMD_GAME)] //Teleport/Raging Demon
unsafe fn ryu_speciallwstepb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let RAGING_DEMON = &mut FIGHTER_BOOL_7[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if *RAGING_DEMON {
			frame(lua_state, 101.0);
			if is_excute(fighter) {
				HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
				notify_event_msc_cmd!(fighter, 0x1e0aba2d68u64, 7);
			}
			frame(lua_state, 103.0);
			if is_excute(fighter) {
				FT_SET_FINAL_FEAR_FACE(fighter, 40);
				REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
				FT_START_CUTIN(fighter);
				SlowModule::set_whole(module_accessor, 4, 0);
				MotionModule::set_rate(module_accessor, 0.5);	
			}
			frame(lua_state, 110.0);
			if is_excute(fighter) {
				sv_animcmd::CAM_ZOOM_OUT(lua_state);
				SlowModule::clear_whole(module_accessor);
			}
			frame(lua_state, 119.0);
			if is_excute(fighter) {
				MotionModule::set_rate(module_accessor, 1.0);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.5, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.1), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			}
			wait(lua_state, 5.0);
			if is_excute(fighter) {
				CameraModule::end_final_zoom_out(module_accessor);
			}
			wait(lua_state, 25.0);
			if is_excute(fighter) {
				AttackModule::clear_all(module_accessor);
			}
		}
	}
	else {
		if is_excute(fighter) {
			SEARCH(fighter, 0, 0, Hash40::new("top"), 10.0, 0.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_BODY_HEAD, false);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_speciallwstepb", category = ACMD_EFFECT)]
unsafe fn ryu_speciallwstepb_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x11345bc2deu64), Hash40::new("top"), 0, 11.5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 3.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x0d0da6e3c0u64), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.10000002, 0, 0, 0, 0, 0, 0, false);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "sound_speciallwstepb", category = ACMD_SOUND)]
unsafe fn ryu_speciallwstepb_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let RAGING_DEMON = &mut FIGHTER_BOOL_7[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if *RAGING_DEMON {
			frame(lua_state, 101.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new("vc_ryu_final02"));
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), false);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), true);
			}
			frame(lua_state, 102.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), true);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), false);
			}
			frame(lua_state, 103.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), false);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), true);
			}
			frame(lua_state, 104.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), true);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), false);
			}
			frame(lua_state, 106.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), false);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_mouth"), true);
			}
			frame(lua_state, 107.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voiceb_mouth"), true);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_mouth"), false);
			}
			frame(lua_state, 108.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voiceb_mouth"), false);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), true);
			}
			frame(lua_state, 109.0);
			if is_excute(fighter) {
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), true);
				ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), false);
			}
		}
		else {
			frame(lua_state, 103.0);
			if is_excute(fighter) {
				PLAY_SE(fighter, Hash40::new("vc_ryu_special_l01"));
			}
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1285d1fd55u64));
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new_raw(0x136cfa04e3u64));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_STEP(fighter, Hash40::new_raw(0x121cf0616du64));
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "expression_speciallwstepb", category = ACMD_EXPRESSION)]
unsafe fn ryu_speciallwstepb_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_dash"), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_speciallw", category = ACMD_GAME)] //Raging Demon Hit
unsafe fn ryu_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let MAJIN_TELEPORT = &mut FIGHTER_FLOAT_2[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			*MAJIN_TELEPORT = 0.0;
			HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_OFF), 0);
			JostleModule::set_status(module_accessor, false);
		}
		frame(lua_state, 105.0);
		if is_excute(fighter) {
			MAJIN_GRAB[get_player_number(module_accessor)] = 4;
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 100, 10, 0, 15.0, 0.0, 4.5, 9.2, Some(0.0), Some(4.5), Some(-1.8), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 1, Hash40::new("top"), 0.0, 366, 100, 10, 0, 15.0, 0.0, 4.5, 9.2, Some(0.0), Some(4.5), Some(-1.8), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 10.0, false);
		}
		frame(lua_state, 120.0);
		if is_excute(fighter) {
			MAJIN_GRAB[get_player_number(module_accessor)] = 0;
		}
		frame(lua_state, 179.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 180.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 50, 0, 50, 20.0, 0.0, 4.5, 9.2, Some(0.0), Some(4.5), Some(-1.8), 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
			AttackModule::set_damage_shake_scale(module_accessor, 0.0);
			VisibilityModule::set_whole(module_accessor, true);
			MAJIN_GRAB[get_player_number(module_accessor)] = 5;
		}
		frame(lua_state, 183.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 186.0);
		if is_excute(fighter) {
			HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
			JostleModule::set_status(module_accessor, true);
			MAJIN_GRAB[get_player_number(module_accessor)] = 0;
			MAJIN_DEMON_TARGET[get_player_number(module_accessor)] = 8;
			notify_event_msc_cmd!(fighter, 0x1e0aba2d68u64, 7);
		}
	}
	else {
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_1 as u8);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_speciallw", category = ACMD_EFFECT)]
unsafe fn ryu_speciallw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 180.0);
		if is_excute(fighter) {
			let Vec1 = Vector2f{x: 30.0, y: 30.0};
			let Vec2 = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			EffectModule::request_post_effect_line_circle(module_accessor, Hash40::new("critical_hit"), Hash40::new("handr"), Vec1, Vec2, true, 0.0, 0.0);
		}
	}
	else {
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.783999979);
			}
			wait(lua_state, 3.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 1, 0.666999996);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.588);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			frame(lua_state, 10.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x17ae418e54u64), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				LAST_EFFECT_SET_RATE(fighter, 1.20000005);
			}
		}
		if PostureModule::lr(module_accessor) > 0.0 {
			EFFECT(fighter, Hash40::new_raw(0x17dc5cd0feu64), Hash40::new("top"), 4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		else{
			EFFECT(fighter, Hash40::new_raw(0x172653ed9du64), Hash40::new("top"), -4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_speciallw", category = ACMD_SOUND)]
unsafe fn ryu_speciallw_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let RD_OVER_100 = &mut FIGHTER_BOOL_8[get_player_number(module_accessor)];

	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 180.0);
		if is_excute(fighter) {
			if !*RD_OVER_100 {
				PLAY_SE(fighter, Hash40::new("vc_ryu_appeal03"));
			}
			else {
				PLAY_SE(fighter, Hash40::new("vc_ryu_final03"));
				PLAY_SE_NO_3D(fighter, Hash40::new("se_item_homerunbat_l"));
			}
		}
	}
	else {
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				PLAY_SE(fighter, Hash40::new_raw(0x121bb568f6u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12587bbe19u64));
		
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				PLAY_SE(fighter, Hash40::new_raw(0x126cb25860u64));
				PLAY_SE(fighter, Hash40::new_raw(0x122f7c8e8fu64));
			}
			else {
				PLAY_SE(fighter, Hash40::new_raw(0x12f2d6cdc3u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12b1181b2cu64));
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_speciallw", category = ACMD_EXPRESSION)]
unsafe fn ryu_speciallw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 105.0);
		if is_excute(fighter) {		
			FILL_SCREEN_MODEL_COLOR(fighter, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
			FILL_SCREEN_MODEL_COLOR(fighter, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::CHAR, 205);
		}
		frame(lua_state, 180.0);
		if is_excute(fighter) {
			CANCEL_FILL_SCREEN(fighter, 0, 1);
			CANCEL_FILL_SCREEN(fighter, 1, 1);
			SUCCESSFUL_CLIP[get_player_number(module_accessor)] = 31;
			CAM_ZOOM_IN_arg5(fighter, 1.0, 0.0, 1.2, 0.0, 0.0);
			SlowModule::set_whole(module_accessor, 4, 0);
			let Vec1 = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor) + 10.0, z: PostureModule::pos_z(module_accessor)};
			let Vec2 = Vector3f{x: 1.0, y: 1.0, z: 1.0};
			req_shinsyoryu_hit_effect(module_accessor, &Vec1, &Vec2, true, true);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_arg10(fighter, 0, 0.600000024, 70, 8, 0.400000006, 0, 12, 40, 12, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			}
			else {
				ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
			}
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
			}
			else {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
				RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			}
		}
		frame(lua_state, 42.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 54.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialairlw", category = ACMD_GAME)] //Demon Flip Punch
unsafe fn ryu_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			if *DEMON_FLIP_INPUT {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 300, 80, 0, 10, 4.4, 0.0, 3.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 84, 14, 0, 60, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 48, 80, 0, 20, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				AttackModule::set_add_reaction_frame(module_accessor, 0, 8.0, false);
				AttackModule::set_add_reaction_frame(module_accessor, 1, 8.0, false);
				AttackModule::set_add_reaction_frame(module_accessor, 2, 8.0, false);
			}
			 else {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 300, 80, 0, 10, 4.4, 0.0, 3.0, 9.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 84, 14, 0, 60, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 48, 80, 0, 20, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		wait(lua_state, 5.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 5.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(lua_state, 133.0);
		if is_excute(fighter) {
			WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
	else {
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_1 as u8);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_specialairlw", category = ACMD_EFFECT)]
unsafe fn ryu_specialairlw_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.8000002, 0, 50, 0, 0, 0.800000012, true, 0, 0.5);
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), -0.5, 10.6999998, 2.5, 50, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true, 0, 0.800000012);
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 2.5, 9, 0, 0, 0, 0.800000012, false, 0.5);
		}
	}
	else {
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.783999979);
			}
			wait(lua_state, 3.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 1, 0.666999996);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.588);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			frame(lua_state, 10.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x17ae418e54u64), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				LAST_EFFECT_SET_RATE(fighter, 1.20000005);
			}
		}
		if PostureModule::lr(module_accessor) > 0.0 {
			EFFECT(fighter, Hash40::new_raw(0x17dc5cd0feu64), Hash40::new("top"), 4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		else{
			EFFECT(fighter, Hash40::new_raw(0x172653ed9du64), Hash40::new("top"), -4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_specialairlw", category = ACMD_SOUND)]
unsafe fn ryu_specialairlw_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_attack04"));
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x142e922876u64));
		}
	}
	else {
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				PLAY_SE(fighter, Hash40::new_raw(0x121bb568f6u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12587bbe19u64));
		
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				PLAY_SE(fighter, Hash40::new_raw(0x126cb25860u64));
				PLAY_SE(fighter, Hash40::new_raw(0x122f7c8e8fu64));
			}
			else {
				PLAY_SE(fighter, Hash40::new_raw(0x12f2d6cdc3u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12b1181b2cu64));
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialairlw", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialairlw_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
		}
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_arg10(fighter, 0, 0.600000024, 70, 8, 0.400000006, 0, 12, 40, 12, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			}
			else {
				ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
			}
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
			}
			else {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
				RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			}
		}
		frame(lua_state, 42.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 54.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_speciallwturn", category = ACMD_GAME)] //Demon Flip Ground Throw
unsafe fn ryu_speciallwturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) { 
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			JostleModule::set_status(module_accessor, false); 
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 35, 80, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(10.0 * PostureModule::lr(module_accessor)), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			MAJIN_GRAB[get_player_number(module_accessor)] = 0;
			MAJIN_DEMON_TARGET[get_player_number(module_accessor)] = 8;
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			JostleModule::set_status(module_accessor, true);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			REVERSE_LR(fighter);
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_1 as u8);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_speciallwturn", category = ACMD_EFFECT)]
unsafe fn ryu_speciallwturn_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.783999979);
			}
			wait(lua_state, 3.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 1, 0.666999996);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.588);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			frame(lua_state, 10.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x17ae418e54u64), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				LAST_EFFECT_SET_RATE(fighter, 1.20000005);
			}
		}
		if PostureModule::lr(module_accessor) > 0.0 {
			EFFECT(fighter, Hash40::new_raw(0x17dc5cd0feu64), Hash40::new("top"), 4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		else{
			EFFECT(fighter, Hash40::new_raw(0x172653ed9du64), Hash40::new("top"), -4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_speciallwturn", category = ACMD_SOUND)]
unsafe fn ryu_speciallwturn_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_special_l02"));
		}
	}
	else {
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				PLAY_SE(fighter, Hash40::new_raw(0x121bb568f6u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12587bbe19u64));
		
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				PLAY_SE(fighter, Hash40::new_raw(0x126cb25860u64));
				PLAY_SE(fighter, Hash40::new_raw(0x122f7c8e8fu64));
			}
			else {
				PLAY_SE(fighter, Hash40::new_raw(0x12f2d6cdc3u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12b1181b2cu64));
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_speciallwturn", category = ACMD_EXPRESSION)]
unsafe fn ryu_speciallwturn_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_arg10(fighter, 0, 0.600000024, 70, 8, 0.400000006, 0, 12, 40, 12, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			}
			else {
				ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
			}
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
			}
			else {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
				RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			}
		}
		frame(lua_state, 42.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 54.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialairlwturn", category = ACMD_GAME)] //Demon Flip Grab
unsafe fn ryu_specialairlwturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) { 
		frame(lua_state, 106.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 10, 0, 10, 2.5, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.1), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			AttackModule::set_add_reaction_frame(module_accessor, 0, 20.0, false);
		}
		wait(lua_state, 6.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			REVERSE_LR(fighter);
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 10, 0, 100, 3.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_1 as u8);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 0, 4.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 5.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
				AttackModule::set_attack_level(module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_attack_level(module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
				AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
				AttackModule::set_no_finish_camera(module_accessor, 1, true, false);
			}
		}
		frame(lua_state, 13.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
		}
	}
	
}

#[acmd_script( agent = "ryu", script = "effect_specialairlwturn", category = ACMD_EFFECT)]
unsafe fn ryu_specialairlwturn_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_3 {
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.783999979);
			}
			wait(lua_state, 3.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 1, 0.666999996);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				FLASH(fighter, 1, 1, 0.39199999, 0.588);
			}
			wait(lua_state, 2.0);
			if is_excute(fighter) {
				COL_NORMAL(fighter);
			}
			frame(lua_state, 10.0);
			if is_excute(fighter) {
				EFFECT(fighter, Hash40::new_raw(0x17ae418e54u64), Hash40::new("top"), 14, 10.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
				LAST_EFFECT_SET_RATE(fighter, 1.20000005);
			}
		}
		if PostureModule::lr(module_accessor) > 0.0 {
			EFFECT(fighter, Hash40::new_raw(0x17dc5cd0feu64), Hash40::new("top"), 4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
		else{
			EFFECT(fighter, Hash40::new_raw(0x172653ed9du64), Hash40::new("top"), -4, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_RATE(fighter, 1.39999998);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_specialairlwturn", category = ACMD_SOUND)]
unsafe fn ryu_specialairlwturn_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				PLAY_SE(fighter, Hash40::new_raw(0x121bb568f6u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12587bbe19u64));
		
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				PLAY_SE(fighter, Hash40::new_raw(0x126cb25860u64));
				PLAY_SE(fighter, Hash40::new_raw(0x122f7c8e8fu64));
			}
			else {
				PLAY_SE(fighter, Hash40::new_raw(0x12f2d6cdc3u64));
				PLAY_SE(fighter, Hash40::new_raw(0x12b1181b2cu64));
			}
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialairlwturn", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialairlwturn_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_arg10(fighter, 0, 0.600000024, 70, 8, 0.400000006, 0, 12, 40, 12, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0db2772ed7u64), 0, false, 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x0dc5701e41u64), 0, false, 0);
			}
			else {
				ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_attackm"), 0, false, 0);
			}
		}
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_1 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackm"), 0);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_RYU_SAVING_LV_2 {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				RUMBLE_HIT(fighter, Hash40::new("rb_kind_attackl"), 0);
			}
			else {
				QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
				RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			}
		}
		frame(lua_state, 42.0);
		if is_excute(fighter) {
			AreaModule::erase_wind(module_accessor, 0);
		}
		frame(lua_state, 54.0);
		if is_excute(fighter) {
			ItemModule::set_have_item_visibility(module_accessor, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialairlwstepf", category = ACMD_GAME)] //Demon Flip Air Throw
unsafe fn ryu_specialairlwstepf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			JostleModule::set_status(module_accessor, false);
		}
		frame(lua_state, 109.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.0, 35, 80, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(10.0 * PostureModule::lr(module_accessor)), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			MAJIN_GRAB[get_player_number(module_accessor)] = 0;
			MAJIN_DEMON_TARGET[get_player_number(module_accessor)] = 8;
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			JostleModule::set_status(module_accessor, true);
		}
	}
	else {

	}
}

#[acmd_script( agent = "ryu", script = "effect_specialairlwstepf", category = ACMD_EFFECT)]
unsafe fn ryu_specialairlwstepf_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x11345bc2deu64), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_specialairlwstepf", category = ACMD_SOUND)]
unsafe fn ryu_specialairlwstepf_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_special_l02"));
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1285d1fd55u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialairlwstepf", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialairlwstepf_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_dash"), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_specialairlwstepb", category = ACMD_GAME)] //Demon Flip Kick
unsafe fn ryu_specialairlwstepb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			SET_SPEED_EX(fighter, 2, -3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			if *DEMON_FLIP_INPUT {
				ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 65, 60, 0, 65, 4.2, 5.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			}
			 else {
				ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 65, 60, 0, 65, 4.2, 5.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			}
		}
		frame(lua_state, 134.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
	}
	else {

	}
}

#[acmd_script( agent = "ryu", script = "effect_specialairlwstepb", category = ACMD_EFFECT)]
unsafe fn ryu_specialairlwstepb_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.8000002, 0, 43, 0, 0, 0.800000012, true, 0, 0.5);
		}
		frame(lua_state, 113.0);
		if is_excute(fighter) {
			EFFECT_FLIP_ALPHA(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), -0.5, 10.6999998, 2.5, 43, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true, 0, 0.800000012);
			EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182au64), Hash40::new("top"), 0, 2.5, 9, 0, 0, 0, 0.800000012, false, 0.5);
		}
	}
	else {
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x11345bc2deu64), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_specialairlwstepb", category = ACMD_SOUND)]
unsafe fn ryu_specialairlwstepb_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 107.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_attack07"));
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x142e922876u64));
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1285d1fd55u64));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_specialairlwstepb", category = ACMD_EXPRESSION)]
unsafe fn ryu_specialairlwstepb_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {

	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
		}
		frame(lua_state, 5.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new("rb_kind_dash"), 0, true, 0);
		}
	}
}

#[acmd_script( agent = "ryu", script = "game_final", category = ACMD_GAME)] //KKZ
unsafe fn ryu_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 101.0);
		if is_excute(fighter) {
			CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
			FT_SET_FINAL_FEAR_FACE(fighter, 40);
			REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final2.nuanmb"), true);
			FT_START_CUTIN(fighter);
			SlowModule::set_whole(module_accessor, 8, 0);
		}
		frame(lua_state, 127.0);
		if is_excute(fighter) {
			SlowModule::clear_whole(module_accessor);
			CameraModule::end_final_zoom_out(module_accessor);
		}
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 90, 60, 0, 60, 8.0, 0.0, 3.0, -8.0, Some(0.0), Some(3.0), Some(4.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 90, 60, 0, 60, 8.0, 0.0, 3.0, -14.0, Some(0.0), Some(3.0), Some(10.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 90, 20, 0, 60, 8.0, 0.0, 3.0, -8.0, Some(0.0), Some(80.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 90, 20, 0, 60, 8.0, 0.0, 3.0, 4.0, Some(0.0), Some(80.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			ATTACK(fighter, 0, 0, Hash40::new("s_headbandr1"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("s_headbandr2"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("s_headbandr3"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 3, 0, Hash40::new("s_headbandl1"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 4, 0, Hash40::new("s_headbandl2"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 5, 0, Hash40::new("s_headbandl3"), 5.0, 100, 100, 180, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
		}
		frame(lua_state, 143.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			ATTACK(fighter, 0, 0, Hash40::new("s_headbandr1"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("s_headbandr2"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("s_headbandr3"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 3, 0, Hash40::new("s_headbandl1"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 4, 0, Hash40::new("s_headbandl2"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 5, 0, Hash40::new("s_headbandl3"), 5.0, 100, 100, 180, 0, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 4, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);			
		}
		frame(lua_state, 147.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			ATTACK(fighter, 0, 0, Hash40::new("s_headbandr1"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("s_headbandr2"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("s_headbandr3"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 3, 0, Hash40::new("s_headbandl1"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 4, 0, Hash40::new("s_headbandl2"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 5, 0, Hash40::new("s_headbandl3"), 15.0, 70, 100, 0, 80, 21.0, 0.0, 0.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
		}
		wait(lua_state, 3.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			notify_event_msc_cmd!(fighter, 0x1e0aba2d68u64, 7);
		}
		frame(lua_state, 210.0);
		if is_excute(fighter) {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
			SLOW_OPPONENT(fighter, 80.0, 50.0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			FT_SET_FINAL_FEAR_FACE(fighter, 40);
			REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), true);
			FT_START_CUTIN(fighter);
		}
		frame(lua_state, 28.0);
		if is_excute(fighter) {
			CAM_ZOOM_OUT(fighter);
		}
		frame(lua_state, 49.0);
		if is_excute(fighter) {
			SlowModule::set_whole(module_accessor, 2, 0);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 10, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 2.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);			
			AttackModule::set_no_dead_all(module_accessor, true, false);
	
		}
		frame(lua_state, 50.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
			}
			ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 80, 95, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);
			SlowModule::clear_whole(module_accessor);
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
		}
		frame(lua_state, 52.0);
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.0, 367, 100, 120, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
		wait(lua_state, 10.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 80, 120, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		frame(lua_state, 73.0);
		if is_excute(fighter) {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_FINAL_AIR_END, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_final", category = ACMD_EFFECT)] 
unsafe fn ryu_final_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 104.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1993768ea0u64), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.4, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1993768ea0u64), true, true);
			EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.699999988, 0, 0, 0, 0, 0, 0, false);
			LAST_EFFECT_SET_RATE(fighter, 0.800000012);
		}
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandr1"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandr2"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandr3"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandl1"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandl2"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
			EFFECT_FOLLOW(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("s_headbandl3"), 0, 0, 0, 0, 0, 0, 3.4, false);
			LAST_EFFECT_SET_COLOR(fighter, 0.27948, 0.04, 0.782004);
		}
		frame(lua_state, 147.0);
		if is_excute(fighter) {
//			let Vec1 = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor) + 10.0, z: PostureModule::pos_z(module_accessor)};
//			let Vec2 = Vector3f{x: 1.0, y: 1.0, z: 1.0};
//			req_shinsyoryu_hit_effect(module_accessor, &Vec1, &Vec2, true, true);
		}
		frame(lua_state, 149.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder_flash"), false, false);
			EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
		}
	}
	else {
		if is_excute(fighter) {
			EffectModule::req_screen(module_accessor, Hash40::new_raw(0x173f188191u64), false, false, false);
		}
		frame(lua_state, 27.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x1a84c85435u64), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 28.0);
		if is_excute(fighter) {
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, false);
			LAST_EFFECT_SET_RATE(fighter, 0.800000012);
		}
		frame(lua_state, 30.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new_raw(0x1967fc6e2eu64), Hash40::new("handr"), 1, 0, 0, 0, 90, 0, 1, true);
		}
		frame(lua_state, 48.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new_raw(0x180bbf24e9u64), Hash40::new("trans"), 8.0 * PostureModule::lr(module_accessor), 4, 0, 17, -22.0 * PostureModule::lr(module_accessor), 25.0 * PostureModule::lr(module_accessor), 0.699999988, true);
		}
		frame(lua_state, 50.0);
		if is_excute(fighter) {
			EffectModule::request_post_effect_line_circle(module_accessor, Hash40::new("critical_hit"), Hash40::new("handr"), Vector2f{x: 30.0, y: 30.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0}, true, 0.0, 0.0);
			LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, false);
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1967fc6e2eu64), false, false);
		}
		frame(lua_state, 53.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x180bbf24e9u64), false, true);
			EffectModule::remove_post_effect_line(module_accessor, 4, true);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "sound_final", category = ACMD_SOUND)] 
unsafe fn ryu_final_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 103.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_final01"));
		}
		frame(lua_state, 118.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_final01_02"));
		}
	}
	else {
		frame(lua_state, 30.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x11bd66ccc1u64));
		}
		frame(lua_state, 50.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x0e90b9fb79u64));
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "expression_final", category = ACMD_EXPRESSION)] 
unsafe fn ryu_final_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_RAD_arg9(fighter, 1, 4, 0.00999999978, 8, 0.600000024, -4, 12, 35, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			sv_animcmd::START_INFO_FLASH_EYE(lua_state);
		}
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_rush"), 0, false, 0);
		}
		frame(lua_state, 147.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_rush"), 0, false, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_RAD_arg9(fighter, 1, 4, 0.00999999978, 8, 0.600000024, -4, 12, 35, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			sv_animcmd::START_INFO_FLASH_EYE(lua_state);
		}
		frame(lua_state, 50.0);
		if is_excute(fighter) {
			RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_rush"), 0, false, 0);
		}
		
	}
}

#[acmd_script( agent = "ryu", script = "game_finalair", category = ACMD_GAME)]
unsafe fn ryu_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 1.0);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 80.0, 50.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), true);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 49.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 2, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 10, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 2.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);			
		AttackModule::set_no_dead_all(module_accessor, true, false);

	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
		}
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 80, 95, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);
		SlowModule::clear_whole(module_accessor);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.0, 367, 100, 120, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 80, 120, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
}

#[acmd_script( agent = "ryu", script = "game_finalair2", category = ACMD_GAME)] 
unsafe fn ryu_finalair2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			WHOLE_HIT(fighter, *HIT_STATUS_XLU);
			SLOW_OPPONENT(fighter, 10.0, 70.0);
			CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		}
		frame(lua_state, 140.0);
		if is_excute(fighter) {
			FT_SET_FINAL_FEAR_FACE(fighter, 50);
			REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
			FT_START_CUTIN(fighter);
		}
		frame(lua_state, 161.0);
		if is_excute(fighter){
			CAM_ZOOM_OUT(fighter);
		}
		frame(lua_state, 189.0);
		if is_excute(fighter){
			ArticleModule::generate_article(module_accessor, *FIGHTER_RYU_GENERATE_ARTICLE_SHINKUHADOKEN, false, 0);
		}
		frame(lua_state, 269.0);
		if is_excute(fighter){
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
		}
	}
	else {
		if is_excute(fighter) {
			WHOLE_HIT(fighter, *HIT_STATUS_XLU);
			SLOW_OPPONENT(fighter, 10.0, 70.0);
			CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		}
		frame(lua_state, 10.0);
		if is_excute(fighter) {
			FT_SET_FINAL_FEAR_FACE(fighter, 50);
			REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
			FT_START_CUTIN(fighter);
		}
		frame(lua_state, 31.0);
		if is_excute(fighter){
			CAM_ZOOM_OUT(fighter);
		}
		frame(lua_state, 70.0);
		if is_excute(fighter){
			ArticleModule::generate_article(module_accessor, *FIGHTER_RYU_GENERATE_ARTICLE_SHINKUHADOKEN, false, 0);
		}
		frame(lua_state, 75.0);
		if is_excute(fighter){
			WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_finalair2", category = ACMD_EFFECT)] 
unsafe fn ryu_finalair2_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			EffectModule::req_screen(module_accessor, Hash40::new("bg_ryu_final_shinkuhado"), false, false, false);
		}
		frame(lua_state, 135.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x1993768ea0u64), Hash40::new("handr"), 10, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame(lua_state, 156.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_hold"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame(lua_state, 178.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new("ryu_hadoken_hold"), true, true);
		}
		frame(lua_state, 189.0);
		if is_excute(fighter){
			EFFECT(fighter, Hash40::new_raw(0x1927bb0b1cu64), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame(lua_state, 190.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1993768ea0u64), true, true);
		}
		frame(lua_state, 196.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x19abdec128u64), Hash40::new("handl"), 0, 1, 0, 0, 0, 0, 1, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame(lua_state, 197.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x11f7d0b8cau64), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		if is_excute(fighter) {
			EffectModule::req_screen(module_accessor, Hash40::new("bg_ryu_final_shinkuhado"), false, false, false);
		}
		frame(lua_state, 2.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1993768ea0u64), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
		}
		frame(lua_state, 70.0);
		if is_excute(fighter){
			EFFECT(fighter, Hash40::new_raw(0x1927bb0b1cu64), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
		}
		frame(lua_state, 71.0);
		if is_excute(fighter) {
			EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1993768ea0u64), true, true);
		}
		frame(lua_state, 76.0);
		if is_excute(fighter) {
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x19abdec128u64), Hash40::new("handl"), 0, 1, 0, 0, 0, 0, 1, true);
		}
		frame(lua_state, 77.0);
		if is_excute(fighter) {
			FOOT_EFFECT(fighter, Hash40::new_raw(0x11f7d0b8cau64), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_finalair2", category = ACMD_SOUND)] 
unsafe fn ryu_finalair2_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 131.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_final02_01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_appeal03"));
		}
		wait(lua_state, 69.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_final04_02"));
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_ryu_final02_01"));
			PLAY_SE(fighter, Hash40::new("vc_ryu_final01"));
		}
		wait(lua_state, 69.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("vc_ryu_final01_02"));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_finalair2", category = ACMD_EXPRESSION)] 
unsafe fn ryu_finalair2_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_RAD_arg9(fighter, 1, 4, -0.01, 8, 0.6, -4, 12, 35, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			//START_INFO_FLASH_EYE(fighter);
		}
		frame(lua_state, 134.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_KEEPSMALL);
		}
		frame(lua_state, 190.0);
		if is_excute(fighter) {
//			QUAKE_STOP(fighter, *CAMERA_QUAKE_KIND_KEEPSMALL);
		}
		frame(lua_state, 193.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_attackll"), 0, false, 0);
		}
		frame(lua_state, 207.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x15109fb778u64), 0, false, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
			AREA_WIND_2ND_RAD_arg9(fighter, 1, 4, -0.01, 8, 0.6, -4, 12, 35, 80);
			ItemModule::set_have_item_visibility(module_accessor, false, 0);
			//START_INFO_FLASH_EYE(fighter);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_KEEPSMALL);
		}
		frame(lua_state, 70.0);
		if is_excute(fighter) {
			//QUAKE_STOP(fighter, *CAMERA_QUAKE_KIND_KEEPSMALL);
		}
		frame(lua_state, 73.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_attackll"), 0, false, 0);
		}
		frame(lua_state, 87.0);
		if is_excute(fighter) {
			ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x15109fb778u64), 0, false, 0);
		}
	}
}

#[acmd_script( agent = "ryu_shinkuhadoken", script = "game_move", category = ACMD_GAME)]
unsafe fn ryu_shinkuhadoken_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if is_majin(module_accessor, *WEAPON_KIND_RYU_SHINKUHADOKEN) {
			MotionModule::set_rate(module_accessor, 2.5);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 32, 100, 90, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 366, 100, 100, 0, 30.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 366, 100, 100, 0, 50.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 32, 100, 90, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 366, 100, 75, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 366, 100, 80, 0, 35.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);	
		}
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 1, Hash40::new("top"), 10.0, 50, 75, 0, 90, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 6.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "ryu_shinkuhadoken", script = "effect_move", category = ACMD_EFFECT)]
unsafe fn ryu_shinkuhadoken_move_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			EffectModule::req_screen(module_accessor, Hash40::new("bg_ryu_final_shinkuhado2"), false, false, false);
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1b2fe4447du64), Hash40::new("top"), 0, 0, 1, 45, 0, 0, 1.2, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1c344e57c8u64), Hash40::new("top"), 0, 0, 2, 45, 0, 0, 1.5, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
		}
		frame (lua_state, 90.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x1bb53e9f10u64), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	}
	else {
		if is_excute(fighter) {
			EffectModule::req_screen(module_accessor, Hash40::new("bg_ryu_final_shinkuhado2"), false, false, false);
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1b2fe4447du64), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, true);
			EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1c344e57c8u64), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.5, true);
		}
		frame (lua_state, 90.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new_raw(0x1bb53e9f10u64), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_appealsr", category = ACMD_SOUND)]
unsafe fn ryu_appealsr_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1169742a38u64));
		}
		wait(lua_state, 15.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x0f3bb0e770u64));
		}
	}
	else {
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x122cf85ae1u64));
		}		
	}
}

#[acmd_script( agent = "ryu", script = "sound_appealsl", category = ACMD_SOUND)]
unsafe fn ryu_appealsl_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 110.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x1169742a38u64));
		}
		wait(lua_state, 15.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x0f3bb0e770u64));
		}
	}
	else {
		frame(lua_state, 11.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x126f368c0eu64));
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new_raw(0x122cf85ae1u64));
		}		
	}
}

#[acmd_script( agent = "ryu", script = "game_throwf", category = ACMD_GAME)] 
unsafe fn ryu_throwf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 35, 70, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		}
		frame(lua_state, 111.0);
		if is_excute(fighter) {
			CHECK_FINISH_CAMERA(fighter, 15, 7);
			let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
			lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 5.0, y: 1.0, z: 0.0 });
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 50, 45, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
			ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		}
		frame(lua_state, 15.0);
		if is_excute(fighter) {
			CHECK_FINISH_CAMERA(fighter, 15, 7);
			let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
			lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 5.0, y: 1.0, z: 0.0 });
		}
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
	}
}

#[acmd_script( agent = "ryu", script = "effect_throwf", category = ACMD_EFFECT)] 
unsafe fn ryu_throwf_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 114.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
		}
	}
	else {
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		}
		frame(lua_state, 18.0);
		if is_excute(fighter) {
			EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "ryu", script = "sound_throwf", category = ACMD_SOUND)] 
unsafe fn ryu_throwf_sound(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
		}
		frame(lua_state, 108.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
		}
	}
	else {
		frame(lua_state, 1.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
		}
		frame(lua_state, 12.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
		}
	}
}

#[acmd_script( agent = "ryu", script = "expression_throwf", category = ACMD_EXPRESSION)] 
unsafe fn ryu_throwf_expression(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_majin(module_accessor, *FIGHTER_KIND_RYU) {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 112.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
		}
	}
	else {
		if is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 16.0);
		if is_excute(fighter) {
			QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
		}
	}
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movew", category = ACMD_GAME)]
unsafe fn ryu_hadoken_movew(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		wait(lua_state, 7.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
	 }
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		wait(lua_state, 7.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 0, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.2);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "effect_movew", category = ACMD_EFFECT)]
unsafe fn ryu_hadoken_movew_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
			if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 66, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 66, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
	}
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
			}
		}
		 else {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movem", category = ACMD_GAME)]
unsafe fn ryu_hadoken_movem(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		wait(lua_state, 6.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
	 }
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		wait(lua_state, 6.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 9.5, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.13);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "effect_movem", category = ACMD_EFFECT)]
unsafe fn ryu_hadoken_movem_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
		if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 45, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 45, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
	}
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
			}
		}
		 else {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "game_moves", category = ACMD_GAME)]
unsafe fn ryu_hadoken_moves(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40::new("ryu_hadoken_smoke"));
			AttackModule::set_optional_hit_effect(module_accessor, 1, Hash40::new("ryu_hadoken_smoke"));
			AttackModule::set_optional_hit_effect(module_accessor, 2, Hash40::new("ryu_hadoken_smoke"));
		}
		wait(lua_state, 5.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
	 }
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 6.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 10, 0, 67, 5.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
			ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
		}
		wait(lua_state, 5.0);
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 10, 0, 58, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 10, 0, 58, 5.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
			if is_excute(fighter) {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 0, 10, 0, 68, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
				ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 1.06);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "effect_moves", category = ACMD_EFFECT)]
unsafe fn ryu_hadoken_moves_effect(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if is_majin(owner_module_accessor, *FIGHTER_KIND_RYU) {
		if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
		else {
			if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 33, 0, 0, 0.600000024, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
			 else {
				if is_excute(fighter) {
					EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 33, 0, 0, 0.649999976, false);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
				}
			}
		}
	}
	else {
		if WorkModule::is_flag(module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.600000024, false);
			}
		}
		 else {
			if is_excute(fighter) {
				EFFECT_FOLLOW(fighter, Hash40::new("ryu_hadoken_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.649999976, false);
			}
		}
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movespw", category = ACMD_GAME)]
unsafe fn ryu_hadoken_movespw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	wait(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movespm", category = ACMD_GAME)]
unsafe fn ryu_hadoken_movespm(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	wait(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movesps", category = ACMD_GAME)]
unsafe fn ryu_hadoken_movesps(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 1.1, 80, 10, 0, 42, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 1.1, 366, 10, 0, 40, 5.5, 0.0, 0.5, 0.0, Some(0.0), Some(-3.2), Some(0.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 4.0, false);
	}
	
}

#[acmd_script( agent = "ryu", script = "game_catch", category = ACMD_GAME)]
unsafe fn ryu_catch(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "ryu", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ryu_catchdash(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "ryu", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ryu_catchturn(fighter: &mut L2CAgentBase) {
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

pub unsafe fn get_command_stick_direction(module_accessor: &mut app::BattleObjectModuleAccessor, stick: bool) -> i32 {
	let status_kind = StatusModule::status_kind(module_accessor);
	let mut stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
	if !stick {
		stick_x = SUB_STICK[get_player_number(module_accessor)].x * PostureModule::lr(module_accessor);
	}
	if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
		stick_x *= -1.0;
	}

	if stick_x >= 0.2 {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 3;
		}
		else {
			return 6;
		}
	}
	else if stick_x <= -0.2 {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 1;
		}
		else {
			return 4;
		}
	}
	else {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 2;
		}
		else {
			return 0;
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
pub fn ryu_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = fighter.global_table[SITUATION_KIND].get_int() as i32;
		let cat1 = fighter.global_table[CMD_CAT1].get_int() as i32;
		let cat4 = fighter.global_table[CMD_CAT4].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();


		let PJAB_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];
		let MAJIN_TELEPORT = &mut FIGHTER_FLOAT_2[get_player_number(module_accessor)];
		let SHAKU_INPUT = &mut FIGHTER_FLOAT_3[get_player_number(module_accessor)];
		let DEMON_GRAB_TARGET = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let HADOKEN_HIT_EFF = &mut FIGHTER_VEC3F_1[get_player_number(module_accessor)];
		let PJAB_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let PTILT_CHECK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let G_DEMON_FLIP = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		let MAJIN_ZANKU = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];
		let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
		let DEMON_FOLLOWUP = &mut FIGHTER_BOOL_6[get_player_number(module_accessor)];
		let RAGING_DEMON = &mut FIGHTER_BOOL_7[get_player_number(module_accessor)];


		if fighter_kind == FIGHTER_KIND_RYU {
			if is_majin(module_accessor, fighter_kind) {
				if let L2CValueType::Void = globals["majin_globals_set"].val_type {
					globals["demon_flip_strength"] = 0.into();
					globals["shaku_level"] = 0.into();
					globals["special_timer"] = 0.into();	
					globals["demon_level"] = 0.into();
					globals["demon_timer"] = 0.into();	
					globals["air_tatsu"] = false.into();
					globals["teleport_level"] = 0.0.into();
					globals["majin_globals_set"] = true.into();
				}
				if sv_information::is_ready_go() == false {
					globals["demon_level"] = 0.into();
					globals["demon_timer"] = 0.into();	();
					globals["demon_flip_strength"] = 0.into();
					globals["shaku_level"] = 0.into();
					*DEMON_GRAB_TARGET = 8;
				}

				LookupSymbol(
					&mut FIGHTER_MANAGER_ADDR,
					"_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
					.as_bytes()
					.as_ptr(),
				);
				let _fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);

				DamageModule::set_damage_mul(module_accessor, 1.2); //Majin always takes 1.2x damage
				
				//Prox Check. Even though Majin and Ryu share the same code after determining that they're in prox range, Majin has a new PTilt with more vertical prox range
				
				for i in 0 .. TOTAL_FIGHTER {
					let o = i as usize;
					if (PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(i))).abs() < 14.0 && (PostureModule::pos_y(module_accessor) - PostureModule::pos_y(get_boma(i))).abs() < 8.0 && o != get_player_number(module_accessor) {
						*PTILT_CHECK = true;
						break;
					}	
					else {
						*PTILT_CHECK = false;
					}
				}

				//Since Majin and Ryu share a motion_list, whoever's version of a move has less FAF needs to be handled here instead of their motion_list

				if motion_kind == hash40("attack_s3_s_w") && MotionModule::frame(module_accessor) >= 21.0 {
					CancelModule::enable_cancel(module_accessor);
				}
				if motion_kind == hash40("attack_air_n") && MotionModule::frame(module_accessor) >= 26.0 {
					CancelModule::enable_cancel(module_accessor);
				}
				if motion_kind == hash40("special_s_end") {
					if MotionModule::frame(module_accessor) >= 20.0 {
						CancelModule::enable_cancel(module_accessor);
					}
					if WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W && MotionModule::frame(module_accessor) >= 16.0 {
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						}
						if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) != 0 {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, true);
						}
					}
				}
				if motion_kind == hash40("special_air_s_end") && MotionModule::frame(module_accessor) >= 20.0 {
					CancelModule::enable_cancel(module_accessor);
				}

				//Kara Cancel HFTilt

				if motion_kind == hash40("attack_s3_s_s") && MotionModule::frame(module_accessor) < 105.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) || (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)) {
						let added_pos = Vector3f{x: (MotionModule::frame(module_accessor) - 97.0) * PostureModule::lr(module_accessor), y: 0.0, z: 0.0};
						PostureModule::add_pos(module_accessor, &added_pos);
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
					}
				}


				//Making sure Hadokens come out at the correct heights (without this block, fireballs will still have the correct trajectory but they'll come out lower until you do a regular grounded hadoken)
				if (motion_kind == hash40("special_n") || motion_kind == hash40("special_n2")) && !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
					*MAJIN_ZANKU = false;
				}
				if motion_kind == hash40("special_air_n") {
					*MAJIN_ZANKU = true;
				}
				WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP);

				//Handling the input for Majin's Shakunetsu Hadoken

				if globals["special_timer"].get_int() as i32 != 0 { //Unless we've already started to make a 63214 motion, start counting down, else increment the input stage and set the counter to 10
					if !(globals["shaku_level"].get_int() as i32 == 5 && (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD))) {
						globals["special_timer"] = (globals["special_timer"].get_int() - 1).into(); 
					}
				}
				else {
					globals["shaku_level"] = 0.into();
					if status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND { //If it's been >10 frames since the last input, unless we're already in the Shaku status, reset the input stage.
						*SHAKU_INPUT = 0.0;
					}
				}
				if get_command_stick_direction(module_accessor, true) == 6 {
					globals["special_timer"] = 10.into();
					globals["shaku_level"] = 1.into();
				}
				else if get_command_stick_direction(module_accessor, true) == 3 && globals["shaku_level"].get_int() as i32 == 1 {
					globals["special_timer"] = 10.into();
					globals["shaku_level"] = 2.into();
				}
				else if get_command_stick_direction(module_accessor, true) == 2 && globals["shaku_level"].get_int() as i32 == 2 {
					globals["special_timer"] = 10.into();
					globals["shaku_level"] = 3.into();
				}
				else if get_command_stick_direction(module_accessor, true) == 1 && globals["shaku_level"].get_int() as i32 == 3 {
					globals["special_timer"] = 10.into();
					globals["shaku_level"] = 4.into();
				}
				else if get_command_stick_direction(module_accessor, true) == 4 && globals["shaku_level"].get_int() as i32 == 4 {
					globals["special_timer"] = 10.into();
					globals["shaku_level"] = 5.into();
					if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
					|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
					|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
					|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
					|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
					|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
					|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
					|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
						*SHAKU_INPUT = PostureModule::lr(module_accessor);
					}
				}
				else if globals["shaku_level"].get_int() as i32 == 5 { //If you reach the last input stage, the only requirement to do a Shaku input is that you're holding backwards
					if get_command_stick_direction(module_accessor, true) == 4 {
						if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
						|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
						|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
						|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
						|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
						|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
						|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
						|| ControlModule::check_button_release(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
							*SHAKU_INPUT = PostureModule::lr(module_accessor);
						}
					}
					else {
						globals["shaku_level"] = 0.into();
					}
				}
				if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND { //Prevents Majin from facing the wrong direction during Shaku
					if MotionModule::frame(module_accessor) < 119.0 {
						ControlModule::set_main_stick_x(module_accessor, *SHAKU_INPUT);
					}
				}

				//Raging Demon
				if situation_kind == *SITUATION_KIND_GROUND {
					if motion_kind == hash40("special_lw") {
						ShakeModule::stop(module_accessor);
						PhysicsModule::set_swing_rate(module_accessor, 0.0);
					}
					if globals["demon_timer"].get_int() as i32 != 0 { //Unless we've already started to make a 63214 motion, start counting down, else increment the input stage and set the counter to 10
						globals["demon_timer"] = (globals["demon_timer"].get_int() - 1).into(); 
					}
					else {
						globals["demon_level"] = 0.into();
						if status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B { //If it's been >10 frames since the last input, unless we're already in the Shaku status, reset the input stage.
							*RAGING_DEMON = false;
						}
					}
					if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
						if globals["demon_level"].get_int() as i32 == 0 || globals["demon_level"].get_int() as i32 == 1 || globals["demon_level"].get_int() as i32 == 3 {
							globals["demon_timer"] = 10.into();
							globals["demon_level"] = (globals["demon_level"].get_int() as i32 + 1).into();
						}
					}
					if ControlModule::get_flick_x(module_accessor) < 3 && get_command_stick_direction(module_accessor, true) == 6 && globals["demon_level"].get_int() as i32 == 2 {
						globals["demon_timer"] = 10.into();
						globals["demon_level"] = 3.into();
					}
					if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL) || motion_kind == hash40("attack_11_w") || motion_kind == hash40("attack_11_s") || motion_kind == hash40("attack_11_near_s") {
						if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && globals["demon_level"].get_int() as i32 == 4 && WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
							globals["demon_timer"] = 10.into();
							globals["demon_level"] = 5.into();
							*MAJIN_TELEPORT = PostureModule::lr(module_accessor);
							*RAGING_DEMON = true;
							ControlModule::clear_command(module_accessor, true);
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
						}
					}	
				}
				else {
					globals["demon_timer"] = 0.into();
					globals["demon_level"] = 0.into();
					*RAGING_DEMON = false;
				}

				//Causing Tatsu to go into its ending animation when you land at any point during the move

				if motion_kind == hash40("special_air_s") {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, true);
					}
				}

				if motion_kind == hash40("special_air_s_start") || motion_kind == hash40("special_s_start") || motion_kind == hash40("special_s") {
					if motion_kind == hash40("special_air_s_start") {
						globals["air_tatsu"] = true.into();
					}
					else if globals["air_tatsu"].get_bool() {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, true);
					}
				}
				else {
					globals["air_tatsu"] = false.into();
				}

				if motion_kind == hash40("special_hi") {
					if MotionModule::frame(module_accessor) < 4.0 {
						let added_pos = Vector3f{x: 0.8, y: 0.0, z: 0.0};
						PostureModule::add_pos(module_accessor, &added_pos);
					}
				}
				if motion_kind == hash40("special_hi_command") {
					if MotionModule::frame(module_accessor) < 4.0 {
						let added_pos = Vector3f{x: 1.2, y: 0.0, z: 0.0};
						PostureModule::add_pos(module_accessor, &added_pos);
					}
				}

				//Demon Flip's code in general

				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					if situation_kind == *SITUATION_KIND_GROUND { 
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
					}
				}
				else if status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT 
				&& status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK
				&& status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN
				&& status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F
				&& status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B
				&& motion_kind != hash40("special_lw_start")
				&& motion_kind != hash40("special_air_lw_start")
				&& situation_kind == *SITUATION_KIND_GROUND {
					globals["demon_flip_strength"] = 0.into();
					*DEMON_FLIP_INPUT = false;
				}
				if motion_kind == hash40("special_lw_start") {
					if estimate_frame(module_accessor, 100.0) {
						WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
						KineticModule::clear_speed_all(module_accessor);
						let mut flipVec = Vector3f{x: 0.0, y: 3.2, z: 0.0};
						if *DEMON_FLIP_INPUT {
							flipVec.y += 0.2;
						}
						if globals["demon_flip_strength"].get_int() as i32 == 1 {
							flipVec.x = 1.0;
							if *DEMON_FLIP_INPUT {
								flipVec.x -= 0.2;
							}
						}
						if globals["demon_flip_strength"].get_int() as i32 == 2 {
							flipVec.x = 1.4;
						}
						if globals["demon_flip_strength"].get_int() as i32 == 3 {
							flipVec.x = 1.8;
							if *DEMON_FLIP_INPUT {
								flipVec.x += 0.2;
							}
						}
						KineticModule::add_speed(module_accessor, &flipVec);
						WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
						if *DEMON_FLIP_INPUT {
							HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
						}
					}
					if estimate_frame(module_accessor, 120.0) {
						let mut counterFlipVec = Vector3f{x: 0.0, y: -1.2, z: 0.0};
						if *DEMON_FLIP_INPUT {
							counterFlipVec.y -= 0.4;
						}
						if *DEMON_FLIP_INPUT {
							HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
						}
						KineticModule::add_speed(module_accessor, &counterFlipVec);
					}
					if MotionModule::frame(module_accessor) >= 120.0 {
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN, true); //Grab
							*DEMON_FOLLOWUP = false;
						}
						else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true); //Kick
							*DEMON_FOLLOWUP = false;
						}
						else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK, true); //Punch
							*DEMON_FOLLOWUP = false;
						}
					}
					if estimate_frame(module_accessor, 128.0) {
						let downFlipVec = Vector3f{x: 0.0, y: -5.6, z: 0.0};
						KineticModule::add_speed(module_accessor, &downFlipVec);
					}
				}

				if *G_DEMON_FLIP && motion_kind == hash40("jump_squat") {
					ControlModule::set_main_stick_x(module_accessor, 0.0);
				}

				if motion_kind == hash40("special_air_lw_start") || (*G_DEMON_FLIP && motion_kind == hash40("jump_squat")) {
					if (MotionModule::frame(module_accessor) < 106.0 && motion_kind == hash40("special_air_lw_start")) || (MotionModule::frame(module_accessor) < 5.0 && motion_kind == hash40("jump_squat")) {
						KineticModule::clear_speed_all(module_accessor);
					}
					if MotionModule::frame(module_accessor) < 106.0 && globals["demon_flip_strength"].get_int() as i32 == 0 {
						if motion_kind == hash40("jump_squat") {
							if (*DEMON_FLIP_INPUT
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) 
							|| (*DEMON_FLIP_INPUT == false && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
								if MotionModule::frame(module_accessor) < 2.0 {
									globals["demon_flip_strength"] = 1.into();
								}
								else {
									globals["demon_flip_strength"] = 2.into();
								}
							}
							else if MotionModule::frame(module_accessor) == 5.0 {
								globals["demon_flip_strength"] = 3.into();
							}	
						}
						else {
							if (*DEMON_FLIP_INPUT
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
							&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) 
							|| (*DEMON_FLIP_INPUT == false && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
								if MotionModule::frame(module_accessor) < 102.0 {
									globals["demon_flip_strength"] = 1.into();
								}
								else {
									globals["demon_flip_strength"] = 2.into();
								}
							}
							else if MotionModule::frame(module_accessor) == 105.0 {
								globals["demon_flip_strength"] = 3.into();
							}
	
						}
					}
					if MotionModule::frame(module_accessor) == 106.0 {
						WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
						KineticModule::clear_speed_all(module_accessor);
						let mut flipVec = Vector3f{x: 0.0, y: 3.0, z: 0.0};
						if *DEMON_FLIP_INPUT {
							flipVec.y += 0.2;
						}
						if globals["demon_flip_strength"].get_int() as i32 == 1 {
							flipVec.x = 1.0;
							if *DEMON_FLIP_INPUT {
								flipVec.x -= 0.2;
							}
	
						}
						if globals["demon_flip_strength"].get_int() as i32 == 2 {
							flipVec.x = 1.4;
						}
						if globals["demon_flip_strength"].get_int() as i32 == 3 {
							flipVec.x = 1.8;
							if *DEMON_FLIP_INPUT {
								flipVec.x += 0.2;
							}
						}
						KineticModule::add_speed(module_accessor, &flipVec);
						WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
						if *DEMON_FLIP_INPUT {
							HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
						}
					}
					if MotionModule::frame(module_accessor) == 126.0 {
						let mut counterFlipVec = Vector3f{x: 0.0, y: -2.8, z: 0.0};
						if *DEMON_FLIP_INPUT {
							counterFlipVec.y -= 0.4;
						}
						KineticModule::add_speed(module_accessor, &counterFlipVec);
						if *DEMON_FLIP_INPUT {
							HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
						}
					}
					if MotionModule::frame(module_accessor) >= 126.0 {
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN, true); //Grab
							*DEMON_FOLLOWUP = false;
						}
						else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true); //Kick
							*DEMON_FOLLOWUP = false;
						}
						else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 {
							*DEMON_FOLLOWUP = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK, true); //Punch
							*DEMON_FOLLOWUP = false;
						}
					}
					if estimate_frame(module_accessor, 134.0) {
						let downFlipVec = Vector3f{x: 0.0, y: -3.6, z: 0.0};
						KineticModule::add_speed(module_accessor, &downFlipVec);
					}
				}

				//Demon Flip Grab

				if motion_kind == hash40("special_air_lw_turn") || motion_kind == hash40("special_air_lw_step_f") || motion_kind == hash40("special_lw_turn") /*yeah ok so i didn't really plan this part out ahead of time but 
				special_lw_turn = ground throw, special_air_lw_step_f = air throw*/ { 
					JostleModule::set_status(module_accessor, false);
					if motion_kind == hash40("special_air_lw_turn") {
						if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
							AttackModule::clear_all(module_accessor);
							MAJIN_GRAB[get_player_number(module_accessor)] = 2;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
						}
					}
					else if *DEMON_GRAB_TARGET != 8 {
						let x = PostureModule::pos_x(&mut *get_boma(*DEMON_GRAB_TARGET));
						let y = PostureModule::pos_y(&mut *get_boma(*DEMON_GRAB_TARGET));
						let z = PostureModule::pos_z(&mut *get_boma(*DEMON_GRAB_TARGET));
						let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
						if MotionModule::frame(module_accessor) >= 102.0 && MotionModule::frame(module_accessor) < 104.0  {
							pos.y = y + 8.0;
						}
						if MotionModule::frame(module_accessor) >= 104.0 && MotionModule::frame(module_accessor) < 109.0 {
							pos.x += 2.5 * PostureModule::lr(module_accessor);
							pos.y = y + 11.0;
						}
						if MotionModule::frame(module_accessor) == 107.0 {
							if situation_kind == *SITUATION_KIND_AIR {
								pos.x += 4.0 * PostureModule::lr(module_accessor);
							}
							else {
								pos.x += 6.0 * PostureModule::lr(module_accessor);
							}
						}
						if situation_kind == *SITUATION_KIND_AIR {
							if MotionModule::frame(module_accessor) > 103.0 {
								pos.y -= 2.0;
							}
							if MotionModule::frame(module_accessor) == 102.0 {
								pos.x = x - 6.0 * PostureModule::lr(module_accessor);
							}
						}
						PostureModule::set_pos(module_accessor, &pos);
					}
					else if MotionModule::frame(module_accessor) < 112.0 {
						let pos = Vector3f{x: PostureModule::lr(module_accessor), y: 0.0, z: 0.0};
						PostureModule::add_pos(module_accessor, &pos);
					}
					if motion_kind == hash40("special_lw_turn") && situation_kind == *SITUATION_KIND_GROUND && MotionModule::frame(module_accessor) > 12.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					}
				}

				if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B && status_kind != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
					*MAJIN_TELEPORT = 0.0;
					globals["teleport_level"] = 0.0.into();
				}

				//Set up special canceling on Up Smash and HJab 1, as well as allowing special canceling into Shakunetsu, Demon Flip and Teleport at all

				if (WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) && motion_kind == hash40("attack_hi4") && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL))
				|| ((motion_kind == hash40("attack_11_s") || motion_kind == hash40("attack_11_near_s")) && AttackModule::is_infliction_status(module_accessor, *COLLISION_PART_MASK_ALL) && MotionModule::frame(module_accessor) < 114.0) {
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					} 
					if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, true);
					}
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
					} 
					if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, true);
					}
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					} 
					if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, true);
					}
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);								
					}
					if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) != 0 {
						*DEMON_FLIP_INPUT = true;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, true);
					}
					if ((cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4) != 0 || (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6) != 0) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
						let mut stick = 1.0;
						if ControlModule::get_stick_x(module_accessor) < 0.0 {
							stick = -1.0;
						}
						globals["teleport_level"] = 0.0.into();
						*MAJIN_TELEPORT = stick;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					}
				}

				//Canceling other ground moves into teleport

				if WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) && (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) 
				|| AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD)) && situation_kind == *SITUATION_KIND_GROUND && motion_kind != hash40("attack_lw4") {
					let cat4 = fighter.global_table[CMD_CAT4].get_int() as i32;
					if ((cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4) != 0 || (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6) != 0) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
						let mut stick = 1.0;
						if ControlModule::get_stick_x(module_accessor) < 0.0 {
							stick = -1.0;
						}
						globals["teleport_level"] = 0.0.into();
						*MAJIN_TELEPORT = stick;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					}
				}

				//Fixes some weird buffer issues

				if AttackModule::is_infliction_status(module_accessor, *COLLISION_CATEGORY_MASK_ALL) && *G_DEMON_FLIP { 
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
				}
				if motion_kind == hash40("attack_lw3_w") && MotionModule::frame(module_accessor) < 100.0 {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_lw3_w"), 1.0, 1.0, false, 0.0, false, false);
				}
				if motion_kind == hash40("attack_near_w") && MotionModule::frame(module_accessor) < 100.0 {
					MotionModule::change_motion(module_accessor, Hash40::new("attack_near_w"), 1.0, 1.0, false, 0.0, false, false);
				}

				//Teleport

				//Handling the teleport input assuming no cancel
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) && (WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH)
				|| WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) || status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
				|| status_kind == *FIGHTER_RYU_STATUS_KIND_DASH_BACK) {
					let cat4 = fighter.global_table[CMD_CAT4].get_int() as i32;
					if (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4) != 0 || (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6) != 0 {
						let mut stick = 1.0;
						if ControlModule::get_stick_x(module_accessor) < 0.0 {
							stick = -1.0;
						}
						globals["teleport_level"] = 0.0.into();
						*MAJIN_TELEPORT = stick;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					}
				}

				if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B && situation_kind == *SITUATION_KIND_GROUND {
					let mut pos = *PostureModule::pos(module_accessor);
					let offset = ModelModule::joint_global_offset_from_top(module_accessor, Hash40{hash: hash40("handl")}, &mut pos);		
					MAJIN_OFFSET[get_player_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor) + offset.x, y: PostureModule::pos_y(module_accessor) + offset.y + 8.0, z: PostureModule::pos_z(module_accessor) + offset.z};
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
						MAJIN_GRAB[get_player_number(module_accessor)] = 3;
						globals["teleport_level"] = 0.0.into();
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK, true);
					}
					
					//Teleport distance depending on when Shield is released

					if !*RAGING_DEMON {
						if globals["teleport_level"].get_num() == 0.0 {
							if MotionModule::frame(module_accessor) < 108.0 && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
							   globals["teleport_level"] = 1.0.into();
						   }
						   else if MotionModule::frame(module_accessor) < 116.0 && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
							   globals["teleport_level"] = 2.0.into();
						   }
						   else if MotionModule::frame(module_accessor) >= 116.0 {
							   globals["teleport_level"] = 3.0.into();
						   }
					   }
					}
					else {
						globals["teleport_level"] = 3.5.into();
					}

					if globals["teleport_level"].get_num() != 3.5 {
						if MotionModule::frame(module_accessor) >= 116.0 || MotionModule::frame(module_accessor) == 100.0 { //Enable autoturn while teleporting
							if TOTAL_FIGHTER == 2 {
								let mut not_player = 0;
								if get_player_number(module_accessor) == 0 {
									not_player = 1;
								}
	
								if ((PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(not_player))) / (PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(not_player))).abs()) != PostureModule::lr(module_accessor) * -1.0 {
									PostureModule::reverse_lr(module_accessor);
									PostureModule::update_rot_y_lr(module_accessor);
								}
							}
						}
						if MotionModule::frame(module_accessor) >= 143.0 {
							JostleModule::set_status(module_accessor, true);
							HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
							*MAJIN_TELEPORT = 0.0;
							globals["teleport_level"] = 0.0.into();
							if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
								StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
							}
						}
					}

					if MotionModule::frame(module_accessor) >= 116.0 && MotionModule::frame(module_accessor) < 149.0 {
						let teleport_length_per_frame = Vector3f{x: 0.5 * *MAJIN_TELEPORT * globals["teleport_level"].get_num(), y: 0.0, z: 0.0};
						PostureModule::add_pos(module_accessor, &teleport_length_per_frame);
						JostleModule::set_status(module_accessor, false);
						HitModule::set_whole(module_accessor, app::HitStatus(*HIT_STATUS_XLU), 0);
					}
				}

				//Fix PJab > L.Jab cancel

				if motion_kind == hash40("attack_11_near_s") || motion_kind == hash40("attack_11_w") {
					if motion_kind == hash40("attack_11_near_s") {
						if MotionModule::frame(module_accessor) > 106.0 && !CancelModule::is_enable_cancel(module_accessor) {
							*PJAB_FRAME = MotionModule::frame(module_accessor) - 100.0;
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

				//Purple Hadoken Effect

				if (*HADOKEN_HIT_EFF).x != 0.0 || (*HADOKEN_HIT_EFF).y != 0.0 || (*HADOKEN_HIT_EFF).z != 0.0 {
					EFFECT(fighter, Hash40::new("ryu_hadoken_hit"), Hash40::new("top"), (*HADOKEN_HIT_EFF).z, (*HADOKEN_HIT_EFF).y, (*HADOKEN_HIT_EFF).x * PostureModule::lr(module_accessor), 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.0, 1.0);
					*HADOKEN_HIT_EFF = Vector3f{x: 0.0, y: 0.0, z: 0.0};
				}


				if !READY_GO[get_player_number(module_accessor)] {
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("gamemodel"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup1"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup2"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup3"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup4"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup5"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_dogiup6"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_eye"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_eye2"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_openblink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_halfblink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_blink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_facen_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_talk_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_heavyattack_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_capture_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_ouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_heavyouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_facen_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_heavyattack_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_escape_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_ouch_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_capture_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_down_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_voicec_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_voiceb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_patternb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ryu_result_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_gamemodel"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup1"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_eye"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_eye2"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_openblink"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_halfblink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_blink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_brow"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_talk_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_capture_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_ouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), true);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_escape_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_capture_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_ouch_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_down_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voiceb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_patternb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_result_mouth"), false);
					*DEMON_GRAB_TARGET = 8;
				}
				READY_GO[get_player_number(module_accessor)] = sv_information::is_ready_go();
			}
			else {
				if !sv_information::is_ready_go() {
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_gamemodel"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup1"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup2"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup3"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup4"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup5"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_dogiup6"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_eye"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_eye2"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_openblink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_halfblink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_blink"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_talk_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_capture_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_ouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyouch_brow"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_facen_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_heavyattack_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_escape_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_capture_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_ouch_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_down_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voicec_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_voiceb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_patternb_mouth"), false);
					ModelModule::set_mesh_visibility(module_accessor, Hash40::new("majin_result_mouth"), false);
				}
				//Giving Ryu his regular cancel frames

				if (motion_kind == hash40("attack_11_s") && MotionModule::frame(module_accessor) >= 32.0)
				|| (motion_kind == hash40("attack_near_w") && MotionModule::frame(module_accessor) >= 29.0)
				|| (motion_kind == hash40("attack_11_near_s") && MotionModule::frame(module_accessor) >= 37.0)
				|| (motion_kind == hash40("attack_s3_s_s") && MotionModule::frame(module_accessor) >= 34.0)
				|| (motion_kind == hash40("attack_lw3_w") && MotionModule::frame(module_accessor) >= 14.0)
				|| (motion_kind == hash40("attack_lw3_s") && MotionModule::frame(module_accessor) >= 28.0)
				|| (motion_kind == hash40("attack_s4_s") && MotionModule::frame(module_accessor) >= 44.0)
				|| (motion_kind == hash40("attack_lw4") && MotionModule::frame(module_accessor) >= 41.0)
				|| (motion_kind == hash40("attack_air_f") && MotionModule::frame(module_accessor) >= 34.0)
				|| (motion_kind == hash40("attack_air_lw") && MotionModule::frame(module_accessor) >= 45.0)
				|| (motion_kind == hash40("special_air_n") && MotionModule::frame(module_accessor) >= 53.0)
				|| (motion_kind == hash40("special_air_n_empty") && MotionModule::frame(module_accessor) >= 29.0)
				|| (motion_kind == hash40("special_lw") && MotionModule::frame(module_accessor) >= 55.0)
				|| (motion_kind == hash40("special_lw_turn") && MotionModule::frame(module_accessor) >= 55.0)
				|| (motion_kind == hash40("special_lw_step_f") && MotionModule::frame(module_accessor) >= 14.0)
				|| (motion_kind == hash40("special_lw_step_b") && MotionModule::frame(module_accessor) >= 14.0)
				|| (motion_kind == hash40("special_air_lw") && MotionModule::frame(module_accessor) >= 55.0)
				|| (motion_kind == hash40("special_air_lw_turn") && MotionModule::frame(module_accessor) >= 55.0)
				|| (motion_kind == hash40("special_air_lw_step_f") && MotionModule::frame(module_accessor) >= 14.0)
				|| (motion_kind == hash40("special_air_lw_step_b") && MotionModule::frame(module_accessor) >= 14.0)
				|| (motion_kind == hash40("final_air2") && MotionModule::frame(module_accessor) >= 118.0)
				|| (motion_kind == hash40("throw_f") && MotionModule::frame(module_accessor) >= 41.0)
				|| (motion_kind == hash40("appeal_s_r") && MotionModule::frame(module_accessor) >= 29.0)
				|| (motion_kind == hash40("appeal_s_l") && MotionModule::frame(module_accessor) >= 29.0) {
					CancelModule::enable_cancel(module_accessor);
				}

				if (motion_kind == hash40("attack_11_s") && MotionModule::frame(module_accessor) >= 52.0)
				|| (motion_kind == hash40("attack_near_w") && MotionModule::frame(module_accessor) >= 39.0)
				|| (motion_kind == hash40("attack_11_near_s") && MotionModule::frame(module_accessor) >= 41.0)
				|| (motion_kind == hash40("attack_s3_s_s") && MotionModule::frame(module_accessor) >= 59.0)
				|| (motion_kind == hash40("attack_lw3_w") && MotionModule::frame(module_accessor) >= 15.0)
				|| (motion_kind == hash40("attack_lw3_s") && MotionModule::frame(module_accessor) >= 30.0)
				|| (motion_kind == hash40("attack_s4_s") && MotionModule::frame(module_accessor) >= 79.0)
				|| (motion_kind == hash40("attack_lw4") && MotionModule::frame(module_accessor) >= 45.0)
				|| (motion_kind == hash40("attack_air_f") && MotionModule::frame(module_accessor) >= 36.0)
				|| (motion_kind == hash40("attack_air_lw") && MotionModule::frame(module_accessor) >= 46.0)
				|| (motion_kind == hash40("special_air_n") && MotionModule::frame(module_accessor) >= 82.0)
				|| (motion_kind == hash40("special_air_n_empty") && MotionModule::frame(module_accessor) >= 82.0)
				|| (motion_kind == hash40("special_lw") && MotionModule::frame(module_accessor) >= 68.0)
				|| (motion_kind == hash40("special_lw_turn") && MotionModule::frame(module_accessor) >= 68.0)
				|| (motion_kind == hash40("special_lw_step_f") && MotionModule::frame(module_accessor) >= 25.0)
				|| (motion_kind == hash40("special_lw_step_b") && MotionModule::frame(module_accessor) >= 25.0)
				|| (motion_kind == hash40("special_air_lw") && MotionModule::frame(module_accessor) >= 68.0)
				|| (motion_kind == hash40("special_air_lw_turn") && MotionModule::frame(module_accessor) >= 68.0)
				|| (motion_kind == hash40("special_air_lw_step_f") && MotionModule::frame(module_accessor) >= 19.0)
				|| (motion_kind == hash40("special_air_lw_step_b") && MotionModule::frame(module_accessor) >= 19.0)
				|| (motion_kind == hash40("final_air2") && MotionModule::frame(module_accessor) >= 118.0)
				|| (motion_kind == hash40("throw_f") && MotionModule::frame(module_accessor) >= 57.0)
				|| (motion_kind == hash40("appeal_s_r") && MotionModule::frame(module_accessor) >= 69.0)
				|| (motion_kind == hash40("appeal_s_l") && MotionModule::frame(module_accessor) >= 69.0) {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					}
				}

				if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
					if MotionModule::frame(module_accessor) >= 49.0 {
						CancelModule::enable_cancel(module_accessor);
					}
				}	

				if (motion_kind == hash40("special_lw_start") || motion_kind == hash40("special_air_lw_start")) && MotionModule::frame(module_accessor) > 57.0 {
					if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) < -0.6 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN, true);
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK, true);
					}
					WorkModule::set_int(module_accessor, *FIGHTER_RYU_SAVING_LV_3, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV);
				}

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

			if motion_kind == hash40("special_n") && MotionModule::frame(module_accessor) >= 53.0 {
				CancelModule::enable_cancel(module_accessor);
			}
			if motion_kind == hash40("special_n") && MotionModule::frame(module_accessor) >= 82.0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			if motion_kind == hash40("attack_hi3_s") && MotionModule::frame(module_accessor) >= 41.0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}

			if (motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_lw4_hold")) && MotionModule::frame(module_accessor) >= 61.0 {
				MotionModule::set_rate(module_accessor, 0.0);
			}
			
			//Runs the same way for Ryu and Majin, so no need to put the newcomer check
			
			let mut frame = MotionModule::frame(module_accessor);
			if is_majin(module_accessor, fighter_kind) && (motion_kind == hash40("attack_11_s") || motion_kind == hash40("attack_11_near_s") || motion_kind == hash40("attack_s3_s_s")) {
				frame -= 100.0;
			}
			if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && frame < 4.0 {
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
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_RYU_HADOKEN )]
fn ryu_hadoken_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let MAJIN_ZANKU = &mut FIGHTER_BOOL_4[get_player_number(owner_module_accessor)];

		if is_majin(module_accessor, weapon_kind) {
			if *MAJIN_ZANKU {
				if motion_kind == hash40("move_w") {
					let offset = Vector3f{ x: 0.0, y: - 1.5, z: 0.0 };
					PostureModule::add_pos(module_accessor, &offset);            
				}
				else if motion_kind == hash40("move_m") {
					let offset = Vector3f{ x: 0.0, y: - 1.2, z: 0.0 };
					PostureModule::add_pos(module_accessor, &offset);            
				}
				else if motion_kind == hash40("move_s") {
					let offset = Vector3f{ x: 0.0, y: - 0.9, z: 0.0 };
					PostureModule::add_pos(module_accessor, &offset);            
				}
			}
		}

		if (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) 
		|| AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD)) && (ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
		|| ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R))  {
			WorkModule::on_flag(owner_module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
	}
}


#[weapon_frame( agent = WEAPON_KIND_RYU_SHINKUHADOKEN )]
fn ryu_shinkuhadoken_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if is_majin(module_accessor, weapon_kind) {
			if motion_kind == hash40("move") {
				let offset = Vector3f{ x: 0.0, y: - 1.2, z: 0.0 };
				PostureModule::add_pos(module_accessor, &offset);            
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		ryu_attack13w,
		ryu_attack11s,
		ryu_attack11s_effect,
		ryu_attack11s_sound,
		ryu_attack11s_expression,
		ryu_attack11nears,
		ryu_attack11nears_effect,
		ryu_attack11nears_sound,
		ryu_attack11nears_expression,
		ryu_attacks3ss,
		ryu_attacks3ss_effect,
		ryu_attacks3ss_sound,
		ryu_attacks3ss_expression,
		ryu_attacks3sw,
		ryu_attacknearw,
		ryu_attacknearw_effect,
		ryu_attacknearw_sound,
		ryu_attacknearw_expression,
		ryu_attackhi3w,
		ryu_attacklw3w,
		ryu_attacklw3w_effect,
		ryu_attacklw3w_sound,
		ryu_attacklw3w_expression,
		ryu_attacklw3s,
		ryu_attacklw3s_effect,
		ryu_attacklw3s_sound,
		ryu_attacklw3s_expression,
		ryu_attacks4s,
		ryu_attacks4s_effect,
		ryu_attacks4s_sound,
		ryu_attacks4s_expression,
		ryu_attacks4charge_effect,
		ryu_attacks4charge_sound,
		ryu_attacks4charge_expression,
		ryu_attackhi4,
		ryu_attacklw4,
		ryu_attacklw4_effect,
		ryu_attacklw4_sound,
		ryu_attacklw4_expression,
		ryu_attacklw4charge_effect,
		ryu_attacklw4charge_sound,
		ryu_attacklw4charge_expression,
		ryu_attackairn,
		ryu_attackairf,
		ryu_attackairf_effect,
		ryu_attackairf_sound,
		ryu_attackairf_expression,
		ryu_attackairlw,
		ryu_attackairlw_effect,
		ryu_attackairlw_sound,
		ryu_attackairlw_expression,
		ryu_specialn_effect,
		ryu_specialairn,
		ryu_specialairn_effect,
		ryu_specialairn_sound,
		ryu_specialairn_expression,
		ryu_specialn2,
		ryu_specialn2_effect,
		ryu_specialn2_sound,
		ryu_specialn2_expression,
		ryu_specialsstart,
		ryu_specials,
		ryu_specialairsstart,
		ryu_specialairs,
		ryu_specialhi,
		ryu_specialhicommand,
		ryu_specialairhi,
		ryu_specialairhicommand,
		ryu_speciallwstepf,
		ryu_speciallwstepf_effect,
		ryu_speciallwstepf_sound,
		ryu_speciallwstepf_expression,
		ryu_speciallwstepb,
		ryu_speciallwstepb_effect,
		ryu_speciallwstepb_sound,
		ryu_speciallwstepb_expression,
		ryu_speciallw,
		ryu_speciallw_effect,
		ryu_speciallw_sound,
		ryu_speciallw_expression,
		ryu_specialairlw,
		ryu_specialairlw_effect,
		ryu_specialairlw_sound,
		ryu_specialairlw_expression,
		ryu_speciallwturn,
		ryu_speciallwturn_effect,
		ryu_speciallwturn_sound,
		ryu_speciallwturn_expression,
		ryu_specialairlwturn,
		ryu_specialairlwturn_effect,
		ryu_specialairlwturn_sound,
		ryu_specialairlwturn_expression,
		ryu_specialairlwstepf,
		ryu_specialairlwstepf_effect,
		ryu_specialairlwstepf_sound,
		ryu_specialairlwstepf_expression,
		ryu_specialairlwstepb,
		ryu_specialairlwstepb_effect,
		ryu_specialairlwstepb_sound,
		ryu_specialairlwstepb_expression,
		ryu_final,
		ryu_final_effect,
		ryu_final_sound,
		ryu_final_expression,
		ryu_finalair,
		ryu_finalair2,
		ryu_finalair2_effect,
		ryu_finalair2_sound,
		ryu_finalair2_expression,
		ryu_shinkuhadoken_move,
		ryu_shinkuhadoken_move_effect,
		ryu_appealsr_sound,
		ryu_appealsl_sound,
		ryu_hadoken_movew,
		ryu_hadoken_movew_effect,
		ryu_hadoken_movem,
		ryu_hadoken_movem_effect,
		ryu_hadoken_moves,
		ryu_hadoken_moves_effect,
		ryu_hadoken_movespw,
		ryu_hadoken_movespm,
		ryu_hadoken_movesps,
		ryu_catch,
		ryu_catchdash,
		ryu_catchturn,
		ryu_throwf,
		ryu_throwf_effect,
		ryu_throwf_sound,
		ryu_throwf_expression
	);
	smashline::install_agent_frames!(ryu_functions);
	smashline::install_agent_frames!(ryu_hadoken_functions, ryu_shinkuhadoken_functions);
}

