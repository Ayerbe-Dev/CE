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
use crate::custom::FINAL_TRANSFORM;


#[acmd_script( agent = "ness", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn ness_attackairf(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 5.0, 0.0, 4.8, 10.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 1.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 74, 60, 0, 16, 5.0, 0.0, 4.8, 10.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 74, 60, 0, 16, 1.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 128, 0, 32, 8.0, 0.0, 4.8, 9.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "ness", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn ness_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.5, 367, 100, 30, 0, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 2.5, 367, 100, 30, 0, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 6.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
		ATTACK(fighter, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 3.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.18);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "ness", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn ness_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(8.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ness", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn ness_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.2, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(10.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ness", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn ness_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "ness_pkstarstorm", script = "game_move", category = ACMD_GAME, low_priority)]
unsafe fn ness_pkstarstorm_move(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 42, 110, 0, 67, 16.0, 0.0, 12.0, 0.0, Some(0.0), Some(40.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, false, -1.0, false);
	}
	
}

#[acmd_script( agent = "ness", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn ness_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, false, 0);
		let paula_pos = Vector3f{x: PostureModule::pos_x(module_accessor) - 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		let poo_pos = Vector3f{x: PostureModule::pos_x(module_accessor) + 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		ArticleModule::set_pos(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, paula_pos);
		ArticleModule::set_pos(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, poo_pos);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
	}
	frame(lua_state, 51.0);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 299;
		WorkModule::on_flag(module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_FINAL_START);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	frame(lua_state, 350.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	
}

#[acmd_script( agent = "ness", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn ness_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, false, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, false, 0);
		let paula_pos = Vector3f{x: PostureModule::pos_x(module_accessor) - 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		let poo_pos = Vector3f{x: PostureModule::pos_x(module_accessor) + 10.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor) - 10.0};
		ArticleModule::set_pos(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, paula_pos);
		ArticleModule::set_pos(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, poo_pos);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 5.0, 30.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
	}
	frame(lua_state, 51.0);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 299;
		WorkModule::on_flag(module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_FINAL_START);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		CancelModule::enable_cancel(module_accessor);
	}
	frame(lua_state, 350.0);
	if is_excute(fighter) {
		ArticleModule::remove_exist(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PAULA, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		ArticleModule::remove_exist(module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_POO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_NESS )]
pub fn ness_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
            if !WorkModule::is_flag(module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_FINAL_START) {
                WorkModule::on_flag(module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_FINAL_START);
            }
            FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
        }
        else {
            WorkModule::off_flag(module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_FINAL_START);
        }
    }
}

pub fn install() {
	install_acmd_scripts!(
		ness_attackairf,
		ness_attackairhi,
		ness_catch,
		ness_catchdash,
		ness_catchturn,
		ness_final,
		ness_finalair,
		ness_pkstarstorm_move
	);
	smashline::install_agent_frames!(
		ness_functions
	);
}

