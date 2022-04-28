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
use crate::custom::FINAL_TRANSFORM;
use std::mem;


#[acmd_script( agent = "snake", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn snake_attackdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("neck"), 11.0, 45, 45, 0, 95, 4.5, 1.9, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 45, 45, 0, 95, 3.0, 0.0, 5.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.0, 110, 75, 0, 40, 4.5, 1.9, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 110, 75, 0, 40, 3.0, 0.0, 5.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "snake", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn snake_attacks3s(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 4.0, 78, 10, 0, 42, 4.5, 3.2, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
		ATTACK(fighter, 1, 0, Hash40::new("legr"), 4.0, 65, 10, 0, 20, 4.5, 3.2, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	
}

#[acmd_script( agent = "snake", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn snake_attackairlw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		SIZE1[get_player_number(module_accessor)] = 5.5;
		SIZE2[get_player_number(module_accessor)] = 4.5;
		SIZE3[get_player_number(module_accessor)] = 7.0;
		SIZE4[get_player_number(module_accessor)] = 6.0;
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 367, 100, 50, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 367, 100, 35, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 367, 100, 25, 0, SIZE2[get_player_number(module_accessor)], 0.0, 5.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 3.0;
			SIZE2[get_player_number(module_accessor)] += 3.0;
			SIZE3[get_player_number(module_accessor)] += 3.0;
			SIZE4[get_player_number(module_accessor)] += 3.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		let dair_speed1 = Vector3f{ x: 0.0,y: -0.2,z: 0.0  };
		KineticModule::add_speed(module_accessor, &dair_speed1);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 100, 35, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 100, 25, 0, SIZE2[get_player_number(module_accessor)], 0.0, 5.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 3.0;
			SIZE2[get_player_number(module_accessor)] += 3.0;
			SIZE3[get_player_number(module_accessor)] += 3.0;
			SIZE4[get_player_number(module_accessor)] += 3.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		let dair_speed2 = Vector3f{ x: 0.0, y: 0.5, z: 0.0 };
		KineticModule::add_speed(module_accessor, &dair_speed2);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 100, 35, 0, SIZE1[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 100, 25, 0, SIZE2[get_player_number(module_accessor)], 0.0, 5.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 3.0;
			SIZE2[get_player_number(module_accessor)] += 3.0;
			SIZE3[get_player_number(module_accessor)] += 3.0;
			SIZE4[get_player_number(module_accessor)] += 3.0;
		}
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, SIZE3[get_player_number(module_accessor)], 0.0, -1.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, SIZE4[get_player_number(module_accessor)], 0.0, 5.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		let dair_speed3 = Vector3f{ x: 0.0, y: 1.2, z: 0.0  };
		KineticModule::add_speed(module_accessor, &dair_speed3);
	}
	frame(lua_state, 53.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "snake_nikita_missile", script = "game_explosion", category = ACMD_GAME, low_priority)]
unsafe fn snake_nikitamissile_explosion(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 50, 90, 0, 50, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
		QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_erase") }, 0, false, 0 as u32);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_explosion") }, 0, false, 0 as u32);
	}
	
}

#[acmd_script( agent = "snake_nikita_missile", script = "game_hiexplosion", category = ACMD_GAME, low_priority)]
unsafe fn snake_nikitamissile_hiexplosion(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 50, 70, 0, 75, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
		QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_erase") }, 0, false, 0 as u32);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_explosion") }, 0, false, 0 as u32);
	}
	
}

#[acmd_script( agent = "snake_nikita_missile", script = "game_fallexplosion", category = ACMD_GAME, low_priority)]
unsafe fn snake_nikitamissile_fallexplosion(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 50, 90, 0, 50, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
		QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_erase") }, 0, false, 0 as u32);
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		ControlModule::set_rumble(module_accessor, Hash40{hash: hash40("rbkind_explosion") }, 0, false, 0 as u32);
	}
	
}

#[acmd_script( agent = "snake", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn snake_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, 7.0, Some(0.0), Some(8.2), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "snake", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn snake_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, 7.0, Some(0.0), Some(8.2), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	frame(lua_state, 31.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.769);
	}
	
}

#[acmd_script( agent = "snake", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn snake_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.909);
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, -11.0, Some(0.0), Some(8.2), Some(-17.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.69);
	}
	
}

#[acmd_script( agent = "snake", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn snake_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 15, 0, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart02.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_FLARE_GRENADES, false, 0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ArticleModule::shoot(module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_FLARE_GRENADES, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 20, 0);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		SlowModule::clear_whole(module_accessor);
		VisibilityModule::set_whole(module_accessor, false);
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 635;
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
	
}

#[acmd_script( agent = "snake", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn snake_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 15, 0, 0);
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart02.nuanmb"), false);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		ArticleModule::generate_article(module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_FLARE_GRENADES, false, 0);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		ArticleModule::shoot(module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_FLARE_GRENADES, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
	}
	frame(lua_state, 25.0);
	if is_excute(fighter) {
		SlowModule::set_whole(module_accessor, 20, 0);
	}
	frame(lua_state, 26.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
		SlowModule::clear_whole(module_accessor);
		VisibilityModule::set_whole(module_accessor, false);
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 635;
	}
	frame(lua_state, 59.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
pub fn snake_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;

		if fighter_kind == *FIGHTER_KIND_SNAKE {
			if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
				FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
			}

			if FINAL_TRANSFORM[get_player_number(module_accessor)] == 1 {
				VisibilityModule::set_whole(module_accessor, true);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		snake_attackdash,
		snake_attacks3s,
		snake_attackairlw,
		snake_nikitamissile_explosion,
		snake_nikitamissile_hiexplosion,
		snake_nikitamissile_fallexplosion,
		snake_catch,
		snake_catchdash,
		snake_catchturn,
		snake_finalstart,
		snake_finalairstart
	);
	smashline::install_agent_frames!(snake_functions);
}

