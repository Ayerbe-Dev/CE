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


#[acmd_script( agent = "fox", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn fox_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("tail1"), 8.0, 77, 50, 0, 70, 3.2, 1.0, 0.0, 0.0, Some(1.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail2"), 8.0, 75, 50, 0, 70, 2.8, 1.5, 0.0, 0.0, Some(1.5), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail2"), 7.0, 72, 50, 0, 70, 3.0, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 3.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 3.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "fox", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn fox_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 35, 100, 0, 10, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 35, 100, 0, 10, 4.0, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.18);
		ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 35, 110, 0, 0, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 35, 110, 0, 0, 3.5, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "fox", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn fox_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.7;
		SIZE2[get_player_number(module_accessor)] = 6.2;
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("tail1"), 5.0, 92, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 1, 0, Hash40::new("tail2"), 5.0, 92, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 2, 0, Hash40::new("tail3"), 5.0, 92, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 3, 0, Hash40::new("tail1"), 5.0, 367, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 4, 0, Hash40::new("tail2"), 5.0, 367, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		ATTACK(fighter, 5, 0, Hash40::new("tail3"), 5.0, 367, 130, 30, 0, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FOX_TAIL, *ATTACK_REGION_TAIL);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 4.0;
			SIZE2[get_player_number(module_accessor)] += 4.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 85, 108, 0, 30, SIZE1[get_player_number(module_accessor)], 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("footr"), 10.0, 85, 108, 0, 30, SIZE2[get_player_number(module_accessor)], 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 85, 108, 0, 30, SIZE2[get_player_number(module_accessor)], 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "fox", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn fox_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 3.0;
		SIZE2[get_player_number(module_accessor)] = 3.2;
		SIZE3[get_player_number(module_accessor)] = 4.5;
		SIZE4[get_player_number(module_accessor)] = 7.0;
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 5.0);
	for _ in 0..6 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 367, 100, 20, 0, SIZE1[get_player_number(module_accessor)], 0.0, 8.2, -0.5, Some(0.0), Some(9.0), Some(2.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			ATTACK(fighter, 1, 0, Hash40::new("toel"), 1.4, 367, 30, 0, 65, SIZE2[get_player_number(module_accessor)], -0.5, -0.5, 0.0, Some(-0.5), Some(-0.5), Some(0.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 2.0;
				SIZE2[get_player_number(module_accessor)] += 2.0;
				SIZE3[get_player_number(module_accessor)] += 2.0;
				SIZE4[get_player_number(module_accessor)] += 2.0;
			}
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 1.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 140, 0, 70, SIZE3[get_player_number(module_accessor)], 0.0, 9.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 140, 0, 70, SIZE4[get_player_number(module_accessor)], 0.0, 2.8, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "fox", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn fox_catch(fighter: &mut L2CAgentBase) {
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
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "fox", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn fox_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.12);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 2.4, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(11.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "fox", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn fox_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.12);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "fox", script = "game_finallockon", category = ACMD_GAME, low_priority)]
unsafe fn fox_finallockon(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 20, -20);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		SLOW_OPPONENT(fighter, 4.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 2, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 25);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finallockon.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 200, 0, 18.0, 0.0, 14.0, 20.0, Some(0.0), Some(14.0), Some(36.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);

	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_FOX_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
		fighter.clear_lua_stack();
		lua_args!(fighter, 0);
		sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	
}

#[acmd_script( agent = "fox", script = "game_finalairlockon", category = ACMD_GAME, low_priority)]
unsafe fn fox_finalairlockon(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, -5, -5, 20, -20);
		ArticleModule::remove_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, 0);
		SLOW_OPPONENT(fighter, 4.0, 30.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 2, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 25);
		REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finallockon.nuanmb"), false, false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::clear_whole(module_accessor);
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 200, 0, 18.0, 0.0, 14.0, 20.0, Some(0.0), Some(14.0), Some(36.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::on_flag(module_accessor, *FIGHTER_FOX_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
		fighter.clear_lua_stack();
		lua_args!(fighter, 0);
		sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(lua_state);
		fighter.clear_lua_stack();
	}
	frame(lua_state, 34.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
	
}

pub fn install() {
	install_acmd_scripts!(
		fox_attacklw3,
		fox_attackairn,
		fox_attackairhi,
		fox_attackairlw,
		fox_catch,
		fox_catchdash,
		fox_catchturn,
		fox_finallockon,
		fox_finalairlockon
	);
}

