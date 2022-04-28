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
use crate::custom::DUCKHUNT_POS;
use crate::custom::DUCKHUNT_OFFSET;


#[acmd_script( agent = "duckhunt", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_attackhi3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	FT_MOTION_RATE(fighter, 0.8);
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 93, 130, 0, 35, 7.0, 0.0, 13.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	FT_MOTION_RATE(fighter, 1);
	wait(lua_state, 5.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "duckhunt", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, 6.0, 0.0, 18.5, 5.0, Some(0.0), Some(18.5), Some(-2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, 6.0, 0.0, 21.5, 4.0, Some(0.0), Some(21.5), Some(-5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, 6.0, 0.0, 23.5, 4.0, Some(0.0), Some(23.5), Some(-5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 91, 120, 0, 50, 8.5, 0.0, 22.5, -3.0, Some(0.0), Some(22.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 91, 120, 0, 50, 8.5, 0.0, 24.5, -3.0, Some(0.0), Some(24.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
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

#[acmd_script( agent = "duckhunt", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 130, 20, 0, 20, 4.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 255, 20, 0, 20, 5.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 367, 20, 0, 20, 4.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 367, 20, 0, 20, 5.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 10, 7.5, 0.0, -8.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 90, 0, 10, 7.5, 0.0, -8.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "duckhunt_clay", script = "game_hit", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_clay_hit(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 60, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.3, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 100, 100, 20, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.3, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 100, 0, 30, 3.0, 0.0, 9.0, -6.0, Some(0.0), Some(-9.0), Some(6.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 100, 0, 30, 3.0, 0.0, 0.5, -11.0, Some(0.0), Some(-1.1), Some(11.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 60, 100, 0, 30, 3.0, 0.0, -8.0, -8.0, Some(0.0), Some(8.0), Some(8.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	
}

#[acmd_script( agent = "duckhunt", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "duckhunt", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(12.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "duckhunt", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-18.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "duckhunt", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_finalstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 4.0, 30.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_FINALDUCK, false, 0);
        SlowModule::set_whole(module_accessor, 10, 0);
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
        SlowModule::set_whole(module_accessor, 2, 0);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -10.0 * PostureModule::lr(module_accessor), 0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 30, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 100, 200, 0, 17.0, 0.0, 12.0, -10.0, Some(0.0), Some(12.0), Some(28.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn duckhunt_finalairstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 4.0, 30.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_FINALDUCK, false, 0);
        SlowModule::set_whole(module_accessor, 10, 0);
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA_arg3(fighter, Hash40::new("d04finalstart.nuanmb"), false, false);
        FT_START_CUTIN(fighter);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
        SlowModule::set_whole(module_accessor, 2, 0);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -10.0 * PostureModule::lr(module_accessor), 0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        SlowModule::set_whole(module_accessor, 30, 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 100, 200, 0, 17.0, 0.0, 12.0, -10.0, Some(0.0), Some(12.0), Some(28.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(module_accessor);
        CAM_ZOOM_OUT(fighter);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_DUCKHUNT )]
pub fn duckhunt_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		
		if fighter_kind == FIGHTER_KIND_DUCKHUNT {
			DUCKHUNT_POS[get_player_number(module_accessor)] = Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)};
			if status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_CLOSING_EAR {
				CancelModule::enable_cancel(module_accessor);
			}
			if motion_kind == hash40("attack_s4_s") {
				if MotionModule::frame(module_accessor) == 11.0 {
					WorkModule::set_int(module_accessor, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
					WorkModule::set_int(module_accessor, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
					ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 100, 66, 0, 4.7, 0.0, 6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
				}
				if MotionModule::frame(module_accessor) == 17.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 45, 100, 70, 0, 6.9, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 6.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 45, 100, 70, 0, 6.9, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
				}
				if MotionModule::frame(module_accessor) == 23.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 125, 0, 50, 7.8, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 6.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 125, 0, 50, 7.8, 0.0, 6.0, 26.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
				}
			}
			if motion_kind == hash40("attack_hi4") {
				if MotionModule::frame(module_accessor) == 5.0 {
					WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
				}
				if MotionModule::frame(module_accessor) == 6.0 {
					WorkModule::set_int(module_accessor, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
					WorkModule::set_int(module_accessor, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
					ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 367, 100, 0, 0, 5.3, 0.0, 11.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					let hitVec = Vector2f{x: -7.0, y: 10.0};
					AttackModule::set_vec_target_pos(module_accessor, 0, Hash40{hash: hash40("top")}, &hitVec, 10.0 as u32, false);
				}
				if MotionModule::frame(module_accessor) == 14.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 367, 100, 0, 0, 8.3, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 12.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z - 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 367, 100, 0, 0, 6.3, 0.0, 15.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
					let hitVec = Vector2f{x: 0.0, y: 20.0};
					AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{hash: hash40("top")}, &hitVec, 8.0 as u32, false);
				}
				if MotionModule::frame(module_accessor) == 22.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 85, 120, 0, 45, 9.8, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 10.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 85, 120, 0, 45, 7.8, 0.0, 21.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
				}				
			}
			if motion_kind == hash40("attack_lw4") {
				if MotionModule::frame(module_accessor) == 2.0 {
					WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
				}
				if MotionModule::frame(module_accessor) == 6.0 {
					WorkModule::set_int(module_accessor, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
					WorkModule::set_int(module_accessor, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
					ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 0, 0, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					let hitVec = Vector2f{x: -9.0, y: 2.0};
					AttackModule::set_vec_target_pos(module_accessor, 0, Hash40{hash: hash40("top")}, &hitVec, 9.0 as u32, false);
				}
				if MotionModule::frame(module_accessor) == 14.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 367, 100, 0, 0, 6.1, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 5.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z - 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 367, 100, 0, 0, 6.1, 0.0, 5.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
					}
					let hitVec = Vector2f{x: 12.0, y: 3.0};
					AttackModule::set_vec_target_pos(module_accessor, 1, Hash40{hash: hash40("top")}, &hitVec, 9.0 as u32, false);
				}
				if MotionModule::frame(module_accessor) == 22.0 {
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && DUCKHUNT_OFFSET[get_player_number(module_accessor)].x != 8.0 {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 150, 155, 0, 37, 7.8, 0.0, DUCKHUNT_OFFSET[get_player_number(module_accessor)].y + 7.0 - PostureModule::pos_y(module_accessor), DUCKHUNT_OFFSET[get_player_number(module_accessor)].z + 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
					if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
						ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 150, 155, 0, 37, 7.8, 0.0, 7.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
					}
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		duckhunt_attackhi3,
		duckhunt_attackairhi,
		duckhunt_attackairlw,
		duckhunt_clay_hit,
		duckhunt_catch,
		duckhunt_catchdash,
		duckhunt_catchturn,
		duckhunt_finalstart,
		duckhunt_finalairstart
	);
	smashline::install_agent_frames!(duckhunt_functions);
}

