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
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use crate::custom::FIGHTER_BOOL_4;
use crate::custom::FIGHTER_BOOL_5;
use crate::custom::FIGHTER_VEC2F_1;
use crate::custom::jump_checker_buffer;
use crate::custom::estimate_frame;


#[acmd_script( agent = "pzenigame", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 15, 0, 30, 4.2, 0.0, 5.0, -5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 15, 0, 20, 4.2, 0.0, 5.0, -9.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 65, 15, 0, 60, 4.2, 0.0, 5.0, -5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 65, 15, 0, 60, 4.2, 0.0, 5.0, -9.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 40, 95, 0, 55, 5.8, 0.0, 5.0, -5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 40, 95, 0, 55, 6.0, 0.0, 5.0, -9.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 98, 100, 10, 0, 4.2, 0.0, 1.0, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 98, 100, 10, 0, 4.4, 0.0, -1.5, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 98, 100, 10, 0, 4.6, 0.0, -4.0, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 10, 0, 4.2, 0.0, 1.0, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 4, 0, Hash40::new("top"), 1.5, 367, 100, 10, 0, 4.4, 0.0, -1.5, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 5, 0, Hash40::new("top"), 1.5, 367, 100, 10, 0, 4.6, 0.0, -4.0, 0.0, None, None, None, 0.8, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 15.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 35, 85, 0, 50, 5.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 35, 85, 0, 50, 6.0, 0.0, -4.5, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.0, 4.0, Some(0.0), Some(6.0), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 8.0, 96, 130, 0, 50, 8.0, 0.0, 13.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 96, 130, 0, 50, 3.0, 0.0, 20.0, 3.0, Some(0.0), Some(27.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 2, 1, Hash40::new("top"), 8.0, 96, 130, 0, 50, 4.0, 0.0, 30.0, 7.0, Some(0.0), Some(30.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
		ATTACK(fighter, 3, 1, Hash40::new("top"), 8.0, 96, 130, 0, 50, 4.0, 0.0, 30.0, -1.0, Some(0.0), Some(30.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	wait(lua_state, 7.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 60, 3.5, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(13.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 361, 92, 0, 35, 4.6, 7.5, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 92, 0, 35, 3.8, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.0, 361, 92, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("arml"), 13.0, 34, 97, 0, 32, 4.8, 7.5, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 5.4, 0.0, 8.0, 7.5, Some(0.0), Some(8.0), Some(18.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.0, 5.0, Some(0.0), Some(6.0), Some(18.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 5.4, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-17.200001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 40, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
	}
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -60.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	frame(lua_state, 58.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 95.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 130.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 167.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 205.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_PLIZARDON_STATUS_FINAL_FLAG_LAST_DAIMONJI);
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "plizardon", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 40, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
	}
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -60.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	frame(lua_state, 58.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 95.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 130.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 167.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 205.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_PLIZARDON_STATUS_FINAL_FLAG_LAST_DAIMONJI);
		ArticleModule::generate_article(module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_DAIMONJI, false, 0);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "plizardon_daimonji", script = "game_move", category = ACMD_GAME, low_priority)]
unsafe fn plizardon_daimonji_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if WorkModule::is_flag(module_accessor, *WEAPON_PLIZARDON_DAIMONJI_INSTANCE_WORK_ID_FLAG_LAST) {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 20, 100, 60, 0, 15.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
			AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		}
	}
	 else {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 20, 100, 50, 0, 8.0, 0.0, 7.0, 5.5, Some(0.0), Some(7.0), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 100, 60, 0, 8.5, 0.0, -8.0, 5.5, Some(0.0), Some(-8.0), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
			AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
			AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		}
	}
	
}

#[acmd_script( agent = "pfushigisou", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn pfushigisou_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
		VisibilityModule::set_int64(module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 40, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -50.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 100, 45, 5, 6.5, 0.0, 10.0, 200.0, Some(0.0), Some(10.0), Some(200.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 10, 100, 45, 5, 6.5, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(200.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
	}
	frame(lua_state, 230.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 231.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 128, 0, 80, 9.0, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(200.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	frame(lua_state, 240.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "pfushigisou", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn pfushigisou_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
		VisibilityModule::set_int64(module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 40, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -50.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 100, 45, 5, 6.5, 0.0, 10.0, 200.0, Some(0.0), Some(10.0), Some(200.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 10, 100, 45, 5, 6.5, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(200.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
	}
	frame(lua_state, 230.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 231.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 128, 0, 80, 9.0, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(200.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	frame(lua_state, 240.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 35, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -40.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 30, 100, 20, 0, 5.0, 0.0, 0.0, 25.0, Some(0.0), Some(-8.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 330, 100, 20, 0, 5.0, 0.0, 10.0, 25.0, Some(0.0), Some(17.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 2, true, true, -1.0, false);
	}
	frame(lua_state, 66.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 30, 100, 20, 0, 5.0, 0.0, 0.0, 25.0, Some(0.0), Some(-18.0), Some(60.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 330, 100, 20, 0, 5.0, 0.0, 10.0, 25.0, Some(0.0), Some(26.0), Some(60.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 2, true, true, -1.0, false);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 110, 100, 80, 0, 10.0, 0.0, -19.0, 63.0, Some(0.0), Some(-22.0), Some(70.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 250, 100, 80, 0, 10.0, 0.0, 28.0, 63.0, Some(0.0), Some(31.0), Some(70.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 3, true, true, -1.0, false);
	}
	frame(lua_state, 76.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 110, 100, 80, 0, 10.0, 0.0, -19.0, 63.0, Some(0.0), Some(-30.0), Some(90.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 250, 100, 80, 0, 10.0, 0.0, 28.0, 63.0, Some(0.0), Some(40.0), Some(90.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 3, true, true, -1.0, false);
	}
	frame(lua_state, 240.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[acmd_script( agent = "pzenigame", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn pzenigame_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_START_CUTIN_arg1(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 35, 100, 100, 0, 20.0, 0.0, 10.0, 0.0, None, None, None, 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -40.0 * PostureModule::lr(module_accessor), 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 60, -56, 18, -8);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 30, 100, 20, 0, 5.0, 0.0, 0.0, 25.0, Some(0.0), Some(-8.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 330, 100, 20, 0, 5.0, 0.0, 10.0, 25.0, Some(0.0), Some(17.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 2, true, true, -1.0, false);
	}
	frame(lua_state, 66.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 30, 100, 20, 0, 5.0, 0.0, 0.0, 25.0, Some(0.0), Some(-18.0), Some(60.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 330, 100, 20, 0, 5.0, 0.0, 10.0, 25.0, Some(0.0), Some(26.0), Some(60.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 7, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 2, true, true, -1.0, false);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 110, 100, 80, 0, 10.0, 0.0, -19.0, 63.0, Some(0.0), Some(-22.0), Some(70.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 250, 100, 80, 0, 10.0, 0.0, 28.0, 63.0, Some(0.0), Some(31.0), Some(70.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 3, true, true, -1.0, false);
	}
	frame(lua_state, 76.0);
	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 110, 100, 80, 0, 10.0, 0.0, -19.0, 63.0, Some(0.0), Some(-30.0), Some(90.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 250, 100, 80, 0, 10.0, 0.0, 28.0, 63.0, Some(0.0), Some(40.0), Some(90.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 20, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
		AttackModule::set_no_uniq_effect_all(module_accessor, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 3, true, true, -1.0, false);
	}
	frame(lua_state, 240.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 317.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_PZENIGAME )]
pub fn pzenigame_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();
		let SWITCH_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let SHELL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
		let PT_SPECIAL = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_PZENIGAME {
			if let L2CValueType::Void = globals["shell_smash"].val_type {
				globals["shell_smash"] = false.into();
			}

			if situation_kind == *SITUATION_KIND_AIR && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) && *SWITCH_TIMER == 0 {
				WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
			}

			if status_kind == *FIGHTER_STATUS_KIND_DAMAGE
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
				if globals["shell_smash"].get_bool() {
					globals["shell_smash"] = false.into();
				}
			}
			if status_kind == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false {
				*SHELL_TIMER = 0;
			}
			if *SHELL_TIMER > 0 {
				*SHELL_TIMER -= 1;
			}
			if *SHELL_TIMER == 1 {
				EFFECT(/*Effect*/ fighter, Hash40::new("sys_flash"), /*Bone*/ Hash40::new("bust"), /*X*/ 0, /*Y*/ 0, /*Z*/ 0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
				LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.0);	
			}
			if *PT_SPECIAL && status_kind != *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT, true);
			}
			if status_kind == FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT && *PT_SPECIAL {
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("item_gekikara_wait")}, 0.0, 1.0, false, 0.0, false, false);
				globals["shell_smash"] = true.into();
				*PT_SPECIAL = false;	
			}
			if motion_kind == hash40("item_gekikara_wait") && globals["shell_smash"].get_bool() {
				*SHELL_TIMER = 900;
				MotionModule::set_rate(module_accessor, 0.25);
				if MotionModule::frame(module_accessor) >= 2.5 && MotionModule::frame(module_accessor) < 2.7 {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 361, 68, 0, 50, 19.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
					EFFECT(/*Effect*/ fighter, Hash40::new("pzenigame_takinobori_end"), /*Bone*/ Hash40::new("bust"), /*X*/ 0, /*Y*/ 0, /*Z*/ 0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.3, 0, 0, 0, 0, 0, 0, true);
					PLAY_SE(fighter, Hash40::new_raw(0x16119b0cbfu64));
					LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.0);
				}
				if MotionModule::frame(module_accessor) >= 2.7 && MotionModule::frame(module_accessor) < 4.0 {
					EFFECT(/*Effect*/ fighter, Hash40::new("pzenigame_takinobori_end"), /*Bone*/ Hash40::new("bust"), /*X*/ 0, /*Y*/ 0, /*Z*/ 0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.3, 0, 0, 0, 0, 0, 0, true);
					LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.0);
				}
				if MotionModule::frame(module_accessor) >= 2.7 {
					AttackModule::clear_all(module_accessor);
				}
				if MotionModule::frame(module_accessor) >= 10.0 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					globals["shell_smash"] = false.into();
				}
			}
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_PFUSHIGISOU )]
pub fn pfushigisou_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();
		let SWITCH_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let IVY_POS = &mut FIGHTER_VEC2F_1[get_player_number(module_accessor)];
		let PT_SPECIAL = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_PFUSHIGISOU {
			if let L2CValueType::Void = globals["solar_beam_frame"].val_type {
				globals["solar_beam_frame"] = 0.0.into();
			}
			if let L2CValueType::Void = globals["solar_frame"].val_type {
				globals["solar_frame"] = 0.0.into();
			}
			if let L2CValueType::Void = globals["solar_hit"].val_type {
				globals["solar_hit"] = false.into();
			}
			if let L2CValueType::Void = globals["solar_beam_charge"].val_type {
				globals["solar_beam_charge"] = false.into();
			}
			if let L2CValueType::Void = globals["solar_beam"].val_type {
				globals["solar_beam"] = false.into();
			}

			if situation_kind == *SITUATION_KIND_AIR && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) && *SWITCH_TIMER == 0 {
				WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
			}

			if sv_information::is_ready_go() == false || status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
				globals["solar_beam_frame"] = 0.0.into();
			}
			if globals["solar_beam_frame"].get_num() == 300.0 {
				if EffectModule::is_exist_common(module_accessor, Hash40{hash: hash40("pfushigisou_tanemg")}) == false {
					EFFECT(/*Effect*/ fighter, Hash40::new("pfushigisou_tanemg"), /*Bone*/ Hash40::new("flower"), /*X*/ 0, /*Y*/ 0, /*Z*/ 0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
					LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.0);	
				}
			}
			if motion_kind != hash40("special_s") && motion_kind != hash40("appeal_s_l") && motion_kind != hash40("appeal_s_r") {
				globals["solar_beam"] = false.into();
				globals["solar_beam_charge"] = false.into();
				globals["solar_hit"] = false.into();
			}
			if motion_kind == hash40("appeal_s_r") || motion_kind == hash40("appeal_s_l") {
				globals["solar_frame"] = MotionModule::frame(module_accessor).into();
			}
			else if motion_kind != hash40("special_s") {
				globals["solar_frame"] = 0.0.into();
			}
			if *PT_SPECIAL && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
			}
			if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S && *PT_SPECIAL {
				if globals["solar_beam_frame"].get_num() < 300.0 {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("appeal_s_r")}, 0.0, 1.0, false, 0.0, false, false);
					globals["solar_beam_charge"] = true.into();
				}
				else {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("appeal_s_l")}, 0.0, 3.0, false, 0.0, false, false);
					globals["solar_beam_frame"] = 0.0.into();
					globals["solar_beam"] = true.into();
				}
				*PT_SPECIAL = false;
			}
			if status_kind == *FIGHTER_STATUS_KIND_DAMAGE
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
				if globals["solar_beam_charge"].get_bool() {
					globals["solar_beam_frame"] = 0.0.into();
				}
			}
			if motion_kind == hash40("appeal_s_r") && globals["solar_beam_charge"].get_bool() {
				MotionModule::set_rate(module_accessor, globals["solar_beam_frame"].get_num() / 100.0);
				let mulSpeed = Vector3f{x: 1.0, y: 0.0, z: 1.0};
				if MotionModule::frame(module_accessor) == 0.0 {
					KineticModule::mul_speed(module_accessor, &mulSpeed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
				}
				if MotionModule::frame(module_accessor) >= 52.0 {
					MotionModule::set_frame(module_accessor, 14.0, true);
				}
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
						globals["solar_beam_charge"] = false.into();
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
						globals["solar_beam_charge"] = false.into();
					}
				}
				if jump_checker_buffer(module_accessor, cat) {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						globals["solar_beam_charge"] = false.into();
					}
					else {
						if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
							globals["solar_beam_charge"] = false.into();
						}
					}
				}
				if globals["solar_beam_frame"].get_num() >= 300.0 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					EFFECT(/*Effect*/ fighter, Hash40::new("sys_flash"), /*Bone*/ Hash40::new("bust"), /*X*/ 0, /*Y*/ 0, /*Z*/ 0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
					LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.0);	
					globals["solar_beam_charge"] = false.into();
				}
				else {
					globals["solar_beam_frame"] = (globals["solar_beam_frame"].get_num() + 1.0).into();
				}
			}
			if motion_kind == hash40("appeal_s_l") && globals["solar_beam"].get_bool() {
				if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
					if MotionModule::frame(module_accessor) >= 230.0 && MotionModule::frame(module_accessor) < 234.0 {
						globals["solar_hit"] = true.into();
					}
				}
				if MotionModule::frame(module_accessor) < 59.0 {
					*IVY_POS = Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)};
				}
				else if MotionModule::frame(module_accessor) < 280.0 {
					PostureModule::set_pos_2d(module_accessor, &*IVY_POS);
				}
				
				if MotionModule::frame(module_accessor) >= 59.0 && MotionModule::frame(module_accessor) < 63.0 {
					EFFECT(/*Effect*/ fighter, Hash40::new("finptrainer_solar_beam"), /*Bone*/ Hash40::new("top"), /*X*/ 0, /*Y*/ 8.5, /*Z*/ 16.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
					LAST_EFFECT_SET_RATE(fighter, /*Rate*/ 1.5);
					ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 10, 5, 6.5, 0.0, 10.0, 150.0, Some(0.0), Some(10.0), Some(170.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
					ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 0, 100, 10, 5, 6.5, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(170.0), 0.9, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
				}
				if MotionModule::frame(module_accessor) >= 230.0 && MotionModule::frame(module_accessor) < 234.0 {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 128, 0, 80, 9.0, 0.0, 10.0, 15.0, Some(0.0), Some(10.0), Some(170.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
					AttackModule::clear(module_accessor, 1, false);
				}
				if MotionModule::frame(module_accessor) >= 280.0 {
					AttackModule::clear_all(module_accessor);
				}
				if MotionModule::frame(module_accessor) >= 316.0 {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					}
					globals["solar_beam"] = false.into();
				}
			}
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_PLIZARDON )]
pub fn plizardon_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let SWITCH_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let ROCK_SMASH = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let PT_SPECIAL = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_PLIZARDON {
			if situation_kind == *SITUATION_KIND_AIR && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) && *SWITCH_TIMER == 0 {
				WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
			}

			if *PT_SPECIAL && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			}
			if motion_kind != hash40("special_air_lw_rock") && motion_kind != hash40("special_lw_rock") && *ROCK_SMASH {
				*ROCK_SMASH = false;
			}
			if status_kind == FIGHTER_STATUS_KIND_SPECIAL_N && *PT_SPECIAL {
				if situation_kind == *SITUATION_KIND_GROUND {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("special_lw_rock")}, 0.0, 1.0, false, 0.0, false, false);
				}
				else {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("special_air_lw_rock")}, 0.0, 1.0, false, 0.0, false, false);
				}
				*ROCK_SMASH = true;
				*PT_SPECIAL = false;
			}
			if motion_kind == hash40("special_air_lw_rock") || motion_kind == hash40("special_lw_rock") {
//				let mulSpeed = Vector3f{x: 1.0, y: 0.0, z: 1.0};
//				let mulAccel = Vector3f{x: 1.0, y: 0.5, z: 1.0};
//				let conAccel = Vector3f{x: 0.0, y: 1.0, z: 1.0};
				if MotionModule::frame(module_accessor) == 0.0 {
//					KineticModule::mul_speed(module_accessor, &mulSpeed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
					KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
				}
//				KineticModule::mul_accel(module_accessor, &mulAccel, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//				KineticModule::mul_speed(module_accessor, &conAccel, *FIGHTER_KINETIC_ENERGY_ID_NONE);
				if MotionModule::frame(module_accessor) == 5.0 {
					damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
				}
				if MotionModule::frame(module_accessor) == 24.0 {
					damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);					
					ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 80, 0, 60, 7.8, 0.0, 9.0, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
				}
				if estimate_frame(module_accessor, 26.0) {
					AttackModule::clear_all(module_accessor);
				}				
				if MotionModule::frame(module_accessor) >= 27.0 && MotionModule::frame(module_accessor) < 31.0 {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 50, 0, 60, 15.6, 0.0, 9.0, 11.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
				}
				if MotionModule::frame(module_accessor) >= 31.0 {
					AttackModule::clear_all(module_accessor);
				}
				if MotionModule::frame(module_accessor) >= 59.0 {
					CancelModule::enable_cancel(module_accessor);
				}
				if MotionModule::frame(module_accessor) >= 61.0 {
					if situation_kind == SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
					}
					*ROCK_SMASH = false;
				}
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_PTRAINER_PTRAINER )]
fn ptrainer_ptrainer_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let battle_object = app::sv_system::battle_object(fighter.lua_state_agent);
		let _instance = mem::transmute::<&mut app::BattleObject, &mut app::Weapon>(battle_object);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let SWITCH_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		let PT_RIGHT = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let PT_LEFT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		let PT_SWITCH = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];

		if weapon_kind == WEAPON_KIND_PTRAINER_PTRAINER {
			if *SWITCH_TIMER > 0 {
				*SWITCH_TIMER -= 1;
			}
			if *PT_LEFT {
				if status_kind != *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW {
					StatusModule::change_status_request_from_script(module_accessor, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW, true);
				}
				else {
					if MotionModule::frame(module_accessor) >= 25.0 {
						StatusModule::change_status_request_from_script(module_accessor, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW, true);
						*PT_LEFT = false;
					}
				}
			}
			else if *PT_RIGHT {
				if status_kind != *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW {
					StatusModule::change_status_request_from_script(module_accessor, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW, true);
				}
				*PT_RIGHT = false;
			}
			else if status_kind == *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW && ((*PT_SWITCH && MotionModule::frame(module_accessor) >= 26.0) || *PT_SWITCH == false) {
				StatusModule::change_status_request_from_script(module_accessor, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_WAIT, true);
				*PT_SWITCH = false;
				*SWITCH_TIMER = 140;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		pzenigame_attackairb,
		pzenigame_attackairlw,
		pzenigame_catch,
		pzenigame_catchdash,
		pzenigame_catchturn,
		plizardon_attackhi3,
		plizardon_attacklw3,
		plizardon_attackairf,
		plizardon_catch,
		plizardon_catchdash,
		plizardon_catchturn,
		plizardon_final,
		plizardon_finalair,
		plizardon_daimonji_move,
		pfushigisou_final,
		pfushigisou_finalair,
		pzenigame_final,
		pzenigame_finalair,
	);
	smashline::install_agent_frames!(
		pzenigame_functions,
		pfushigisou_functions,
		plizardon_functions,
	);
	smashline::install_agent_frames!(ptrainer_ptrainer_functions);
}

