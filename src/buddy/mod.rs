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


#[acmd_script( agent = "buddy", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn buddy_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
		HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
		HIT_NO(fighter, 14, *HIT_STATUS_NORMAL);
		HIT_NO(fighter, 15, *HIT_STATUS_NORMAL);
		HIT_NO(fighter, 16, *HIT_STATUS_NORMAL);
		HIT_NO(fighter, 17, *HIT_STATUS_NORMAL);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 4.0, 3.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 10.0, 86, 119, 0, 42, 4.4, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footr"), 10.0, 86, 119, 0, 42, 4.6, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("legl"), 10.0, 86, 119, 0, 42, 4.4, 2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		HIT_NO(fighter, 12, *HIT_STATUS_OFF);
		HIT_NO(fighter, 13, *HIT_STATUS_OFF);
		HIT_NO(fighter, 14, *HIT_STATUS_OFF);
		HIT_NO(fighter, 15, *HIT_STATUS_OFF);
		HIT_NO(fighter, 16, *HIT_STATUS_OFF);
		HIT_NO(fighter, 17, *HIT_STATUS_OFF);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn buddy_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.1);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	for _ in 0..7 {
		if is_excute(fighter) {
			MotionModule::set_rate(module_accessor, 1.0);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 0.8, 48, 100, 60, 0, 4.8, 0.0, 4.2, -4.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 0.8, 335, 100, 40, 0, 4.8, 0.0, 14.6, -4.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 0.8, 140, 100, 40, 0, 4.8, 0.0, 14.6, 5.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 0.8, 130, 100, 75, 0, 4.8, 0.0, 4.2, 5.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 0.8, 367, 100, 100, 0, 11.8, 0.0, 8.6, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 1.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 1, Hash40::new("top"), 4.2, 48, 68, 0, 84, 11.8, 0.0, 8.6, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
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

#[acmd_script( agent = "buddy", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn buddy_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 15.0, 44, 86, 0, 36, 4.2, 1.4, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 15.0, 44, 86, 0, 36, 4.8, 4.8, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 12.0, 44, 84, 0, 34, 4.2, 1.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 44, 84, 0, 34, 4.8, 4.8, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
		ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn buddy_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 5.8;
		SIZE2[get_player_number(module_accessor)] = 6.4;
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 100, 30, 0, 80, 4.4, 0.0, 12.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 130, 30, 0, 80, 4.8, 0.0, 11.8, 9.4, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 50, 30, 0, 80, 4.8, 0.0, 11.8, -9.4, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 100, 30, 0, 80, 4.4, 0.0, 14.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 178, 30, 0, 60, 4.8, 0.0, 18.6, 8.8, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 2, 30, 0, 60, 4.8, 0.0, 18.6, -8.8, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		ATTACK(fighter, 0, 1, Hash40::new("top"), 5.8, 76, 94, 0, 42, SIZE1[get_player_number(module_accessor)], 0.0, 13.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 1, Hash40::new("top"), 5.8, 76, 94, 0, 42, SIZE2[get_player_number(module_accessor)], 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn buddy_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.2, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn buddy_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn buddy_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.2, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-15.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn buddy_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_SET_FINAL_SMASH_LIGHT(lua_state);
		fighter.clear_lua_stack();
		ArticleModule::generate_article(module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_BIGBIRD, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_BIGBIRDBASE, false, 0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 15, -10);
		SLOW_OPPONENT(fighter, 15.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 75);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 4, 0);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 13.0, 0.0, 9.0, 26.0, Some(0.0), Some(18.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.5), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 13.0, 0.0, 9.0, 26.0, Some(0.0), Some(30.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(10.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(35.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(35.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(20.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(40.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(45.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(45.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(55.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 120.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	
}

#[acmd_script( agent = "buddy", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn buddy_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_SET_FINAL_SMASH_LIGHT(lua_state);
		fighter.clear_lua_stack();
		ArticleModule::generate_article(module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_BIGBIRD, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_BIGBIRDBASE, false, 0);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 15, -10);
		SLOW_OPPONENT(fighter, 15.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 75);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04final.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		SlowModule::set_whole(module_accessor, 4, 0);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 30.0);
	if is_excute(fighter) {
		QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 13.0, 0.0, 9.0, 26.0, Some(0.0), Some(18.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.5), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 13.0, 0.0, 9.0, 26.0, Some(0.0), Some(30.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(10.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 33.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(35.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(15.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(35.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(20.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(40.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(45.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(45.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 100, 100, 0, 15.0, 0.0, 11.0, 26.0, Some(0.0), Some(55.0), Some(26.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 120.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	
}

pub fn install() {
	install_acmd_scripts!(
		buddy_attackhi3,
		buddy_attackairn,
		buddy_attackairf,
		buddy_attackairhi,
		buddy_catch,
		buddy_catchdash,
		buddy_catchturn,
		buddy_final,
		buddy_finalair
	);
}

