//region: lmake_readme insert "readme.md"
//! **mem5_common - commons for mem5 wasm and server**
//!
//! version: 19.9.9  
//! Look also at the workspace readme https://github.com/LucianoBestia/mem5_game  
//!
//! ## mem5_common
//! Learning to code Rust for a http + WebSocket.  
//! Here are just the structures, that are in common between frontend and backend.  
//!
//!
//!
//!

//endregion: lmake_readme insert "readme.md"

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target no-modules returns an error: export `run` not found 
    clippy::missing_inline_in_public_items,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
//endregion

//region: use statements
use strum_macros::{Display, AsRefStr};
use serde_derive::{Serialize, Deserialize};
//endregion

///`WsMessage` enum for WebSocket
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize)]
pub enum WsMessage {
    ///MsgDummy
    MsgDummy {
        ///anything
        dummy: String,
    },
    ///Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        ///anything
        test: String,
    },
    ///response from WebSocket server for first message
    MsgResponseWsUid {
        ///WebSocket Uid
        your_ws_uid: usize,
        ///server version
        server_version: String,
    },
    ///invite
    MsgInvite {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// my nickname
        my_nickname: String,
        ///content folder name
        asked_folder_name: String,
    },
    /// accept play
    MsgPlayAccept {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players for the server to know whom to send msg
        players_ws_uid: String,
        /// my nickname
        my_nickname: String,
    },
    /// player1 initialize the game data and sends it to all players
    /// I will send json string to not confuse the server with vectors
    MsgGameDataInit {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players for the server to know whom to send msg
        players_ws_uid: String,
        ///json of vector of players with nicknames and order data
        players: String,
        ///vector of cards status
        card_grid_data: String,
        ///json of game_config
        game_config: String,
    },
    ///player click
    MsgPlayerClick1stCard {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_first_click: usize,
    },
    ///player click success
    MsgPlayerClick2ndCardPoint {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_second_click: usize,
    },
    ///take turn begin
    MsgPlayerClick2ndCardTakeTurnBegin {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_second_click: usize,
    },
    ///Play Again
    MsgPlayerClick2ndCardGameOverPlayAgainBegin {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
    },
    ///player change
    MsgTakeTurnEnd {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
    },
}

///the game can be in various statuses and that differentiate the UI and actions
/// all players have the same game status
#[derive(Display, AsRefStr, Serialize, Deserialize, Clone)]
#[allow(clippy::pub_enum_variant_names)]
pub enum GameStatus {
    /// invite ask begin
    StatusInviteAskBegin,
    ///Player1 MsgInvite Asking
    StatusInviteAsking,
    ///Player2 MsgInvite Asked
    StatusInviteAsked,
    ///StatusPlayAccepted
    StatusPlayAccepted,
    ///Play before first card
    StatusPlayBefore1stCard,
    ///Play before second card
    StatusPlayBefore2ndCard,
    ///take turn begin
    StatusTakeTurnBegin,
    ///take turn end
    StatusTakeTurnEnd,
    ///end game
    StatusGameOverPlayAgainBegin,
    ///StatusReconnect after a lost connection
    StatusReconnect,
}

///data for one player
#[derive(Serialize, Deserialize)]
pub struct Player {
    ///ws_uid
    pub ws_uid: usize,
    ///nickname
    pub nickname: String,
    ///field for src attribute for HTML element image and filename of card image
    pub points: usize,
}
//endregion
