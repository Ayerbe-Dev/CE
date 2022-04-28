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


#[acmd_script( agent = "koopa", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn koopa_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new_raw(0x09aee445d1u64), 7.0, 361, 100, 50, 0, 4.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0, 361, 100, 50, 0, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.0, 130, 100, 30, 0, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new_raw(0x0954eb78b2u64), 8.0, 34, 111, 0, 20, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 34, 111, 0, 20, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.0, 34, 111, 0, 20, 5.6, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "koopa", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn koopa_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
        MotionModule::set_rate(module_accessor, 1.34);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.0);
        CATCH(fighter, 0, Hash40::new("top"), 6.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(17.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA );
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "koopa", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn koopa_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
        MotionModule::set_rate(module_accessor, 1.34);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.0);
        CATCH(fighter, 0, Hash40::new("top"), 4.8, 0.0, 6.0, 2.0, Some(0.0), Some(6.0), Some(16.200001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "koopa", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn koopa_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        GrabModule::set_rebound(module_accessor, true);
        MotionModule::set_rate(module_accessor, 1.34);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor, 1.0);
        CATCH(fighter, 0, Hash40::new("top"), 4.8, 0.0, 6.0, 2.0, Some(0.0), Some(6.0), Some(16.200001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(module_accessor, false);
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_appealhil", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_appealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopa_attack07"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_appealhir", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_appealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopa_attack07"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attack12", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_swing_m"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackairb", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackair_b01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackairf", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackairhi", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackair_h01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackairlw", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackairn", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_n01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackdash", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackdash"));
    }
    wait(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_koopa_landing02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attackhi3", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attacklw3", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attacks3", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01"));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attacks3hi", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01"));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_attacks3lw", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sound = sv_math::rand(hash40("fighter"), 4);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01"));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
        }
        if sound == 1 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        }
        if sound == 2 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
        }
        if sound == 3 {
            PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_batswing4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_batswing4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemheavyget", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemheavyget(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_heavyget"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemheavythrowb4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemheavythrowb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemheavythrowf4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemheavythrowf4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemheavythrowhi4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemheavythrowhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemheavythrowlw4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemheavythrowlw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowairb4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowairb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowairf4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowairf4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowairhi4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowairhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowairlw4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowairlw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowb4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowf4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowf4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowhi4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_itemlightthrowlw4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_itemlightthrowlw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_jumpback", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_jumpback(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
            PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
        }
         else {
            PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
            PLAY_SE(fighter, Hash40::new("se_koopa_jump01"));
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_jumpfront", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_jumpfront(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
            PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
        }
         else {
            PLAY_SE(fighter, Hash40::new("vc_koopa_jump01"));
            PLAY_SE(fighter, Hash40::new("se_koopa_jump01"));
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_lipstickswing4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_lipstickswing4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack05"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_ottotto", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_ottotto(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_ottotto"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_starrodswing4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_starrodswing4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack05"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_throwb", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack04"));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_throwf", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack03"));
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_throwhi", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopa_attack02"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_smash_l01"));
    }
    wait(lua_state, 33.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_common_throw_02"));
        //sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_win1", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_win1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_special_n01_win01"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01"));
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_landing02"));
    }
    frame(lua_state, 115.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
    }
    frame(lua_state, 123.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_win2", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_win2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_step_right_m_win02"));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_step_left_m_win02"));
    }
    frame(lua_state, 49.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_step_right_m_win02"));
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_koopa_step_left_m_win02"));
    }
    frame(lua_state, 84.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_attack01"));
    }
    frame(lua_state, 87.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01_win02"));
    }
    frame(lua_state, 94.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_landing01"));
    }
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_attack07"));
    }
    frame(lua_state, 123.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_s01_win02"));
    }
    
}

#[acmd_script( agent = "koopa", script = "sound_win3", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_sound_win3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_koopa_win01_02"));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n01_win03"));
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_special_n03_win03"));
    }
    frame(lua_state, 99.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_koopa_special_n02_win03"));
    }
    frame(lua_state, 124.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_landing02"));
    }
    
}

#[acmd_script( agent = "koopa_koopag", script = "game_attack", category = ACMD_GAME, low_priority)]
unsafe fn koopa_koopag_attack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 40.0, 361, 40, 0, 80, 11.0, 0.0, 6.0, 66.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_no_squat_damage_reaction_mul(module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        WorkModule::on_flag(module_accessor, *WEAPON_KOOPA_KOOPAG_INSTANCE_WORK_ID_FLAG_REQUEST_RUMBLE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
pub fn koopa_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;

		if fighter_kind == FIGHTER_KIND_KOOPA {
			if motion_kind == hash40("special_s_catch") || motion_kind == hash40("special_air_s_catch") {
				if MotionModule::frame(module_accessor) < 6.0 {
					MotionModule::set_rate(module_accessor, 0.75);
				}
				else {
					MotionModule::set_rate(module_accessor, 1.0);
				}
			}
		}
	}
}

pub fn install() {
    install_acmd_scripts!(
        koopa_attacklw3,
        koopa_catch,
        koopa_catchdash,
        koopa_catchturn,
        koopa_koopag_attack,
        koopa_sound_appealhil,
        koopa_sound_appealhir,
        koopa_sound_attack12,
        koopa_sound_attackairb,
        koopa_sound_attackairf,
        koopa_sound_attackairhi,
        koopa_sound_attackairlw,
        koopa_sound_attackairn,
        koopa_sound_attackdash,
        koopa_sound_attackhi3,
        koopa_sound_attacklw3,
        koopa_sound_attacks3,
        koopa_sound_attacks3hi,
        koopa_sound_attacks3lw,
        koopa_sound_batswing4,
        koopa_sound_itemheavyget,
        koopa_sound_itemheavythrowb4,
        koopa_sound_itemheavythrowf4,
        koopa_sound_itemheavythrowhi4,
        koopa_sound_itemheavythrowlw4,
        koopa_sound_itemlightthrowairb4,
        koopa_sound_itemlightthrowairf4,
        koopa_sound_itemlightthrowairhi4,
        koopa_sound_itemlightthrowairlw4,
        koopa_sound_itemlightthrowb4,
        koopa_sound_itemlightthrowf4,
        koopa_sound_itemlightthrowhi4,
        koopa_sound_itemlightthrowlw4,
        koopa_sound_jumpback,
        koopa_sound_jumpfront,
        koopa_sound_lipstickswing4,
        koopa_sound_ottotto,
        koopa_sound_starrodswing4,
        koopa_sound_throwb,
        koopa_sound_throwf,
        koopa_sound_throwhi,
        koopa_sound_win1,
        koopa_sound_win2,
        koopa_sound_win3,
    );
    smashline::install_agent_frames!(koopa_functions);
}

