// https://github.com/paritytech/cargo-contract/issues/1130
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink_lang as ink;

// https://use.ink/ink-vs-solidity
#[ink::contract]
mod flipper_game {
    use ink::storage::{
        // https://use.ink/ink-vs-solidity/#mapping-declaration
        traits::SpreadAllocate,
        Mapping,
    };
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::primitives::Hash;

    // https://use.ink/ink-vs-solidity/#errors-and-returning
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Provide a detailed comment on the error
        RevertError,
    }
    
    // result type
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct CreatedGame {
        #[ink(topic)]
        game_id: u128,
        block_hash_previous: Hash,
        #[ink(topic)]
        created_by: AccountId,
        #[ink(topic)]
        created_at_block: BlockNumber,
        end_guesses_at_block: BlockNumber,
    }

    #[ink(event)]
    pub struct AddedPlayerToGame {
        #[ink(topic)]
        game_id: u128,
        #[ink(topic)]
        player_address: AccountId,
        #[ink(topic)]
        created_at_block: BlockNumber,
    }

    #[ink(event)]
    pub struct AddedGuessForPlayerOfGame {
        #[ink(topic)]
        game_id: u128,
        #[ink(topic)]
        player_guess_id: u128,
        #[ink(topic)]
        guess_by_player_address: AccountId,
        guess: u128,
        created_at_block: BlockNumber,
    }

    #[ink(event)]
    pub struct AddedAnswerForGame {
        #[ink(topic)]
        game_id: u128,
        #[ink(topic)]
        answer_id: u128,
        answer_by_address: AccountId,
        answer: u128,
        #[ink(topic)]
        created_at_block: BlockNumber,
    }

    #[ink::storage_item]
    pub struct GameStruct {
        // only u128 allowed in ink!
        // https://use.ink/ink-vs-solidity#uint256
        id: u128,
        start_game_at_block_hash: Hash,
        created_by: AccountId,
        created_at_block: BlockNumber,
        end_guesses_at_block: BlockNumber,
    }

    #[ink::storage_item]
    pub struct PlayerStruct {
        player_address: AccountId,
        game_id: u128,
        created_at_block: BlockNumber,
    }

    #[ink::storage_item]
    pub struct PlayerGuessStruct {
        id: u128,
        game_id: u128,
        guess_by_player_address: AccountId,
        guess: u128,
        created_at_block: BlockNumber,
    }

    #[ink::storage_item]
    pub struct RequestBlockStruct {
        requested_randomness_at_block: BlockNumber,
        requested_fulfill_at_block: BlockNumber,
    }

    #[ink::storage_item]
    pub struct AnswerStruct {
        id: u128,
        game_id: u128,
        answer: u128,
        created_at_block: BlockNumber,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FlipperGame {
        BLOCKS_ALLOW_GUESS: BlockNumber,
        BLOCKS_ALLOW_RANDOMNESS: BlockNumber,
        BLOCKS_ALLOW_FULFILL: BlockNumber,
        block_number: BlockNumber,
        // block_hash_now: Hash,
        block_hash_previous: Hash,
        has_called_fallback_fn: bool,
        s_owner: AccountId,
        flipper_game_random_number_contract_address: AccountId,
        game_id: u128,
        player_guess_id: u128,
        answer_id: u128,
        games_list: Vec<GameStruct>,
        // TODO - consider replacing `Mapping` with another storage type supported by ink!
        // https://use.ink/datastructures/mapping/#considerations-when-using-the-mapping-type
        // https://docs.rs/ink/latest/ink/storage/struct.Mapping.html
        // https://docs.rs/ink/4.2.1/ink/
        player_balance: Mapping<AccountId, Balance>,
        game_for_game_id: Mapping<u128, GameStruct>,
        players_for_game_id: Mapping<u128, Vec<PlayerStruct>>,
        game_ids_for_player_id: Mapping<AccountId, Vec<u128>>,
        player_guesses_for_player_of_game_id: Mapping<u128, Vec<PlayerGuessStruct>>,
        requested_at_block_for_game_id: Mapping<u128, Vec<RequestBlockStruct>>,
        answer_for_game_id: Mapping<u128, Vec<AnswerStruct>>,
    }

    impl FlipperGame {
        // https://substrate.stackexchange.com/questions/3711/ink-mapping-default-initialization
        #[ink(constructor)]
        pub fn default() -> Self {
            // required for mappings
            ink_lang::utils::initialize_contract(|contract| {/*...*/})

            Self {
                BLOCKS_ALLOW_GUESS: 100,
                BLOCKS_ALLOW_RANDOMNESS: 12, // 70 sec @ 6 sec/block
                BLOCKS_ALLOW_FULFILL: 7, // 40 sec @ 6 sec/block
                block_number: Self::env().block_number(),
                block_hash_now: Default::default(),
                block_hash_previous: Default::default(),
                has_called_fallback_fn: false,
                s_owner: Self::env().caller(),
                flipper_game_random_number_contract_address: Default::default(),
                game_id: 0,
                player_guess_id: 0,
                answer_id: 0,
                games_list: Vec::new(),
                player_balance: Mapping::new(),
                game_for_game_id: Mapping::new(),
                players_for_game_id: Mapping::new(),
                game_ids_for_player_id: Mapping::new(),
                player_guesses_for_player_of_game_id: Mapping::new(),
                requested_at_block_for_game_id: Mapping::new(),
                answer_for_game_id: Mapping::new(),
            }
        }

        // #[ink(constructor, payable)]
        #[ink(constructor)]
        pub fn new() -> Self {
            // required for mappings
            ink_lang::utils::initialize_contract(|contract| {/*...*/})

            Self {
                BLOCKS_ALLOW_GUESS: 100,
                BLOCKS_ALLOW_RANDOMNESS: 12, // 70 sec @ 6 sec/block
                BLOCKS_ALLOW_FULFILL: 7, // 40 sec @ 6 sec/block
                block_number: Self::env().block_number(),
                // TODO - how to get the blockhash using ink! see related TODO further below
                // block_hash_now: ???,
                // TODO - how to get the blockhash using ink! see related TODO further below
                // block_hash_previous: ???,
                has_called_fallback_fn: false,
                s_owner: Self::env().caller(),
                game_id: 0,
                player_guess_id: 0,
                answer_id: 0,
                games_list: Vec::new(),
                player_balance: Mapping::new(),
                game_for_game_id: Mapping::new(),
                players_for_game_id: Mapping::new(),
                game_ids_for_player_id: Mapping::new(),
                player_guesses_for_player_of_game_id: Mapping::new(),
                requested_at_block_for_game_id: Mapping::new(),
                answer_for_game_id: Mapping::new(),
            }
        }

        #[ink(message, payable)]
        pub fn create_game(&mut self, _initial_guess: u128) -> Result<()> {
            assert!(_initial_guess <= 20, "initial guess too high");

            self.block_number = self.env().block_number();
            // TODO - how to get the blockhash using ink! as below is how to do it in Solidity
            // https://substrate.stackexchange.com/questions/2993/inkhow-to-get-the-latest-confirmed-block-hash
            // block_hash_previous = blockhash(block_number - 1),
            self.game_id += 1;

            let game_instance = GameStruct {
                id: game_id,
                start_game_at_block_hash: self.block_hash_previous,
                created_by: self.env().caller(),
                created_at_block: self.env().block_number(),
                end_guesses_at_block: self.env().block_number() + self.BLOCKS_ALLOW_GUESS
            };
            self.game_for_game_id.insert(&game_id, &game_instance);
            let latest = self.game_for_game_id.get(&game_id).unwrap();
            ink::env::debug_print!("flipper_game updated game_for_game_id for game_id {} is {:#?}\n", &game_id, &latest);
            self.games_list.push(game_instance);

            // emit event
            self.env().emit_event(CreatedGame {
                game_id: game_id,
                block_hash_previous: self.block_hash_previous,
                created_by: self.env().caller(),
                created_at_block: self.env().block_number(),
                end_guesses_at_block: self.env().block_number() + self.BLOCKS_ALLOW_GUESS,
            });

            Ok(())
        }

        #[ink(message, payable)]
        pub fn add_player_to_game(&mut self, _game_id: u128, _initial_guess: u128) -> Result<()> {
            self.block_number = self.env().block_number();

            assert!(self.has_player_for_game_id(_game_id).is_ok(), "only one instance of same player address per game");
            assert!(self.env().block_number() >= self.game_for_game_id.get(&_game_id).unwrap().created_at_block);
            assert!(self.env().block_number() <= self.game_for_game_id.get(&_game_id).unwrap().end_guesses_at_block,
                "add player to game only allowed before block number when guesses end");

            let player_instance = PlayerStruct {
                player_address: self.env().caller(),
                game_id: _game_id,
                created_at_block: self.env().block_number(),
            };

            let mut new_players_for_game_id = self.players_for_game_id.get(&_game_id).unwrap();
            new_players_for_game_id.push(player_instance);
            self.players_for_game_id.insert(&_game_id, &new_players_for_game_id);

            self.add_guess_for_player_of_game(&_game_id, &_initial_guess);
    
            // emit event
            self.env().emit_event(AddedPlayerToGame {
                game_id: _game_id,
                player_address: self.env().caller(),
                created_at_block: self.env().block_number(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn has_player_for_game_id(&mut self, _game_id: u128, _initial_guess: u128) -> Result<()> {
            for (i = 0; i < self.players_for_game_id.get(&_game_id).unwrap().length; i++) {
                if (self.players_for_game_id.get(&_game_id).unwrap()[i].player_address == self.env().caller()) {
                    return Err(Error::RevertError);
                }
            }

            Ok(())
        }

        #[ink(message)]
        pub fn has_player_guessed_for_game_id(&mut self, _game_id: u128) -> Result<()> {
            for (i = 0; i < self.game_ids_for_player_id.get(&self.env().caller()).unwrap().length; i++) {
                if (self.game_ids_for_player_id.get(&self.env().caller()).unwrap()[i] == _game_id) {
                    return Err(Error::RevertError);
                }
            }

            Ok(())
        }

        #[ink(message)]
        fn add_guess_for_player_of_game(&mut self, _game_id: u128, _guess: u128) -> Result<u128> {
            assert!(_guess <= 20, "guess too high");
            assert!(has_player_guessed_for_game_id(_game_id).is_ok(), "only one guess per player per game");
            self.block_number = self.env().block_number();
            assert!(self.env().block_number() >= self.game_for_game_id.get(&_game_id).unwrap().created_at_block);
            assert!(self.env().block_number() <= self.game_for_game_id.get(&_game_id).unwrap().end_guesses_at_block,
                "guesses only allowed before block number when guesses end");
            self.player_guess_id += 1;

            let player_guess_instance = PlayerGuessStruct {
                id: player_guess_id,
                game_id: _game_id,
                guess_by_player_address: self.env().caller(),
                guess: _guess,
                created_at_block: self.env().block_number(),
            };

            playerGuessesForPlayerOfGameId[_gameId].push(playerGuessInstance);
            gameIdsForPlayerId[msg.sender].push(_gameId);

            let mut new_player_guesses_for_player_of_game_id = self.player_guesses_for_player_of_game_id.get(&_game_id).unwrap();
            new_player_guesses_for_player_of_game_id.push(player_guess_instance);
            self.player_guesses_for_player_of_game_id.insert(&_game_id, &new_player_guesses_for_player_of_game_id);
            let mut new_game_ids_for_player_id = self.game_ids_for_player_id.get(self.env().caller());
            new_game_ids_for_player_id.push(_game_id);
            self.game_ids_for_player_id.insert(&self.env().caller(), &new_game_ids_for_player_id);

            // emit event
            self.env().emit_event(AddedGuessForPlayerOfGame {
                game_id: _game_id,
                player_guess_id: player_guess_id,
                guess_by_player_address: self.env().caller(),
                guess: _guess,
                created_at_block: self.env().block_number(),
            });

            Ok(_guess)
        }

        #[ink(message, payable)]
        fn request_randomness_answer_of_game(&mut self, _game_id: u128) -> Result<()> {
            // TODO - replace this Solidity with ink!
            // FlipperGameRandomNumber instanceFlipperGameRandomNumber =
            //     FlipperGameRandomNumber(flipperGameRandomNumberContractAddress);
            // let min_fee: u128 = instanceFlipperGameRandomNumber.MIN_FEE();

            // Make sure that the value sent is enough
            assert!(self.env().transferred_value() >= min_fee, "Insufficient fulfillment fee");
            ink::env::debug_print!("flipper_game contract balance for _game_id {} is {:#?}\n", _game_id, self.env().balance());
            
            self.block_number = self.env().block_number();
            assert!(self.env().block_number() >= self.game_for_game_id.get(&_game_id).unwrap().created_at_block);
            // TODO - enable only in production
            // require(self.env().block_number() > self.game_for_game_id.get(&_game_id).unwrap().end_guesses_at_block,
            //     "answer only allowed after block number when guesses end");
    
            let mut new_requested_at_block_for_game_id = self.requested_at_block_for_game_id.get(&_game_id).unwrap();
            new_requested_at_block_for_game_id.requested_randomness_at_block = self.env().block_number();
            self.requested_at_block_for_game_id.insert(&_game_id, &new_requested_at_block_for_game_id);

            // TODO - replace this Solidity with ink!
            // instanceFlipperGameRandomNumber.requestRandomness{value: self.env().transferred_value()}(address(this), _game_id);

            Ok(())
        }

        #[ink(message, payable)]
        fn request_fulfill_answer_of_game(&mut self, _game_id: u128) -> Result<()> {
            self.block_number = self.env().block_number();
            assert!(self.env().block_number() >= self.game_for_game_id.get(&_game_id).unwrap().created_at_block);

            // TODO - enable only in production
            // require(self.env().block_number() > self.game_for_game_id.get(&_game_id).unwrap().end_guesses_at_block,
            //     "answer only allowed after block number when guesses end");
            // TODO - enable only in production
            // require(self.env().block_number() >= self.requested_at_block_for_game_id.get(&_game_id).unwrap().requested_randomness_at_block + self.BLOCKS_ALLOW_RANDOMNESS,
            //     "request fulfill only after waiting sufficient blocks after requesting randomness");

            let mut new_requested_at_block_for_game_id = self.requested_at_block_for_game_id.get(&_game_id).unwrap();
            new_requested_at_block_for_game_id.requested_fulfill_at_block = self.env().block_number();
            self.requested_at_block_for_game_id.insert(&_game_id, &new_requested_at_block_for_game_id);

            // TODO - replace this Solidity with ink!
            // FlipperGameRandomNumber instanceFlipperGameRandomNumber =
            //     FlipperGameRandomNumber(flipperGameRandomNumberContractAddress);
            // instanceFlipperGameRandomNumber.fulfillRequest();

            Ok(())
        }

        #[ink(message)]
        fn fetch_and_add_answer_to_game(&mut self, _game_id: u128) -> Result<()> {
            self.block_number = self.env().block_number();
            assert!(self.env().block_number() >= self.game_for_game_id.get(&_game_id).unwrap().created_at_block);
            // TODO - enable only in production
            // require(self.env().block_number() > self.game_for_game_id.get(&_game_id).unwrap().end_guesses_at_block,
            //     "answer only allowed after block number when guesses end");
            // TODO - enable only in production
            // require(self.env().block_number() >= self.requested_at_block_for_game_id.get(&_game_id).unwrap().requested_fulfill_at_block + self.BLOCKS_ALLOW_FULFILL,
            //     "fetch answer only after waiting sufficient blocks to fulfill");
    
            // TODO - replace this Solidity with ink!
            // FlipperGameRandomNumber instanceFlipperGameRandomNumber =
            //     FlipperGameRandomNumber(flipperGameRandomNumberContractAddress);
            // uint256 answer = instanceFlipperGameRandomNumber.getFlippedValueForGameId(_gameId);
    
            assert!(answer <= 20, "answer not within expected range of values");
    
            // emit event
            self.env().emit_event(AddedAnswerForGame {
                game_id: _game_id,
                answer_id: self.answer_id,
                answer_by_address: self.env().caller(),
                answer: answer,
                created_at_block: self.env().block_number(),
            });

            Ok(())
        }

        #[ink(message)]
        fn set_flipper_game_random_number_contract_address(&mut self, _flipper_game_random_number_contract_address: AccountId) -> Result<()> {
            assert!(self.s_owner == self.env().caller()); // similar to onlyOwner in Solidity
            self.flipper_game_random_number_contract_address = _flipper_game_random_number_contract_address;

            Ok(())
        }

        // Fallback function 
        // https://use.ink/macros-attributes/selector#examples
        #[ink(message, payable, selector = _)]
        fn fallback(&self) -> Result<()> {
            self.has_called_fallback_fn = true;
            player_balance[self.env().caller()] += self.env().transferred_value();

            Ok(())
        }
    }

    // TODO - add unit tests
}
