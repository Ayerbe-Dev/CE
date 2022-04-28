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


#[acmd_script( agent = "lucario", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn lucario_attack11(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 2.0);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 20, 0, 35, 2.5, 0.0, 9.0, 3.8, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 361, 20, 0, 25, 2.2, -1.5, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 2, 0, Hash40::new("arml"), 2.5, 180, 20, 0, 15, 2.2, 2.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
		ATTACK(fighter, 3, 0, Hash40::new("arml"), 2.5, 361, 20, 0, 15, 2.2, 2.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 2, 6.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 3, 6.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.34);
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	frame(lua_state, 29.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	
}

#[acmd_script( agent = "lucario", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn lucario_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.67);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 110, 0, 37, 4.0, 0.0, 2.8, 9.0, Some(0.0), Some(3.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "lucario", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn lucario_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "lucario", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn lucario_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "lucario", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn lucario_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-16.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "lucario_auraball", script = "game_charge", category = ACMD_GAME, low_priority)]
unsafe fn lucario_auraball_charge(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 83, 80, 0, 10, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	
}

#[acmd_script( agent = "lucario_auraball", script = "game_chargemax", category = ACMD_GAME, low_priority)]
unsafe fn lucario_auraball_chargemax(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 83, 80, 0, 10, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	
}

#[acmd_script( agent = "lucario", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn lucario_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
//		SLOW_OPPONENT(fighter, 9.0, 130.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 6);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		VisibilityModule::set_whole(module_accessor, false);
		ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		if DamageModule::damage(module_accessor, 0) < 70.0 {
			FINAL_TRANSFORM[get_player_number(module_accessor)] = 900;
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
	}
}

#[acmd_script( agent = "lucario", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn lucario_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
//		SLOW_OPPONENT(fighter, 9.0, 130.0);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 6);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		VisibilityModule::set_whole(module_accessor, false);
		ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		if DamageModule::damage(module_accessor, 0) < 70.0 {
			FINAL_TRANSFORM[get_player_number(module_accessor)] = 900;
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
		}
	}
}

#[acmd_script( agent = "lucario_lucariom", script = "game_finaljump", category = ACMD_GAME, low_priority)]
unsafe fn lucario_lucariom_finaljump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 40, 40, 0, 65, 7.0, 0.0, 3.0, 0.0, Some(0.0), Some(12.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
    }
    
}

#[acmd_script( agent = "lucario_lucariom", script = "game_finalattack", category = ACMD_GAME, low_priority)]
unsafe fn lucario_lucariom_finalattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("rot"), 0.7, 366, 100, 10, 0, 13.0, 0.0, -14.0, 0.0, Some(0.0), Some(-14.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 1, Hash40::new("rot"), 0.7, 90, 100, 7, 0, 13.0, 0.0, -650.0, 0.0, Some(0.0), Some(-10.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 1, Hash40::new("rot"), 0.7, 90, 70, 4, 0, 13.0, 0.0, -650.0, 0.0, Some(0.0), Some(-10.0), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 2, true, true, -1.0, false);
        ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_explosionl"), 10, false, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_loopattack"), 0, true, 0);
    }
    frame(lua_state, 180.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 189.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 190.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("rot"), 10.0, 60, 120, 0, 90, 17.0, 0.0, -650.0, 0.0, Some(0.0), Some(-10.0), Some(0.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_explosionl"), 0, false, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
pub fn lucario_functions (fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if fighter_kind == FIGHTER_KIND_LUCARIO {
			if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
				if FINAL_TRANSFORM[get_player_number(module_accessor)] != 1 {
					AttackModule::set_power_mul(module_accessor, 1.67 / WorkModule::get_float(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_PREV_AURAPOWER));
				}
				else {
					AttackModule::set_power_mul(module_accessor, 1.0);
				}
				FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
			}

			if motion_kind == hash40("appeal_hi_l") || motion_kind == hash40("appeal_hi_r") {
				if MotionModule::frame(module_accessor) == 0.0 {
					EFFECT(fighter,/*Effect*/ Hash40::new("lucario_aura"), /*Bone*/ Hash40::new("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
				}
				if MotionModule::frame(module_accessor) == 21.0 
				|| MotionModule::frame(module_accessor) == 26.0 
				|| MotionModule::frame(module_accessor) == 31.0 
				|| MotionModule::frame(module_accessor) == 36.0
				|| MotionModule::frame(module_accessor) == 41.0 {
					DamageModule::add_damage(module_accessor, 1.0, 0);
				}
				if MotionModule::frame(module_accessor) == 50.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
						MotionModule::set_frame(module_accessor, 24.0, false);
						EFFECT(fighter, /*Effect*/ Hash40::new("lucario_aura"), /*Bone*/ Hash40::new("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, 0, 0, 0, 0, 0, 0, true);
					}
				}
			}
			else {
				EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("lucario_aura")}, false, false);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		lucario_attack11,
		lucario_attacklw3,
		lucario_catch,
		lucario_catchdash,
		lucario_catchturn,
		lucario_auraball_charge,
		lucario_auraball_chargemax,
		lucario_finalstart,
		lucario_finalairstart,
		lucario_lucariom_finaljump,
		lucario_lucariom_finalattack
	);
	smashline::install_agent_frames!(lucario_functions);
}

