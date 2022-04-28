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
use crate::custom::FINAL_HIT;


#[acmd_script( agent = "krool", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn krool_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 70, 0, 40, 10.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 13, 9.5, 0.8, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "krool", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn krool_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 15.5, 361, 76, 0, 36, 6.5, 0.0, 8.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 361, 76, 0, 36, 7.0, 0.0, 6.2, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 15.5, 361, 76, 0, 36, 6.5, 0.0, 11.5, 17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 361, 76, 0, 36, 7.0, 0.0, 10.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 70, 0, 30, 4.7, 0.0, 15.0, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 70, 0, 30, 6.0, 0.0, 11.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "krool", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn krool_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
		HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 14.5, 45, 80, 0, 15, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("armr"), 19.0, 270, 80, 0, 15, 7.4, 7.7, 3.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("armr"), 19.0, 45, 85, 0, 45, 7.4, 7.7, 3.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "krool", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn krool_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "krool", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn krool_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.8, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(13.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "krool", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn krool_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-21.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}    
}

#[acmd_script( agent = "krool", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn krool_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		SLOW_OPPONENT(fighter, 30.0, 60.0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 30, 100, 90, 0, 8.0, 0.0, 6.0, 13.0, Some(0.0), Some(6.0), Some(60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		let vec = Vector2f{ x: 60.0, y: 4.0 };
		AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec, 1, false);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		FINAL_HIT[get_player_number(module_accessor)] = AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT);
	}
}

#[acmd_script( agent = "krool", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn krool_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		SLOW_OPPONENT(fighter, 30.0, 60.0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 41.0);
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 30, 100, 90, 0, 8.0, 0.0, 6.0, 13.0, Some(0.0), Some(6.0), Some(60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		let vec = Vector2f{ x: 60.0, y: 4.0 };
		AttackModule::set_vec_target_pos(module_accessor, 0, Hash40::new("top"), &vec, 1, false);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		FINAL_HIT[get_player_number(module_accessor)] = AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT);
	}
}

#[acmd_script( agent = "krool", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn krool_finaldash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		CAM_ZOOM_OUT(fighter);
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 100, 200, 0, 14.0, 0.0, 12.0, 3.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 100, 200, 0, 14.0, 0.0, 12.0, 3.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
	}
	
}

#[acmd_script( agent = "krool", script = "game_finalairdash", category = ACMD_GAME, low_priority)]
unsafe fn krool_finalairdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		CAM_ZOOM_OUT(fighter);
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 100, 200, 0, 14.0, 0.0, 12.0, 3.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if FINAL_HIT[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 100, 200, 0, 14.0, 0.0, 12.0, 3.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_KROOL )]
pub fn krool_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		
		if fighter_kind == FIGHTER_KIND_KROOL && WorkModule::is_flag(module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_ENABLE_SUPER_ARMOR) && status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
			damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		krool_attackairn,
		krool_attackairf,
		krool_attackairb,
		krool_catch,
		krool_catchdash,
		krool_catchturn,
		krool_finalstart,
		krool_finalairstart,
		krool_finaldash,
		krool_finalairdash
	);
	smashline::install_agent_frames!(krool_functions);
}

