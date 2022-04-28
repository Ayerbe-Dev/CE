use smash::app::{self, sv_module_access::*, lua_bind::*, sv_math, smashball, *};
use smash::lib::{lua_const::{self, *}, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
use smash::lua2cpp::{self, *};
use smash::{hash40, phx::*};
use smashline::*;
use smash_script::{macros::*, *};
use smash::crc32::*;
use std::mem;
use std::string::String;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::ITEM_MANAGER_ADDR;
use crate::*;
use skyline::nn::ro::LookupSymbol;
use skyline::patching::patch_str;

/*
Generic variables are generally preferred for the sake of saving on memory. If a variable is specifically named, it's either because it's used by multiple fighters for the exact
same purpose, or because it's too specialized to work as a generic variable.

Each variable's reason for being specific will be noted.
*/

//PARRY VARIABLES - Used by all fighters

pub static mut BASE_SHIELD_DAMAGE: [f32;9] = [0.0;9]; //Helps calculate parry stun frames
pub static mut PARRIED_COUNT: [i32;9] = [0;9]; //Tracks how many times an attacker's move has been parried within that status
pub static mut ATTACKER_OVER_DEFENDER: [bool;9] = [false;9]; //Tracks who in the parry has a higher player number, prevents frame advantage from being dependent on port priority

//C-STICK DRIFT PREVENTION VARIABLES - Used by all fighters

pub static mut SUB_STICK: [Vector2f;9] = [Vector2f{x:0.0, y: 0.0};9]; //Tracks a player's C-Stick value (no, the game does not track this value on its own)
pub static mut AERIAL_KIND: [i32;9] = [0;9]; //Tracks a player's C-Stick aerial, used for correcting the return value of ControlModule::get_attack_air_kind
pub static mut ATTACK_CANCELED: [bool;9] = [false;9]; //Tracks whether or not a player entered the jump squat status through an Attack Cancel

//JAB CANCEL VARIABLES - Used by all fighters

pub static mut IN_JAB: [bool;9] = [false;9]; //Determines if the attacker connected with their cancelable Jab
pub static mut IN_HITSTUN: [bool;9] = [false;9]; //Determines if the defender should be tracking when hitstun ends
pub static mut JAB_CANCEL_OK: [bool;9] = [false;9]; //Determines if the defender has left hitstun after being hit by a cancelable Jab
pub static mut MOTION_FRAME: [f32;9] = [0.0;9];  //Determines what frame the attacker will be allowed to cancel their Jab on
pub static mut FRAME_CHECK: [bool;9] = [false;9]; //Determines if the attacker should skip a frame due to port priority

//GRAB VARIABLES - Used by all fighters against a specific fighter, but distinguishing between different potential attackers isn't worth the effort

pub static mut GIGA_GRAB: [i32;9] = [0;9]; //Tracks what phase of a grab or what throw Giga Bowser is in
pub static mut GIGA_GRABBED: [i32;9] = [0;9]; //Tracks remaining frames before the opponent is grab released
pub static mut MAJIN_GRAB: [i32;9] = [0;9]; //Tracks what phase of the throw Majin is in
pub static mut MAJIN_OFFSET: [Vector3f;9] = [Vector3f{x:0.0, y: 0.0, z: 0.0};9]; //Tracks what position Majin is in

//CLIP VARIABLES - Used by all fighters

pub static mut GET_CLIPPED_NERD: [bool;9] = [false;9];
pub static mut TALKIN_MAD_SHIT: [i32;9] = [0;9];
pub static mut SUCCESSFUL_CLIP: [i32;9] = [0;9];
pub static mut NERD_GOT_CLIPPED: [Vector4f;9] = [Vector4f{x: 0.0, y: 0.0, z: 0.0, w: 0.0};9];

//HITBOX VARIABLES - Used by all fighters

pub static mut SIZE1: [f32;9] = [0.0;9];
pub static mut SIZE2: [f32;9] = [0.0;9];
pub static mut SIZE3: [f32;9] = [0.0;9];
pub static mut SIZE4: [f32;9] = [0.0;9];
pub static mut SIZE5: [f32;9] = [0.0;9];
pub static mut SIZE6: [f32;9] = [0.0;9];
pub static mut SIZE7: [f32;9] = [0.0;9];
pub static mut SIZE8: [f32;9] = [0.0;9];
pub static mut SIZE9: [f32;9] = [0.0;9];

//VISION VARIABLES - Used by all fighters only against Shulk, too specific to be used anywhere else

pub static mut MONADO_POS: [[Vector2f;9];120] = [[Vector2f{x: 0.0, y: 0.0};9];120];
pub static mut MONADO_MOTION: [[u64;9];120] = [[0;9];120];
pub static mut MONADO_STATUS: [[i32;9];120] = [[0;9];120];
pub static mut MONADO_SITUATION: [[i32;9];120] = [[0;9];120];
pub static mut MONADO_FRAME: [[Vector3f;9];120] = [[Vector3f{x: 0.0, y: 0.0, z: 0.0};9];120];
pub static mut MONADO_LR: [[Vector2f;9];120] = [[Vector2f{x: 0.0, y: 0.0};9];120];
pub static mut MONADO_TIMER: [usize;9] = [0;9];
pub static mut REWOUND: [i32;9] = [-1;9];
pub static mut REWIND_ENDING: [bool;9] = [false;9];

//DATA VARIABLES - Keeps track of various game data, all variables are either used by all fighters or keep track of game elements that aren't character specific

pub static mut FIGHTER_NAME: [u64;9] = [0;9]; //Tracks the name underneath the fighter portrait corresponding to a fighter, used to check if certain newcomers were selected
pub static mut MODEL_DATA_POS: [u64;9] = [0;9]; //Tracks the location of the model_data for each player, used in the level 2 set_mesh_visibility hook to determine who's asking
pub static mut ALL_FIGHTERS_LAST_STOCK: bool = false; //Take a wild guess.
pub static mut TOTAL_FIGHTER: i32 = 1; //Tracks how many fighters are present
pub static mut INKLING_PRESENT: bool = false; //Tracks whether or not Inkling is in a match, used to avoid some unnecessary checks in the float param hook
pub static mut METER_ENABLED: bool = false; //Tracks if FS Meter is enabled
pub static mut READY_GO: [bool;9] = [false;9]; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub static mut LAST_DAMAGE: [f32;9] = [0.0;9]; //Tracks whatever your last damage value was
pub static mut TRAINING_SPAWN: i32 = 3;
pub static mut CPU_STATE: i32 = 0;
pub static mut INPUT_TIMER: i32 = 0;

//SPECIAL SMASH VARIABLES - Aren't tied to any one fighter

pub static mut SPECIAL_SMASH_SIZE: i32 = 0; //Checks which mode was selected in the "Size" slot
pub static mut SPECIAL_SMASH_HEAD: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_BODY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_STATUS: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_GRAVITY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_RATE: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_CAMERA: i32 = 0; //Etc.
pub static mut READY_GO_TIMER: i32 = 0; //Determines how many frames to suspend all players while respawning in Tennis and One-Hit modes
pub static mut SPAWN_POS: [Vector3f;9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0};9]; //Tracks what position to spawn the nets in in Basketball mode, and where to respawn players in Volleyball and One-Hit mode
pub static mut TEMP_SPAWN_POS: [Vector3f;9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0};9]; //Used to randomize spawn pos in Volleyball mode
pub static mut SPAWN_SIDE: [bool;9] = [false;9]; //Tracks if a player's spawn position was on the right or left

//ONE-HIT VARIABLES

pub static mut GOT_HIT: [i32;9] = [0;9]; //Tracks if a player got hit during One-Hit mode
pub static mut HIT_PLAYER: i32 = -1; //Tracks which players need to be respawned

//BASKETBALL VARIABLES

pub static mut HIGH_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the right net
pub static mut LOW_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the left net

//VOLLEYBALL VARIABLES

pub static mut BALL_BOUNCED: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 9999.0}; //Tracks stats about the volleyball to determine who to KO
pub static mut ALREADY_BOUNCED: bool = false; //Tracks if the ball has bounced at least once since being thrown
pub static mut FIRST_BOUNCE: bool = false; //Allows the throwing player to bounce the ball on their own side once
pub static mut BALL_OWNER: i32 = 9; //Which player will start with the ball
pub static mut LAST_TO_HIT_BALL: usize = 9; //The last player to have hit the ball
pub static mut BALL_ID: u32 = 0; //The battle object ID of the ball itself
pub static mut BALL_VICTIMS: [i32;4] = [9;4]; //Which players are to be KOd

//FINAL SMASH VARIABLES - Used by all fighters

pub static mut DRAIN_FULL_METER: [bool;9] = [false;9]; //Flags the meter to be reset back to 0
pub static mut USED_FS: [bool;9] = [false;9]; //Flags when you just used a Final Smash in Special Smash
pub static mut SUB_METER: [f32;9] = [0.0;9]; //Adds or subtracts from the meter value
pub static mut CAN_DRAIN_METER_KEPT: [i32;9] = [0;9]; //Frame timer, once it hits 0 the amount of meter kept on KO will decrease
pub static mut METER_TIMER: [i32;9] = [0;9]; //Frame timer, every time it hits 60 the amount of meter kept goes down by 2
pub static mut REMAINING_METER: [i32;9] = [0;9]; //The amount of meter kept
pub static mut FINAL_HIT: [bool;9] = [false;9]; //Tracks whether or not a fighter has connected with their FS
pub static mut FINAL_WAIT: [bool;9] = [false;9]; //Used by certain fighters to put a delay between using the FS and it actually activating
pub static mut FINAL_TRANSFORM: [i32;9] = [0;9]; //Determines how long before a transformation FS ends
pub static mut FS_METER: [f32;9] = [0.0;9]; //Tracks meter
pub static mut FS_METER_2: [f32;9] = [0.0;9]; //Tracks meter for Ivysaur
pub static mut FS_METER_3: [f32;9] = [0.0;9]; //Tracks meter for Charizard
pub static mut START_FS: [bool;9] = [false;9]; //Used for canceling taunt startup into FS

//MISC VARIABLES

pub static mut COUNTERHIT_CHECK: [bool;9] = [false;9]; //Tracks if a fighter is in the startup of a normal attack; Used by Little Mac and Incineroar but is held by the defender
pub static mut COUNTERHIT_SUCCESS: [bool;9] = [false;9]; //Tracks if a fighter has landed a successful counterhit, MIGHT be specific to Incineroar after Mac's changes, I'll check later
pub static mut EXTRA_LAG: [i32;9] = [0;9]; //Tracks if a fighter has either whiffed an aerial or thrown an item while in the air, determines extra landing lag
pub static mut B_CHECK: [bool;9] = [false;9]; //Tracks if a fighter used a certain special move in the air
pub static mut INKED: [i32;9] = [0;9]; //Tracks whether or not the opponent is standing in ink, and if they are, whether or not the ink is friendly
pub static mut ROLLER_POS: [[Vector3f;9];10] = [[Vector3f{x: 0.0, y: 0.0, z: 0.0};9];10]; 
pub static mut BULLET_POS: [[Vector4f;9];8] = [[Vector4f{x:0.0, y:0.0, z: 0.0, w: 0.0};9];8];
pub static mut DUCKHUNT_POS: [Vector2f;9] = [Vector2f{x:0.0, y:0.0};9];
pub static mut DUCKHUNT_OFFSET: [Vector3f;9] = [Vector3f{x:0.0, y:0.0, z: 0.0};9];
pub static mut GIGA_POS: [Vector2f;9] = [Vector2f{x:0.0, y:0.0};9];
pub static mut GIGA_OFFSET: [Vector3f;9] = [Vector3f{x:0.0, y:0.0, z: 0.0};9];
pub static mut DEMON_DEATH_TIMER: [i32;9] = [0;9];
pub static mut LUIGI_FINAL_HIT: [bool;9] = [false;9];
pub static mut MAJIN_DEMON_TARGET: [usize;9] = [8;9];
pub static mut GIGA_GRAB_TARGET: [usize;9] = [8;9];
pub static mut STOCK_COUNT: [u64;9] = [3;9];

//Fighter-Specific Variables

pub static mut FIGHTER_INT_1: [i32;9] = [0;9];
pub static mut FIGHTER_INT_2: [i32;9] = [0;9];
pub static mut FIGHTER_INT_3: [i32;9] = [0;9];
pub static mut FIGHTER_INT_4: [i32;9] = [0;9];
pub static mut FIGHTER_INT_5: [i32;9] = [0;9];
pub static mut FIGHTER_INT_6: [i32;9] = [0;9];
pub static mut FIGHTER_INT_7: [i32;9] = [0;9];
pub static mut FIGHTER_INT_8: [i32;9] = [0;9];
pub static mut FIGHTER_INT_9: [i32;9] = [0;9];
pub static mut FIGHTER_INT_10: [i32;9] = [0;9];
pub static mut FIGHTER_INT_11: [i32;9] = [0;9];

pub static mut FIGHTER_FLOAT_1: [f32;9] = [0.0;9];
pub static mut FIGHTER_FLOAT_2: [f32;9] = [0.0;9];
pub static mut FIGHTER_FLOAT_3: [f32;9] = [0.0;9];

pub static mut FIGHTER_U64_1: [u64;9] = [0;9];

pub static mut FIGHTER_U32_1: [u32;9] = [0;9];

pub static mut FIGHTER_U8_1: [u8;9] = [0;9];
pub static mut FIGHTER_U8_2: [u8;9] = [0;9];
pub static mut FIGHTER_U8_3: [u8;9] = [0;9];
pub static mut FIGHTER_U8_4: [u8;9] = [0;9];

pub static mut FIGHTER_BOOL_1: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_2: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_3: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_4: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_5: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_6: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_7: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_8: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_9: [bool;9] = [false;9];

pub static mut FIGHTER_VEC2F_1: [Vector2f;9] = [Vector2f{x:0.0, y:0.0};9]; 
pub static mut FIGHTER_VEC3F_1: [Vector3f;9] = [Vector3f{x:0.0, y:0.0, z: 0.0};9];

pub static mut FRAME_COUNTER: i32 = 0;

#[allow(unused_unsafe)]

pub unsafe fn read_tag(addr: u64) -> String {
	let mut s: Vec<u8> = vec![];

	let mut addr = addr as *const u16;
	loop {
		if *addr == 0_u16 {
			break;
		}
		s.push(*(addr as *const u8));
		addr = addr.offset(1);
	}
	// No null terminator needed

	std::str::from_utf8(&s).unwrap().to_owned()
}

pub unsafe fn get_boma(entry_id: i32) -> *mut app::BattleObjectModuleAccessor {
	let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

pub unsafe fn get_player_number(module_accessor:  &mut app::BattleObjectModuleAccessor) -> usize {
	let mut player_number = 8;
	if app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
		player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
	}
	else if app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	}
	else {
		let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		while app::utility::get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
			owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		}
		player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	}
	return player_number;
}

pub unsafe fn get_attacker_number(module_accessor: &mut app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

pub unsafe fn estimate_frame(module_accessor: &mut app::BattleObjectModuleAccessor, frame: f32) -> bool {
	if MotionModule::frame(module_accessor) >= frame && MotionModule::frame(module_accessor) < frame + 1.0 {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn get_meter_gain(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> f32 {
	if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
		return 0.0;
	}
	if fighter_kind == *FIGHTER_KIND_BRAVE {
		return 0.1;
	}
	else if fighter_kind == *FIGHTER_KIND_KEN {
		if ALL_FIGHTERS_LAST_STOCK {
			return 0.04;
		}
		else {
			return 0.0;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_PEACH
	|| fighter_kind == *FIGHTER_KIND_FALCO
	|| fighter_kind == *FIGHTER_KIND_GANON
	|| fighter_kind == *FIGHTER_KIND_CHROM
	|| fighter_kind == *FIGHTER_KIND_LITTLEMAC
	|| fighter_kind == *FIGHTER_KIND_GAOGAEN {
		return 0.03;
	}
	else if fighter_kind == *FIGHTER_KIND_LINK
	|| fighter_kind == *FIGHTER_KIND_LUIGI
	|| fighter_kind == *FIGHTER_KIND_PURIN
	|| fighter_kind == *FIGHTER_KIND_POPO
	|| fighter_kind == *FIGHTER_KIND_ZELDA
	|| fighter_kind == *FIGHTER_KIND_MEWTWO
	|| fighter_kind == *FIGHTER_KIND_CHROM
	|| fighter_kind == *FIGHTER_KIND_PIT
	|| fighter_kind == *FIGHTER_KIND_PITB
	|| fighter_kind == *FIGHTER_KIND_SNAKE
	|| fighter_kind == *FIGHTER_KIND_PIKMIN
	|| fighter_kind == *FIGHTER_KIND_MURABITO
	|| fighter_kind == *FIGHTER_KIND_ROCKMAN
	|| fighter_kind == *FIGHTER_KIND_REFLET
	|| fighter_kind == *FIGHTER_KIND_MIIGUNNER
	|| fighter_kind == *FIGHTER_KIND_SIMON
	|| fighter_kind == *FIGHTER_KIND_SHIZUE
	|| fighter_kind == *FIGHTER_KIND_PACKUN
	|| fighter_kind == *FIGHTER_KIND_PICKEL
	|| fighter_kind == *FIGHTER_KIND_EDGE {
		return 0.02; 
	}
	else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
		if FS_METER_2[get_player_number(module_accessor)] < 68.0 {
			FS_METER_2[get_player_number(module_accessor)] += 0.02;
			return 0.02;
		}
		else {
			return 0.0;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_SAMUSD
	|| fighter_kind == *FIGHTER_KIND_SAMUSD {
		return 0.01;
	}
	else {
		return 0.0;
	}
}

pub unsafe fn get_meter_gain_attack(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> f32 {
	if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
		return 0.0;
	}
	if fighter_kind == *FIGHTER_KIND_MARIO 
	|| fighter_kind == *FIGHTER_KIND_YOSHI
	|| fighter_kind == *FIGHTER_KIND_FOX 
	|| fighter_kind == *FIGHTER_KIND_KIRBY
	|| fighter_kind == *FIGHTER_KIND_PIKACHU
	|| fighter_kind == *FIGHTER_KIND_NESS
	|| fighter_kind == *FIGHTER_KIND_CAPTAIN
	|| fighter_kind == *FIGHTER_KIND_ZELDA
	|| fighter_kind == *FIGHTER_KIND_PICHU
	|| fighter_kind == *FIGHTER_KIND_MARTH
	|| fighter_kind == *FIGHTER_KIND_LUCINA
	|| fighter_kind == *FIGHTER_KIND_YOUNGLINK
	|| fighter_kind == *FIGHTER_KIND_ROY
	|| fighter_kind == *FIGHTER_KIND_GAMEWATCH
	|| fighter_kind == *FIGHTER_KIND_SZEROSUIT
	|| fighter_kind == *FIGHTER_KIND_SONIC
	|| fighter_kind == *FIGHTER_KIND_DIDDY
	|| fighter_kind == *FIGHTER_KIND_LUCAS
	|| fighter_kind == *FIGHTER_KIND_LUCARIO
	|| fighter_kind == *FIGHTER_KIND_ROBOT
	|| fighter_kind == *FIGHTER_KIND_TOONLINK
	|| fighter_kind == *FIGHTER_KIND_WOLF
	|| fighter_kind == *FIGHTER_KIND_ROSETTA
	|| fighter_kind == *FIGHTER_KIND_GEKKOUGA
	|| fighter_kind == *FIGHTER_KIND_MIIFIGHTER
	|| fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
	|| fighter_kind == *FIGHTER_KIND_PALUTENA
	|| fighter_kind == *FIGHTER_KIND_SHULK
	|| fighter_kind == *FIGHTER_KIND_KAMUI
	|| fighter_kind == *FIGHTER_KIND_BAYONETTA
	|| fighter_kind == *FIGHTER_KIND_INKLING
	|| fighter_kind == *FIGHTER_KIND_RICHTER
	|| fighter_kind == *FIGHTER_KIND_BUDDY
	|| fighter_kind == *FIGHTER_KIND_MASTER
	|| fighter_kind == *FIGHTER_KIND_EFLAME
	|| fighter_kind == *FIGHTER_KIND_ELIGHT
	|| fighter_kind == *FIGHTER_KIND_DEMON
	|| fighter_kind == *FIGHTER_KIND_TRAIL {
		return 0.915;
	}
	else if fighter_kind == *FIGHTER_KIND_PZENIGAME {
		if FS_METER[get_player_number(module_accessor)] < 68.0 {
			return 0.915;
		}
		else {
			return 0.0;
		}
	}
	else if is_majin(module_accessor, fighter_kind) {
		if StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
			return 0.0;
		}
		else {
			return 0.915;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_RYU
	|| fighter_kind == *FIGHTER_KIND_PACMAN {
		if ALL_FIGHTERS_LAST_STOCK || fighter_kind != *FIGHTER_KIND_RYU {
			return 2.6;
		}
		else {
			return 0.0;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_METAKNIGHT
	|| fighter_kind == *FIGHTER_KIND_CLOUD
	|| fighter_kind == *FIGHTER_KIND_JACK
	|| fighter_kind == *FIGHTER_KIND_DOLLY {
		if !WorkModule::is_flag(module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) || fighter_kind != *FIGHTER_KIND_JACK {
			return 1.2;
		}
		else {
			return 0.0;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_SAMUS
	|| fighter_kind == *FIGHTER_KIND_SHEIK {
		return 0.68;
	}
	else {
		return 0.0;
	}
}

pub unsafe fn get_meter_gain_damage(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> f32 {
	if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
		return 0.0;
	}
	if fighter_kind != *FIGHTER_KIND_PLIZARDON && fighter_kind != *FIGHTER_KIND_PFUSHIGISOU && fighter_kind != *FIGHTER_KIND_PZENIGAME && FS_METER_3[get_player_number(module_accessor)] < 200.0 {
		let taken_damage = DamageModule::damage(module_accessor, 0) - LAST_DAMAGE[get_player_number(module_accessor)];
		FS_METER_3[get_player_number(module_accessor)] += taken_damage * 1.5;
	}

	if fighter_kind == *FIGHTER_KIND_DONKEY
	|| fighter_kind == *FIGHTER_KIND_KOOPA
	|| fighter_kind == *FIGHTER_KIND_MARIOD
//	|| fighter_kind == *FIGHTER_KIND_KOOPAG
	|| fighter_kind == *FIGHTER_KIND_IKE
	|| fighter_kind == *FIGHTER_KIND_DEDEDE
	|| fighter_kind == *FIGHTER_KIND_WIIFIT
	|| fighter_kind == *FIGHTER_KIND_KOOPAJR
	|| fighter_kind == *FIGHTER_KIND_RIDLEY
	|| fighter_kind == *FIGHTER_KIND_KROOL
	|| fighter_kind == *FIGHTER_KIND_TANTAN {
		return 1.5;
	}
	else if fighter_kind == *FIGHTER_KIND_PLIZARDON {
		if FS_METER_3[get_player_number(module_accessor)] < 68.0 {
			let taken_damage = DamageModule::damage(module_accessor, 0) - LAST_DAMAGE[get_player_number(module_accessor)];
			FS_METER_3[get_player_number(module_accessor)] += taken_damage * 1.5;
			return 1.5;
		}
		else {
			return 0.0;
		}
	}
	else if fighter_kind == *FIGHTER_KIND_WIIFIT
	|| fighter_kind == *FIGHTER_KIND_MARIOD {
		return 1.7;
	}
	else if fighter_kind == *FIGHTER_KIND_WARIO {
		return 0.75;
	}
	else if fighter_kind == *FIGHTER_KIND_FALCO
	|| fighter_kind == *FIGHTER_KIND_GANON
	|| fighter_kind == *FIGHTER_KIND_CHROM
	|| fighter_kind == *FIGHTER_KIND_LITTLEMAC
	|| fighter_kind == *FIGHTER_KIND_GAOGAEN {
		return -0.25;
	}
	else {
		return 0.0;
	}
}

pub unsafe fn change_aegis_without_special_lw(fighter: &mut L2CFighterCommon) {
	let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let frame = MotionModule::frame(module_accessor);
	let SWITCH_MOTION = &mut FIGHTER_U64_1[get_player_number(module_accessor)];
	let SWITCH_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];

	if fighter_kind == *FIGHTER_KIND_EFLAME {
		EFFECT_FOLLOW(fighter, Hash40::new("eflame_change_start"), Hash40::new("top"), 0.0, 10.0, 0.0, 0, 0, 0, 1.3, true);
		EffectModule::detach_kind(module_accessor, Hash40::new("eflame_change_start"), -1);
	}
	else {
		EFFECT_FOLLOW(fighter, Hash40::new("elight_change_start"), Hash40::new("top"), 0.0, 10.0, 0.0, 0, 0, 0, 1.3, true);
		EffectModule::detach_kind(module_accessor, Hash40::new("elight_change_start"), -1);
	}

	*SWITCH_MOTION = motion_kind;
	*SWITCH_FRAME = frame + 8.0;
	WorkModule::on_flag(module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_GENERATE_CHANGER);
}

pub unsafe fn is_majin(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
	if (fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *WEAPON_KIND_RYU_HADOKEN || fighter_kind == *WEAPON_KIND_RYU_SHINKUHADOKEN) && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("AKUMA") {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn is_skeleton(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
	if fighter_kind == *FIGHTER_KIND_SZEROSUIT && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("ELMA") {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn is_merrier(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
	if fighter_kind == *FIGHTER_KIND_KOOPAJR && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("DR. EGGMAN") {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn is_gbland(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
	if fighter_kind == *FIGHTER_KIND_PIKACHU && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("ORCANE") {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn is_forest(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
	if fighter_kind == *FIGHTER_KIND_LUCARIO && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("HAWLUCHA") {
		return true;
	}
	else {
		return false;
	}
}

pub unsafe fn set_cstick(module_accessor: &mut app::BattleObjectModuleAccessor) {
	if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
		SUB_STICK[get_player_number(module_accessor)].x = ControlModule::get_stick_x(module_accessor);
		SUB_STICK[get_player_number(module_accessor)].y = ControlModule::get_stick_y(module_accessor);
	}
	if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
		SUB_STICK[get_player_number(module_accessor)] = Vector2f{x: 0.0, y: 0.0};
	}
}

pub unsafe fn jump_checker_buffer(module_accessor: &mut app::BattleObjectModuleAccessor, cat: i32) -> bool {
	if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 && ControlModule::is_enable_flick_jump(module_accessor){
		return true;
	}
	else if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
		return true;
	}
	else {
		return false;
	}
}

unsafe fn one_hit_mode(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32) {
	if READY_GO_TIMER != 0 && get_player_number(module_accessor) == 0 {
		READY_GO_TIMER -= 1;
	}

	if status_kind == *FIGHTER_STATUS_KIND_DAMAGE
	|| status_kind == *FIGHTER_STATUS_KIND_SLIP
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL { //When someone gets hit, start a timer
		GOT_HIT[get_player_number(module_accessor)] += 1;			
	} 
	else {
		GOT_HIT[get_player_number(module_accessor)] = 0;
	}

	if GOT_HIT[get_player_number(module_accessor)] == 2 { //If the timer reaches 2, check to see if anyone else has been hit
		for i in 0..TOTAL_FIGHTER + 1 {
			if GOT_HIT[i as usize] != 0 {
				if i as usize != get_player_number(module_accessor) {
					GOT_HIT[i as usize] = 3;
					break; //If anyone has, don't do anything
				}
			}
			else if i == TOTAL_FIGHTER { //If no one has been hit, kill the one player who has and tell everyone else to reset their positions
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
			}
		}
	}

	if HIT_PLAYER != -1 && HIT_PLAYER != get_player_number(module_accessor) as i32 {
		DamageModule::add_damage(module_accessor, DamageModule::damage(module_accessor, 0) * -1.0, 0); //Reset opponents to 0% if they were hit by something like a fox laser
	}
	
	if READY_GO_TIMER != 0 {
		let spawnVec = Vector2f{x: SPAWN_POS[get_player_number(module_accessor)].x, y: SPAWN_POS[get_player_number(module_accessor)].y};
		let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
		GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
		PostureModule::set_pos_2d(module_accessor, &spawnVec);
		StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
		if status_kind != *FIGHTER_STATUS_KIND_WAIT && STOCK_COUNT[get_player_number(module_accessor)] != 0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
		PostureModule::set_lr(module_accessor, SPAWN_POS[get_player_number(module_accessor)].z);
		GroundModule::set_correct(module_accessor, correct_kind);
		PostureModule::update_rot_y_lr(module_accessor);
	}

	if status_kind == *FIGHTER_STATUS_KIND_DEAD {
		HIT_PLAYER = get_player_number(module_accessor) as i32;
	}

	if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
		if STOCK_COUNT[get_player_number(module_accessor)] != 0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
		READY_GO_TIMER = 181;		
		HIT_PLAYER = -1;
	}
}

unsafe fn tennis_mode(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, lua_state: u64) {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind != *FIGHTER_KIND_NANA {

		if READY_GO_TIMER != 0 && get_player_number(module_accessor) == 0 {
			READY_GO_TIMER -= 1;
		}
	
		//Disable blast zones
	
		let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < dead_range(lua_state).x || pos.x > dead_range(lua_state).y || pos.y > dead_range(lua_state).z || pos.y < dead_range(lua_state).w {
			if pos.x < dead_range(lua_state).x { //Right
				pos.x = dead_range(lua_state).y;
			}
			if pos.x > dead_range(lua_state).y { //Left
				pos.x = dead_range(lua_state).x;
			}
			if pos.y > dead_range(lua_state).z { //Up
				pos.y = dead_range(lua_state).w;
			}
			if pos.y < dead_range(lua_state).w { //Down
				pos.y = dead_range(lua_state).z;
			} 
			let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
			GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
			PostureModule::set_pos(module_accessor, &pos);
			GroundModule::set_correct(module_accessor, correct_kind);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
	
		//Invisible wall preventing fighters from entering each others' territory
	
		if SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) < 3.0 {
			let newVec = Vector3f{x: 3.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
			PostureModule::set_pos(module_accessor, &newVec);
		}
		else if !SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) > -3.0 {
			let newVec = Vector3f{x: -3.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
			PostureModule::set_pos(module_accessor, &newVec);
		}
	
		//The process of KOing an opponent
	
		if BALL_BOUNCED.z != 9999.0 {
			if BALL_BOUNCED.y == 0.0 { //If the ball bounced on the ground, KO everyone who was on the same side as the ball
				if (BALL_BOUNCED.x > 3.0 && SPAWN_SIDE[get_player_number(module_accessor)]) || (BALL_BOUNCED.x < -3.0 && !SPAWN_SIDE[get_player_number(module_accessor)]) {
					for i in 0..3 {
						if BALL_VICTIMS[i] == 9 {
							BALL_VICTIMS[i] = get_player_number(module_accessor) as i32;
							break;
						}
					}
				}	
			}
			else { //If the ball went out of bounds, KO everyone who was on the same side as whoever last hit the ball
				if SPAWN_SIDE[get_player_number(module_accessor)] == SPAWN_SIDE[LAST_TO_HIT_BALL] {
					for i in 0..3 {
						if BALL_VICTIMS[i] == 9 {
							BALL_VICTIMS[i] = get_player_number(module_accessor) as i32;
							break;
						}
					}
				}
			}
			if get_player_number(module_accessor) as i32 == TOTAL_FIGHTER - 1 { //Once all victims have gotten a chance to be put on the list, KO them
				for i in 0..3 {
					if BALL_VICTIMS[i] != 9 {
						StatusModule::change_status_request_from_script(&mut *get_boma(BALL_VICTIMS[i]), *FIGHTER_STATUS_KIND_DEAD, true);
					}
				}
				LookupSymbol(
					&mut ITEM_MANAGER_ADDR,
					"_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
						.as_bytes()
						.as_ptr(),
				);
				let item_manager = *(ITEM_MANAGER_ADDR as *mut *mut app::ItemManager);
				lua_bind::ItemManager::remove_item_from_id(item_manager, BALL_ID); //Then get rid of the soccerball and reset these values so multiple stocks aren't taken
				BALL_BOUNCED = Vector3f{x: 0.0, y: 0.0, z: 9999.0};
			}
		}
	
		if !READY_GO[get_player_number(module_accessor)] { //The frame the game starts, randomly pick a player to give the ball
			READY_GO_TIMER = 0;
			if get_player_number(module_accessor) == 0 {
				BALL_OWNER = sv_math::rand(hash40("fighter"), TOTAL_FIGHTER);
			}
			if get_player_number(module_accessor) as i32 == BALL_OWNER {
				ItemModule::have_item(module_accessor, app::ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
				BALL_ID = ItemModule::get_have_item_id(module_accessor, 0) as u32;
			}
			LAST_TO_HIT_BALL = 9;
			ALREADY_BOUNCED = false;
			FIRST_BOUNCE = false;
		}
		if READY_GO_TIMER != 0 { //Lock everyone in place while waiting
			if READY_GO_TIMER == 180 { 
				let mut new_pos = SPAWN_POS[sv_math::rand(hash40("fighter"), TOTAL_FIGHTER) as usize]; //Give each player a temporary spawn position that matches one of the previously used spawn positions, and doesn't match
				//any of the other players' temporary spawn positions
				let mut pos_shuffle = false;
				while pos_shuffle == false {
					pos_shuffle = true; //Allow the loop to end, then compare new_pos with the randomly selected positions of all previous fighters. If there's a match, pick a new position and force the loop to restart
					for i in 0..TOTAL_FIGHTER {
						if new_pos.x == TEMP_SPAWN_POS[i as usize].x && new_pos.y == TEMP_SPAWN_POS[i as usize].y && new_pos.z == TEMP_SPAWN_POS[i as usize].z {
							new_pos = SPAWN_POS[sv_math::rand(hash40("fighter"), TOTAL_FIGHTER) as usize];
							pos_shuffle = false;
						}
					}
				}
				TEMP_SPAWN_POS[get_player_number(module_accessor)] = new_pos;
			}
			if READY_GO_TIMER == 179 {
				SPAWN_POS[get_player_number(module_accessor)] = TEMP_SPAWN_POS[get_player_number(module_accessor)];
				TEMP_SPAWN_POS[get_player_number(module_accessor)] = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			}
			let spawnVec = Vector2f{x: SPAWN_POS[get_player_number(module_accessor)].x, y: SPAWN_POS[get_player_number(module_accessor)].y};
			let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
			GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
			PostureModule::set_pos_2d(module_accessor, &spawnVec);
			StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
			if status_kind != *FIGHTER_STATUS_KIND_WAIT && STOCK_COUNT[get_player_number(module_accessor)] != 0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			SPAWN_SIDE[get_player_number(module_accessor)] = PostureModule::pos_x(module_accessor) > 0.0;
			PostureModule::set_lr(module_accessor, SPAWN_POS[get_player_number(module_accessor)].z);
			GroundModule::set_correct(module_accessor, correct_kind);
			PostureModule::update_rot_y_lr(module_accessor);
			LAST_TO_HIT_BALL = 9;
			if READY_GO_TIMER == 1 { //After everyone has respawned, randomly choose a player who was just KOd and give them the ball
				if get_player_number(module_accessor) == 0 {
					BALL_OWNER = 9;
					while BALL_OWNER == 9 {
						let owner = BALL_VICTIMS[sv_math::rand(hash40("fighter"), 4) as usize];
						if owner != 9 && STOCK_COUNT[owner as usize] != 0 {
							BALL_OWNER = owner;
						}
					}
					BALL_VICTIMS = [9, 9, 9, 9];
				}
				if get_player_number(module_accessor) as i32 == BALL_OWNER {
					ItemModule::have_item(module_accessor, app::ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
					BALL_ID = ItemModule::get_have_item_id(module_accessor, 0) as u32;
				}
			}
			ALREADY_BOUNCED = false;
			FIRST_BOUNCE = false;
		}
		if get_player_number(module_accessor) as i32 == BALL_OWNER {
			if ItemModule::is_have_item(module_accessor, 0) {
				LAST_TO_HIT_BALL = get_player_number(module_accessor);
			}
		}
	
		if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
			if STOCK_COUNT[get_player_number(module_accessor)] != 0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			READY_GO_TIMER = 181;
		}
	}
}

unsafe fn fun_di_mode(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, fighter: &mut L2CFighterCommon) {
	if status_kind == *FIGHTER_STATUS_KIND_THROWN
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U	
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
		let hitstun = WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
		if hitstun == 0 {
			L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
		}
	}
}

unsafe fn smash4_parry(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, fighter: &mut L2CFighterCommon, globals: &mut L2CValue) {
	if let L2CValueType::Void = globals["perfect_shield"].val_type {
		globals["perfect_shield"] = false.into();
		globals["shield_health"] = 50.0.into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_GUARD {
		if !globals["perfect_shield"].get_bool() {
			PLAY_SE(fighter, app::FighterUtil::get_just_shield_se(fighter_kind));
		}
		globals["perfect_shield"] = true.into();
		WorkModule::mul_float(module_accessor, 0.66, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
		WorkModule::set_float(module_accessor, globals["shield_health"].get_num(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
	}
	else {
		globals["shield_health"] = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD).into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
		if globals["perfect_shield"].get_bool() {
			CancelModule::enable_cancel(module_accessor);
		}
		globals["perfect_shield"] = false.into();
	}
}

unsafe fn basketball_mode(module_accessor: &mut app::BattleObjectModuleAccessor, lua_state: u64, fighter: &mut L2CFighterCommon) {
	let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
	if pos.x < dead_range(lua_state).x || pos.x > dead_range(lua_state).y || pos.y > dead_range(lua_state).z || pos.y < dead_range(lua_state).w {
		if pos.x < dead_range(lua_state).x { //Right
			pos.x = dead_range(lua_state).y;
		}
		if pos.x > dead_range(lua_state).y { //Left
			pos.x = dead_range(lua_state).x;
		}
		if pos.y > dead_range(lua_state).z { //Up
			pos.y = dead_range(lua_state).w;
		}
		if pos.y < dead_range(lua_state).w { //Down
			pos.y = dead_range(lua_state).z;
		} 
		let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
		GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
		PostureModule::set_pos(module_accessor, &pos);
		GroundModule::set_correct(module_accessor, correct_kind);
	}
	AttackModule::set_no_dead_all(module_accessor, true, false);
	WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
	if get_player_number(module_accessor) == 0 {
		if EffectModule::is_exist_common(module_accessor, Hash40{hash: hash40("sys_jump_aerial")}) == false {
			let high_offset = Vector3f{
				x: HIGH_SPAWN_POS.x - PostureModule::pos_x(module_accessor), 
				y: HIGH_SPAWN_POS.y - PostureModule::pos_y(module_accessor),
				z: 0.0
			};
			let low_offset = Vector3f{
				x: LOW_SPAWN_POS.x - PostureModule::pos_x(module_accessor), 
				y: LOW_SPAWN_POS.y - PostureModule::pos_y(module_accessor),
				z: 0.0
			};
			let lr = imported_rot_y_lr(module_accessor) / 90.0;
			EFFECT(/*Effect*/ fighter, Hash40::new("sys_jump_aerial"), /*Bone*/ Hash40::new("trans"), /*X*/ 0, /*Y*/ high_offset.y, /*Z*/ high_offset.x * lr, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 90, /*Size?*/ 2.0, 0, 0, 0, 0, 0, 0, true);
			EFFECT(/*Effect*/ fighter, Hash40::new("sys_jump_aerial"), /*Bone*/ Hash40::new("trans"), /*X*/ 0, /*Y*/ low_offset.y, /*Z*/ low_offset.x * lr, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 90, /*Size?*/ 2.0, 0, 0, 0, 0, 0, 0, true);
		}	
	}
}

unsafe fn special_ce_mode(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, fighter: &mut L2CFighterCommon, fighter_information: &mut app::FighterInformation) {
	if is_preview_mode() {
		if fighter_kind == *FIGHTER_KIND_MARIO {
			VisibilityModule::set_whole(module_accessor, false);
		}
		if PostureModule::scale(module_accessor) == 1.5 {
			SPECIAL_SMASH_SIZE = 1;
		}
		else if PostureModule::scale(module_accessor) == 0.35 {
			SPECIAL_SMASH_SIZE = 2;
		}
		else {
			SPECIAL_SMASH_SIZE = 0;
		}
		
		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FLOWER) {
			SPECIAL_SMASH_HEAD = 1;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
			SPECIAL_SMASH_HEAD = 2;
		}
		else {
			SPECIAL_SMASH_HEAD = 0;
		}

		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
			SPECIAL_SMASH_BODY = 1;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
			SPECIAL_SMASH_BODY = 2;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) {
			SPECIAL_SMASH_BODY = 3;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
			SPECIAL_SMASH_BODY = 4;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) {
			SPECIAL_SMASH_BODY = 5;
		}
		else if ItemModule::is_attach_item(module_accessor, app::ItemKind(*ITEM_KIND_BACKSHIELD)) {
			SPECIAL_SMASH_BODY = 6;
		}
		else {
			SPECIAL_SMASH_BODY = 0;
		}

		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
			app::sv_animcmd::FT_DISABLE_CURRY_FACE(fighter.lua_state_agent);
			SPECIAL_SMASH_STATUS = 1;
		}
		else if ItemModule::is_attach_item(module_accessor, app::ItemKind(*ITEM_KIND_BADGE)) {
			SPECIAL_SMASH_STATUS = 2;
		}
		else {
			SPECIAL_SMASH_STATUS = 0;
		}

		if MotionModule::whole_rate(module_accessor) == 0.5 {
			SPECIAL_SMASH_RATE = 1;
		}
		else if MotionModule::whole_rate(module_accessor) == 1.2 {
			SPECIAL_SMASH_RATE = 2;
		}
		else {
			SPECIAL_SMASH_RATE = 0;
		}
	}
	else if !sv_information::is_ready_go() {
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
		PostureModule::set_scale(module_accessor, 1.0, true);
		EffectModule::kill_kind(module_accessor, Hash40::new("sys_curry_shot"), true, true);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL);
		if fighter_kind != *FIGHTER_KIND_SNAKE {
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK);
		}
		ItemModule::remove_all(module_accessor);
		lua_bind::FighterInformation::gravity(fighter_information);
		if HIGH_SPAWN_POS.z == 1.0 || LOW_SPAWN_POS.z == 1.0 {
			HIGH_SPAWN_POS = Vector3f{x: 30.0, y: 0.0, z: 0.0};
			LOW_SPAWN_POS = Vector3f{x: -30.0, y: 0.0, z: 0.0};
		}
		if PostureModule::pos_x(module_accessor) > HIGH_SPAWN_POS.x - 30.0 {
			HIGH_SPAWN_POS = Vector3f{x: PostureModule::pos_x(module_accessor) + 30.0, y: PostureModule::pos_y(module_accessor) + 50.0, z: 0.0};
		}
		if PostureModule::pos_x(module_accessor) < LOW_SPAWN_POS.x + 30.0 {
			LOW_SPAWN_POS = Vector3f{x: PostureModule::pos_x(module_accessor) - 30.0, y: PostureModule::pos_y(module_accessor) + 50.0, z: 0.0};
		}
		SPAWN_SIDE[get_player_number(module_accessor)] = PostureModule::pos_x(module_accessor) > 0.0;
		SPAWN_POS[get_player_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::lr(module_accessor)};
	}
	else {
		HIGH_SPAWN_POS.z = 1.0;
		LOW_SPAWN_POS.z = 1.0;
		if (USED_FS[get_player_number(module_accessor)] && !WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE))
		|| (fighter_kind == *FIGHTER_KIND_EFLAME && (status_kind == *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT || status_kind == *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_STANDBY))
		|| (fighter_kind == *FIGHTER_KIND_ELIGHT && (status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_STANDBY)) {
			if (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FLOWER) && SPECIAL_SMASH_HEAD == 1) 
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) && SPECIAL_SMASH_HEAD == 2)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) && SPECIAL_SMASH_BODY == 3)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) && SPECIAL_SMASH_BODY == 4)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) && SPECIAL_SMASH_BODY == 5)
			|| (ItemModule::is_attach_item(module_accessor, app::ItemKind(*ITEM_KIND_BACKSHIELD)) && SPECIAL_SMASH_BODY == 6)
			|| (ItemModule::is_attach_item(module_accessor, app::ItemKind(*ITEM_KIND_BADGE)) && SPECIAL_SMASH_STATUS == 2) {
				for i in 0..TOTAL_FIGHTER {
					ItemModule::remove_all(&mut *get_boma(i));
				}
				WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
				USED_FS[get_player_number(module_accessor)] = false;
			}
		}
		
		if SPECIAL_SMASH_BODY == 3 {
			tennis_mode(module_accessor, status_kind, fighter.lua_state_agent);
		}
		if SPECIAL_SMASH_BODY == 4 {
			fun_di_mode(module_accessor, status_kind, fighter);
		}
		if SPECIAL_SMASH_BODY == 5 {
			one_hit_mode(module_accessor, status_kind);
		}
		if SPECIAL_SMASH_BODY == 6 {
			basketball_mode(module_accessor, fighter.lua_state_agent, fighter);
		}
		if SPECIAL_SMASH_STATUS == 1 {
			app::sv_animcmd::FT_DISABLE_CURRY_FACE(fighter.lua_state_agent);
			let mut globals = fighter.globals_mut().clone();
			smash4_parry(module_accessor, fighter_kind, status_kind, fighter, &mut globals);
		}
		//if SPECIAL_SMASH_STATUS == 2, parry reflects are enabled using a function hook
		if SPECIAL_SMASH_SIZE == 1 {
			AttackModule::set_attack_power_mul_scale(module_accessor, 1.0);
			PostureModule::set_scale(module_accessor, 1.0, true);
		}
		if SPECIAL_SMASH_SIZE == 2 {
			AttackModule::set_attack_power_mul_scale(module_accessor, 1.0);
			PostureModule::set_scale(module_accessor, 1.0, true);
		}
		if SPECIAL_SMASH_HEAD != 0 || SPECIAL_SMASH_BODY != 0 || SPECIAL_SMASH_STATUS != 0 {
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
			EffectModule::kill_kind(module_accessor, Hash40::new("sys_curry_shot"), true, true);
			if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
				ItemModule::remove_all(module_accessor);
			}
		}
	}
}

unsafe fn status_kind_damage(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, globals: &mut L2CValue) {
	if status_kind == *FIGHTER_STATUS_KIND_DAMAGE
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
	|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
	|| status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE	{
		if status_kind != *FIGHTER_STATUS_KIND_TREAD_DAMAGE {
			if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
				if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && (ControlModule::get_stick_y(module_accessor) < -0.66) && (KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= -0.5) {
						WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				}
			}

			if SPECIAL_SMASH_HEAD == 2 {
				CancelModule::enable_cancel(module_accessor);
			}

			if SPECIAL_SMASH_BODY == 6 {
				if (SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) > LOW_SPAWN_POS.x - 15.0 && PostureModule::pos_x(module_accessor) < LOW_SPAWN_POS.x + 15.0 
				&& PostureModule::pos_y(module_accessor) > LOW_SPAWN_POS.y - 15.0 && PostureModule::pos_y(module_accessor) < LOW_SPAWN_POS.y + 15.0) 
				|| (!SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) > HIGH_SPAWN_POS.x - 15.0 && PostureModule::pos_x(module_accessor) < HIGH_SPAWN_POS.x + 15.0 
				&& PostureModule::pos_y(module_accessor) > HIGH_SPAWN_POS.y - 15.0 && PostureModule::pos_y(module_accessor) < HIGH_SPAWN_POS.y + 15.0) {
					WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
				}
			}

			if TALKIN_MAD_SHIT[get_attacker_number(module_accessor)] == 1 {
				if situation_kind == *SITUATION_KIND_AIR {
					TALKIN_MAD_SHIT[get_attacker_number(module_accessor)] = 2;
				}
				else {
					TALKIN_MAD_SHIT[get_attacker_number(module_accessor)] = 0;
				}
			}

			if IN_JAB[get_attacker_number(module_accessor)] {
				if status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR {
					IN_HITSTUN[get_player_number(module_accessor)] = true;			
				}
			}

			if StatusModule::is_situation_changed(module_accessor) {
				ControlModule::clear_command(module_accessor, true);
			}

			EXTRA_LAG[get_player_number(module_accessor)] = 0;
			PARRIED_COUNT[get_player_number(module_accessor)] = 0;
			globals["microdash_check"] = false.into();
			let offset_x = (PostureModule::pos_2d(module_accessor).x - DUCKHUNT_POS[get_attacker_number(module_accessor)].x).abs();
			DUCKHUNT_OFFSET[get_attacker_number(module_accessor)] = Vector3f{x: get_player_number(module_accessor) as f32, y: PostureModule::pos_y(module_accessor), z: offset_x};		
		}
		B_CHECK[get_player_number(module_accessor)] = false;
		AERIAL_KIND[get_player_number(module_accessor)] = 0;
	}
	else if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_AIR
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
		if DUCKHUNT_OFFSET[get_attacker_number(module_accessor)].x == get_player_number(module_accessor) as f32 {
			DUCKHUNT_OFFSET[get_attacker_number(module_accessor)].x = 8.0;
		}
		if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR && MotionModule::frame(module_accessor) > 7.0 {
			GET_CLIPPED_NERD[get_player_number(module_accessor)] = true;			
		}
	}
	else {
		GET_CLIPPED_NERD[get_player_number(module_accessor)] = false;
	}
}

unsafe fn background_effects(module_accessor: &mut app::BattleObjectModuleAccessor, fighter: &mut L2CFighterCommon) {
	if SUCCESSFUL_CLIP[get_player_number(module_accessor)] > 0 {
		SUCCESSFUL_CLIP[get_player_number(module_accessor)] -= 1;
	}
	if SUCCESSFUL_CLIP[get_player_number(module_accessor)] == 1 {
		SlowModule::clear_whole(module_accessor);
		app::sv_animcmd::CAM_ZOOM_OUT(fighter.lua_state_agent);
	}

	if NERD_GOT_CLIPPED[get_player_number(module_accessor)].x != 0.0 && NERD_GOT_CLIPPED[get_player_number(module_accessor)].y != 0.0 && NERD_GOT_CLIPPED[get_player_number(module_accessor)].z != 0.0 && NERD_GOT_CLIPPED[get_player_number(module_accessor)].w != 0.0 {
		PLAY_SE_NO_3D(fighter, Hash40::new("se_item_homerunbat_l"));
		CAM_ZOOM_IN_arg5(fighter, NERD_GOT_CLIPPED[get_player_number(module_accessor)].x, 0.0, NERD_GOT_CLIPPED[get_player_number(module_accessor)].y, NERD_GOT_CLIPPED[get_player_number(module_accessor)].z, NERD_GOT_CLIPPED[get_player_number(module_accessor)].w);
		NERD_GOT_CLIPPED[get_player_number(module_accessor)] = Vector4f{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
	}
}

unsafe fn pogo(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, cat: i32) {
	if fighter_kind == FIGHTER_KIND_BUDDY || fighter_kind == FIGHTER_KIND_KAMUI {
		if motion_kind == hash40("attack_air_lw") {
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || MotionModule::frame(module_accessor) >= 30.0 {
				if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
					if jump_checker_buffer(module_accessor, cat) {			
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
					}
				}
			}
		}
	}
}

unsafe fn b_reverse(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, globals: &mut L2CValue) {
	if fighter_kind == FIGHTER_KIND_KROOL {
		if motion_kind == hash40("special_air_lw") {
			if globals["assignment_check"].get_bool() == false {
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) > -0.66 {
					globals["momentum"] = true.into();
				}
				globals["assignment_check"] = true.into();
			}
			if MotionModule::frame(module_accessor) <= 3.0 {
				if globals["turnaround"].get_bool() {
					PostureModule::reverse_lr(module_accessor);
					PostureModule::update_rot_y_lr(module_accessor);
					globals["turnaround"] = false.into();
				}
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
					if globals["turnaround"].get_bool() == false {
						PostureModule::reverse_lr(module_accessor);
						PostureModule::update_rot_y_lr(module_accessor);
					}
					if globals["momentum"].get_bool() {
						let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
						KineticModule::mul_speed(module_accessor, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
						globals["momentum"] = false.into();
					}
				}
			}
		}
		else {
			if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
				globals["reverse_timer"] = 0.0.into();
				globals["turnaround"] = true.into();
			}
			else {
				if globals["reverse_timer"].get_num() >= 4.0 {
					globals["turnaround"] = false.into();
				}
				globals["reverse_timer"] = (globals["reverse_timer"].get_num() + 1.0).into()
			}
			globals["assignment_check"] = false.into();
			globals["momentum"] = false.into();
		}
	}
	if fighter_kind == FIGHTER_KIND_SNAKE {
		if motion_kind == hash40("special_hi_start") || motion_kind == hash40("special_air_hi_start") {
			if globals["assignment_check"].get_bool() == false {
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) > -0.66 {
					globals["momentum"] = true.into();
				}
				globals["assignment_check"] = true.into();
			}
			if MotionModule::frame(module_accessor) <= 3.0 && MotionModule::frame(module_accessor) > 0.0 {
				if globals["turnaround"].get_bool() {
					PostureModule::reverse_lr(module_accessor);
					PostureModule::update_rot_y_lr(module_accessor);
					globals["turnaround"] = false.into();
				}
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
					if globals["turnaround"].get_bool() == false {
						PostureModule::reverse_lr(module_accessor);
						PostureModule::update_rot_y_lr(module_accessor);
					}
					if globals["momentum"].get_bool() {
						let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
						KineticModule::mul_speed(module_accessor, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
						globals["momentum"] = false.into();
					}
				}
			}
		}
		else {
			if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
				globals["reverse_timer"] = 0.0.into();
				globals["turnaround"] = true.into();
			}
			else {
				if globals["reverse_timer"].get_num() >= 4.0 {
					globals["turnaround"] = false.into();
				}
				globals["reverse_timer"] = (globals["reverse_timer"].get_num() + 1.0).into()
			}
			globals["assignment_check"] = false.into();
			globals["momentum"] = false.into();
		}
	}
}

unsafe fn directional_airdodge(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, fighter: &mut L2CFighterCommon) {
	if motion_kind == hash40("escape_air_slide") {
		if fighter_kind == *FIGHTER_KIND_GEKKOUGA && FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
			CancelModule::enable_cancel(module_accessor);
		}
		else {
			if MotionModule::frame(module_accessor) == 13.0 {
				KineticModule::clear_speed_all(module_accessor);
				KineticModule::unable_energy_all(module_accessor);
			}
			if MotionModule::frame(module_accessor) == 37.0 {
				KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
		}
	}
}

unsafe fn grab_break(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) {
	if fighter_kind == FIGHTER_KIND_POPO {
		if motion_kind == hash40("catch_dash") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 9.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_turn") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 10.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
	}
	if fighter_kind == FIGHTER_KIND_INKLING {
		if motion_kind == hash40("catch") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 9.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_dash") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 10.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_turn") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 11.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
	}
}

unsafe fn shorthop_buffer(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat: i32) {
	if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT || StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT || status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || (jump_checker_buffer(module_accessor, cat) && situation_kind == *SITUATION_KIND_AIR && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) {
		if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || (jump_checker_buffer(module_accessor, cat) && situation_kind == *SITUATION_KIND_AIR && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) {
			if StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_DASH && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_TURN_DASH {
				if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) && ControlModule::get_attack_air_kind(module_accessor) == 0 && ATTACK_CANCELED[get_player_number(module_accessor)] == false || (jump_checker_buffer(module_accessor, cat) && situation_kind == *SITUATION_KIND_AIR && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) {
					if SUB_STICK[get_player_number(module_accessor)].x != 0.0 || SUB_STICK[get_player_number(module_accessor)].y != 0.0 {
						if SUB_STICK[get_player_number(module_accessor)].x * PostureModule::lr(module_accessor) >= 0.4 {
							AERIAL_KIND[get_player_number(module_accessor)] = 2;
						}
						if SUB_STICK[get_player_number(module_accessor)].x * PostureModule::lr(module_accessor) <= -0.4 {
							AERIAL_KIND[get_player_number(module_accessor)] = 3;
						}
						if SUB_STICK[get_player_number(module_accessor)].y >= 0.4 {
							AERIAL_KIND[get_player_number(module_accessor)] = 4;
						}
						if SUB_STICK[get_player_number(module_accessor)].y <= -0.4 {
							AERIAL_KIND[get_player_number(module_accessor)] = 5;
						}
					}
				}
			}
			let fighter_kind = app::utility::get_kind(module_accessor);
			if (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(module_accessor) && ControlModule::get_stick_y(module_accessor) > 0.7)) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) && (!is_majin(module_accessor, fighter_kind) || !FIGHTER_BOOL_3[get_player_number(module_accessor)]) { //G_DEMON_FLIP
				WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
			}
			else {
				WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
			}
		}
		else {
			ATTACK_CANCELED[get_player_number(module_accessor)] = false;
			AERIAL_KIND[get_player_number(module_accessor)] = 0;
			if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
				SUB_STICK[get_player_number(module_accessor)] = Vector2f{x: 0.0, y: 0.0};
			}
		}
	}
	if ATTACK_CANCELED[get_player_number(module_accessor)] {
		AERIAL_KIND[get_player_number(module_accessor)] = 0;
	}
	if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
		AERIAL_KIND[get_player_number(module_accessor)] = 0;
	}
}

unsafe fn landing_lag(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
	if situation_kind == SITUATION_KIND_AIR {
		if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW {
			EXTRA_LAG[get_player_number(module_accessor)] = 3;
		}
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				if EXTRA_LAG[get_player_number(module_accessor)] != 3 {
					EXTRA_LAG[get_player_number(module_accessor)] = 0;
				}
			}
			else if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
				if EXTRA_LAG[get_player_number(module_accessor)] != 3 {
					EXTRA_LAG[get_player_number(module_accessor)] = 1;
				}
			}
			else {
				EXTRA_LAG[get_player_number(module_accessor)] = 2;
			}
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
	|| status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
		EXTRA_LAG[get_player_number(module_accessor)] = 0;
		PARRIED_COUNT[get_player_number(module_accessor)] = 0;
	}
	if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_LANDING
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
	|| StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_LANDING_LIGHT {
		EXTRA_LAG[get_player_number(module_accessor)] = 0;
	}
}

unsafe fn air_specials(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32) {
	if (fighter_kind == FIGHTER_KIND_SHEIK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW)
	|| (fighter_kind == FIGHTER_KIND_MEWTWO && (status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 || motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi"))) 
	|| (fighter_kind == FIGHTER_KIND_PITB && status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)
	|| (fighter_kind == FIGHTER_KIND_SZEROSUIT && status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START)
	|| (fighter_kind == FIGHTER_KIND_LITTLEMAC && status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP)
	|| (fighter_kind == FIGHTER_KIND_MIIFIGHTER && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START)
	|| (fighter_kind == FIGHTER_KIND_KOOPAJR && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT)
	|| (is_majin(module_accessor, fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW)  {
		B_CHECK[get_player_number(module_accessor)] = true;
	}
	if situation_kind != SITUATION_KIND_AIR {
		B_CHECK[get_player_number(module_accessor)] = false;
	}
	if B_CHECK[get_player_number(module_accessor)] == false {
		if fighter_kind == FIGHTER_KIND_LITTLEMAC {
			if WorkModule::is_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
				WorkModule::off_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
			}
		}
	}
}

unsafe fn microdash(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, globals: &mut L2CValue) {
	if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
		if MotionModule::frame(module_accessor) == 0.0 {
			globals["microdash_check"] = false.into();
		}
		if MotionModule::frame(module_accessor) > 4.0 && globals["microdash_check"].get_bool() == true {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN_BRAKE, true);
			globals["microdash_success"] = true.into();
			globals["microdash_check"] = false.into();
		}
		if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
			if MotionModule::frame(module_accessor) > 0.0 && MotionModule::frame(module_accessor) <= 1.0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN_BRAKE, true);
				globals["microdash_success"] = true.into();
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
			}
			if MotionModule::frame(module_accessor) <= 4.0 && MotionModule::frame(module_accessor) > 1.0 {
				globals["microdash_check"] = true.into();
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
				WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
			}
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE && globals["microdash_success"].get_bool() == true {
		MotionModule::set_rate(module_accessor, 3.0);
		WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
		WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
		if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_F {
			globals["microdash_success"] = false.into();
		}
	}
	if status_kind != *FIGHTER_STATUS_KIND_ESCAPE_F && status_kind != *FIGHTER_STATUS_KIND_ESCAPE_B {
		if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_RUN_BRAKE && (StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DASH || StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_TURN_DASH) {
			if globals["microdash_success"].get_bool() == true {
				globals["microdash_success"] = false.into();
			}
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_F {
		if globals["microdash_success"].get_bool() {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_RUN_BRAKE, true);
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_B {
		if globals["microdash_success"].get_bool() {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN_RUN, true);
		}
	}
	if status_kind != *FIGHTER_STATUS_KIND_RUN_BRAKE && status_kind != *FIGHTER_STATUS_KIND_ESCAPE_F && status_kind != *FIGHTER_STATUS_KIND_ESCAPE_B && status_kind != *FIGHTER_STATUS_KIND_DASH && status_kind != *FIGHTER_STATUS_KIND_TURN_DASH && status_kind != *FIGHTER_STATUS_KIND_TURN_RUN {
		globals["microdash_success"] = false.into();

	}
}

unsafe fn jab_cancel(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32) {
	if IN_HITSTUN[get_player_number(module_accessor)] {
		if (status_kind != *FIGHTER_STATUS_KIND_DAMAGE && status_kind != *FIGHTER_STATUS_KIND_DAMAGE_AIR) || WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
			JAB_CANCEL_OK[get_attacker_number(module_accessor)] = true;
			IN_HITSTUN[get_player_number(module_accessor)] = false;
			if get_player_number(module_accessor) > get_attacker_number(module_accessor) { 
				// If the attacker has a lower player number, they're going to receive JAB_CANCEL_OK a frame after it's sent, so if enable Cancel is run immediately, the inputs the attacker makes won't have 
				// an effect until 2 frames after this flag is sent. Therefore, if get player number > get attacker number, we can skip the frame check outright, let the attacker act immediately, and they'll
				// still be -2
				FRAME_CHECK[get_attacker_number(module_accessor)] = false;
			}
		}
	}
	if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
		&& ((fighter_kind == FIGHTER_KIND_MARIO && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_DONKEY && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_LINK && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_YOSHI && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_KIRBY && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_FOX && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_LUIGI && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_NESS && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_CAPTAIN && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_PURIN && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_PEACH && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_DAISY && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_KOOPA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_POPO && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_NANA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_SHEIK && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_ZELDA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_MARIOD && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_FALCO && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_MARTH && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_LUCINA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_YOUNGLINK && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_MEWTWO && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_GAMEWATCH && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_KOOPAG && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_METAKNIGHT && motion_kind == hash40("attack_s3_s2"))
		|| (fighter_kind == FIGHTER_KIND_PIT && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_PITB && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_SZEROSUIT && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_WARIO && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_SNAKE && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_IKE && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_PZENIGAME && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_PFUSHIGISOU && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_PLIZARDON && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_DIDDY && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_LUCAS && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_SONIC && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_DEDEDE && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_PIKMIN && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_LUCARIO && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_ROBOT && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_TOONLINK && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_WOLF && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_WIIFIT && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_ROSETTA && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_LITTLEMAC && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_GEKKOUGA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_MIIFIGHTER && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_MIIGUNNER && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_MIISWORDSMAN && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_PALUTENA && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_PACMAN && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_REFLET && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_SHULK && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_KOOPAJR && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_DUCKHUNT && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_RYU && motion_kind == hash40("attack_11_w"))
		|| (fighter_kind == FIGHTER_KIND_KEN && motion_kind == hash40("attack_11_w"))
		|| (fighter_kind == FIGHTER_KIND_CLOUD && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_KAMUI && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_BAYONETTA && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_BAYONETTA && motion_kind == hash40("attack_s3_s2"))
		|| (fighter_kind == FIGHTER_KIND_INKLING && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_RIDLEY && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_SIMON && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_RICHTER && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_KROOL && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_SHIZUE && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_GAOGAEN && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_PACKUN && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_JACK && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_BRAVE && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_BUDDY && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_DOLLY && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_MASTER && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_TANTAN && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_EDGE && motion_kind == hash40("attack_11")) 
		|| (fighter_kind == FIGHTER_KIND_EFLAME && motion_kind == hash40("attack_11"))
		|| (fighter_kind == FIGHTER_KIND_ELIGHT && motion_kind == hash40("attack_12"))
		|| (fighter_kind == FIGHTER_KIND_TRAIL && motion_kind == hash40("attack_12"))) {
		IN_JAB[get_player_number(module_accessor)] = true;
		if JAB_CANCEL_OK[get_player_number(module_accessor)] { //If the opponent is not out of hitstun but has been jabbed
			if FRAME_CHECK[get_player_number(module_accessor)] { //If we're allowed to store the frame the opponent got out
				MOTION_FRAME[get_player_number(module_accessor)] = MotionModule::frame(module_accessor);
				FRAME_CHECK[get_player_number(module_accessor)] = false;
			}
			else {
				if MOTION_FRAME[get_player_number(module_accessor)] <= (MotionModule::frame(module_accessor) - 1.0) {
					CancelModule::enable_cancel(module_accessor);
				}
			}
		}
	}
	else {
		MOTION_FRAME[get_player_number(module_accessor)] = 0.0;
		FRAME_CHECK[get_player_number(module_accessor)] = true;
		IN_JAB[get_player_number(module_accessor)] = false;
		JAB_CANCEL_OK[get_player_number(module_accessor)] = false;
	}
}

unsafe fn pivot_crossup(module_accessor: &mut app::BattleObjectModuleAccessor, motion_kind: u64, globals: &mut L2CValue) {
	if motion_kind == hash40("catch_turn") {
		if MotionModule::frame(module_accessor) == 0.0 {
			JostleModule::set_status(module_accessor, false);
		}
		if GrabModule::is_grab(module_accessor, 0) {
			globals["grab_check"] = true.into();
		}
		else {
			if globals["grab_check"].get_bool() {
				JostleModule::set_status(module_accessor, true);
				globals["grab_check"] = false.into();
			}
		}
	}
	if motion_kind == hash40("turn_run") {
		JostleModule::set_status(module_accessor, false);
	}
	if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_TURN_RUN {
		JostleModule::set_status(module_accessor, true);
	}
}

unsafe fn dash_drop(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32) {
	if status_kind == *FIGHTER_STATUS_KIND_RUN || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
		if GroundModule::is_passable_ground(module_accessor) && ControlModule::get_stick_y(module_accessor) < -0.66 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
		}
	}
}

unsafe fn shield_release(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, globals: &mut L2CValue) {
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
		globals["shield_check"] = true.into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
		globals["shield_check"] = false.into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
		if globals["shield_check"].get_bool() {
			MotionModule::set_rate(module_accessor, 1.0);
		}
	}
}

unsafe fn shield(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32, cat: i32, cat2: i32) {
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
		if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
		}
		if status_kind != *FIGHTER_STATUS_KIND_GUARD_OFF {
			if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 {
				if GroundModule::is_passable_ground(module_accessor) {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
				}
			}
		}
	}
}

unsafe fn counterhit(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32) {
	if [
		*FIGHTER_STATUS_KIND_ATTACK,
		*FIGHTER_STATUS_KIND_ATTACK_S3,
		*FIGHTER_STATUS_KIND_ATTACK_HI3,
		*FIGHTER_STATUS_KIND_ATTACK_LW3,
		*FIGHTER_STATUS_KIND_ATTACK_S4_START,
		*FIGHTER_STATUS_KIND_ATTACK_HI4_START,
		*FIGHTER_STATUS_KIND_ATTACK_LW4_START,
		*FIGHTER_STATUS_KIND_ATTACK_S4,
		*FIGHTER_STATUS_KIND_ATTACK_HI4,
		*FIGHTER_STATUS_KIND_ATTACK_LW4,
		*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
		*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
		*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
		*FIGHTER_STATUS_KIND_ATTACK_DASH,
		*FIGHTER_STATUS_KIND_ATTACK_AIR,
		*FIGHTER_STATUS_KIND_SPECIAL_N,
		*FIGHTER_STATUS_KIND_SPECIAL_S,
		*FIGHTER_STATUS_KIND_SPECIAL_LW,
		*FIGHTER_STATUS_KIND_SPECIAL_HI,
		*FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
		*FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR,
		*FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START,
		*FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD,
		*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING,
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK,
		*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
		*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING,
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER,
		*FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3,
		*FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3,
	].contains(&status_kind) {
		if estimate_frame(module_accessor, 0.0) {
			COUNTERHIT_CHECK[get_player_number(module_accessor)] = true;
		}
		if AttackModule::is_attack(module_accessor, 0, false) {
			COUNTERHIT_CHECK[get_player_number(module_accessor)] = false;
		}
	}
}

unsafe fn parry(module_accessor: &mut app::BattleObjectModuleAccessor, attacker_module_accessor: &mut app::BattleObjectModuleAccessor, motion_kind: u64, status_kind: i32, globals: &mut L2CValue) {
	if motion_kind == hash40("just_shield_off") {
		if globals["parry_lag_set"].get_bool() == false { 
			//Real Parry Lag is set to the same number for both players. Since sys_line_system_control_fighter does not run on every frame during parries, if Real Parry Lag is above 13, parry lag decreases
			//by 2 per frame instead of 1 per frame to compensate.
			globals["parry_lag"] = 18.0.into();
			globals["real_parry_lag"] = 18.0.into();
			if app::utility::get_category(attacker_module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER { //We only want to set this if the thing that hit you is a fighter because running get_player_number on a weapon will instantly panic on non player 1
				BASE_SHIELD_DAMAGE[get_player_number(attacker_module_accessor)] = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER) * WorkModule::get_float(module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
				if (get_player_number(module_accessor) as i32) < (get_player_number(attacker_module_accessor) as i32) {
					ATTACKER_OVER_DEFENDER[get_player_number(attacker_module_accessor)] = true;
				}
			}
			globals["parry_lag_set"] = true.into();
		}
		if MotionModule::frame(module_accessor) < 13.9 {
			WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
			WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
		}
		else {
			WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
			WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
		}
		if MotionModule::frame(module_accessor) >= 2.3 {
			if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
				if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
					ShieldModule::set_shield_type(module_accessor, app::ShieldType(*SHIELD_TYPE_JUST_SHIELD), 0, 0);
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, true);	
				}
				else {
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
					WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
				}
			}
		}
	}
	else {
		globals["parry_lag_set"] = false.into();
	}
	if BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] != 0.0 {
		globals["reset_frame"] = MotionModule::frame(module_accessor).into();
		globals["reset_motion"] = motion_kind.into();
		if ATTACKER_OVER_DEFENDER[get_player_number(module_accessor)] {
//			if (((25.0 - ((BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] * 0.8) + 2.0).floor()) as i32) as f32) <= 3.0 { 
//				//Trying to cancel parry lag too early results in the move freaking out and looping, so if a move has enough shieldstun to where it should take less than 1 frame of parry lag, it takes 10
//				globals["parry_lag"] = 1.0.into();
//				globals["real_parry_lag"] = 1.0.into();
//			}
//			else {
				globals["parry_lag"] = (((25.0 - ((BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] * 0.8) + 2.0).floor()) as i32) as f32).into(); 
				//If I ever want to change the parry advantage and forget how the formula works, it's the 25 at the beginning.
				globals["real_parry_lag"] = 18.0.into();
//			}
		}
		else { /*If the defender's player number is higher than the attacker's, their parry_lag will be set and decreased on the frame they parry, but the attacker won't have theirs set until the next frame 
		(since their instance of sys_line_system_control_fighter has already run), so if the defender has a higher player ID, they take 1 less frame of parry lag to compensate*/	
//			if (((24.0 - ((BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] * 0.8) + 2.0).floor()) as i32) as f32) <= 3.0 {
//				globals["parry_lag"] = 1.0.into();
//				globals["real_parry_lag"] = 1.0.into();
//			}
//			else {
				globals["parry_lag"] = (((24.0 - ((BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] * 0.8) + 2.0).floor()) as i32) as f32).into(); 
				globals["real_parry_lag"] = 18.0.into();
//			}
		}
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
			PARRIED_COUNT[get_player_number(module_accessor)] += 1;
		}
		ATTACKER_OVER_DEFENDER[get_player_number(module_accessor)] = false;
		BASE_SHIELD_DAMAGE[get_player_number(module_accessor)] = 0.0;
	}
	if globals["parry_lag"].get_num() > 0.0 {
		if globals["real_parry_lag"].get_num() > 12.0 {
			globals["parry_lag"] = (globals["parry_lag"].get_num() - 1.0).into();
		}
		globals["parry_lag"] = (globals["parry_lag"].get_num() - 1.0).into();
		globals["real_parry_lag"] = (globals["real_parry_lag"].get_num() - 1.0).into();
	}
	else if globals["parry_lag"].get_num() == 0.0 {
		StopModule::end_stop(module_accessor);
		globals["parry_lag"] = (-1.0).into();
		globals["real_parry_lag"] = (-1.0).into();
		let fighter_kind = app::utility::get_kind(module_accessor);
		if fighter_kind == *FIGHTER_KIND_SIMON 
		|| fighter_kind == *FIGHTER_KIND_RICHTER {
			let BELMONT_POST_PARRY = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
			*BELMONT_POST_PARRY = true;	
		}
	}
	if globals["reset_frame"].get_num() != -1.0 {
		if globals["reset_motion"].get_int() == motion_kind {
			if MotionModule::frame(module_accessor) <= 1.0 {
				MotionModule::set_frame(module_accessor, globals["reset_frame"].get_num() + 2.0, true);
			}
			else if MotionModule::frame(module_accessor) > globals["reset_frame"].get_num() {
				globals["reset_frame"] = (-1.0).into();
				globals["reset_motion"] = (hash40("invalid")).into();
			}
		}
		else {
			globals["reset_frame"] = (-1.0).into();
			globals["reset_motion"] = (hash40("invalid")).into();
		}
	}
}

unsafe fn character_specific(module_accessor: &mut app::BattleObjectModuleAccessor, attacker_module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32, fighter_kinetic_energy_motion: &mut app::FighterKineticEnergyMotion, fighter_kinetic_energy_gravity: &mut app::FighterKineticEnergyGravity, globals: &mut L2CValue, fighter: &mut L2CFighterCommon) {
	//Shulk
	
	globals["rewind_timer"] = (globals["rewind_timer"].get_int() as i32 + 1).into();
	if globals["rewind_timer"].get_int() as i32 == 120 {
		globals["rewind_timer"] = 0.into();
	}
	MONADO_TIMER[get_player_number(module_accessor)] = globals["rewind_timer"].get_int() as usize;

	if fighter_kind == *FIGHTER_KIND_SHULK && (status_kind == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_ATTACK || status_kind == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT) {
		let rewound_boma = &mut *sv_battle_object::module_accessor(WorkModule::get_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_LW_TARGET_OBJECT_ID) as u32);
		if app::utility::get_category(rewound_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
			if situation_kind == *SITUATION_KIND_GROUND {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			else {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			}
			EffectModule::kill_status_effect(rewound_boma);
			REWOUND[get_player_number(rewound_boma)] = get_player_number(module_accessor) as i32;
		}
	}

	if sv_information::is_ready_go() == false {
		REWOUND[get_player_number(module_accessor)] = -1;
		REWIND_ENDING[get_player_number(module_accessor)] = false;
		MONADO_MOTION = [[0;9];120];
		MONADO_STATUS = [[0;9];120];
		MONADO_SITUATION = [[0;9];120];
		MONADO_FRAME = [[Vector3f{x: 0.0, y: 0.0, z: 0.0};9];120];
		MONADO_LR = [[Vector2f{x: 0.0, y: 0.0};9];120];
		MONADO_POS = [[Vector2f{x: 0.0, y: 0.0};9];120];
	}
	if REWIND_ENDING[get_player_number(module_accessor)] {
		if MotionModule::frame(module_accessor) >= (FighterMotionModuleImpl::get_cancel_frame(module_accessor, Hash40{hash: motion_kind}, true) - 1.0) 
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_END	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_START
		|| status_kind == *FIGHTER_STATUS_KIND_DEAD {
			REWIND_ENDING[get_player_number(module_accessor)] = false;
		}
	}
	if REWOUND[get_player_number(module_accessor)] == -1 {
		if globals["rewind_start"].get_bool() {
			globals["rewind_start"] = false.into();
			lua_bind::FighterKineticEnergyGravity::set_speed(fighter_kinetic_energy_gravity, globals["prewind_gravity"].get_num());
		}
		let mut is_attack = 0.0;
		if AttackModule::is_attack(module_accessor, 0, false) || GrabModule::is_grab(module_accessor, 0) {
			is_attack = 1.0;
		}
		MONADO_MOTION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = motion_kind;
		MONADO_STATUS[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = status_kind;
		MONADO_SITUATION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = situation_kind;
		MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector3f{x: MotionModule::frame(module_accessor), y: MotionModule::rate(module_accessor), z: is_attack};
		MONADO_LR[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector2f{x: PostureModule::lr(module_accessor), y: imported_rot_y_lr(module_accessor)};
		MONADO_POS[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)};
	}
	else {
		if globals["rewind_start"].get_bool() == false {
			globals["prewind_gravity"] = KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY).into();
			EFFECT_FOLLOW(fighter, Hash40::new("shulk_vision"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
			EffectModule::set_rate_last(module_accessor, 0.5);
			globals["rewind_start"] = true.into();
		}
		KineticModule::sleep(module_accessor, true);
		JostleModule::set_status(module_accessor, false);
		if MotionModule::rate(module_accessor) != 0.0 {
			globals["last_nonhitlag_rate"] = MotionModule::rate(module_accessor).into();
		}
		if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT)
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_BEETLE	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_DRIVER	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_ITEM	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_JUMP	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_NABBIT	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_WAIT	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI	
		|| status_kind == *FIGHTER_STATUS_KIND_CAPTURE_YOSHI
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_GANON	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_REFLET	
		|| status_kind == *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_GANON
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_DIDDY
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY
		|| status_kind == *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY
		|| status_kind == *FIGHTER_STATUS_KIND_THROWN
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_END	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL	
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SONG_START
		|| status_kind == *FIGHTER_STATUS_KIND_DEAD {
			REWOUND[get_player_number(module_accessor)] = -1;
			KineticModule::sleep(module_accessor, false);
			if MotionModule::is_loop_flag(module_accessor) == false {
				REWIND_ENDING[get_player_number(module_accessor)] = true;
			}
			MotionModule::set_rate(module_accessor, globals["last_nonhitlag_rate"].get_num());
		}
		if AttackModule::is_infliction(get_boma(REWOUND[get_player_number(module_accessor)]), *COLLISION_KIND_MASK_HIT) {
			REWOUND[get_player_number(module_accessor)] = -1;
			if MotionModule::is_loop_flag(module_accessor) == false {
				REWIND_ENDING[get_player_number(module_accessor)] = true;
			}
		}
		let motion = MONADO_MOTION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)];
		if motion != 0 {
			StatusModule::set_situation_kind(module_accessor, app::SituationKind(MONADO_SITUATION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)]), false);
			if (motion != motion_kind) || (motion == motion_kind && MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].x < MotionModule::frame(module_accessor)) {
				MotionModule::change_motion(module_accessor, Hash40{hash: motion}, MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].x, MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].y, false, 0.0, false, false);
			}
			MotionModule::set_rate(module_accessor, MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].y);
			MotionModule::set_frame(module_accessor, MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].x, true);
			PostureModule::set_lr(module_accessor, MONADO_LR[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].x);
			if imported_rot_y_lr(module_accessor) != MONADO_LR[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].y {
				PostureModule::reverse_rot_y_lr(module_accessor);
			}
			PostureModule::set_pos_2d(module_accessor, &MONADO_POS[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)]);

			if MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)].z == 0.0 {
				AttackModule::clear_all(module_accessor);
				GrabModule::clear_all(module_accessor);
			}
			MONADO_STATUS[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = 0;
			MONADO_MOTION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = 0;
			MONADO_SITUATION[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = *SITUATION_KIND_NONE;
			MONADO_FRAME[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			MONADO_LR[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector2f{x: 0.0, y: 0.0};
			MONADO_POS[globals["rewind_timer"].get_int() as usize][get_player_number(module_accessor)] = Vector2f{x: 0.0, y: 0.0};
		}
		else {
			JostleModule::set_status(module_accessor, true);
			REWOUND[get_player_number(module_accessor)] = -1;
			if MotionModule::is_loop_flag(module_accessor) == false {
				REWIND_ENDING[get_player_number(module_accessor)] = true;
			}
			KineticModule::sleep(module_accessor, false);
			MotionModule::set_rate(module_accessor, globals["last_nonhitlag_rate"].get_num());
		}
	}
	
	//Inkling

	if INKED[get_player_number(module_accessor)] == 1 {
		if status_kind == FIGHTER_STATUS_KIND_WALK
		|| status_kind == FIGHTER_STATUS_KIND_RUN {
			lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.7);
		}
	}
	else {
		if status_kind == FIGHTER_STATUS_KIND_WALK
		|| status_kind == FIGHTER_STATUS_KIND_RUN {
			if lua_bind::FighterKineticEnergyMotion::get_speed_mul(fighter_kinetic_energy_motion) as f32 == 0.7 
			|| lua_bind::FighterKineticEnergyMotion::get_speed_mul(fighter_kinetic_energy_motion) as f32 == 1.3 {
				lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 1.0);
			}
		}
	}

	//Luigi

	if LUIGI_FINAL_HIT[get_player_number(module_accessor)] {
		EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
		LUIGI_FINAL_HIT[get_player_number(module_accessor)] = false;
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
		let attacker_kind = app::utility::get_kind(attacker_module_accessor);
		let attacker_status_kind = StatusModule::status_kind(attacker_module_accessor);	
		if attacker_kind == *FIGHTER_KIND_LUIGI && attacker_status_kind == *FIGHTER_STATUS_KIND_FINAL && EffectModule::is_exist_common(module_accessor, Hash40{hash: hash40("sys_thunder_flash")}) == false {
			EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
		}
	}

	//Majin

	if MAJIN_DEMON_TARGET[get_attacker_number(module_accessor)] == get_player_number(module_accessor) {
		if MAJIN_GRAB[get_attacker_number(module_accessor)] == 1 {
			if motion_kind != hash40("capture_wait_hi") {
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("capture_wait_hi")}, 0.0, 1.0, true, 0.0, false, false);
			}	
			if PostureModule::pos_x(module_accessor) > PostureModule::pos_x(get_boma(get_attacker_number(module_accessor) as i32)) {
				PostureModule::set_lr(module_accessor, -1.0);
			}
			else {
				PostureModule::set_lr(module_accessor, 1.0);
			}
			PostureModule::update_rot_y_lr(module_accessor);
			if MAJIN_OFFSET[get_attacker_number(module_accessor)].x != 0.0 {
				PostureModule::set_pos(module_accessor, &MAJIN_OFFSET[get_attacker_number(module_accessor)]);
			}
			if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D 
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_BURY {
				MAJIN_GRAB[get_attacker_number(module_accessor)] = 0;
				WorkModule::set_int(module_accessor, 8, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR);
			}
		}
		if MAJIN_GRAB[get_attacker_number(module_accessor)] == 2 {
			if fighter_kind == *FIGHTER_KIND_KOOPAG {
				MAJIN_GRAB[get_attacker_number(module_accessor)] = 0;
			}
			else {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DAMAGE, true);
				MAJIN_OFFSET[get_attacker_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
				MAJIN_GRAB[get_attacker_number(module_accessor)] = 1; 
			}
		}
		if MAJIN_GRAB[get_attacker_number(module_accessor)] == 3 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DAMAGE, true);
			MAJIN_GRAB[get_attacker_number(module_accessor)] = 1;
		}
		if MAJIN_GRAB[get_attacker_number(module_accessor)] == 4 {
			VisibilityModule::set_whole(module_accessor, false);
		}
		if MAJIN_GRAB[get_attacker_number(module_accessor)] == 5 {
			VisibilityModule::set_whole(module_accessor, true);
		}
	}

	//Giga Bowser

	if GIGA_GRAB_TARGET[get_attacker_number(module_accessor)] == get_player_number(module_accessor) {
		if GIGA_GRAB[GIGA_GRAB_TARGET[get_player_number(module_accessor)]] == 1 && motion_kind != hash40("capture_wait_hi") {
			MotionModule::change_motion(module_accessor, Hash40{hash: hash40("capture_wait_hi")}, 0.0, 1.0, true, 0.0, false, false);
		}
		if GIGA_GRAB[get_attacker_number(module_accessor)] == 1 {
			if GrabModule::is_rebound(module_accessor) || fighter_kind == *FIGHTER_KIND_KOOPAG {
				GIGA_GRAB[get_attacker_number(module_accessor)] = 9;
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
			}
			else {
				GIGA_GRABBED[get_attacker_number(module_accessor)] = 90 + ((DamageModule::damage(module_accessor, 0) * 1.7) as i32); 
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			}
		}
		if GIGA_GRABBED[get_attacker_number(module_accessor)] > 0 {
			if PostureModule::pos_x(module_accessor) > PostureModule::pos_x(get_boma(get_attacker_number(module_accessor) as i32)) {
				PostureModule::set_lr(module_accessor, -1.0);
			}
			else {
				PostureModule::set_lr(module_accessor, 1.0);
			}
			PostureModule::update_rot_y_lr(module_accessor);
			if GIGA_OFFSET[get_attacker_number(module_accessor)].x != 0.0 {
				PostureModule::set_pos(module_accessor, &GIGA_OFFSET[get_attacker_number(module_accessor)]);
			}

			if GIGA_GRAB[get_attacker_number(module_accessor)] == 1 || GIGA_GRAB[get_attacker_number(module_accessor)] == 3 {
				GIGA_GRABBED[get_attacker_number(module_accessor)] -= 1;
			}
			if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
			|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
			|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP)
			|| ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
				GIGA_GRABBED[get_attacker_number(module_accessor)] -= 8;
			}
		}
		if GIGA_GRABBED[get_attacker_number(module_accessor)] <= 0 && motion_kind == hash40("capture_wait_hi") && GIGA_GRAB[get_attacker_number(module_accessor)] == 1 {
			GIGA_GRABBED[get_attacker_number(module_accessor)] = 0;
			GIGA_GRAB[get_attacker_number(module_accessor)] = 0;
			GIGA_GRAB_TARGET[get_attacker_number(module_accessor)] = 8;
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			WorkModule::set_int(module_accessor, 8, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR);
		}
		if GIGA_GRAB[get_attacker_number(module_accessor)] == 7 {
			StatusModule::set_situation_kind(module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), true);
		}
		if GIGA_GRAB[get_attacker_number(module_accessor)] > 0 && GIGA_GRAB[get_attacker_number(module_accessor)] != 3 && 
		(status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D 
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
		|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
		|| status_kind == *FIGHTER_STATUS_KIND_BURY) {
			GIGA_GRABBED[get_attacker_number(module_accessor)] = 0;
			if GIGA_GRAB[get_attacker_number(module_accessor)] != 8 {
				GIGA_GRAB[get_attacker_number(module_accessor)] = 0;
			}
			WorkModule::set_int(module_accessor, 8, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR);
		}
	}
}

#[smashline::fighter_frame_callback]
fn custom_fighter_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let attacker_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID)) as u32);
	//	let battle_object = app::sv_system::battle_object(fighter.lua_state_agent);
	//	let _instance = mem::transmute::<&mut app::BattleObject, &mut app::Fighter>(battle_object);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		let fighter_kinetic_energy_gravity = mem::transmute::<u64, &mut app::FighterKineticEnergyGravity>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));
		let mut globals = fighter.globals_mut().clone();
		let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
		let cat2 = fighter.global_table[CMD_CAT2].get_int() as i32;
		let cat4 = fighter.global_table[CMD_CAT4].get_int() as i32;
		let hitlag = fighter.global_table[IN_HITLAG].get_bool();

		LookupSymbol(
			&mut FIGHTER_MANAGER_ADDR,
			"_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
				.as_bytes()
				.as_ptr(),
		);
		LookupSymbol(
			&mut FIGHTER_CUTIN_MANAGER_ADDR,
			"_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
				.as_bytes()
				.as_ptr(),
		);

		let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
		let fighter_information = &mut *lua_bind::FighterManager::get_fighter_information(fighter_manager, app::FighterEntryID(get_player_number(module_accessor) as i32));
		STOCK_COUNT[get_player_number(module_accessor)] = lua_bind::FighterInformation::stock_count(fighter_information);

		if let L2CValueType::Void = globals["globals_set"].val_type {
			globals["parry_lag_set"] = false.into();
			globals["parry_lag"] = (-1.0).into();
			globals["real_parry_lag"] = (-1.0).into();
			globals["reset_frame"] = (-1.0).into();
			globals["reset_motion"] = (hash40("invalid")).into();
			globals["rewound"] = false.into();
			globals["rewind_timer"] = 0.into();
			globals["rewind_start"] = false.into();
			globals["prewind_gravity"] = 0.0.into();
			globals["last_nonhitlag_rate"] = 0.0.into();
			globals["reverse_timer"] = 0.0.into();
			globals["assignment_check"] = false.into();
			globals["turnaround"] = false.into();
			globals["momentum"] = false.into();
			globals["seed_check"] = false.into();
			globals["seed_id"] = 0.into();
			globals["seed_changed"] = false.into();
			globals["crit_ok"] = 0.into();
			globals["microdash_check"] = false.into();
			globals["microdash_success"] = false.into();
			globals["grab_check"] = false.into();
			globals["shield_check"] = false.into();
			globals["globals_set"] = true.into();
			FIGHTER_INT_1[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_2[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_3[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_4[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_5[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_6[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_7[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_8[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_9[get_player_number(module_accessor)] = 0;
			FIGHTER_INT_10[get_player_number(module_accessor)] = 0;

			FIGHTER_FLOAT_1[get_player_number(module_accessor)] = 0.0;
			FIGHTER_FLOAT_2[get_player_number(module_accessor)] = 0.0;
			FIGHTER_FLOAT_3[get_player_number(module_accessor)] = 0.0;

			FIGHTER_U64_1[get_player_number(module_accessor)] = 0;

			FIGHTER_U32_1[get_player_number(module_accessor)] = 0;

			FIGHTER_U8_1[get_player_number(module_accessor)] = 0;
			FIGHTER_U8_2[get_player_number(module_accessor)] = 0;
			FIGHTER_U8_3[get_player_number(module_accessor)] = 0;
			FIGHTER_U8_4[get_player_number(module_accessor)] = 0;

			FIGHTER_BOOL_1[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_2[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_3[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_4[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_5[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_6[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_7[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_8[get_player_number(module_accessor)] = false;
			FIGHTER_BOOL_9[get_player_number(module_accessor)] = false;

			FIGHTER_VEC2F_1[get_player_number(module_accessor)] = Vector2f{x:0.0, y:0.0}; 
			FIGHTER_VEC3F_1[get_player_number(module_accessor)] = Vector3f{x:0.0, y:0.0, z: 0.0};

			START_FS[get_player_number(module_accessor)] = false;
		}

		if app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			if sv_information::is_ready_go() == false {
				GET_CLIPPED_NERD = [false;9];
				let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
				let name_base = text + 0x52c3758;
				FIGHTER_NAME[get_player_number(module_accessor)] = hash40(&read_tag(name_base + 0x260 * get_player_number(module_accessor) as u64 + 0x8e));
				if get_player_number(module_accessor) as i32 == 0 {
					TOTAL_FIGHTER = 1;
					INKLING_PRESENT = false;

				}
				else {
					if fighter_kind != *FIGHTER_KIND_NANA {
						TOTAL_FIGHTER += 1;
					}
				}
				if fighter_kind == FIGHTER_KIND_INKLING {
					INKLING_PRESENT = true;
				}
				METER_ENABLED = false;
				FS_METER[get_player_number(module_accessor)] = 0.0;
				FS_METER_2[get_player_number(module_accessor)] = 0.0;
				FS_METER_3[get_player_number(module_accessor)] = 0.0;
				LAST_DAMAGE[get_player_number(module_accessor)] = 0.0;
				FINAL_TRANSFORM[get_player_number(module_accessor)] = 0;
				DRAIN_FULL_METER[get_player_number(module_accessor)] = false;
				SUB_METER[get_player_number(module_accessor)] = 0.0;
				READY_GO_TIMER = 0;
				if TRAINING_SPAWN == 0 {
					if ControlModule::get_stick_x(&mut *get_boma(0)) < -0.6 {
						TRAINING_SPAWN = 1;
					}
					else if ControlModule::get_stick_x(&mut *get_boma(0)) < 0.6 {
						if ControlModule::get_stick_y(&mut *get_boma(0)) < -0.6 {
							TRAINING_SPAWN = 4;
						}
						else {
							TRAINING_SPAWN = 2;
						}
					}
					else {
						TRAINING_SPAWN = 3;
					}
				}

				if smashball::is_training_mode() && get_player_number(module_accessor) == 1 {
//					CPU_STATE = ai__cp_type(fighter.lua_state_agent);
					INPUT_TIMER = 0;
				}
			}
			else if is_preview_mode() {
				TOTAL_FIGHTER = 1;
			}
			else {
				for i in 0..TOTAL_FIGHTER {
					if STOCK_COUNT[i as usize] > 1 {
						ALL_FIGHTERS_LAST_STOCK = false;
						break;
					}
					else {
						ALL_FIGHTERS_LAST_STOCK = true;
					}
				}
				if TRAINING_SPAWN != 0 {
					if TRAINING_SPAWN != 3 {
						if get_stage_id() == *lua_const::StageID::Training && smashball::is_training_mode() {
							let mut training_pos = Vector2f{x: 0.0, y: 0.0};
							if TRAINING_SPAWN == 1 {
								if get_player_number(module_accessor) == 0 {
									training_pos.x = -500.0;
									training_pos.y = 47.39;
								}
								else if get_player_number(module_accessor) == 1 {
									training_pos.x = -480.0;
								}
								else if get_player_number(module_accessor) == 2 {
									training_pos.x = -455.0;
								}
								else {
									training_pos.x = -430.0;
								}
							}
							else if TRAINING_SPAWN == 2 {
								if get_player_number(module_accessor) == 0 {
									training_pos.x = -292.1055;
									training_pos.y = 24.119;
								}
								else if get_player_number(module_accessor) == 1 {
									training_pos.x = -207.8945;
									training_pos.y = 24.119;
								}
								else if get_player_number(module_accessor) == 2 {
									training_pos.x = -249.92;
									training_pos.y = 47.189;
								}
								else {
									training_pos.x = -249.92;
								}
							}
							else if TRAINING_SPAWN == 4 {
								if get_player_number(module_accessor) == 0 {
									training_pos.x = -292.1055;
								}
								else if get_player_number(module_accessor) == 1 {
									training_pos.x = -207.8945;
								}
								else if get_player_number(module_accessor) == 2 {
									training_pos.x = -249.92;
								}
								else {
									training_pos.x = -249.92;
								}
							}
							let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
							GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
							PostureModule::set_pos_2d(module_accessor, &training_pos);
							GroundModule::set_correct(module_accessor, correct_kind);
						}
					}

					if get_player_number(module_accessor) as i32 == TOTAL_FIGHTER - 1 {
						TRAINING_SPAWN = 0;
					}
				}
			}
			
			let boma = module_accessor as *mut app::BattleObjectModuleAccessor as u64;
			let model_data = *((*((boma + 0x78) as *const u64) + 0x10) as *const u64);
			MODEL_DATA_POS[get_player_number(module_accessor)] = model_data;

			if (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) && fighter_kind != *FIGHTER_KIND_DOLLY) || (fighter_kind == *FIGHTER_KIND_DOLLY && FS_METER[get_player_number(module_accessor)] >= 100.0 && DamageModule::damage(module_accessor, 0) >= 100.0) {
				USED_FS[get_player_number(module_accessor)] = true;
				
				fighter.clear_lua_stack();
				lua_args!(fighter, 0);
				sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(fighter.lua_state_agent);
				fighter.clear_lua_stack();
				if motion_kind != hash40("just_shield_off") {
					ModelModule::disable_gold_eye(module_accessor);	
				}
				sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);

				if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
					let FINAL_SMASH_CANCEL = &mut FIGHTER_BOOL_9[get_player_number(module_accessor)];
					if *FINAL_SMASH_CANCEL && (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) 
					|| AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD)) && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
					|| ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R))  {
						WorkModule::on_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
					}
				}

				if (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
				|| ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) 
				&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && (WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL) ||
				(MotionModule::frame(module_accessor) <= 4.0 && status_kind == *FIGHTER_STATUS_KIND_APPEAL) || (fighter_kind == *FIGHTER_KIND_DOLLY 
					&& WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL))) {
					if CancelModule::is_enable_cancel(module_accessor) {
						lua_bind::FighterManager::set_final(fighter_manager, app::FighterEntryID(get_player_number(module_accessor) as i32), app::FighterAvailableFinal{_address: *lua_const::FighterAvailableFinal::CHARGE as u8}, 0);
					}
					else {
						START_FS[get_player_number(module_accessor)] = true;
						CancelModule::enable_cancel(module_accessor);
					}
				}
				if START_FS[get_player_number(module_accessor)] {
					lua_bind::FighterManager::set_final(fighter_manager, app::FighterEntryID(get_player_number(module_accessor) as i32), app::FighterAvailableFinal{_address: *lua_const::FighterAvailableFinal::CHARGE as u8}, 0);
				}

				if get_meter_gain_damage(module_accessor, fighter_kind) == 0.0 {
					if CAN_DRAIN_METER_KEPT[get_player_number(module_accessor)] != 0 {
						CAN_DRAIN_METER_KEPT[get_player_number(module_accessor)] -= 1;
					}
					else if REMAINING_METER[get_player_number(module_accessor)] != 0 {
						if METER_TIMER[get_player_number(module_accessor)] == 59 {
							REMAINING_METER[get_player_number(module_accessor)] -= 2;
							METER_TIMER[get_player_number(module_accessor)] = 0;
						}
						else {
							METER_TIMER[get_player_number(module_accessor)] += 1;
						}
					}
				}
			}
			else {
				CAN_DRAIN_METER_KEPT[get_player_number(module_accessor)] = 600;
				REMAINING_METER[get_player_number(module_accessor)] = 100;
			}
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				if get_meter_gain_damage(module_accessor, fighter_kind) > 0.75 && FS_METER_3[get_player_number(module_accessor)] != 0.0 && FS_METER_3[get_player_number(module_accessor)] != 200.0 {
					SUB_METER[get_player_number(module_accessor)] = FS_METER_3[get_player_number(module_accessor)] * -1.0;
					FS_METER_3[get_player_number(module_accessor)] = 0.0;
				}
				LAST_DAMAGE[get_player_number(module_accessor)] = 0.0;
				if FINAL_TRANSFORM[get_player_number(module_accessor)] > 0 {
					FINAL_TRANSFORM[get_player_number(module_accessor)] = 1;
				}
			}

			if fighter_kind != *FIGHTER_KIND_BRAVE {
				WorkModule::set_int(module_accessor, 59, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE_FINAL_ADD_GAUGE_COUNTER);
			}
			else if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE_FINAL_ADD_GAUGE_COUNTER) == 0 {
				let SEED = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
				let meter_interval = (((*SEED as f32) / 4.26).floor()) as i32;
				WorkModule::set_int(module_accessor, meter_interval, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE_FINAL_ADD_GAUGE_COUNTER);
			}

			if FS_METER[get_player_number(module_accessor)] > 200.0 {
				FS_METER[get_player_number(module_accessor)] = 200.0;
			}
			if FS_METER[get_player_number(module_accessor)] < 0.0 {
				FS_METER[get_player_number(module_accessor)] = 0.0;
			}
			if FS_METER_2[get_player_number(module_accessor)] > 200.0 {
				FS_METER_2[get_player_number(module_accessor)] = 200.0;
			}
			if FS_METER_2[get_player_number(module_accessor)] < 0.0 {
				FS_METER_2[get_player_number(module_accessor)] = 0.0;
			}
			if FS_METER_3[get_player_number(module_accessor)] > 200.0 {
				FS_METER_3[get_player_number(module_accessor)] = 200.0;
			}
			if FS_METER_3[get_player_number(module_accessor)] < 0.0 {
				FS_METER_3[get_player_number(module_accessor)] = 0.0;
			}
			if DEMON_DEATH_TIMER[get_player_number(module_accessor)] > 0 {
				DEMON_DEATH_TIMER[get_player_number(module_accessor)] -= 1;
				if DEMON_DEATH_TIMER[get_player_number(module_accessor)] == 1 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
				}
			}
			if status_kind == *FIGHTER_STATUS_KIND_THROWN && LinkModule::is_link(module_accessor, *LINK_NO_CAPTURE) { //Check if we're still linked to an opponent during the throw
				let parent_boma = &mut *sv_battle_object::module_accessor(LinkModule::get_parent_object_id(module_accessor, *LINK_NO_CAPTURE) as u32); //If it's safe, this SHOULD get the boma for the player that threw us
				let parent_kind = app::utility::get_kind(parent_boma);
				let parent_status_kind = StatusModule::status_kind(parent_boma);

				if parent_kind == *FIGHTER_KIND_MIIFIGHTER && parent_status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW { 
					//Forcing the opponent into the counter thrown status wasn't working, and since I can't print the correct status at the moment, just forcing the regular thrown status 
					//with the correct animation should work
					let mut throw_motion_kind = hash40("miifighter_thrown");
					let mut body_type = 0;
					let motion_share_type = WorkModule::get_param_int64(module_accessor, hash40("param_motion"), hash40("motion_share"));
					if motion_share_type == hash40("motion_share_type_taro") {
						body_type = *BODY_TYPE_MOTION_DX;
					}
					else if motion_share_type == hash40("motion_share_type_big") {
						body_type = *BODY_TYPE_MOTION_BIG;
					}
					else if motion_share_type == hash40("motion_share_type_girl") {
						body_type = *BODY_TYPE_MOTION_GIRL;
					}

					if body_type != 0 {
						throw_motion_kind = FighterMotionModuleImpl::add_body_type_hash(module_accessor, Hash40::new_raw(throw_motion_kind), body_type);
					}
					if motion_kind != throw_motion_kind {
						MotionModule::change_motion(module_accessor, Hash40::new_raw(throw_motion_kind), 0.0, 1.0, false, 0.0, false, false);
					}
				}
			}

			if get_player_number(module_accessor) == 0 {
				if INPUT_TIMER == 420 {
					INPUT_TIMER = 0;
				}
				else {
					INPUT_TIMER += 1;
				}
				if FRAME_COUNTER == 59 {
					FRAME_COUNTER = 0;
				}
				else {
					FRAME_COUNTER+=1;
				}
			}

			set_cstick(module_accessor);
			parry(module_accessor, attacker_module_accessor, motion_kind, status_kind, &mut globals);
			directional_airdodge(module_accessor, fighter_kind, motion_kind, fighter);
			grab_break(module_accessor, fighter_kind, motion_kind);
			shorthop_buffer(module_accessor, status_kind, situation_kind, cat);
			landing_lag(module_accessor, status_kind, situation_kind);

			dash_drop(module_accessor, status_kind);
			if !is_majin(module_accessor, fighter_kind) {
				microdash(module_accessor, status_kind, &mut globals);
			}

			pivot_crossup(module_accessor, motion_kind, &mut globals);

			pogo(module_accessor, fighter_kind, motion_kind, cat);
			b_reverse(module_accessor, fighter_kind, motion_kind, &mut globals);
			air_specials(module_accessor, fighter_kind, motion_kind, status_kind, situation_kind);

			special_ce_mode(module_accessor, fighter_kind, status_kind, fighter, fighter_information);
			background_effects(module_accessor, fighter);
			jab_cancel(module_accessor, fighter_kind, motion_kind, status_kind);
			counterhit(module_accessor, status_kind);
			status_kind_damage(module_accessor, status_kind, situation_kind, cat2, &mut globals);

			shield(module_accessor, status_kind, cat, cat2);
			shield_release(module_accessor, status_kind, &mut globals);

			character_specific(module_accessor, attacker_module_accessor, fighter_kind, motion_kind, status_kind, situation_kind, fighter_kinetic_energy_motion, fighter_kinetic_energy_gravity, &mut globals, fighter);
			if !is_majin(module_accessor, fighter_kind) {
				READY_GO[get_player_number(module_accessor)] = sv_information::is_ready_go(); //For Majin specifically, we update READY_GO in his opff
			}
		}
	}
}

/*fn custom_article_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let battle_object = sv_system::battle_object(fighter.lua_state_agent);
		let _instance = mem::transmute::<&mut app::BattleObject, &mut app::Weapon>(battle_object);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let _owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);
		let _status_kind = StatusModule::status_kind(module_accessor);
		let _owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let _situation_kind = StatusModule::situation_kind(module_accessor);
		let _owner_situation_kind = StatusModule::situation_kind(owner_module_accessor);
		let weapon_kind = utility::get_kind(module_accessor) as i32;
		let mut _globals = fighter.globals_mut().clone();
	}
}*/


pub fn install() {
	smashline::install_agent_frame_callbacks!(custom_fighter_functions);
	//smash_script::add_weapon_frame_callbacks!(custom_article_functions);
}