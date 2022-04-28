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


#[acmd_script( agent = "jack", script = "game_speciallw", category = ACMD_GAME, low_priority)]
unsafe fn jack_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
	}
	
}

#[acmd_script( agent = "jack", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn jack_specialairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
		shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
	}
	frame(lua_state, 32.0);
	if is_excute(fighter) {
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
		shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
	}
	
}

#[acmd_script( agent = "jack", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn jack_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.1, 4.5, Some(0.0), Some(8.1), Some(9.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "jack", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn jack_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "jack", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn jack_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -7.0, Some(0.0), Some(6.6), Some(-16.700001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
}

#[acmd_script( agent = "jack", script = "game_finaldash", category = ACMD_GAME, low_priority)]
unsafe fn jack_finaldash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 7.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_final"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    
}

#[acmd_script( agent = "jack", script = "game_finalairdash", category = ACMD_GAME, low_priority)]
unsafe fn jack_finalairdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 7.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_final"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    
}

#[acmd_script( agent = "jack", script = "game_finalturn", category = ACMD_GAME, low_priority)]
unsafe fn jack_finalturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 7.0, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_final"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    
}

#[acmd_script( agent = "jack", script = "game_finalairturn", category = ACMD_GAME, low_priority)]
unsafe fn jack_finalairturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 80, 0, 7.0, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_final"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, false);
    }
    
}



#[fighter_frame( agent = FIGHTER_KIND_JACK )]
pub fn jack_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();

		if fighter_kind == FIGHTER_KIND_JACK {
			if let L2CValueType::Void = globals["lose_meter_ok"].val_type {
				globals["lose_meter_ok"] = false.into();
			}
			if let L2CValueType::Void = globals["joker_last_frame"].val_type {
				globals["joker_last_frame"] = 0.0.into();
			}
			if let L2CValueType::Void = globals["meter_loss"].val_type {
				globals["meter_loss"] = 0.0.into();
			}

			if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE && WorkModule::is_flag(module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_BARRAGE_LW) == false {
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					CancelModule::enable_cancel(module_accessor);
				}
			}
			if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD {
				globals["lose_meter_ok"] = true.into();
				if WorkModule::is_flag(module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_LW_FLAG_ENDURE) {
					globals["joker_last_frame"] = 0.0.into();
				}
				else {
					globals["joker_last_frame"] = (MotionModule::frame(module_accessor)).into();
				}
			}
			if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ATTACK {
				globals["joker_last_frame"] = 0.0.into();
			}
			if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END {
				if globals["lose_meter_ok"].get_bool() {
					globals["meter_loss"] = (globals["joker_last_frame"].get_num() / 2.0).into();
					FighterSpecializer_Jack::add_rebel_gauge(module_accessor, app::FighterEntryID (WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)), globals["meter_loss"].get_num() * -1.0);
					globals["lose_meter_ok"] = false.into();
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		jack_speciallw,
		jack_specialairlw,
		jack_catch,
		jack_catchdash,
		jack_catchturn,
		jack_finaldash,
		jack_finalturn,
		jack_finalairdash,
		jack_finalairturn
	);
	smashline::install_agent_frames!(jack_functions);
}

